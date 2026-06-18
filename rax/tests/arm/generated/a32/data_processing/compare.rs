//! A32 data_processing compare tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers_32::*;

// ============================================================================
// aarch32_TST_rr_A Tests
// ============================================================================

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_tst_rr_a1_a_field_cond_0_min_10_01100010() {
    // Encoding: 0x01100010
    // Test aarch32_TST_rr_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rs=0, type1=0, Rm=0, Rn=0
    let encoding: u32 = 0x01100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_tst_rr_a1_a_field_cond_1_poweroftwo_10_11100010() {
    // Encoding: 0x11100010
    // Test aarch32_TST_rr_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, type1=0, Rn=0, cond=1, Rs=0
    let encoding: u32 = 0x11100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_tst_rr_a1_a_field_cond_2_poweroftwo_10_21100010() {
    // Encoding: 0x21100010
    // Test aarch32_TST_rr_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rs=0, cond=2, Rn=0, Rm=0
    let encoding: u32 = 0x21100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_tst_rr_a1_a_field_cond_3_poweroftwo_10_31100010() {
    // Encoding: 0x31100010
    // Test aarch32_TST_rr_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rs=0, cond=3, Rn=0, type1=0, Rm=0
    let encoding: u32 = 0x31100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_tst_rr_a1_a_field_cond_4_poweroftwo_10_41100010() {
    // Encoding: 0x41100010
    // Test aarch32_TST_rr_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rs=0, type1=0, cond=4
    let encoding: u32 = 0x41100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_tst_rr_a1_a_field_cond_5_poweroftwo_10_51100010() {
    // Encoding: 0x51100010
    // Test aarch32_TST_rr_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rs=0, cond=5, Rn=0, type1=0
    let encoding: u32 = 0x51100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_tst_rr_a1_a_field_cond_6_poweroftwo_10_61100010() {
    // Encoding: 0x61100010
    // Test aarch32_TST_rr_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=6, Rs=0, type1=0, Rm=0
    let encoding: u32 = 0x61100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_tst_rr_a1_a_field_cond_7_poweroftwo_10_71100010() {
    // Encoding: 0x71100010
    // Test aarch32_TST_rr_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=7, Rn=0, Rs=0, type1=0, Rm=0
    let encoding: u32 = 0x71100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_tst_rr_a1_a_field_cond_8_poweroftwo_10_81100010() {
    // Encoding: 0x81100010
    // Test aarch32_TST_rr_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rm=0, cond=8, Rn=0, Rs=0
    let encoding: u32 = 0x81100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_tst_rr_a1_a_field_cond_9_poweroftwo_10_91100010() {
    // Encoding: 0x91100010
    // Test aarch32_TST_rr_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, cond=9, Rn=0, Rs=0, Rm=0
    let encoding: u32 = 0x91100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_tst_rr_a1_a_field_cond_10_poweroftwo_10_a1100010() {
    // Encoding: 0xA1100010
    // Test aarch32_TST_rr_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, type1=0, Rn=0, Rm=0, Rs=0
    let encoding: u32 = 0xA1100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_tst_rr_a1_a_field_cond_11_poweroftwo_10_b1100010() {
    // Encoding: 0xB1100010
    // Test aarch32_TST_rr_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rs=0, cond=11, Rm=0, type1=0, Rn=0
    let encoding: u32 = 0xB1100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_tst_rr_a1_a_field_cond_12_poweroftwo_10_c1100010() {
    // Encoding: 0xC1100010
    // Test aarch32_TST_rr_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rm=0, Rs=0, Rn=0, cond=12
    let encoding: u32 = 0xC1100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_tst_rr_a1_a_field_cond_13_poweroftwo_10_d1100010() {
    // Encoding: 0xD1100010
    // Test aarch32_TST_rr_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rs=0, Rm=0, Rn=0, type1=0, cond=13
    let encoding: u32 = 0xD1100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_tst_rr_a1_a_field_cond_14_poweroftwo_10_e1100010() {
    // Encoding: 0xE1100010
    // Test aarch32_TST_rr_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, type1=0, Rs=0, Rn=0, Rm=0
    let encoding: u32 = 0xE1100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_tst_rr_a1_a_field_cond_15_max_10_f1100010() {
    // Encoding: 0xF1100010
    // Test aarch32_TST_rr_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, type1=0, Rm=0, Rn=0, Rs=0
    let encoding: u32 = 0xF1100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_tst_rr_a1_a_field_rn_0_min_10_01100010() {
    // Encoding: 0x01100010
    // Test aarch32_TST_rr_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rs=0, Rn=0, type1=0, cond=0, Rm=0
    let encoding: u32 = 0x01100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_tst_rr_a1_a_field_rn_1_poweroftwo_10_01110010() {
    // Encoding: 0x01110010
    // Test aarch32_TST_rr_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=1, type1=0, cond=0, Rm=0, Rs=0
    let encoding: u32 = 0x01110010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field Rs 8 +: 4`
/// Requirement: FieldBoundary { field: "Rs", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_tst_rr_a1_a_field_rs_0_min_10_01100010() {
    // Encoding: 0x01100010
    // Test aarch32_TST_rr_A1_A field Rs = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rm=0, type1=0, Rs=0, cond=0
    let encoding: u32 = 0x01100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field Rs 8 +: 4`
/// Requirement: FieldBoundary { field: "Rs", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_tst_rr_a1_a_field_rs_1_poweroftwo_10_01100110() {
    // Encoding: 0x01100110
    // Test aarch32_TST_rr_A1_A field Rs = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rs=1, type1=0
    let encoding: u32 = 0x01100110;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_tst_rr_a1_a_field_type1_0_min_10_01100010() {
    // Encoding: 0x01100010
    // Test aarch32_TST_rr_A1_A field type1 = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, type1=0, Rn=0, cond=0, Rs=0
    let encoding: u32 = 0x01100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_tst_rr_a1_a_field_type1_1_poweroftwo_10_01100030() {
    // Encoding: 0x01100030
    // Test aarch32_TST_rr_A1_A field type1 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, type1=1, Rm=0, Rs=0, cond=0
    let encoding: u32 = 0x01100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_tst_rr_a1_a_field_type1_3_max_10_01100070() {
    // Encoding: 0x01100070
    // Test aarch32_TST_rr_A1_A field type1 = 3 (Max)
    // ISET: A32
    // Fields: Rs=0, Rn=0, Rm=0, cond=0, type1=3
    let encoding: u32 = 0x01100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_tst_rr_a1_a_field_rm_0_min_10_01100010() {
    // Encoding: 0x01100010
    // Test aarch32_TST_rr_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rs=0, type1=0, Rm=0, Rn=0
    let encoding: u32 = 0x01100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_tst_rr_a1_a_field_rm_1_poweroftwo_10_01100011() {
    // Encoding: 0x01100011
    // Test aarch32_TST_rr_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, type1=0, Rn=0, Rs=0, Rm=1
    let encoding: u32 = 0x01100011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_tst_rr_a1_a_combo_0_10_01100010() {
    // Encoding: 0x01100010
    // Test aarch32_TST_rr_A1_A field combination: cond=0, Rn=0, Rs=0, type1=0, Rm=0
    // ISET: A32
    // Fields: Rn=0, cond=0, Rm=0, Rs=0, type1=0
    let encoding: u32 = 0x01100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_tst_rr_a1_a_special_cond_0_condition_eq_16_01100010() {
    // Encoding: 0x01100010
    // Test aarch32_TST_rr_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rs=0, Rn=0, Rm=0, type1=0, cond=0
    let encoding: u32 = 0x01100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_tst_rr_a1_a_special_cond_1_condition_ne_16_11100010() {
    // Encoding: 0x11100010
    // Test aarch32_TST_rr_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, Rn=0, Rs=0, Rm=0, type1=0
    let encoding: u32 = 0x11100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_tst_rr_a1_a_special_cond_2_condition_cs_hs_16_21100010() {
    // Encoding: 0x21100010
    // Test aarch32_TST_rr_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=2, type1=0, Rs=0
    let encoding: u32 = 0x21100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_tst_rr_a1_a_special_cond_3_condition_cc_lo_16_31100010() {
    // Encoding: 0x31100010
    // Test aarch32_TST_rr_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rm=0, Rn=0, type1=0, cond=3, Rs=0
    let encoding: u32 = 0x31100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_tst_rr_a1_a_special_cond_4_condition_mi_16_41100010() {
    // Encoding: 0x41100010
    // Test aarch32_TST_rr_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rn=0, Rs=0, type1=0, Rm=0, cond=4
    let encoding: u32 = 0x41100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_tst_rr_a1_a_special_cond_5_condition_pl_16_51100010() {
    // Encoding: 0x51100010
    // Test aarch32_TST_rr_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rm=0, Rs=0, Rn=0, cond=5, type1=0
    let encoding: u32 = 0x51100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_tst_rr_a1_a_special_cond_6_condition_vs_16_61100010() {
    // Encoding: 0x61100010
    // Test aarch32_TST_rr_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, Rm=0, type1=0, Rn=0, Rs=0
    let encoding: u32 = 0x61100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_tst_rr_a1_a_special_cond_7_condition_vc_16_71100010() {
    // Encoding: 0x71100010
    // Test aarch32_TST_rr_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=7, Rs=0, type1=0
    let encoding: u32 = 0x71100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_tst_rr_a1_a_special_cond_8_condition_hi_16_81100010() {
    // Encoding: 0x81100010
    // Test aarch32_TST_rr_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rs=0, type1=0, cond=8, Rn=0, Rm=0
    let encoding: u32 = 0x81100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_tst_rr_a1_a_special_cond_9_condition_ls_16_91100010() {
    // Encoding: 0x91100010
    // Test aarch32_TST_rr_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, Rm=0, Rn=0, type1=0, Rs=0
    let encoding: u32 = 0x91100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_tst_rr_a1_a_special_cond_10_condition_ge_16_a1100010() {
    // Encoding: 0xA1100010
    // Test aarch32_TST_rr_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rn=0, Rs=0, cond=10, Rm=0, type1=0
    let encoding: u32 = 0xA1100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_tst_rr_a1_a_special_cond_11_condition_lt_16_b1100010() {
    // Encoding: 0xB1100010
    // Test aarch32_TST_rr_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rn=0, Rm=0, type1=0, cond=11, Rs=0
    let encoding: u32 = 0xB1100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_tst_rr_a1_a_special_cond_12_condition_gt_16_c1100010() {
    // Encoding: 0xC1100010
    // Test aarch32_TST_rr_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rm=0, Rs=0, cond=12, type1=0, Rn=0
    let encoding: u32 = 0xC1100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_tst_rr_a1_a_special_cond_13_condition_le_16_d1100010() {
    // Encoding: 0xD1100010
    // Test aarch32_TST_rr_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rn=0, cond=13, type1=0, Rm=0, Rs=0
    let encoding: u32 = 0xD1100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_tst_rr_a1_a_special_cond_14_condition_al_16_e1100010() {
    // Encoding: 0xE1100010
    // Test aarch32_TST_rr_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: cond=14, Rm=0, Rs=0, Rn=0, type1=0
    let encoding: u32 = 0xE1100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_tst_rr_a1_a_special_cond_15_condition_nv_16_f1100010() {
    // Encoding: 0xF1100010
    // Test aarch32_TST_rr_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: type1=0, Rm=0, Rn=0, cond=15, Rs=0
    let encoding: u32 = 0xF1100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "s" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"s\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_tst_rr_a1_a_invalid_0_10_01100010() {
    // Encoding: 0x01100010
    // Test aarch32_TST_rr_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "s" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rs=0, Rn=0, cond=0, type1=0, Rm=0
    let encoding: u32 = 0x01100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_tst_rr_a1_a_invalid_1_10_01100010() {
    // Encoding: 0x01100010
    // Test aarch32_TST_rr_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: type1=0, Rs=0, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x01100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_tst_rr_a1_a_flags_zeroresult_0_01110012() {
    // Test aarch32_TST_rr_A1_A flag computation: ZeroResult
    // Encoding: 0x01110012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x01110012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_tst_rr_a1_a_flags_zeroresult_1_01110012() {
    // Test aarch32_TST_rr_A1_A flag computation: ZeroResult
    // Encoding: 0x01110012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x01110012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_tst_rr_a1_a_flags_negativeresult_2_01110012() {
    // Test aarch32_TST_rr_A1_A flag computation: NegativeResult
    // Encoding: 0x01110012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x01110012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_tst_rr_a1_a_flags_unsignedoverflow_3_01110012() {
    // Test aarch32_TST_rr_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x01110012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x01110012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_tst_rr_a1_a_flags_unsignedoverflow_4_01110012() {
    // Test aarch32_TST_rr_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x01110012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x2);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x01110012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_tst_rr_a1_a_flags_signedoverflow_5_01110012() {
    // Test aarch32_TST_rr_A1_A flag computation: SignedOverflow
    // Encoding: 0x01110012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x01110012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_tst_rr_a1_a_flags_signedoverflow_6_01110012() {
    // Test aarch32_TST_rr_A1_A flag computation: SignedOverflow
    // Encoding: 0x01110012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x01110012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TST_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_tst_rr_a1_a_flags_positiveresult_7_01110012() {
    // Test aarch32_TST_rr_A1_A flag computation: PositiveResult
    // Encoding: 0x01110012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u32 = 0x01110012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

// ============================================================================
// aarch32_TEQ_i_A Tests
// ============================================================================

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_teq_i_a1_a_field_cond_0_min_0_03300000() {
    // Encoding: 0x03300000
    // Test aarch32_TEQ_i_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm12=0
    let encoding: u32 = 0x03300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_teq_i_a1_a_field_cond_1_poweroftwo_0_13300000() {
    // Encoding: 0x13300000
    // Test aarch32_TEQ_i_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, Rn=0, imm12=0
    let encoding: u32 = 0x13300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_teq_i_a1_a_field_cond_2_poweroftwo_0_23300000() {
    // Encoding: 0x23300000
    // Test aarch32_TEQ_i_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, cond=2, Rn=0
    let encoding: u32 = 0x23300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_teq_i_a1_a_field_cond_3_poweroftwo_0_33300000() {
    // Encoding: 0x33300000
    // Test aarch32_TEQ_i_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm12=0, cond=3
    let encoding: u32 = 0x33300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_teq_i_a1_a_field_cond_4_poweroftwo_0_43300000() {
    // Encoding: 0x43300000
    // Test aarch32_TEQ_i_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm12=0, cond=4
    let encoding: u32 = 0x43300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_teq_i_a1_a_field_cond_5_poweroftwo_0_53300000() {
    // Encoding: 0x53300000
    // Test aarch32_TEQ_i_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm12=0, cond=5
    let encoding: u32 = 0x53300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_teq_i_a1_a_field_cond_6_poweroftwo_0_63300000() {
    // Encoding: 0x63300000
    // Test aarch32_TEQ_i_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, cond=6, Rn=0
    let encoding: u32 = 0x63300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_teq_i_a1_a_field_cond_7_poweroftwo_0_73300000() {
    // Encoding: 0x73300000
    // Test aarch32_TEQ_i_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, cond=7, Rn=0
    let encoding: u32 = 0x73300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_teq_i_a1_a_field_cond_8_poweroftwo_0_83300000() {
    // Encoding: 0x83300000
    // Test aarch32_TEQ_i_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, cond=8, Rn=0
    let encoding: u32 = 0x83300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_teq_i_a1_a_field_cond_9_poweroftwo_0_93300000() {
    // Encoding: 0x93300000
    // Test aarch32_TEQ_i_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, Rn=0, imm12=0
    let encoding: u32 = 0x93300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_teq_i_a1_a_field_cond_10_poweroftwo_0_a3300000() {
    // Encoding: 0xA3300000
    // Test aarch32_TEQ_i_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, cond=10, Rn=0
    let encoding: u32 = 0xA3300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_teq_i_a1_a_field_cond_11_poweroftwo_0_b3300000() {
    // Encoding: 0xB3300000
    // Test aarch32_TEQ_i_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=11, imm12=0
    let encoding: u32 = 0xB3300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_teq_i_a1_a_field_cond_12_poweroftwo_0_c3300000() {
    // Encoding: 0xC3300000
    // Test aarch32_TEQ_i_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, Rn=0, imm12=0
    let encoding: u32 = 0xC3300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_teq_i_a1_a_field_cond_13_poweroftwo_0_d3300000() {
    // Encoding: 0xD3300000
    // Test aarch32_TEQ_i_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm12=0, cond=13
    let encoding: u32 = 0xD3300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_teq_i_a1_a_field_cond_14_poweroftwo_0_e3300000() {
    // Encoding: 0xE3300000
    // Test aarch32_TEQ_i_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm12=0, cond=14
    let encoding: u32 = 0xE3300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_teq_i_a1_a_field_cond_15_max_0_f3300000() {
    // Encoding: 0xF3300000
    // Test aarch32_TEQ_i_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: imm12=0, cond=15, Rn=0
    let encoding: u32 = 0xF3300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_teq_i_a1_a_field_rn_0_min_0_03300000() {
    // Encoding: 0x03300000
    // Test aarch32_TEQ_i_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm12=0
    let encoding: u32 = 0x03300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_teq_i_a1_a_field_rn_1_poweroftwo_0_03310000() {
    // Encoding: 0x03310000
    // Test aarch32_TEQ_i_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=1, imm12=0
    let encoding: u32 = 0x03310000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_teq_i_a1_a_field_imm12_0_zero_0_03300000() {
    // Encoding: 0x03300000
    // Test aarch32_TEQ_i_A1_A field imm12 = 0 (Zero)
    // ISET: A32
    // Fields: imm12=0, Rn=0, cond=0
    let encoding: u32 = 0x03300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_teq_i_a1_a_field_imm12_1_poweroftwo_0_03300001() {
    // Encoding: 0x03300001
    // Test aarch32_TEQ_i_A1_A field imm12 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm12=1, Rn=0
    let encoding: u32 = 0x03300001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_teq_i_a1_a_field_imm12_3_poweroftwominusone_0_03300003() {
    // Encoding: 0x03300003
    // Test aarch32_TEQ_i_A1_A field imm12 = 3 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm12=3
    let encoding: u32 = 0x03300003;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_teq_i_a1_a_field_imm12_4_poweroftwo_0_03300004() {
    // Encoding: 0x03300004
    // Test aarch32_TEQ_i_A1_A field imm12 = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm12=4, cond=0
    let encoding: u32 = 0x03300004;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_teq_i_a1_a_field_imm12_7_poweroftwominusone_0_03300007() {
    // Encoding: 0x03300007
    // Test aarch32_TEQ_i_A1_A field imm12 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=7, Rn=0, cond=0
    let encoding: u32 = 0x03300007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_teq_i_a1_a_field_imm12_8_poweroftwo_0_03300008() {
    // Encoding: 0x03300008
    // Test aarch32_TEQ_i_A1_A field imm12 = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm12=8
    let encoding: u32 = 0x03300008;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_teq_i_a1_a_field_imm12_15_poweroftwominusone_0_0330000f() {
    // Encoding: 0x0330000F
    // Test aarch32_TEQ_i_A1_A field imm12 = 15 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm12=15
    let encoding: u32 = 0x0330000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_teq_i_a1_a_field_imm12_16_poweroftwo_0_03300010() {
    // Encoding: 0x03300010
    // Test aarch32_TEQ_i_A1_A field imm12 = 16 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm12=16, cond=0
    let encoding: u32 = 0x03300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_teq_i_a1_a_field_imm12_31_poweroftwominusone_0_0330001f() {
    // Encoding: 0x0330001F
    // Test aarch32_TEQ_i_A1_A field imm12 = 31 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rn=0, imm12=31, cond=0
    let encoding: u32 = 0x0330001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_teq_i_a1_a_field_imm12_32_poweroftwo_0_03300020() {
    // Encoding: 0x03300020
    // Test aarch32_TEQ_i_A1_A field imm12 = 32 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm12=32
    let encoding: u32 = 0x03300020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_teq_i_a1_a_field_imm12_63_poweroftwominusone_0_0330003f() {
    // Encoding: 0x0330003F
    // Test aarch32_TEQ_i_A1_A field imm12 = 63 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=63, Rn=0, cond=0
    let encoding: u32 = 0x0330003F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_teq_i_a1_a_field_imm12_64_poweroftwo_0_03300040() {
    // Encoding: 0x03300040
    // Test aarch32_TEQ_i_A1_A field imm12 = 64 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=0, imm12=64
    let encoding: u32 = 0x03300040;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch32_teq_i_a1_a_field_imm12_127_poweroftwominusone_0_0330007f() {
    // Encoding: 0x0330007F
    // Test aarch32_TEQ_i_A1_A field imm12 = 127 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rn=0, cond=0, imm12=127
    let encoding: u32 = 0x0330007F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_teq_i_a1_a_field_imm12_128_poweroftwo_0_03300080() {
    // Encoding: 0x03300080
    // Test aarch32_TEQ_i_A1_A field imm12 = 128 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=128, cond=0, Rn=0
    let encoding: u32 = 0x03300080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch32_teq_i_a1_a_field_imm12_255_poweroftwominusone_0_033000ff() {
    // Encoding: 0x033000FF
    // Test aarch32_TEQ_i_A1_A field imm12 = 255 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=255, cond=0, Rn=0
    let encoding: u32 = 0x033000FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch32_teq_i_a1_a_field_imm12_256_poweroftwo_0_03300100() {
    // Encoding: 0x03300100
    // Test aarch32_TEQ_i_A1_A field imm12 = 256 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm12=256, Rn=0
    let encoding: u32 = 0x03300100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch32_teq_i_a1_a_field_imm12_511_poweroftwominusone_0_033001ff() {
    // Encoding: 0x033001FF
    // Test aarch32_TEQ_i_A1_A field imm12 = 511 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rn=0, imm12=511, cond=0
    let encoding: u32 = 0x033001FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch32_teq_i_a1_a_field_imm12_512_poweroftwo_0_03300200() {
    // Encoding: 0x03300200
    // Test aarch32_TEQ_i_A1_A field imm12 = 512 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm12=512
    let encoding: u32 = 0x03300200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1023, boundary: PowerOfTwoMinusOne }
/// 2^10 - 1 = 1023
#[test]
fn test_aarch32_teq_i_a1_a_field_imm12_1023_poweroftwominusone_0_033003ff() {
    // Encoding: 0x033003FF
    // Test aarch32_TEQ_i_A1_A field imm12 = 1023 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rn=0, cond=0, imm12=1023
    let encoding: u32 = 0x033003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch32_teq_i_a1_a_field_imm12_1024_poweroftwo_0_03300400() {
    // Encoding: 0x03300400
    // Test aarch32_TEQ_i_A1_A field imm12 = 1024 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=0, imm12=1024
    let encoding: u32 = 0x03300400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 2047, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (2047)
#[test]
fn test_aarch32_teq_i_a1_a_field_imm12_2047_poweroftwominusone_0_033007ff() {
    // Encoding: 0x033007FF
    // Test aarch32_TEQ_i_A1_A field imm12 = 2047 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=2047, Rn=0, cond=0
    let encoding: u32 = 0x033007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 2048, boundary: PowerOfTwo }
/// power of 2 (2^11 = 2048)
#[test]
fn test_aarch32_teq_i_a1_a_field_imm12_2048_poweroftwo_0_03300800() {
    // Encoding: 0x03300800
    // Test aarch32_TEQ_i_A1_A field imm12 = 2048 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm12=2048, Rn=0
    let encoding: u32 = 0x03300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 4095, boundary: Max }
/// maximum immediate (4095)
#[test]
fn test_aarch32_teq_i_a1_a_field_imm12_4095_max_0_03300fff() {
    // Encoding: 0x03300FFF
    // Test aarch32_TEQ_i_A1_A field imm12 = 4095 (Max)
    // ISET: A32
    // Fields: Rn=0, imm12=4095, cond=0
    let encoding: u32 = 0x03300FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_teq_i_a1_a_combo_0_0_03300000() {
    // Encoding: 0x03300000
    // Test aarch32_TEQ_i_A1_A field combination: cond=0, Rn=0, imm12=0
    // ISET: A32
    // Fields: cond=0, imm12=0, Rn=0
    let encoding: u32 = 0x03300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_teq_i_a1_a_special_cond_0_condition_eq_0_03300000() {
    // Encoding: 0x03300000
    // Test aarch32_TEQ_i_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rn=0, imm12=0, cond=0
    let encoding: u32 = 0x03300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_teq_i_a1_a_special_cond_1_condition_ne_0_13300000() {
    // Encoding: 0x13300000
    // Test aarch32_TEQ_i_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rn=0, imm12=0, cond=1
    let encoding: u32 = 0x13300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_teq_i_a1_a_special_cond_2_condition_cs_hs_0_23300000() {
    // Encoding: 0x23300000
    // Test aarch32_TEQ_i_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: imm12=0, Rn=0, cond=2
    let encoding: u32 = 0x23300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_teq_i_a1_a_special_cond_3_condition_cc_lo_0_33300000() {
    // Encoding: 0x33300000
    // Test aarch32_TEQ_i_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: imm12=0, cond=3, Rn=0
    let encoding: u32 = 0x33300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_teq_i_a1_a_special_cond_4_condition_mi_0_43300000() {
    // Encoding: 0x43300000
    // Test aarch32_TEQ_i_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, Rn=0, imm12=0
    let encoding: u32 = 0x43300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_teq_i_a1_a_special_cond_5_condition_pl_0_53300000() {
    // Encoding: 0x53300000
    // Test aarch32_TEQ_i_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: imm12=0, cond=5, Rn=0
    let encoding: u32 = 0x53300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_teq_i_a1_a_special_cond_6_condition_vs_0_63300000() {
    // Encoding: 0x63300000
    // Test aarch32_TEQ_i_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: imm12=0, cond=6, Rn=0
    let encoding: u32 = 0x63300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_teq_i_a1_a_special_cond_7_condition_vc_0_73300000() {
    // Encoding: 0x73300000
    // Test aarch32_TEQ_i_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, imm12=0, Rn=0
    let encoding: u32 = 0x73300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_teq_i_a1_a_special_cond_8_condition_hi_0_83300000() {
    // Encoding: 0x83300000
    // Test aarch32_TEQ_i_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rn=0, cond=8, imm12=0
    let encoding: u32 = 0x83300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_teq_i_a1_a_special_cond_9_condition_ls_0_93300000() {
    // Encoding: 0x93300000
    // Test aarch32_TEQ_i_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, imm12=0, Rn=0
    let encoding: u32 = 0x93300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_teq_i_a1_a_special_cond_10_condition_ge_0_a3300000() {
    // Encoding: 0xA3300000
    // Test aarch32_TEQ_i_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, Rn=0, imm12=0
    let encoding: u32 = 0xA3300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_teq_i_a1_a_special_cond_11_condition_lt_0_b3300000() {
    // Encoding: 0xB3300000
    // Test aarch32_TEQ_i_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: imm12=0, cond=11, Rn=0
    let encoding: u32 = 0xB3300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_teq_i_a1_a_special_cond_12_condition_gt_0_c3300000() {
    // Encoding: 0xC3300000
    // Test aarch32_TEQ_i_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rn=0, cond=12, imm12=0
    let encoding: u32 = 0xC3300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_teq_i_a1_a_special_cond_13_condition_le_0_d3300000() {
    // Encoding: 0xD3300000
    // Test aarch32_TEQ_i_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rn=0, cond=13, imm12=0
    let encoding: u32 = 0xD3300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_teq_i_a1_a_special_cond_14_condition_al_0_e3300000() {
    // Encoding: 0xE3300000
    // Test aarch32_TEQ_i_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rn=0, imm12=0, cond=14
    let encoding: u32 = 0xE3300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_teq_i_a1_a_special_cond_15_condition_nv_0_f3300000() {
    // Encoding: 0xF3300000
    // Test aarch32_TEQ_i_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: imm12=0, cond=15, Rn=0
    let encoding: u32 = 0xF3300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `field i 26 +: 1`
/// Requirement: FieldBoundary { field: "i", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_teq_i_t1_a_field_i_0_min_f00_f0900f00() {
    // Thumb encoding (32): 0xF0900F00
    // Test aarch32_TEQ_i_T1_A field i = 0 (Min)
    // ISET: T32
    // Fields: i=0, Rn=0, imm3=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0900F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `field i 26 +: 1`
/// Requirement: FieldBoundary { field: "i", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_teq_i_t1_a_field_i_1_max_f00_f4900f00() {
    // Thumb encoding (32): 0xF4900F00
    // Test aarch32_TEQ_i_T1_A field i = 1 (Max)
    // ISET: T32
    // Fields: i=1, Rn=0, imm8=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF4900F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_teq_i_t1_a_field_rn_0_min_f00_f0900f00() {
    // Thumb encoding (32): 0xF0900F00
    // Test aarch32_TEQ_i_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: i=0, Rn=0, imm3=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0900F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_teq_i_t1_a_field_rn_1_poweroftwo_f00_f0910f00() {
    // Thumb encoding (32): 0xF0910F00
    // Test aarch32_TEQ_i_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, imm3=0, imm8=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0910F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_teq_i_t1_a_field_imm3_0_zero_f00_f0900f00() {
    // Thumb encoding (32): 0xF0900F00
    // Test aarch32_TEQ_i_T1_A field imm3 = 0 (Zero)
    // ISET: T32
    // Fields: i=0, imm3=0, Rn=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0900F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_teq_i_t1_a_field_imm3_1_poweroftwo_f00_f0901f00() {
    // Thumb encoding (32): 0xF0901F00
    // Test aarch32_TEQ_i_T1_A field imm3 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=1, i=0, imm8=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0901F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch32_teq_i_t1_a_field_imm3_3_poweroftwominusone_f00_f0903f00() {
    // Thumb encoding (32): 0xF0903F00
    // Test aarch32_TEQ_i_T1_A field imm3 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=0, imm3=3, i=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0903F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch32_teq_i_t1_a_field_imm3_7_max_f00_f0907f00() {
    // Thumb encoding (32): 0xF0907F00
    // Test aarch32_TEQ_i_T1_A field imm3 = 7 (Max)
    // ISET: T32
    // Fields: imm3=7, i=0, imm8=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0907F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_teq_i_t1_a_field_imm8_0_zero_f00_f0900f00() {
    // Thumb encoding (32): 0xF0900F00
    // Test aarch32_TEQ_i_T1_A field imm8 = 0 (Zero)
    // ISET: T32
    // Fields: i=0, imm3=0, imm8=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0900F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_teq_i_t1_a_field_imm8_1_poweroftwo_f00_f0900f01() {
    // Thumb encoding (32): 0xF0900F01
    // Test aarch32_TEQ_i_T1_A field imm8 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=1, Rn=0, imm3=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0900F01;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_teq_i_t1_a_field_imm8_3_poweroftwominusone_f00_f0900f03() {
    // Thumb encoding (32): 0xF0900F03
    // Test aarch32_TEQ_i_T1_A field imm8 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rn=0, i=0, imm3=0, imm8=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0900F03;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_teq_i_t1_a_field_imm8_4_poweroftwo_f00_f0900f04() {
    // Thumb encoding (32): 0xF0900F04
    // Test aarch32_TEQ_i_T1_A field imm8 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=4, imm3=0, i=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0900F04;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_teq_i_t1_a_field_imm8_7_poweroftwominusone_f00_f0900f07() {
    // Thumb encoding (32): 0xF0900F07
    // Test aarch32_TEQ_i_T1_A field imm8 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=7, imm3=0, i=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0900F07;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_teq_i_t1_a_field_imm8_8_poweroftwo_f00_f0900f08() {
    // Thumb encoding (32): 0xF0900F08
    // Test aarch32_TEQ_i_T1_A field imm8 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=0, Rn=0, i=0, imm8=8
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0900F08;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_teq_i_t1_a_field_imm8_15_poweroftwominusone_f00_f0900f0f() {
    // Thumb encoding (32): 0xF0900F0F
    // Test aarch32_TEQ_i_T1_A field imm8 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rn=0, imm3=0, imm8=15, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0900F0F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_teq_i_t1_a_field_imm8_16_poweroftwo_f00_f0900f10() {
    // Thumb encoding (32): 0xF0900F10
    // Test aarch32_TEQ_i_T1_A field imm8 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, imm3=0, i=0, imm8=16
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0900F10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_teq_i_t1_a_field_imm8_31_poweroftwominusone_f00_f0900f1f() {
    // Thumb encoding (32): 0xF0900F1F
    // Test aarch32_TEQ_i_T1_A field imm8 = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: i=0, Rn=0, imm8=31, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0900F1F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_teq_i_t1_a_field_imm8_32_poweroftwo_f00_f0900f20() {
    // Thumb encoding (32): 0xF0900F20
    // Test aarch32_TEQ_i_T1_A field imm8 = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, imm8=32, imm3=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0900F20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_teq_i_t1_a_field_imm8_63_poweroftwominusone_f00_f0900f3f() {
    // Thumb encoding (32): 0xF0900F3F
    // Test aarch32_TEQ_i_T1_A field imm8 = 63 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=63, imm3=0, Rn=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0900F3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_teq_i_t1_a_field_imm8_64_poweroftwo_f00_f0900f40() {
    // Thumb encoding (32): 0xF0900F40
    // Test aarch32_TEQ_i_T1_A field imm8 = 64 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, imm8=64, i=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0900F40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 127, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (127)
#[test]
fn test_aarch32_teq_i_t1_a_field_imm8_127_poweroftwominusone_f00_f0900f7f() {
    // Thumb encoding (32): 0xF0900F7F
    // Test aarch32_TEQ_i_T1_A field imm8 = 127 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: i=0, Rn=0, imm8=127, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0900F7F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_teq_i_t1_a_field_imm8_128_poweroftwo_f00_f0900f80() {
    // Thumb encoding (32): 0xF0900F80
    // Test aarch32_TEQ_i_T1_A field imm8 = 128 (PowerOfTwo)
    // ISET: T32
    // Fields: i=0, imm3=0, imm8=128, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0900F80;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 255, boundary: Max }
/// maximum immediate (255)
#[test]
fn test_aarch32_teq_i_t1_a_field_imm8_255_max_f00_f0900fff() {
    // Thumb encoding (32): 0xF0900FFF
    // Test aarch32_TEQ_i_T1_A field imm8 = 255 (Max)
    // ISET: T32
    // Fields: imm3=0, Rn=0, imm8=255, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0900FFF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// i=0 (minimum value)
#[test]
fn test_aarch32_teq_i_t1_a_combo_0_f00_f0900f00() {
    // Thumb encoding (32): 0xF0900F00
    // Test aarch32_TEQ_i_T1_A field combination: i=0, Rn=0, imm3=0, imm8=0
    // ISET: T32
    // Fields: i=0, imm3=0, Rn=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0900F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }), rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_teq_i_t1_a_invalid_0_f00_f0900f00() {
    // Thumb encoding (32): 0xF0900F00
    // Test aarch32_TEQ_i_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: LitInt(15) }
    // ISET: T32
    // Fields: imm8=0, Rn=0, imm3=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0900F00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_teq_i_t1_a_invalid_1_f00_f0900f00() {
    // Thumb encoding (32): 0xF0900F00
    // Test aarch32_TEQ_i_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: i=0, imm3=0, imm8=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0900F00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// zero AND zero (32-bit)
#[test]
fn test_aarch32_teq_i_a1_a_oracle_32_0_6a02003f() {
    // Test TST 32-bit: zero AND zero (oracle)
    // Encoding: 0x6A02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// zero AND zero (64-bit)
#[test]
fn test_aarch32_teq_i_a1_a_oracle_64_0_ea02003f() {
    // Test TST 64-bit: zero AND zero (oracle)
    // Encoding: 0xEA02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// partial overlap (32-bit)
#[test]
fn test_aarch32_teq_i_a1_a_oracle_32_1_6a02003f() {
    // Test TST 32-bit: partial overlap (oracle)
    // Encoding: 0x6A02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    set_w(&mut cpu, 2, 0xF);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// partial overlap (64-bit)
#[test]
fn test_aarch32_teq_i_a1_a_oracle_64_1_ea02003f() {
    // Test TST 64-bit: partial overlap (oracle)
    // Encoding: 0xEA02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    set_w(&mut cpu, 2, 0xF);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// no overlap (32-bit)
#[test]
fn test_aarch32_teq_i_a1_a_oracle_32_2_6a02003f() {
    // Test TST 32-bit: no overlap (oracle)
    // Encoding: 0x6A02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// no overlap (64-bit)
#[test]
fn test_aarch32_teq_i_a1_a_oracle_64_2_ea02003f() {
    // Test TST 64-bit: no overlap (oracle)
    // Encoding: 0xEA02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// MSB set (32-bit)
#[test]
fn test_aarch32_teq_i_a1_a_oracle_32_3_6a02003f() {
    // Test TST 32-bit: MSB set (oracle)
    // Encoding: 0x6A02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// MSB set (64-bit)
#[test]
fn test_aarch32_teq_i_a1_a_oracle_64_3_ea02003f() {
    // Test TST 64-bit: MSB set (oracle)
    // Encoding: 0xEA02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// all ones (32-bit)
#[test]
fn test_aarch32_teq_i_a1_a_oracle_32_4_6a02003f() {
    // Test TST 32-bit: all ones (oracle)
    // Encoding: 0x6A02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// all ones (64-bit)
#[test]
fn test_aarch32_teq_i_a1_a_oracle_64_4_ea02003f() {
    // Test TST 64-bit: all ones (oracle)
    // Encoding: 0xEA02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// alternating (no match) (32-bit)
#[test]
fn test_aarch32_teq_i_a1_a_oracle_32_5_6a02003f() {
    // Test TST 32-bit: alternating (no match) (oracle)
    // Encoding: 0x6A02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    set_w(&mut cpu, 2, 0x55555555);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// alternating (no match) (64-bit)
#[test]
fn test_aarch32_teq_i_a1_a_oracle_64_5_ea02003f() {
    // Test TST 64-bit: alternating (no match) (oracle)
    // Encoding: 0xEA02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x55555555);
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_teq_i_a1_a_flags_zeroresult_0_03310000() {
    // Test aarch32_TEQ_i_A1_A flag computation: ZeroResult
    // Encoding: 0x03310000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x03310000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_teq_i_a1_a_flags_zeroresult_1_03310000() {
    // Test aarch32_TEQ_i_A1_A flag computation: ZeroResult
    // Encoding: 0x03310000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x03310000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_teq_i_a1_a_flags_negativeresult_2_03310000() {
    // Test aarch32_TEQ_i_A1_A flag computation: NegativeResult
    // Encoding: 0x03310000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x03310000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_teq_i_a1_a_flags_unsignedoverflow_3_03310000() {
    // Test aarch32_TEQ_i_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x03310000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x03310000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_teq_i_a1_a_flags_unsignedoverflow_4_03310000() {
    // Test aarch32_TEQ_i_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x03310000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x03310000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_teq_i_a1_a_flags_signedoverflow_5_03310000() {
    // Test aarch32_TEQ_i_A1_A flag computation: SignedOverflow
    // Encoding: 0x03310000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x03310000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_teq_i_a1_a_flags_signedoverflow_6_03310000() {
    // Test aarch32_TEQ_i_A1_A flag computation: SignedOverflow
    // Encoding: 0x03310000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x03310000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TEQ_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_teq_i_a1_a_flags_positiveresult_7_03310000() {
    // Test aarch32_TEQ_i_A1_A flag computation: PositiveResult
    // Encoding: 0x03310000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x32);
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x03310000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// zero AND zero (32-bit)
#[test]
fn test_aarch32_teq_i_t1_a_oracle_32_0_6a02003f() {
    // Test TST 32-bit: zero AND zero (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// zero AND zero (64-bit)
#[test]
fn test_aarch32_teq_i_t1_a_oracle_64_0_ea02003f() {
    // Test TST 64-bit: zero AND zero (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// partial overlap (32-bit)
#[test]
fn test_aarch32_teq_i_t1_a_oracle_32_1_6a02003f() {
    // Test TST 32-bit: partial overlap (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    set_w(&mut cpu, 2, 0xF);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// partial overlap (64-bit)
#[test]
fn test_aarch32_teq_i_t1_a_oracle_64_1_ea02003f() {
    // Test TST 64-bit: partial overlap (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    set_w(&mut cpu, 2, 0xF);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// no overlap (32-bit)
#[test]
fn test_aarch32_teq_i_t1_a_oracle_32_2_6a02003f() {
    // Test TST 32-bit: no overlap (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// no overlap (64-bit)
#[test]
fn test_aarch32_teq_i_t1_a_oracle_64_2_ea02003f() {
    // Test TST 64-bit: no overlap (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// MSB set (32-bit)
#[test]
fn test_aarch32_teq_i_t1_a_oracle_32_3_6a02003f() {
    // Test TST 32-bit: MSB set (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// MSB set (64-bit)
#[test]
fn test_aarch32_teq_i_t1_a_oracle_64_3_ea02003f() {
    // Test TST 64-bit: MSB set (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// all ones (32-bit)
#[test]
fn test_aarch32_teq_i_t1_a_oracle_32_4_6a02003f() {
    // Test TST 32-bit: all ones (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// all ones (64-bit)
#[test]
fn test_aarch32_teq_i_t1_a_oracle_64_4_ea02003f() {
    // Test TST 64-bit: all ones (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// alternating (no match) (32-bit)
#[test]
fn test_aarch32_teq_i_t1_a_oracle_32_5_6a02003f() {
    // Test TST 32-bit: alternating (no match) (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    set_w(&mut cpu, 2, 0x55555555);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// alternating (no match) (64-bit)
#[test]
fn test_aarch32_teq_i_t1_a_oracle_64_5_ea02003f() {
    // Test TST 64-bit: alternating (no match) (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    set_w(&mut cpu, 2, 0x55555555);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_teq_i_t1_a_flags_zeroresult_0_f0910f00() {
    // Test aarch32_TEQ_i_T1_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xF0910F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_teq_i_t1_a_flags_zeroresult_1_f0910f00() {
    // Test aarch32_TEQ_i_T1_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0xF0910F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_teq_i_t1_a_flags_negativeresult_2_f0910f00() {
    // Test aarch32_TEQ_i_T1_A flag computation: NegativeResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF0910F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_teq_i_t1_a_flags_unsignedoverflow_3_f0910f00() {
    // Test aarch32_TEQ_i_T1_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xF0910F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_teq_i_t1_a_flags_unsignedoverflow_4_f0910f00() {
    // Test aarch32_TEQ_i_T1_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u32 = 0xF0910F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_teq_i_t1_a_flags_signedoverflow_5_f0910f00() {
    // Test aarch32_TEQ_i_T1_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xF0910F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_teq_i_t1_a_flags_signedoverflow_6_f0910f00() {
    // Test aarch32_TEQ_i_T1_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0xF0910F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TEQ_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_teq_i_t1_a_flags_positiveresult_7_f0910f00() {
    // Test aarch32_TEQ_i_T1_A flag computation: PositiveResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x32);
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF0910F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

// ============================================================================
// aarch32_CMN_i_A Tests
// ============================================================================

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_cmn_i_a1_a_field_cond_0_min_0_03700000() {
    // Encoding: 0x03700000
    // Test aarch32_CMN_i_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: imm12=0, cond=0, Rn=0
    let encoding: u32 = 0x03700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_cmn_i_a1_a_field_cond_1_poweroftwo_0_13700000() {
    // Encoding: 0x13700000
    // Test aarch32_CMN_i_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, Rn=0, imm12=0
    let encoding: u32 = 0x13700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_cmn_i_a1_a_field_cond_2_poweroftwo_0_23700000() {
    // Encoding: 0x23700000
    // Test aarch32_CMN_i_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=2, Rn=0, imm12=0
    let encoding: u32 = 0x23700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_cmn_i_a1_a_field_cond_3_poweroftwo_0_33700000() {
    // Encoding: 0x33700000
    // Test aarch32_CMN_i_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm12=0, cond=3
    let encoding: u32 = 0x33700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_cmn_i_a1_a_field_cond_4_poweroftwo_0_43700000() {
    // Encoding: 0x43700000
    // Test aarch32_CMN_i_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm12=0, cond=4
    let encoding: u32 = 0x43700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_cmn_i_a1_a_field_cond_5_poweroftwo_0_53700000() {
    // Encoding: 0x53700000
    // Test aarch32_CMN_i_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm12=0, cond=5
    let encoding: u32 = 0x53700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_cmn_i_a1_a_field_cond_6_poweroftwo_0_63700000() {
    // Encoding: 0x63700000
    // Test aarch32_CMN_i_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, Rn=0, imm12=0
    let encoding: u32 = 0x63700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_cmn_i_a1_a_field_cond_7_poweroftwo_0_73700000() {
    // Encoding: 0x73700000
    // Test aarch32_CMN_i_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, cond=7, Rn=0
    let encoding: u32 = 0x73700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_cmn_i_a1_a_field_cond_8_poweroftwo_0_83700000() {
    // Encoding: 0x83700000
    // Test aarch32_CMN_i_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, cond=8, Rn=0
    let encoding: u32 = 0x83700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_cmn_i_a1_a_field_cond_9_poweroftwo_0_93700000() {
    // Encoding: 0x93700000
    // Test aarch32_CMN_i_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=9, imm12=0
    let encoding: u32 = 0x93700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_cmn_i_a1_a_field_cond_10_poweroftwo_0_a3700000() {
    // Encoding: 0xA3700000
    // Test aarch32_CMN_i_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=10, imm12=0
    let encoding: u32 = 0xA3700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_cmn_i_a1_a_field_cond_11_poweroftwo_0_b3700000() {
    // Encoding: 0xB3700000
    // Test aarch32_CMN_i_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=11, Rn=0, imm12=0
    let encoding: u32 = 0xB3700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_cmn_i_a1_a_field_cond_12_poweroftwo_0_c3700000() {
    // Encoding: 0xC3700000
    // Test aarch32_CMN_i_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm12=0, cond=12
    let encoding: u32 = 0xC3700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_cmn_i_a1_a_field_cond_13_poweroftwo_0_d3700000() {
    // Encoding: 0xD3700000
    // Test aarch32_CMN_i_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=13, imm12=0
    let encoding: u32 = 0xD3700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_cmn_i_a1_a_field_cond_14_poweroftwo_0_e3700000() {
    // Encoding: 0xE3700000
    // Test aarch32_CMN_i_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=14, imm12=0
    let encoding: u32 = 0xE3700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_cmn_i_a1_a_field_cond_15_max_0_f3700000() {
    // Encoding: 0xF3700000
    // Test aarch32_CMN_i_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rn=0, imm12=0, cond=15
    let encoding: u32 = 0xF3700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmn_i_a1_a_field_rn_0_min_0_03700000() {
    // Encoding: 0x03700000
    // Test aarch32_CMN_i_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm12=0
    let encoding: u32 = 0x03700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmn_i_a1_a_field_rn_1_poweroftwo_0_03710000() {
    // Encoding: 0x03710000
    // Test aarch32_CMN_i_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=1, imm12=0
    let encoding: u32 = 0x03710000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_cmn_i_a1_a_field_imm12_0_zero_0_03700000() {
    // Encoding: 0x03700000
    // Test aarch32_CMN_i_A1_A field imm12 = 0 (Zero)
    // ISET: A32
    // Fields: cond=0, imm12=0, Rn=0
    let encoding: u32 = 0x03700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_cmn_i_a1_a_field_imm12_1_poweroftwo_0_03700001() {
    // Encoding: 0x03700001
    // Test aarch32_CMN_i_A1_A field imm12 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm12=1
    let encoding: u32 = 0x03700001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_cmn_i_a1_a_field_imm12_3_poweroftwominusone_0_03700003() {
    // Encoding: 0x03700003
    // Test aarch32_CMN_i_A1_A field imm12 = 3 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=3, Rn=0, cond=0
    let encoding: u32 = 0x03700003;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_cmn_i_a1_a_field_imm12_4_poweroftwo_0_03700004() {
    // Encoding: 0x03700004
    // Test aarch32_CMN_i_A1_A field imm12 = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=4, Rn=0, cond=0
    let encoding: u32 = 0x03700004;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_cmn_i_a1_a_field_imm12_7_poweroftwominusone_0_03700007() {
    // Encoding: 0x03700007
    // Test aarch32_CMN_i_A1_A field imm12 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=7, cond=0, Rn=0
    let encoding: u32 = 0x03700007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_cmn_i_a1_a_field_imm12_8_poweroftwo_0_03700008() {
    // Encoding: 0x03700008
    // Test aarch32_CMN_i_A1_A field imm12 = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=8, cond=0, Rn=0
    let encoding: u32 = 0x03700008;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_cmn_i_a1_a_field_imm12_15_poweroftwominusone_0_0370000f() {
    // Encoding: 0x0370000F
    // Test aarch32_CMN_i_A1_A field imm12 = 15 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=15, Rn=0, cond=0
    let encoding: u32 = 0x0370000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_cmn_i_a1_a_field_imm12_16_poweroftwo_0_03700010() {
    // Encoding: 0x03700010
    // Test aarch32_CMN_i_A1_A field imm12 = 16 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=0, imm12=16
    let encoding: u32 = 0x03700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_cmn_i_a1_a_field_imm12_31_poweroftwominusone_0_0370001f() {
    // Encoding: 0x0370001F
    // Test aarch32_CMN_i_A1_A field imm12 = 31 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rn=0, cond=0, imm12=31
    let encoding: u32 = 0x0370001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_cmn_i_a1_a_field_imm12_32_poweroftwo_0_03700020() {
    // Encoding: 0x03700020
    // Test aarch32_CMN_i_A1_A field imm12 = 32 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=32, Rn=0, cond=0
    let encoding: u32 = 0x03700020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_cmn_i_a1_a_field_imm12_63_poweroftwominusone_0_0370003f() {
    // Encoding: 0x0370003F
    // Test aarch32_CMN_i_A1_A field imm12 = 63 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rn=0, cond=0, imm12=63
    let encoding: u32 = 0x0370003F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_cmn_i_a1_a_field_imm12_64_poweroftwo_0_03700040() {
    // Encoding: 0x03700040
    // Test aarch32_CMN_i_A1_A field imm12 = 64 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm12=64, cond=0
    let encoding: u32 = 0x03700040;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch32_cmn_i_a1_a_field_imm12_127_poweroftwominusone_0_0370007f() {
    // Encoding: 0x0370007F
    // Test aarch32_CMN_i_A1_A field imm12 = 127 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=127, Rn=0, cond=0
    let encoding: u32 = 0x0370007F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_cmn_i_a1_a_field_imm12_128_poweroftwo_0_03700080() {
    // Encoding: 0x03700080
    // Test aarch32_CMN_i_A1_A field imm12 = 128 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm12=128
    let encoding: u32 = 0x03700080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch32_cmn_i_a1_a_field_imm12_255_poweroftwominusone_0_037000ff() {
    // Encoding: 0x037000FF
    // Test aarch32_CMN_i_A1_A field imm12 = 255 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=255, cond=0, Rn=0
    let encoding: u32 = 0x037000FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch32_cmn_i_a1_a_field_imm12_256_poweroftwo_0_03700100() {
    // Encoding: 0x03700100
    // Test aarch32_CMN_i_A1_A field imm12 = 256 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm12=256, cond=0
    let encoding: u32 = 0x03700100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch32_cmn_i_a1_a_field_imm12_511_poweroftwominusone_0_037001ff() {
    // Encoding: 0x037001FF
    // Test aarch32_CMN_i_A1_A field imm12 = 511 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=511, Rn=0, cond=0
    let encoding: u32 = 0x037001FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch32_cmn_i_a1_a_field_imm12_512_poweroftwo_0_03700200() {
    // Encoding: 0x03700200
    // Test aarch32_CMN_i_A1_A field imm12 = 512 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm12=512
    let encoding: u32 = 0x03700200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1023, boundary: PowerOfTwoMinusOne }
/// 2^10 - 1 = 1023
#[test]
fn test_aarch32_cmn_i_a1_a_field_imm12_1023_poweroftwominusone_0_037003ff() {
    // Encoding: 0x037003FF
    // Test aarch32_CMN_i_A1_A field imm12 = 1023 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm12=1023, Rn=0
    let encoding: u32 = 0x037003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch32_cmn_i_a1_a_field_imm12_1024_poweroftwo_0_03700400() {
    // Encoding: 0x03700400
    // Test aarch32_CMN_i_A1_A field imm12 = 1024 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=1024, cond=0, Rn=0
    let encoding: u32 = 0x03700400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 2047, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (2047)
#[test]
fn test_aarch32_cmn_i_a1_a_field_imm12_2047_poweroftwominusone_0_037007ff() {
    // Encoding: 0x037007FF
    // Test aarch32_CMN_i_A1_A field imm12 = 2047 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm12=2047, Rn=0
    let encoding: u32 = 0x037007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 2048, boundary: PowerOfTwo }
/// power of 2 (2^11 = 2048)
#[test]
fn test_aarch32_cmn_i_a1_a_field_imm12_2048_poweroftwo_0_03700800() {
    // Encoding: 0x03700800
    // Test aarch32_CMN_i_A1_A field imm12 = 2048 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm12=2048, Rn=0
    let encoding: u32 = 0x03700800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 4095, boundary: Max }
/// maximum immediate (4095)
#[test]
fn test_aarch32_cmn_i_a1_a_field_imm12_4095_max_0_03700fff() {
    // Encoding: 0x03700FFF
    // Test aarch32_CMN_i_A1_A field imm12 = 4095 (Max)
    // ISET: A32
    // Fields: Rn=0, cond=0, imm12=4095
    let encoding: u32 = 0x03700FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_cmn_i_a1_a_combo_0_0_03700000() {
    // Encoding: 0x03700000
    // Test aarch32_CMN_i_A1_A field combination: cond=0, Rn=0, imm12=0
    // ISET: A32
    // Fields: imm12=0, Rn=0, cond=0
    let encoding: u32 = 0x03700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_cmn_i_a1_a_special_cond_0_condition_eq_0_03700000() {
    // Encoding: 0x03700000
    // Test aarch32_CMN_i_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: imm12=0, cond=0, Rn=0
    let encoding: u32 = 0x03700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_cmn_i_a1_a_special_cond_1_condition_ne_0_13700000() {
    // Encoding: 0x13700000
    // Test aarch32_CMN_i_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, Rn=0, imm12=0
    let encoding: u32 = 0x13700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_cmn_i_a1_a_special_cond_2_condition_cs_hs_0_23700000() {
    // Encoding: 0x23700000
    // Test aarch32_CMN_i_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: imm12=0, cond=2, Rn=0
    let encoding: u32 = 0x23700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_cmn_i_a1_a_special_cond_3_condition_cc_lo_0_33700000() {
    // Encoding: 0x33700000
    // Test aarch32_CMN_i_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: imm12=0, cond=3, Rn=0
    let encoding: u32 = 0x33700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_cmn_i_a1_a_special_cond_4_condition_mi_0_43700000() {
    // Encoding: 0x43700000
    // Test aarch32_CMN_i_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, Rn=0, imm12=0
    let encoding: u32 = 0x43700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_cmn_i_a1_a_special_cond_5_condition_pl_0_53700000() {
    // Encoding: 0x53700000
    // Test aarch32_CMN_i_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: imm12=0, Rn=0, cond=5
    let encoding: u32 = 0x53700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_cmn_i_a1_a_special_cond_6_condition_vs_0_63700000() {
    // Encoding: 0x63700000
    // Test aarch32_CMN_i_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, imm12=0, Rn=0
    let encoding: u32 = 0x63700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_cmn_i_a1_a_special_cond_7_condition_vc_0_73700000() {
    // Encoding: 0x73700000
    // Test aarch32_CMN_i_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, imm12=0, Rn=0
    let encoding: u32 = 0x73700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_cmn_i_a1_a_special_cond_8_condition_hi_0_83700000() {
    // Encoding: 0x83700000
    // Test aarch32_CMN_i_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, Rn=0, imm12=0
    let encoding: u32 = 0x83700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_cmn_i_a1_a_special_cond_9_condition_ls_0_93700000() {
    // Encoding: 0x93700000
    // Test aarch32_CMN_i_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rn=0, imm12=0, cond=9
    let encoding: u32 = 0x93700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_cmn_i_a1_a_special_cond_10_condition_ge_0_a3700000() {
    // Encoding: 0xA3700000
    // Test aarch32_CMN_i_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: imm12=0, Rn=0, cond=10
    let encoding: u32 = 0xA3700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_cmn_i_a1_a_special_cond_11_condition_lt_0_b3700000() {
    // Encoding: 0xB3700000
    // Test aarch32_CMN_i_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rn=0, cond=11, imm12=0
    let encoding: u32 = 0xB3700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_cmn_i_a1_a_special_cond_12_condition_gt_0_c3700000() {
    // Encoding: 0xC3700000
    // Test aarch32_CMN_i_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: imm12=0, cond=12, Rn=0
    let encoding: u32 = 0xC3700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_cmn_i_a1_a_special_cond_13_condition_le_0_d3700000() {
    // Encoding: 0xD3700000
    // Test aarch32_CMN_i_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: imm12=0, cond=13, Rn=0
    let encoding: u32 = 0xD3700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_cmn_i_a1_a_special_cond_14_condition_al_0_e3700000() {
    // Encoding: 0xE3700000
    // Test aarch32_CMN_i_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: cond=14, Rn=0, imm12=0
    let encoding: u32 = 0xE3700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_cmn_i_a1_a_special_cond_15_condition_nv_0_f3700000() {
    // Encoding: 0xF3700000
    // Test aarch32_CMN_i_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, Rn=0, imm12=0
    let encoding: u32 = 0xF3700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `field i 26 +: 1`
/// Requirement: FieldBoundary { field: "i", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_cmn_i_t1_a_field_i_0_min_f00_f1100f00() {
    // Thumb encoding (32): 0xF1100F00
    // Test aarch32_CMN_i_T1_A field i = 0 (Min)
    // ISET: T32
    // Fields: i=0, imm8=0, imm3=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1100F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `field i 26 +: 1`
/// Requirement: FieldBoundary { field: "i", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_cmn_i_t1_a_field_i_1_max_f00_f5100f00() {
    // Thumb encoding (32): 0xF5100F00
    // Test aarch32_CMN_i_T1_A field i = 1 (Max)
    // ISET: T32
    // Fields: imm3=0, imm8=0, Rn=0, i=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF5100F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmn_i_t1_a_field_rn_0_min_f00_f1100f00() {
    // Thumb encoding (32): 0xF1100F00
    // Test aarch32_CMN_i_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, imm8=0, imm3=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1100F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmn_i_t1_a_field_rn_1_poweroftwo_f00_f1110f00() {
    // Thumb encoding (32): 0xF1110F00
    // Test aarch32_CMN_i_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=0, Rn=1, i=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1110F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_cmn_i_t1_a_field_imm3_0_zero_f00_f1100f00() {
    // Thumb encoding (32): 0xF1100F00
    // Test aarch32_CMN_i_T1_A field imm3 = 0 (Zero)
    // ISET: T32
    // Fields: i=0, imm3=0, imm8=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1100F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_cmn_i_t1_a_field_imm3_1_poweroftwo_f00_f1101f00() {
    // Thumb encoding (32): 0xF1101F00
    // Test aarch32_CMN_i_T1_A field imm3 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: i=0, Rn=0, imm8=0, imm3=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1101F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch32_cmn_i_t1_a_field_imm3_3_poweroftwominusone_f00_f1103f00() {
    // Thumb encoding (32): 0xF1103F00
    // Test aarch32_CMN_i_T1_A field imm3 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rn=0, imm8=0, i=0, imm3=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1103F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch32_cmn_i_t1_a_field_imm3_7_max_f00_f1107f00() {
    // Thumb encoding (32): 0xF1107F00
    // Test aarch32_CMN_i_T1_A field imm3 = 7 (Max)
    // ISET: T32
    // Fields: i=0, imm8=0, Rn=0, imm3=7
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1107F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_cmn_i_t1_a_field_imm8_0_zero_f00_f1100f00() {
    // Thumb encoding (32): 0xF1100F00
    // Test aarch32_CMN_i_T1_A field imm8 = 0 (Zero)
    // ISET: T32
    // Fields: imm3=0, i=0, Rn=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1100F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_cmn_i_t1_a_field_imm8_1_poweroftwo_f00_f1100f01() {
    // Thumb encoding (32): 0xF1100F01
    // Test aarch32_CMN_i_T1_A field imm8 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: i=0, Rn=0, imm8=1, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1100F01;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_cmn_i_t1_a_field_imm8_3_poweroftwominusone_f00_f1100f03() {
    // Thumb encoding (32): 0xF1100F03
    // Test aarch32_CMN_i_T1_A field imm8 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rn=0, imm3=0, imm8=3, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1100F03;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_cmn_i_t1_a_field_imm8_4_poweroftwo_f00_f1100f04() {
    // Thumb encoding (32): 0xF1100F04
    // Test aarch32_CMN_i_T1_A field imm8 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, imm8=4, imm3=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1100F04;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_cmn_i_t1_a_field_imm8_7_poweroftwominusone_f00_f1100f07() {
    // Thumb encoding (32): 0xF1100F07
    // Test aarch32_CMN_i_T1_A field imm8 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=7, i=0, imm3=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1100F07;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_cmn_i_t1_a_field_imm8_8_poweroftwo_f00_f1100f08() {
    // Thumb encoding (32): 0xF1100F08
    // Test aarch32_CMN_i_T1_A field imm8 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, i=0, imm3=0, imm8=8
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1100F08;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_cmn_i_t1_a_field_imm8_15_poweroftwominusone_f00_f1100f0f() {
    // Thumb encoding (32): 0xF1100F0F
    // Test aarch32_CMN_i_T1_A field imm8 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: i=0, imm8=15, Rn=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1100F0F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_cmn_i_t1_a_field_imm8_16_poweroftwo_f00_f1100f10() {
    // Thumb encoding (32): 0xF1100F10
    // Test aarch32_CMN_i_T1_A field imm8 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, imm3=0, imm8=16, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1100F10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_cmn_i_t1_a_field_imm8_31_poweroftwominusone_f00_f1100f1f() {
    // Thumb encoding (32): 0xF1100F1F
    // Test aarch32_CMN_i_T1_A field imm8 = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: i=0, Rn=0, imm8=31, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1100F1F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_cmn_i_t1_a_field_imm8_32_poweroftwo_f00_f1100f20() {
    // Thumb encoding (32): 0xF1100F20
    // Test aarch32_CMN_i_T1_A field imm8 = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: i=0, imm8=32, imm3=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1100F20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_cmn_i_t1_a_field_imm8_63_poweroftwominusone_f00_f1100f3f() {
    // Thumb encoding (32): 0xF1100F3F
    // Test aarch32_CMN_i_T1_A field imm8 = 63 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: i=0, Rn=0, imm3=0, imm8=63
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1100F3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_cmn_i_t1_a_field_imm8_64_poweroftwo_f00_f1100f40() {
    // Thumb encoding (32): 0xF1100F40
    // Test aarch32_CMN_i_T1_A field imm8 = 64 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, imm3=0, i=0, imm8=64
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1100F40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 127, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (127)
#[test]
fn test_aarch32_cmn_i_t1_a_field_imm8_127_poweroftwominusone_f00_f1100f7f() {
    // Thumb encoding (32): 0xF1100F7F
    // Test aarch32_CMN_i_T1_A field imm8 = 127 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rn=0, i=0, imm3=0, imm8=127
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1100F7F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_cmn_i_t1_a_field_imm8_128_poweroftwo_f00_f1100f80() {
    // Thumb encoding (32): 0xF1100F80
    // Test aarch32_CMN_i_T1_A field imm8 = 128 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=0, imm8=128, i=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1100F80;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 255, boundary: Max }
/// maximum immediate (255)
#[test]
fn test_aarch32_cmn_i_t1_a_field_imm8_255_max_f00_f1100fff() {
    // Thumb encoding (32): 0xF1100FFF
    // Test aarch32_CMN_i_T1_A field imm8 = 255 (Max)
    // ISET: T32
    // Fields: i=0, Rn=0, imm3=0, imm8=255
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1100FFF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// i=0 (minimum value)
#[test]
fn test_aarch32_cmn_i_t1_a_combo_0_f00_f1100f00() {
    // Thumb encoding (32): 0xF1100F00
    // Test aarch32_CMN_i_T1_A field combination: i=0, Rn=0, imm3=0, imm8=0
    // ISET: T32
    // Fields: Rn=0, i=0, imm3=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1100F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }), rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_cmn_i_t1_a_invalid_0_f00_f1100f00() {
    // Thumb encoding (32): 0xF1100F00
    // Test aarch32_CMN_i_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: LitInt(15) }
    // ISET: T32
    // Fields: imm3=0, Rn=0, i=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1100F00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_cmn_i_t1_a_invalid_1_f00_f1100f00() {
    // Thumb encoding (32): 0xF1100F00
    // Test aarch32_CMN_i_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: i=0, imm3=0, imm8=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1100F00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// zero == zero (32-bit)
#[test]
fn test_aarch32_cmn_i_a1_a_oracle_32_0_2b02003f() {
    // Test CMN 32-bit: zero == zero (oracle)
    // Encoding: 0x2B02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x2B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// zero == zero (64-bit)
#[test]
fn test_aarch32_cmn_i_a1_a_oracle_64_0_ab02003f() {
    // Test CMN 64-bit: zero == zero (oracle)
    // Encoding: 0xAB02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xAB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// equal values (32-bit)
#[test]
fn test_aarch32_cmn_i_a1_a_oracle_32_1_2b02003f() {
    // Test CMN 32-bit: equal values (oracle)
    // Encoding: 0x2B02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x2B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// equal values (64-bit)
#[test]
fn test_aarch32_cmn_i_a1_a_oracle_64_1_ab02003f() {
    // Test CMN 64-bit: equal values (oracle)
    // Encoding: 0xAB02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xAB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// greater than (32-bit)
#[test]
fn test_aarch32_cmn_i_a1_a_oracle_32_2_2b02003f() {
    // Test CMN 32-bit: greater than (oracle)
    // Encoding: 0x2B02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x5);
    set_w(&mut cpu, 1, 0xA);
    let encoding: u32 = 0x2B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// greater than (64-bit)
#[test]
fn test_aarch32_cmn_i_a1_a_oracle_64_2_ab02003f() {
    // Test CMN 64-bit: greater than (oracle)
    // Encoding: 0xAB02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x5);
    set_w(&mut cpu, 1, 0xA);
    let encoding: u32 = 0xAB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// less than (32-bit)
#[test]
fn test_aarch32_cmn_i_a1_a_oracle_32_3_2b02003f() {
    // Test CMN 32-bit: less than (oracle)
    // Encoding: 0x2B02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x5);
    set_w(&mut cpu, 2, 0xA);
    let encoding: u32 = 0x2B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// less than (64-bit)
#[test]
fn test_aarch32_cmn_i_a1_a_oracle_64_3_ab02003f() {
    // Test CMN 64-bit: less than (oracle)
    // Encoding: 0xAB02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x5);
    set_w(&mut cpu, 2, 0xA);
    let encoding: u32 = 0xAB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// max value (32-bit)
#[test]
fn test_aarch32_cmn_i_a1_a_oracle_32_4_2b02003f() {
    // Test CMN 32-bit: max value (oracle)
    // Encoding: 0x2B02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x2B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// max value (64-bit)
#[test]
fn test_aarch32_cmn_i_a1_a_oracle_64_4_ab02003f() {
    // Test CMN 64-bit: max value (oracle)
    // Encoding: 0xAB02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xAB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// negative (MSB set) (32-bit)
#[test]
fn test_aarch32_cmn_i_a1_a_oracle_32_5_2b02003f() {
    // Test CMN 32-bit: negative (MSB set) (oracle)
    // Encoding: 0x2B02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x2B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// negative (MSB set) (64-bit)
#[test]
fn test_aarch32_cmn_i_a1_a_oracle_64_5_ab02003f() {
    // Test CMN 64-bit: negative (MSB set) (oracle)
    // Encoding: 0xAB02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xAB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// overflow boundary (32-bit)
#[test]
fn test_aarch32_cmn_i_a1_a_oracle_32_6_2b02003f() {
    // Test CMN 32-bit: overflow boundary (oracle)
    // Encoding: 0x2B02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x2B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// overflow boundary (64-bit)
#[test]
fn test_aarch32_cmn_i_a1_a_oracle_64_6_ab02003f() {
    // Test CMN 64-bit: overflow boundary (oracle)
    // Encoding: 0xAB02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xAB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, true, "V flag should be true");
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_cmn_i_a1_a_flags_zeroresult_0_03710000() {
    // Test aarch32_CMN_i_A1_A flag computation: ZeroResult
    // Encoding: 0x03710000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x03710000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_cmn_i_a1_a_flags_zeroresult_1_03710000() {
    // Test aarch32_CMN_i_A1_A flag computation: ZeroResult
    // Encoding: 0x03710000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x03710000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_cmn_i_a1_a_flags_negativeresult_2_03710000() {
    // Test aarch32_CMN_i_A1_A flag computation: NegativeResult
    // Encoding: 0x03710000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x03710000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_cmn_i_a1_a_flags_unsignedoverflow_3_03710000() {
    // Test aarch32_CMN_i_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x03710000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x03710000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_cmn_i_a1_a_flags_unsignedoverflow_4_03710000() {
    // Test aarch32_CMN_i_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x03710000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x03710000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_cmn_i_a1_a_flags_signedoverflow_5_03710000() {
    // Test aarch32_CMN_i_A1_A flag computation: SignedOverflow
    // Encoding: 0x03710000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x03710000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_cmn_i_a1_a_flags_signedoverflow_6_03710000() {
    // Test aarch32_CMN_i_A1_A flag computation: SignedOverflow
    // Encoding: 0x03710000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x03710000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMN_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_cmn_i_a1_a_flags_positiveresult_7_03710000() {
    // Test aarch32_CMN_i_A1_A flag computation: PositiveResult
    // Encoding: 0x03710000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x32);
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x03710000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// zero == zero (32-bit)
#[test]
fn test_aarch32_cmn_i_t1_a_oracle_32_0_2b02003f() {
    // Test CMN 32-bit: zero == zero (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x2B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// zero == zero (64-bit)
#[test]
fn test_aarch32_cmn_i_t1_a_oracle_64_0_ab02003f() {
    // Test CMN 64-bit: zero == zero (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xAB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// equal values (32-bit)
#[test]
fn test_aarch32_cmn_i_t1_a_oracle_32_1_2b02003f() {
    // Test CMN 32-bit: equal values (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x2B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// equal values (64-bit)
#[test]
fn test_aarch32_cmn_i_t1_a_oracle_64_1_ab02003f() {
    // Test CMN 64-bit: equal values (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xAB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// greater than (32-bit)
#[test]
fn test_aarch32_cmn_i_t1_a_oracle_32_2_2b02003f() {
    // Test CMN 32-bit: greater than (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x5);
    set_w(&mut cpu, 1, 0xA);
    let encoding: u32 = 0x2B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// greater than (64-bit)
#[test]
fn test_aarch32_cmn_i_t1_a_oracle_64_2_ab02003f() {
    // Test CMN 64-bit: greater than (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x5);
    set_w(&mut cpu, 1, 0xA);
    let encoding: u32 = 0xAB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// less than (32-bit)
#[test]
fn test_aarch32_cmn_i_t1_a_oracle_32_3_2b02003f() {
    // Test CMN 32-bit: less than (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xA);
    set_w(&mut cpu, 1, 0x5);
    let encoding: u32 = 0x2B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// less than (64-bit)
#[test]
fn test_aarch32_cmn_i_t1_a_oracle_64_3_ab02003f() {
    // Test CMN 64-bit: less than (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xA);
    set_w(&mut cpu, 1, 0x5);
    let encoding: u32 = 0xAB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// max value (32-bit)
#[test]
fn test_aarch32_cmn_i_t1_a_oracle_32_4_2b02003f() {
    // Test CMN 32-bit: max value (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x2B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// max value (64-bit)
#[test]
fn test_aarch32_cmn_i_t1_a_oracle_64_4_ab02003f() {
    // Test CMN 64-bit: max value (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xAB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// negative (MSB set) (32-bit)
#[test]
fn test_aarch32_cmn_i_t1_a_oracle_32_5_2b02003f() {
    // Test CMN 32-bit: negative (MSB set) (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x2B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// negative (MSB set) (64-bit)
#[test]
fn test_aarch32_cmn_i_t1_a_oracle_64_5_ab02003f() {
    // Test CMN 64-bit: negative (MSB set) (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xAB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// overflow boundary (32-bit)
#[test]
fn test_aarch32_cmn_i_t1_a_oracle_32_6_2b02003f() {
    // Test CMN 32-bit: overflow boundary (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x2B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `CMN X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// overflow boundary (64-bit)
#[test]
fn test_aarch32_cmn_i_t1_a_oracle_64_6_ab02003f() {
    // Test CMN 64-bit: overflow boundary (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xAB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, true, "V flag should be true");
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_cmn_i_t1_a_flags_zeroresult_0_f1110f00() {
    // Test aarch32_CMN_i_T1_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF1110F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_cmn_i_t1_a_flags_zeroresult_1_f1110f00() {
    // Test aarch32_CMN_i_T1_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF1110F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_cmn_i_t1_a_flags_negativeresult_2_f1110f00() {
    // Test aarch32_CMN_i_T1_A flag computation: NegativeResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF1110F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_cmn_i_t1_a_flags_unsignedoverflow_3_f1110f00() {
    // Test aarch32_CMN_i_T1_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xF1110F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_cmn_i_t1_a_flags_unsignedoverflow_4_f1110f00() {
    // Test aarch32_CMN_i_T1_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x2);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF1110F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_cmn_i_t1_a_flags_signedoverflow_5_f1110f00() {
    // Test aarch32_CMN_i_T1_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xF1110F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_cmn_i_t1_a_flags_signedoverflow_6_f1110f00() {
    // Test aarch32_CMN_i_T1_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF1110F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMN_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_cmn_i_t1_a_flags_positiveresult_7_f1110f00() {
    // Test aarch32_CMN_i_T1_A flag computation: PositiveResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x32);
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF1110F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

// ============================================================================
// aarch32_CMP_i_A Tests
// ============================================================================

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_cmp_i_a1_a_field_cond_0_min_0_03500000() {
    // Encoding: 0x03500000
    // Test aarch32_CMP_i_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, imm12=0, Rn=0
    let encoding: u32 = 0x03500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_cmp_i_a1_a_field_cond_1_poweroftwo_0_13500000() {
    // Encoding: 0x13500000
    // Test aarch32_CMP_i_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, Rn=0, imm12=0
    let encoding: u32 = 0x13500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_cmp_i_a1_a_field_cond_2_poweroftwo_0_23500000() {
    // Encoding: 0x23500000
    // Test aarch32_CMP_i_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=2, imm12=0
    let encoding: u32 = 0x23500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_cmp_i_a1_a_field_cond_3_poweroftwo_0_33500000() {
    // Encoding: 0x33500000
    // Test aarch32_CMP_i_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, cond=3, Rn=0
    let encoding: u32 = 0x33500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_cmp_i_a1_a_field_cond_4_poweroftwo_0_43500000() {
    // Encoding: 0x43500000
    // Test aarch32_CMP_i_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=4, Rn=0, imm12=0
    let encoding: u32 = 0x43500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_cmp_i_a1_a_field_cond_5_poweroftwo_0_53500000() {
    // Encoding: 0x53500000
    // Test aarch32_CMP_i_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, Rn=0, cond=5
    let encoding: u32 = 0x53500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_cmp_i_a1_a_field_cond_6_poweroftwo_0_63500000() {
    // Encoding: 0x63500000
    // Test aarch32_CMP_i_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, Rn=0, cond=6
    let encoding: u32 = 0x63500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_cmp_i_a1_a_field_cond_7_poweroftwo_0_73500000() {
    // Encoding: 0x73500000
    // Test aarch32_CMP_i_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=7, imm12=0
    let encoding: u32 = 0x73500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_cmp_i_a1_a_field_cond_8_poweroftwo_0_83500000() {
    // Encoding: 0x83500000
    // Test aarch32_CMP_i_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=8, imm12=0
    let encoding: u32 = 0x83500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_cmp_i_a1_a_field_cond_9_poweroftwo_0_93500000() {
    // Encoding: 0x93500000
    // Test aarch32_CMP_i_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, cond=9, Rn=0
    let encoding: u32 = 0x93500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_cmp_i_a1_a_field_cond_10_poweroftwo_0_a3500000() {
    // Encoding: 0xA3500000
    // Test aarch32_CMP_i_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, Rn=0, imm12=0
    let encoding: u32 = 0xA3500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_cmp_i_a1_a_field_cond_11_poweroftwo_0_b3500000() {
    // Encoding: 0xB3500000
    // Test aarch32_CMP_i_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=11, imm12=0
    let encoding: u32 = 0xB3500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_cmp_i_a1_a_field_cond_12_poweroftwo_0_c3500000() {
    // Encoding: 0xC3500000
    // Test aarch32_CMP_i_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, imm12=0, Rn=0
    let encoding: u32 = 0xC3500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_cmp_i_a1_a_field_cond_13_poweroftwo_0_d3500000() {
    // Encoding: 0xD3500000
    // Test aarch32_CMP_i_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, cond=13, Rn=0
    let encoding: u32 = 0xD3500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_cmp_i_a1_a_field_cond_14_poweroftwo_0_e3500000() {
    // Encoding: 0xE3500000
    // Test aarch32_CMP_i_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, imm12=0, Rn=0
    let encoding: u32 = 0xE3500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_cmp_i_a1_a_field_cond_15_max_0_f3500000() {
    // Encoding: 0xF3500000
    // Test aarch32_CMP_i_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, Rn=0, imm12=0
    let encoding: u32 = 0xF3500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmp_i_a1_a_field_rn_0_min_0_03500000() {
    // Encoding: 0x03500000
    // Test aarch32_CMP_i_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, imm12=0, Rn=0
    let encoding: u32 = 0x03500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmp_i_a1_a_field_rn_1_poweroftwo_0_03510000() {
    // Encoding: 0x03510000
    // Test aarch32_CMP_i_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=1, imm12=0
    let encoding: u32 = 0x03510000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_cmp_i_a1_a_field_imm12_0_zero_0_03500000() {
    // Encoding: 0x03500000
    // Test aarch32_CMP_i_A1_A field imm12 = 0 (Zero)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm12=0
    let encoding: u32 = 0x03500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_cmp_i_a1_a_field_imm12_1_poweroftwo_0_03500001() {
    // Encoding: 0x03500001
    // Test aarch32_CMP_i_A1_A field imm12 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm12=1, cond=0
    let encoding: u32 = 0x03500001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_cmp_i_a1_a_field_imm12_3_poweroftwominusone_0_03500003() {
    // Encoding: 0x03500003
    // Test aarch32_CMP_i_A1_A field imm12 = 3 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=3, Rn=0, cond=0
    let encoding: u32 = 0x03500003;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_cmp_i_a1_a_field_imm12_4_poweroftwo_0_03500004() {
    // Encoding: 0x03500004
    // Test aarch32_CMP_i_A1_A field imm12 = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm12=4, Rn=0
    let encoding: u32 = 0x03500004;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_cmp_i_a1_a_field_imm12_7_poweroftwominusone_0_03500007() {
    // Encoding: 0x03500007
    // Test aarch32_CMP_i_A1_A field imm12 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rn=0, imm12=7, cond=0
    let encoding: u32 = 0x03500007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_cmp_i_a1_a_field_imm12_8_poweroftwo_0_03500008() {
    // Encoding: 0x03500008
    // Test aarch32_CMP_i_A1_A field imm12 = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=8, cond=0, Rn=0
    let encoding: u32 = 0x03500008;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_cmp_i_a1_a_field_imm12_15_poweroftwominusone_0_0350000f() {
    // Encoding: 0x0350000F
    // Test aarch32_CMP_i_A1_A field imm12 = 15 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rn=0, cond=0, imm12=15
    let encoding: u32 = 0x0350000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_cmp_i_a1_a_field_imm12_16_poweroftwo_0_03500010() {
    // Encoding: 0x03500010
    // Test aarch32_CMP_i_A1_A field imm12 = 16 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=16, Rn=0, cond=0
    let encoding: u32 = 0x03500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_cmp_i_a1_a_field_imm12_31_poweroftwominusone_0_0350001f() {
    // Encoding: 0x0350001F
    // Test aarch32_CMP_i_A1_A field imm12 = 31 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm12=31
    let encoding: u32 = 0x0350001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_cmp_i_a1_a_field_imm12_32_poweroftwo_0_03500020() {
    // Encoding: 0x03500020
    // Test aarch32_CMP_i_A1_A field imm12 = 32 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm12=32
    let encoding: u32 = 0x03500020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_cmp_i_a1_a_field_imm12_63_poweroftwominusone_0_0350003f() {
    // Encoding: 0x0350003F
    // Test aarch32_CMP_i_A1_A field imm12 = 63 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm12=63
    let encoding: u32 = 0x0350003F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_cmp_i_a1_a_field_imm12_64_poweroftwo_0_03500040() {
    // Encoding: 0x03500040
    // Test aarch32_CMP_i_A1_A field imm12 = 64 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm12=64, cond=0
    let encoding: u32 = 0x03500040;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch32_cmp_i_a1_a_field_imm12_127_poweroftwominusone_0_0350007f() {
    // Encoding: 0x0350007F
    // Test aarch32_CMP_i_A1_A field imm12 = 127 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=127, cond=0, Rn=0
    let encoding: u32 = 0x0350007F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_cmp_i_a1_a_field_imm12_128_poweroftwo_0_03500080() {
    // Encoding: 0x03500080
    // Test aarch32_CMP_i_A1_A field imm12 = 128 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm12=128
    let encoding: u32 = 0x03500080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch32_cmp_i_a1_a_field_imm12_255_poweroftwominusone_0_035000ff() {
    // Encoding: 0x035000FF
    // Test aarch32_CMP_i_A1_A field imm12 = 255 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm12=255, Rn=0
    let encoding: u32 = 0x035000FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch32_cmp_i_a1_a_field_imm12_256_poweroftwo_0_03500100() {
    // Encoding: 0x03500100
    // Test aarch32_CMP_i_A1_A field imm12 = 256 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=256, cond=0, Rn=0
    let encoding: u32 = 0x03500100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch32_cmp_i_a1_a_field_imm12_511_poweroftwominusone_0_035001ff() {
    // Encoding: 0x035001FF
    // Test aarch32_CMP_i_A1_A field imm12 = 511 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm12=511
    let encoding: u32 = 0x035001FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch32_cmp_i_a1_a_field_imm12_512_poweroftwo_0_03500200() {
    // Encoding: 0x03500200
    // Test aarch32_CMP_i_A1_A field imm12 = 512 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=512, cond=0, Rn=0
    let encoding: u32 = 0x03500200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1023, boundary: PowerOfTwoMinusOne }
/// 2^10 - 1 = 1023
#[test]
fn test_aarch32_cmp_i_a1_a_field_imm12_1023_poweroftwominusone_0_035003ff() {
    // Encoding: 0x035003FF
    // Test aarch32_CMP_i_A1_A field imm12 = 1023 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rn=0, cond=0, imm12=1023
    let encoding: u32 = 0x035003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch32_cmp_i_a1_a_field_imm12_1024_poweroftwo_0_03500400() {
    // Encoding: 0x03500400
    // Test aarch32_CMP_i_A1_A field imm12 = 1024 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=1024, cond=0, Rn=0
    let encoding: u32 = 0x03500400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 2047, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (2047)
#[test]
fn test_aarch32_cmp_i_a1_a_field_imm12_2047_poweroftwominusone_0_035007ff() {
    // Encoding: 0x035007FF
    // Test aarch32_CMP_i_A1_A field imm12 = 2047 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rn=0, cond=0, imm12=2047
    let encoding: u32 = 0x035007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 2048, boundary: PowerOfTwo }
/// power of 2 (2^11 = 2048)
#[test]
fn test_aarch32_cmp_i_a1_a_field_imm12_2048_poweroftwo_0_03500800() {
    // Encoding: 0x03500800
    // Test aarch32_CMP_i_A1_A field imm12 = 2048 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm12=2048, cond=0
    let encoding: u32 = 0x03500800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 4095, boundary: Max }
/// maximum immediate (4095)
#[test]
fn test_aarch32_cmp_i_a1_a_field_imm12_4095_max_0_03500fff() {
    // Encoding: 0x03500FFF
    // Test aarch32_CMP_i_A1_A field imm12 = 4095 (Max)
    // ISET: A32
    // Fields: Rn=0, cond=0, imm12=4095
    let encoding: u32 = 0x03500FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_cmp_i_a1_a_combo_0_0_03500000() {
    // Encoding: 0x03500000
    // Test aarch32_CMP_i_A1_A field combination: cond=0, Rn=0, imm12=0
    // ISET: A32
    // Fields: Rn=0, cond=0, imm12=0
    let encoding: u32 = 0x03500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_cmp_i_a1_a_special_cond_0_condition_eq_0_03500000() {
    // Encoding: 0x03500000
    // Test aarch32_CMP_i_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rn=0, cond=0, imm12=0
    let encoding: u32 = 0x03500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_cmp_i_a1_a_special_cond_1_condition_ne_0_13500000() {
    // Encoding: 0x13500000
    // Test aarch32_CMP_i_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: imm12=0, Rn=0, cond=1
    let encoding: u32 = 0x13500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_cmp_i_a1_a_special_cond_2_condition_cs_hs_0_23500000() {
    // Encoding: 0x23500000
    // Test aarch32_CMP_i_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: imm12=0, cond=2, Rn=0
    let encoding: u32 = 0x23500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_cmp_i_a1_a_special_cond_3_condition_cc_lo_0_33500000() {
    // Encoding: 0x33500000
    // Test aarch32_CMP_i_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: imm12=0, Rn=0, cond=3
    let encoding: u32 = 0x33500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_cmp_i_a1_a_special_cond_4_condition_mi_0_43500000() {
    // Encoding: 0x43500000
    // Test aarch32_CMP_i_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: imm12=0, Rn=0, cond=4
    let encoding: u32 = 0x43500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_cmp_i_a1_a_special_cond_5_condition_pl_0_53500000() {
    // Encoding: 0x53500000
    // Test aarch32_CMP_i_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: imm12=0, Rn=0, cond=5
    let encoding: u32 = 0x53500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_cmp_i_a1_a_special_cond_6_condition_vs_0_63500000() {
    // Encoding: 0x63500000
    // Test aarch32_CMP_i_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: imm12=0, Rn=0, cond=6
    let encoding: u32 = 0x63500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_cmp_i_a1_a_special_cond_7_condition_vc_0_73500000() {
    // Encoding: 0x73500000
    // Test aarch32_CMP_i_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: imm12=0, Rn=0, cond=7
    let encoding: u32 = 0x73500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_cmp_i_a1_a_special_cond_8_condition_hi_0_83500000() {
    // Encoding: 0x83500000
    // Test aarch32_CMP_i_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, imm12=0, Rn=0
    let encoding: u32 = 0x83500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_cmp_i_a1_a_special_cond_9_condition_ls_0_93500000() {
    // Encoding: 0x93500000
    // Test aarch32_CMP_i_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rn=0, cond=9, imm12=0
    let encoding: u32 = 0x93500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_cmp_i_a1_a_special_cond_10_condition_ge_0_a3500000() {
    // Encoding: 0xA3500000
    // Test aarch32_CMP_i_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: imm12=0, cond=10, Rn=0
    let encoding: u32 = 0xA3500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_cmp_i_a1_a_special_cond_11_condition_lt_0_b3500000() {
    // Encoding: 0xB3500000
    // Test aarch32_CMP_i_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rn=0, cond=11, imm12=0
    let encoding: u32 = 0xB3500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_cmp_i_a1_a_special_cond_12_condition_gt_0_c3500000() {
    // Encoding: 0xC3500000
    // Test aarch32_CMP_i_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: imm12=0, Rn=0, cond=12
    let encoding: u32 = 0xC3500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_cmp_i_a1_a_special_cond_13_condition_le_0_d3500000() {
    // Encoding: 0xD3500000
    // Test aarch32_CMP_i_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rn=0, imm12=0, cond=13
    let encoding: u32 = 0xD3500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_cmp_i_a1_a_special_cond_14_condition_al_0_e3500000() {
    // Encoding: 0xE3500000
    // Test aarch32_CMP_i_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: imm12=0, cond=14, Rn=0
    let encoding: u32 = 0xE3500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_cmp_i_a1_a_special_cond_15_condition_nv_0_f3500000() {
    // Encoding: 0xF3500000
    // Test aarch32_CMP_i_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rn=0, cond=15, imm12=0
    let encoding: u32 = 0xF3500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `field Rn 24 +: 3`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmp_i_t1_a_field_rn_0_min_0_28000000() {
    // Thumb encoding (32): 0x28000000
    // Test aarch32_CMP_i_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: imm8=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x28000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `field Rn 24 +: 3`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmp_i_t1_a_field_rn_1_poweroftwo_0_29000000() {
    // Thumb encoding (32): 0x29000000
    // Test aarch32_CMP_i_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=0, Rn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x29000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_cmp_i_t1_a_field_imm8_0_zero_0_28000000() {
    // Thumb encoding (32): 0x28000000
    // Test aarch32_CMP_i_T1_A field imm8 = 0 (Zero)
    // ISET: T32
    // Fields: Rn=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x28000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_cmp_i_t1_a_field_imm8_1_poweroftwo_0_28010000() {
    // Thumb encoding (32): 0x28010000
    // Test aarch32_CMP_i_T1_A field imm8 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, imm8=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x28010000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_cmp_i_t1_a_field_imm8_3_poweroftwominusone_0_28030000() {
    // Thumb encoding (32): 0x28030000
    // Test aarch32_CMP_i_T1_A field imm8 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rn=0, imm8=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x28030000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_cmp_i_t1_a_field_imm8_4_poweroftwo_0_28040000() {
    // Thumb encoding (32): 0x28040000
    // Test aarch32_CMP_i_T1_A field imm8 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=4, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x28040000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_cmp_i_t1_a_field_imm8_7_poweroftwominusone_0_28070000() {
    // Thumb encoding (32): 0x28070000
    // Test aarch32_CMP_i_T1_A field imm8 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rn=0, imm8=7
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x28070000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_cmp_i_t1_a_field_imm8_8_poweroftwo_0_28080000() {
    // Thumb encoding (32): 0x28080000
    // Test aarch32_CMP_i_T1_A field imm8 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=8, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x28080000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_cmp_i_t1_a_field_imm8_15_poweroftwominusone_0_280f0000() {
    // Thumb encoding (32): 0x280F0000
    // Test aarch32_CMP_i_T1_A field imm8 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rn=0, imm8=15
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x280F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_cmp_i_t1_a_field_imm8_16_poweroftwo_0_28100000() {
    // Thumb encoding (32): 0x28100000
    // Test aarch32_CMP_i_T1_A field imm8 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=16, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x28100000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_cmp_i_t1_a_field_imm8_31_poweroftwominusone_0_281f0000() {
    // Thumb encoding (32): 0x281F0000
    // Test aarch32_CMP_i_T1_A field imm8 = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=31, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x281F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_cmp_i_t1_a_field_imm8_32_poweroftwo_0_28200000() {
    // Thumb encoding (32): 0x28200000
    // Test aarch32_CMP_i_T1_A field imm8 = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, imm8=32
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x28200000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_cmp_i_t1_a_field_imm8_63_poweroftwominusone_0_283f0000() {
    // Thumb encoding (32): 0x283F0000
    // Test aarch32_CMP_i_T1_A field imm8 = 63 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rn=0, imm8=63
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x283F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_cmp_i_t1_a_field_imm8_64_poweroftwo_0_28400000() {
    // Thumb encoding (32): 0x28400000
    // Test aarch32_CMP_i_T1_A field imm8 = 64 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, imm8=64
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x28400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 127, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (127)
#[test]
fn test_aarch32_cmp_i_t1_a_field_imm8_127_poweroftwominusone_0_287f0000() {
    // Thumb encoding (32): 0x287F0000
    // Test aarch32_CMP_i_T1_A field imm8 = 127 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=127, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x287F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_cmp_i_t1_a_field_imm8_128_poweroftwo_0_28800000() {
    // Thumb encoding (32): 0x28800000
    // Test aarch32_CMP_i_T1_A field imm8 = 128 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, imm8=128
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x28800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 255, boundary: Max }
/// maximum immediate (255)
#[test]
fn test_aarch32_cmp_i_t1_a_field_imm8_255_max_0_28ff0000() {
    // Thumb encoding (32): 0x28FF0000
    // Test aarch32_CMP_i_T1_A field imm8 = 255 (Max)
    // ISET: T32
    // Fields: Rn=0, imm8=255
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x28FF0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_cmp_i_t1_a_combo_0_0_28000000() {
    // Thumb encoding (32): 0x28000000
    // Test aarch32_CMP_i_T1_A field combination: Rn=0, imm8=0
    // ISET: T32
    // Fields: Rn=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x28000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `field i 26 +: 1`
/// Requirement: FieldBoundary { field: "i", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_cmp_i_t2_a_field_i_0_min_f00_f1b00f00() {
    // Thumb encoding (32): 0xF1B00F00
    // Test aarch32_CMP_i_T2_A field i = 0 (Min)
    // ISET: T32
    // Fields: imm3=0, imm8=0, i=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B00F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `field i 26 +: 1`
/// Requirement: FieldBoundary { field: "i", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_cmp_i_t2_a_field_i_1_max_f00_f5b00f00() {
    // Thumb encoding (32): 0xF5B00F00
    // Test aarch32_CMP_i_T2_A field i = 1 (Max)
    // ISET: T32
    // Fields: i=1, imm8=0, imm3=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF5B00F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmp_i_t2_a_field_rn_0_min_f00_f1b00f00() {
    // Thumb encoding (32): 0xF1B00F00
    // Test aarch32_CMP_i_T2_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: i=0, imm3=0, Rn=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B00F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmp_i_t2_a_field_rn_1_poweroftwo_f00_f1b10f00() {
    // Thumb encoding (32): 0xF1B10F00
    // Test aarch32_CMP_i_T2_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, imm3=0, imm8=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B10F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_cmp_i_t2_a_field_imm3_0_zero_f00_f1b00f00() {
    // Thumb encoding (32): 0xF1B00F00
    // Test aarch32_CMP_i_T2_A field imm3 = 0 (Zero)
    // ISET: T32
    // Fields: imm8=0, Rn=0, i=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B00F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_cmp_i_t2_a_field_imm3_1_poweroftwo_f00_f1b01f00() {
    // Thumb encoding (32): 0xF1B01F00
    // Test aarch32_CMP_i_T2_A field imm3 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, i=0, imm8=0, imm3=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B01F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch32_cmp_i_t2_a_field_imm3_3_poweroftwominusone_f00_f1b03f00() {
    // Thumb encoding (32): 0xF1B03F00
    // Test aarch32_CMP_i_T2_A field imm3 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm3=3, imm8=0, Rn=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B03F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch32_cmp_i_t2_a_field_imm3_7_max_f00_f1b07f00() {
    // Thumb encoding (32): 0xF1B07F00
    // Test aarch32_CMP_i_T2_A field imm3 = 7 (Max)
    // ISET: T32
    // Fields: i=0, imm3=7, imm8=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B07F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_cmp_i_t2_a_field_imm8_0_zero_f00_f1b00f00() {
    // Thumb encoding (32): 0xF1B00F00
    // Test aarch32_CMP_i_T2_A field imm8 = 0 (Zero)
    // ISET: T32
    // Fields: Rn=0, imm3=0, i=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B00F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_cmp_i_t2_a_field_imm8_1_poweroftwo_f00_f1b00f01() {
    // Thumb encoding (32): 0xF1B00F01
    // Test aarch32_CMP_i_T2_A field imm8 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=0, i=0, Rn=0, imm8=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B00F01;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_cmp_i_t2_a_field_imm8_3_poweroftwominusone_f00_f1b00f03() {
    // Thumb encoding (32): 0xF1B00F03
    // Test aarch32_CMP_i_T2_A field imm8 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: i=0, imm3=0, Rn=0, imm8=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B00F03;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_cmp_i_t2_a_field_imm8_4_poweroftwo_f00_f1b00f04() {
    // Thumb encoding (32): 0xF1B00F04
    // Test aarch32_CMP_i_T2_A field imm8 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=0, i=0, imm8=4, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B00F04;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_cmp_i_t2_a_field_imm8_7_poweroftwominusone_f00_f1b00f07() {
    // Thumb encoding (32): 0xF1B00F07
    // Test aarch32_CMP_i_T2_A field imm8 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: i=0, Rn=0, imm8=7, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B00F07;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_cmp_i_t2_a_field_imm8_8_poweroftwo_f00_f1b00f08() {
    // Thumb encoding (32): 0xF1B00F08
    // Test aarch32_CMP_i_T2_A field imm8 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=0, i=0, imm8=8, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B00F08;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_cmp_i_t2_a_field_imm8_15_poweroftwominusone_f00_f1b00f0f() {
    // Thumb encoding (32): 0xF1B00F0F
    // Test aarch32_CMP_i_T2_A field imm8 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: i=0, Rn=0, imm8=15, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B00F0F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_cmp_i_t2_a_field_imm8_16_poweroftwo_f00_f1b00f10() {
    // Thumb encoding (32): 0xF1B00F10
    // Test aarch32_CMP_i_T2_A field imm8 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: i=0, Rn=0, imm8=16, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B00F10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_cmp_i_t2_a_field_imm8_31_poweroftwominusone_f00_f1b00f1f() {
    // Thumb encoding (32): 0xF1B00F1F
    // Test aarch32_CMP_i_T2_A field imm8 = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: i=0, Rn=0, imm3=0, imm8=31
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B00F1F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_cmp_i_t2_a_field_imm8_32_poweroftwo_f00_f1b00f20() {
    // Thumb encoding (32): 0xF1B00F20
    // Test aarch32_CMP_i_T2_A field imm8 = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, imm3=0, imm8=32, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B00F20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_cmp_i_t2_a_field_imm8_63_poweroftwominusone_f00_f1b00f3f() {
    // Thumb encoding (32): 0xF1B00F3F
    // Test aarch32_CMP_i_T2_A field imm8 = 63 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm3=0, imm8=63, Rn=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B00F3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_cmp_i_t2_a_field_imm8_64_poweroftwo_f00_f1b00f40() {
    // Thumb encoding (32): 0xF1B00F40
    // Test aarch32_CMP_i_T2_A field imm8 = 64 (PowerOfTwo)
    // ISET: T32
    // Fields: i=0, Rn=0, imm8=64, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B00F40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 127, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (127)
#[test]
fn test_aarch32_cmp_i_t2_a_field_imm8_127_poweroftwominusone_f00_f1b00f7f() {
    // Thumb encoding (32): 0xF1B00F7F
    // Test aarch32_CMP_i_T2_A field imm8 = 127 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: i=0, imm3=0, Rn=0, imm8=127
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B00F7F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_cmp_i_t2_a_field_imm8_128_poweroftwo_f00_f1b00f80() {
    // Thumb encoding (32): 0xF1B00F80
    // Test aarch32_CMP_i_T2_A field imm8 = 128 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, imm3=0, imm8=128, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B00F80;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 255, boundary: Max }
/// maximum immediate (255)
#[test]
fn test_aarch32_cmp_i_t2_a_field_imm8_255_max_f00_f1b00fff() {
    // Thumb encoding (32): 0xF1B00FFF
    // Test aarch32_CMP_i_T2_A field imm8 = 255 (Max)
    // ISET: T32
    // Fields: imm8=255, i=0, Rn=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B00FFF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// i=0 (minimum value)
#[test]
fn test_aarch32_cmp_i_t2_a_combo_0_f00_f1b00f00() {
    // Thumb encoding (32): 0xF1B00F00
    // Test aarch32_CMP_i_T2_A field combination: i=0, Rn=0, imm3=0, imm8=0
    // ISET: T32
    // Fields: imm3=0, imm8=0, i=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B00F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }), rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_cmp_i_t2_a_invalid_0_f00_f1b00f00() {
    // Thumb encoding (32): 0xF1B00F00
    // Test aarch32_CMP_i_T2_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: LitInt(15) }
    // ISET: T32
    // Fields: imm8=0, imm3=0, i=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B00F00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_cmp_i_t2_a_invalid_1_f00_f1b00f00() {
    // Thumb encoding (32): 0xF1B00F00
    // Test aarch32_CMP_i_T2_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: i=0, imm3=0, imm8=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1B00F00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// zero == zero (32-bit)
#[test]
fn test_aarch32_cmp_i_a1_a_oracle_32_0_6b02003f() {
    // Test CMP 32-bit: zero == zero (oracle)
    // Encoding: 0x6B02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x6B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// zero == zero (64-bit)
#[test]
fn test_aarch32_cmp_i_a1_a_oracle_64_0_eb02003f() {
    // Test CMP 64-bit: zero == zero (oracle)
    // Encoding: 0xEB02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xEB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// equal values (32-bit)
#[test]
fn test_aarch32_cmp_i_a1_a_oracle_32_1_6b02003f() {
    // Test CMP 32-bit: equal values (oracle)
    // Encoding: 0x6B02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x6B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// equal values (64-bit)
#[test]
fn test_aarch32_cmp_i_a1_a_oracle_64_1_eb02003f() {
    // Test CMP 64-bit: equal values (oracle)
    // Encoding: 0xEB02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xEB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// greater than (32-bit)
#[test]
fn test_aarch32_cmp_i_a1_a_oracle_32_2_6b02003f() {
    // Test CMP 32-bit: greater than (oracle)
    // Encoding: 0x6B02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xA);
    set_w(&mut cpu, 2, 0x5);
    let encoding: u32 = 0x6B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// greater than (64-bit)
#[test]
fn test_aarch32_cmp_i_a1_a_oracle_64_2_eb02003f() {
    // Test CMP 64-bit: greater than (oracle)
    // Encoding: 0xEB02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xA);
    set_w(&mut cpu, 2, 0x5);
    let encoding: u32 = 0xEB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// less than (32-bit)
#[test]
fn test_aarch32_cmp_i_a1_a_oracle_32_3_6b02003f() {
    // Test CMP 32-bit: less than (oracle)
    // Encoding: 0x6B02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xA);
    set_w(&mut cpu, 1, 0x5);
    let encoding: u32 = 0x6B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// less than (64-bit)
#[test]
fn test_aarch32_cmp_i_a1_a_oracle_64_3_eb02003f() {
    // Test CMP 64-bit: less than (oracle)
    // Encoding: 0xEB02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xA);
    set_w(&mut cpu, 1, 0x5);
    let encoding: u32 = 0xEB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// max value (32-bit)
#[test]
fn test_aarch32_cmp_i_a1_a_oracle_32_4_6b02003f() {
    // Test CMP 32-bit: max value (oracle)
    // Encoding: 0x6B02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x6B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// max value (64-bit)
#[test]
fn test_aarch32_cmp_i_a1_a_oracle_64_4_eb02003f() {
    // Test CMP 64-bit: max value (oracle)
    // Encoding: 0xEB02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xEB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// negative (MSB set) (32-bit)
#[test]
fn test_aarch32_cmp_i_a1_a_oracle_32_5_6b02003f() {
    // Test CMP 32-bit: negative (MSB set) (oracle)
    // Encoding: 0x6B02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x6B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// negative (MSB set) (64-bit)
#[test]
fn test_aarch32_cmp_i_a1_a_oracle_64_5_eb02003f() {
    // Test CMP 64-bit: negative (MSB set) (oracle)
    // Encoding: 0xEB02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xEB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// overflow boundary (32-bit)
#[test]
fn test_aarch32_cmp_i_a1_a_oracle_32_6_6b02003f() {
    // Test CMP 32-bit: overflow boundary (oracle)
    // Encoding: 0x6B02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x6B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// overflow boundary (64-bit)
#[test]
fn test_aarch32_cmp_i_a1_a_oracle_64_6_eb02003f() {
    // Test CMP 64-bit: overflow boundary (oracle)
    // Encoding: 0xEB02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xEB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_cmp_i_a1_a_flags_zeroresult_0_03510000() {
    // Test aarch32_CMP_i_A1_A flag computation: ZeroResult
    // Encoding: 0x03510000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x03510000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_cmp_i_a1_a_flags_zeroresult_1_03510000() {
    // Test aarch32_CMP_i_A1_A flag computation: ZeroResult
    // Encoding: 0x03510000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x03510000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_cmp_i_a1_a_flags_negativeresult_2_03510000() {
    // Test aarch32_CMP_i_A1_A flag computation: NegativeResult
    // Encoding: 0x03510000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x03510000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_cmp_i_a1_a_flags_unsignedoverflow_3_03510000() {
    // Test aarch32_CMP_i_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x03510000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x03510000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_cmp_i_a1_a_flags_unsignedoverflow_4_03510000() {
    // Test aarch32_CMP_i_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x03510000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x03510000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_cmp_i_a1_a_flags_signedoverflow_5_03510000() {
    // Test aarch32_CMP_i_A1_A flag computation: SignedOverflow
    // Encoding: 0x03510000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x03510000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_cmp_i_a1_a_flags_signedoverflow_6_03510000() {
    // Test aarch32_CMP_i_A1_A flag computation: SignedOverflow
    // Encoding: 0x03510000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x03510000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMP_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_cmp_i_a1_a_flags_positiveresult_7_03510000() {
    // Test aarch32_CMP_i_A1_A flag computation: PositiveResult
    // Encoding: 0x03510000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u32 = 0x03510000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 0 (32)
#[test]
fn test_aarch32_cmp_i_t1_a_lslv_oracle_32_0_28020020() {
    // Test LSLV 32-bit: shift by 0 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x12345678);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x12345678, "W0 should be 0x12345678");
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 0 (64)
#[test]
fn test_aarch32_cmp_i_t1_a_lslv_oracle_64_0_a8020020() {
    // Test LSLV 64-bit: shift by 0 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x12345678);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x12345678,
        "X0 should be 0x0000000012345678"
    );
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4 (32)
#[test]
fn test_aarch32_cmp_i_t1_a_lslv_oracle_32_1_28020020() {
    // Test LSLV 32-bit: shift by 4 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x4);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x23456780, "W0 should be 0x23456780");
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 4 (64)
#[test]
fn test_aarch32_cmp_i_t1_a_lslv_oracle_64_1_a8020020() {
    // Test LSLV 64-bit: shift by 4 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x4);
    set_w(&mut cpu, 1, 0x12345678);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x23456780,
        "X0 should be 0x0000000123456780"
    );
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 8 (32)
#[test]
fn test_aarch32_cmp_i_t1_a_lslv_oracle_32_2_28020020() {
    // Test LSLV 32-bit: shift by 8 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x8);
    set_w(&mut cpu, 1, 0x12345678);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x34567800, "W0 should be 0x34567800");
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 8 (64)
#[test]
fn test_aarch32_cmp_i_t1_a_lslv_oracle_64_2_a8020020() {
    // Test LSLV 64-bit: shift by 8 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x8);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x34567800,
        "X0 should be 0x0000001234567800"
    );
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1 (32)
#[test]
fn test_aarch32_cmp_i_t1_a_lslv_oracle_32_3_28020020() {
    // Test LSLV 32-bit: MSB set, shift 1 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x00000000");
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// MSB set, shift 1 (64)
#[test]
fn test_aarch32_cmp_i_t1_a_lslv_oracle_64_3_a8020020() {
    // Test LSLV 64-bit: MSB set, shift 1 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSB set, max shift (32)
#[test]
fn test_aarch32_cmp_i_t1_a_lslv_oracle_32_4_28020020() {
    // Test LSLV 32-bit: LSB set, max shift (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x3F);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x80000000, "W0 should be 0x80000000");
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSB set, max shift (64)
#[test]
fn test_aarch32_cmp_i_t1_a_lslv_oracle_64_4_a8020020() {
    // Test LSLV 64-bit: LSB set, max shift (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0x3F);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x8000000000000000");
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// all ones, shift 32 (32)
#[test]
fn test_aarch32_cmp_i_t1_a_lslv_oracle_32_5_28020020() {
    // Test LSLV 32-bit: all ones, shift 32 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x20);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "W0 should be 0xFFFFFFFF");
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all ones, shift 32 (64)
#[test]
fn test_aarch32_cmp_i_t1_a_lslv_oracle_64_5_a8020020() {
    // Test LSLV 64-bit: all ones, shift 32 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x20);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0xFFFFFFFF00000000");
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift
#[test]
fn test_aarch32_cmp_i_t1_a_t16_oracle_0_29000000() {
    // Test T16 LSLS: no shift (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF, "R0 should be 0x000000FF");
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4
#[test]
fn test_aarch32_cmp_i_t1_a_t16_oracle_1_29000000() {
    // Test T16 LSLS: shift by 4 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x4);
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF0, "R0 should be 0x00000FF0");
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1
#[test]
fn test_aarch32_cmp_i_t1_a_t16_oracle_2_29000000() {
    // Test T16 LSLS: MSB set, shift 1 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x80000000);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift to MSB
#[test]
fn test_aarch32_cmp_i_t1_a_t16_oracle_3_29000000() {
    // Test T16 LSLS: shift to MSB (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1F);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x80000000, "R0 should be 0x80000000");
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_cmp_i_t1_a_flags_zeroresult_0_29000000() {
    // Test aarch32_CMP_i_T1_A flag computation: ZeroResult
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_cmp_i_t1_a_flags_zeroresult_1_29000000() {
    // Test aarch32_CMP_i_T1_A flag computation: ZeroResult
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_cmp_i_t1_a_flags_negativeresult_2_29000000() {
    // Test aarch32_CMP_i_T1_A flag computation: NegativeResult
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_cmp_i_t1_a_flags_unsignedoverflow_3_29000000() {
    // Test aarch32_CMP_i_T1_A flag computation: UnsignedOverflow
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_cmp_i_t1_a_flags_unsignedoverflow_4_29000000() {
    // Test aarch32_CMP_i_T1_A flag computation: UnsignedOverflow
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x2);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_cmp_i_t1_a_flags_signedoverflow_5_29000000() {
    // Test aarch32_CMP_i_T1_A flag computation: SignedOverflow
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_cmp_i_t1_a_flags_signedoverflow_6_29000000() {
    // Test aarch32_CMP_i_T1_A flag computation: SignedOverflow
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMP_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_cmp_i_t1_a_flags_positiveresult_7_29000000() {
    // Test aarch32_CMP_i_T1_A flag computation: PositiveResult
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// zero == zero (32-bit)
#[test]
fn test_aarch32_cmp_i_t2_a_oracle_32_0_6b02003f() {
    // Test CMP 32-bit: zero == zero (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x6B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// zero == zero (64-bit)
#[test]
fn test_aarch32_cmp_i_t2_a_oracle_64_0_eb02003f() {
    // Test CMP 64-bit: zero == zero (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xEB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// equal values (32-bit)
#[test]
fn test_aarch32_cmp_i_t2_a_oracle_32_1_6b02003f() {
    // Test CMP 32-bit: equal values (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x6B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// equal values (64-bit)
#[test]
fn test_aarch32_cmp_i_t2_a_oracle_64_1_eb02003f() {
    // Test CMP 64-bit: equal values (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xEB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// greater than (32-bit)
#[test]
fn test_aarch32_cmp_i_t2_a_oracle_32_2_6b02003f() {
    // Test CMP 32-bit: greater than (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x5);
    set_w(&mut cpu, 1, 0xA);
    let encoding: u32 = 0x6B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// greater than (64-bit)
#[test]
fn test_aarch32_cmp_i_t2_a_oracle_64_2_eb02003f() {
    // Test CMP 64-bit: greater than (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xA);
    set_w(&mut cpu, 2, 0x5);
    let encoding: u32 = 0xEB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// less than (32-bit)
#[test]
fn test_aarch32_cmp_i_t2_a_oracle_32_3_6b02003f() {
    // Test CMP 32-bit: less than (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xA);
    set_w(&mut cpu, 1, 0x5);
    let encoding: u32 = 0x6B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// less than (64-bit)
#[test]
fn test_aarch32_cmp_i_t2_a_oracle_64_3_eb02003f() {
    // Test CMP 64-bit: less than (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xA);
    set_w(&mut cpu, 1, 0x5);
    let encoding: u32 = 0xEB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// max value (32-bit)
#[test]
fn test_aarch32_cmp_i_t2_a_oracle_32_4_6b02003f() {
    // Test CMP 32-bit: max value (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x6B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// max value (64-bit)
#[test]
fn test_aarch32_cmp_i_t2_a_oracle_64_4_eb02003f() {
    // Test CMP 64-bit: max value (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xEB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// negative (MSB set) (32-bit)
#[test]
fn test_aarch32_cmp_i_t2_a_oracle_32_5_6b02003f() {
    // Test CMP 32-bit: negative (MSB set) (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x6B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// negative (MSB set) (64-bit)
#[test]
fn test_aarch32_cmp_i_t2_a_oracle_64_5_eb02003f() {
    // Test CMP 64-bit: negative (MSB set) (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xEB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// overflow boundary (32-bit)
#[test]
fn test_aarch32_cmp_i_t2_a_oracle_32_6_6b02003f() {
    // Test CMP 32-bit: overflow boundary (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x6B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// overflow boundary (64-bit)
#[test]
fn test_aarch32_cmp_i_t2_a_oracle_64_6_eb02003f() {
    // Test CMP 64-bit: overflow boundary (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xEB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_cmp_i_t2_a_flags_zeroresult_0_f1b10f00() {
    // Test aarch32_CMP_i_T2_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF1B10F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_cmp_i_t2_a_flags_zeroresult_1_f1b10f00() {
    // Test aarch32_CMP_i_T2_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF1B10F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_cmp_i_t2_a_flags_negativeresult_2_f1b10f00() {
    // Test aarch32_CMP_i_T2_A flag computation: NegativeResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF1B10F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_cmp_i_t2_a_flags_unsignedoverflow_3_f1b10f00() {
    // Test aarch32_CMP_i_T2_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xF1B10F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_cmp_i_t2_a_flags_unsignedoverflow_4_f1b10f00() {
    // Test aarch32_CMP_i_T2_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u32 = 0xF1B10F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_cmp_i_t2_a_flags_signedoverflow_5_f1b10f00() {
    // Test aarch32_CMP_i_T2_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xF1B10F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_cmp_i_t2_a_flags_signedoverflow_6_f1b10f00() {
    // Test aarch32_CMP_i_T2_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0xF1B10F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMP_i_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_cmp_i_t2_a_flags_positiveresult_7_f1b10f00() {
    // Test aarch32_CMP_i_T2_A flag computation: PositiveResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x32);
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF1B10F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

// ============================================================================
// aarch32_TST_i_A Tests
// ============================================================================

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_tst_i_a1_a_field_cond_0_min_0_03100000() {
    // Encoding: 0x03100000
    // Test aarch32_TST_i_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm12=0
    let encoding: u32 = 0x03100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_tst_i_a1_a_field_cond_1_poweroftwo_0_13100000() {
    // Encoding: 0x13100000
    // Test aarch32_TST_i_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=1, imm12=0
    let encoding: u32 = 0x13100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_tst_i_a1_a_field_cond_2_poweroftwo_0_23100000() {
    // Encoding: 0x23100000
    // Test aarch32_TST_i_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=2, Rn=0, imm12=0
    let encoding: u32 = 0x23100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_tst_i_a1_a_field_cond_3_poweroftwo_0_33100000() {
    // Encoding: 0x33100000
    // Test aarch32_TST_i_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, Rn=0, cond=3
    let encoding: u32 = 0x33100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_tst_i_a1_a_field_cond_4_poweroftwo_0_43100000() {
    // Encoding: 0x43100000
    // Test aarch32_TST_i_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=4, Rn=0, imm12=0
    let encoding: u32 = 0x43100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_tst_i_a1_a_field_cond_5_poweroftwo_0_53100000() {
    // Encoding: 0x53100000
    // Test aarch32_TST_i_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm12=0, cond=5
    let encoding: u32 = 0x53100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_tst_i_a1_a_field_cond_6_poweroftwo_0_63100000() {
    // Encoding: 0x63100000
    // Test aarch32_TST_i_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, cond=6, Rn=0
    let encoding: u32 = 0x63100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_tst_i_a1_a_field_cond_7_poweroftwo_0_73100000() {
    // Encoding: 0x73100000
    // Test aarch32_TST_i_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, cond=7, Rn=0
    let encoding: u32 = 0x73100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_tst_i_a1_a_field_cond_8_poweroftwo_0_83100000() {
    // Encoding: 0x83100000
    // Test aarch32_TST_i_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, Rn=0, cond=8
    let encoding: u32 = 0x83100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_tst_i_a1_a_field_cond_9_poweroftwo_0_93100000() {
    // Encoding: 0x93100000
    // Test aarch32_TST_i_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, Rn=0, cond=9
    let encoding: u32 = 0x93100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_tst_i_a1_a_field_cond_10_poweroftwo_0_a3100000() {
    // Encoding: 0xA3100000
    // Test aarch32_TST_i_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm12=0, cond=10
    let encoding: u32 = 0xA3100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_tst_i_a1_a_field_cond_11_poweroftwo_0_b3100000() {
    // Encoding: 0xB3100000
    // Test aarch32_TST_i_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm12=0, cond=11
    let encoding: u32 = 0xB3100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_tst_i_a1_a_field_cond_12_poweroftwo_0_c3100000() {
    // Encoding: 0xC3100000
    // Test aarch32_TST_i_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, Rn=0, imm12=0
    let encoding: u32 = 0xC3100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_tst_i_a1_a_field_cond_13_poweroftwo_0_d3100000() {
    // Encoding: 0xD3100000
    // Test aarch32_TST_i_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm12=0, cond=13
    let encoding: u32 = 0xD3100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_tst_i_a1_a_field_cond_14_poweroftwo_0_e3100000() {
    // Encoding: 0xE3100000
    // Test aarch32_TST_i_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, cond=14, Rn=0
    let encoding: u32 = 0xE3100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_tst_i_a1_a_field_cond_15_max_0_f3100000() {
    // Encoding: 0xF3100000
    // Test aarch32_TST_i_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, Rn=0, imm12=0
    let encoding: u32 = 0xF3100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_tst_i_a1_a_field_rn_0_min_0_03100000() {
    // Encoding: 0x03100000
    // Test aarch32_TST_i_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: imm12=0, cond=0, Rn=0
    let encoding: u32 = 0x03100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_tst_i_a1_a_field_rn_1_poweroftwo_0_03110000() {
    // Encoding: 0x03110000
    // Test aarch32_TST_i_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm12=0, Rn=1
    let encoding: u32 = 0x03110000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_tst_i_a1_a_field_imm12_0_zero_0_03100000() {
    // Encoding: 0x03100000
    // Test aarch32_TST_i_A1_A field imm12 = 0 (Zero)
    // ISET: A32
    // Fields: cond=0, imm12=0, Rn=0
    let encoding: u32 = 0x03100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_tst_i_a1_a_field_imm12_1_poweroftwo_0_03100001() {
    // Encoding: 0x03100001
    // Test aarch32_TST_i_A1_A field imm12 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=1, cond=0, Rn=0
    let encoding: u32 = 0x03100001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_tst_i_a1_a_field_imm12_3_poweroftwominusone_0_03100003() {
    // Encoding: 0x03100003
    // Test aarch32_TST_i_A1_A field imm12 = 3 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm12=3
    let encoding: u32 = 0x03100003;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_tst_i_a1_a_field_imm12_4_poweroftwo_0_03100004() {
    // Encoding: 0x03100004
    // Test aarch32_TST_i_A1_A field imm12 = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm12=4
    let encoding: u32 = 0x03100004;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_tst_i_a1_a_field_imm12_7_poweroftwominusone_0_03100007() {
    // Encoding: 0x03100007
    // Test aarch32_TST_i_A1_A field imm12 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rn=0, imm12=7, cond=0
    let encoding: u32 = 0x03100007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_tst_i_a1_a_field_imm12_8_poweroftwo_0_03100008() {
    // Encoding: 0x03100008
    // Test aarch32_TST_i_A1_A field imm12 = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm12=8
    let encoding: u32 = 0x03100008;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_tst_i_a1_a_field_imm12_15_poweroftwominusone_0_0310000f() {
    // Encoding: 0x0310000F
    // Test aarch32_TST_i_A1_A field imm12 = 15 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rn=0, cond=0, imm12=15
    let encoding: u32 = 0x0310000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_tst_i_a1_a_field_imm12_16_poweroftwo_0_03100010() {
    // Encoding: 0x03100010
    // Test aarch32_TST_i_A1_A field imm12 = 16 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=16, cond=0, Rn=0
    let encoding: u32 = 0x03100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_tst_i_a1_a_field_imm12_31_poweroftwominusone_0_0310001f() {
    // Encoding: 0x0310001F
    // Test aarch32_TST_i_A1_A field imm12 = 31 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rn=0, imm12=31, cond=0
    let encoding: u32 = 0x0310001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_tst_i_a1_a_field_imm12_32_poweroftwo_0_03100020() {
    // Encoding: 0x03100020
    // Test aarch32_TST_i_A1_A field imm12 = 32 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm12=32, cond=0
    let encoding: u32 = 0x03100020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_tst_i_a1_a_field_imm12_63_poweroftwominusone_0_0310003f() {
    // Encoding: 0x0310003F
    // Test aarch32_TST_i_A1_A field imm12 = 63 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm12=63, Rn=0
    let encoding: u32 = 0x0310003F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_tst_i_a1_a_field_imm12_64_poweroftwo_0_03100040() {
    // Encoding: 0x03100040
    // Test aarch32_TST_i_A1_A field imm12 = 64 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm12=64
    let encoding: u32 = 0x03100040;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch32_tst_i_a1_a_field_imm12_127_poweroftwominusone_0_0310007f() {
    // Encoding: 0x0310007F
    // Test aarch32_TST_i_A1_A field imm12 = 127 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rn=0, cond=0, imm12=127
    let encoding: u32 = 0x0310007F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_tst_i_a1_a_field_imm12_128_poweroftwo_0_03100080() {
    // Encoding: 0x03100080
    // Test aarch32_TST_i_A1_A field imm12 = 128 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=0, imm12=128
    let encoding: u32 = 0x03100080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch32_tst_i_a1_a_field_imm12_255_poweroftwominusone_0_031000ff() {
    // Encoding: 0x031000FF
    // Test aarch32_TST_i_A1_A field imm12 = 255 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rn=0, imm12=255, cond=0
    let encoding: u32 = 0x031000FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch32_tst_i_a1_a_field_imm12_256_poweroftwo_0_03100100() {
    // Encoding: 0x03100100
    // Test aarch32_TST_i_A1_A field imm12 = 256 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=256, cond=0, Rn=0
    let encoding: u32 = 0x03100100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch32_tst_i_a1_a_field_imm12_511_poweroftwominusone_0_031001ff() {
    // Encoding: 0x031001FF
    // Test aarch32_TST_i_A1_A field imm12 = 511 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=511, cond=0, Rn=0
    let encoding: u32 = 0x031001FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch32_tst_i_a1_a_field_imm12_512_poweroftwo_0_03100200() {
    // Encoding: 0x03100200
    // Test aarch32_TST_i_A1_A field imm12 = 512 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm12=512
    let encoding: u32 = 0x03100200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1023, boundary: PowerOfTwoMinusOne }
/// 2^10 - 1 = 1023
#[test]
fn test_aarch32_tst_i_a1_a_field_imm12_1023_poweroftwominusone_0_031003ff() {
    // Encoding: 0x031003FF
    // Test aarch32_TST_i_A1_A field imm12 = 1023 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=1023, cond=0, Rn=0
    let encoding: u32 = 0x031003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch32_tst_i_a1_a_field_imm12_1024_poweroftwo_0_03100400() {
    // Encoding: 0x03100400
    // Test aarch32_TST_i_A1_A field imm12 = 1024 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=1024, cond=0, Rn=0
    let encoding: u32 = 0x03100400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 2047, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (2047)
#[test]
fn test_aarch32_tst_i_a1_a_field_imm12_2047_poweroftwominusone_0_031007ff() {
    // Encoding: 0x031007FF
    // Test aarch32_TST_i_A1_A field imm12 = 2047 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=2047, cond=0, Rn=0
    let encoding: u32 = 0x031007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 2048, boundary: PowerOfTwo }
/// power of 2 (2^11 = 2048)
#[test]
fn test_aarch32_tst_i_a1_a_field_imm12_2048_poweroftwo_0_03100800() {
    // Encoding: 0x03100800
    // Test aarch32_TST_i_A1_A field imm12 = 2048 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=2048, Rn=0, cond=0
    let encoding: u32 = 0x03100800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 4095, boundary: Max }
/// maximum immediate (4095)
#[test]
fn test_aarch32_tst_i_a1_a_field_imm12_4095_max_0_03100fff() {
    // Encoding: 0x03100FFF
    // Test aarch32_TST_i_A1_A field imm12 = 4095 (Max)
    // ISET: A32
    // Fields: Rn=0, imm12=4095, cond=0
    let encoding: u32 = 0x03100FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_tst_i_a1_a_combo_0_0_03100000() {
    // Encoding: 0x03100000
    // Test aarch32_TST_i_A1_A field combination: cond=0, Rn=0, imm12=0
    // ISET: A32
    // Fields: imm12=0, Rn=0, cond=0
    let encoding: u32 = 0x03100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_tst_i_a1_a_special_cond_0_condition_eq_0_03100000() {
    // Encoding: 0x03100000
    // Test aarch32_TST_i_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rn=0, cond=0, imm12=0
    let encoding: u32 = 0x03100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_tst_i_a1_a_special_cond_1_condition_ne_0_13100000() {
    // Encoding: 0x13100000
    // Test aarch32_TST_i_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rn=0, imm12=0, cond=1
    let encoding: u32 = 0x13100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_tst_i_a1_a_special_cond_2_condition_cs_hs_0_23100000() {
    // Encoding: 0x23100000
    // Test aarch32_TST_i_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rn=0, cond=2, imm12=0
    let encoding: u32 = 0x23100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_tst_i_a1_a_special_cond_3_condition_cc_lo_0_33100000() {
    // Encoding: 0x33100000
    // Test aarch32_TST_i_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, Rn=0, imm12=0
    let encoding: u32 = 0x33100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_tst_i_a1_a_special_cond_4_condition_mi_0_43100000() {
    // Encoding: 0x43100000
    // Test aarch32_TST_i_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rn=0, imm12=0, cond=4
    let encoding: u32 = 0x43100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_tst_i_a1_a_special_cond_5_condition_pl_0_53100000() {
    // Encoding: 0x53100000
    // Test aarch32_TST_i_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rn=0, cond=5, imm12=0
    let encoding: u32 = 0x53100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_tst_i_a1_a_special_cond_6_condition_vs_0_63100000() {
    // Encoding: 0x63100000
    // Test aarch32_TST_i_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, imm12=0, Rn=0
    let encoding: u32 = 0x63100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_tst_i_a1_a_special_cond_7_condition_vc_0_73100000() {
    // Encoding: 0x73100000
    // Test aarch32_TST_i_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: imm12=0, cond=7, Rn=0
    let encoding: u32 = 0x73100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_tst_i_a1_a_special_cond_8_condition_hi_0_83100000() {
    // Encoding: 0x83100000
    // Test aarch32_TST_i_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: imm12=0, Rn=0, cond=8
    let encoding: u32 = 0x83100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_tst_i_a1_a_special_cond_9_condition_ls_0_93100000() {
    // Encoding: 0x93100000
    // Test aarch32_TST_i_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, imm12=0, Rn=0
    let encoding: u32 = 0x93100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_tst_i_a1_a_special_cond_10_condition_ge_0_a3100000() {
    // Encoding: 0xA3100000
    // Test aarch32_TST_i_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, Rn=0, imm12=0
    let encoding: u32 = 0xA3100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_tst_i_a1_a_special_cond_11_condition_lt_0_b3100000() {
    // Encoding: 0xB3100000
    // Test aarch32_TST_i_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: imm12=0, cond=11, Rn=0
    let encoding: u32 = 0xB3100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_tst_i_a1_a_special_cond_12_condition_gt_0_c3100000() {
    // Encoding: 0xC3100000
    // Test aarch32_TST_i_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rn=0, imm12=0, cond=12
    let encoding: u32 = 0xC3100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_tst_i_a1_a_special_cond_13_condition_le_0_d3100000() {
    // Encoding: 0xD3100000
    // Test aarch32_TST_i_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: imm12=0, cond=13, Rn=0
    let encoding: u32 = 0xD3100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_tst_i_a1_a_special_cond_14_condition_al_0_e3100000() {
    // Encoding: 0xE3100000
    // Test aarch32_TST_i_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rn=0, cond=14, imm12=0
    let encoding: u32 = 0xE3100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_tst_i_a1_a_special_cond_15_condition_nv_0_f3100000() {
    // Encoding: 0xF3100000
    // Test aarch32_TST_i_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: imm12=0, cond=15, Rn=0
    let encoding: u32 = 0xF3100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `field i 26 +: 1`
/// Requirement: FieldBoundary { field: "i", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_tst_i_t1_a_field_i_0_min_f00_f0100f00() {
    // Thumb encoding (32): 0xF0100F00
    // Test aarch32_TST_i_T1_A field i = 0 (Min)
    // ISET: T32
    // Fields: imm3=0, imm8=0, i=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0100F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `field i 26 +: 1`
/// Requirement: FieldBoundary { field: "i", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_tst_i_t1_a_field_i_1_max_f00_f4100f00() {
    // Thumb encoding (32): 0xF4100F00
    // Test aarch32_TST_i_T1_A field i = 1 (Max)
    // ISET: T32
    // Fields: imm8=0, i=1, imm3=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF4100F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_tst_i_t1_a_field_rn_0_min_f00_f0100f00() {
    // Thumb encoding (32): 0xF0100F00
    // Test aarch32_TST_i_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: imm3=0, imm8=0, i=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0100F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_tst_i_t1_a_field_rn_1_poweroftwo_f00_f0110f00() {
    // Thumb encoding (32): 0xF0110F00
    // Test aarch32_TST_i_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=0, imm3=0, i=0, Rn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0110F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_tst_i_t1_a_field_imm3_0_zero_f00_f0100f00() {
    // Thumb encoding (32): 0xF0100F00
    // Test aarch32_TST_i_T1_A field imm3 = 0 (Zero)
    // ISET: T32
    // Fields: i=0, imm3=0, Rn=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0100F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_tst_i_t1_a_field_imm3_1_poweroftwo_f00_f0101f00() {
    // Thumb encoding (32): 0xF0101F00
    // Test aarch32_TST_i_T1_A field imm3 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: i=0, Rn=0, imm3=1, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0101F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch32_tst_i_t1_a_field_imm3_3_poweroftwominusone_f00_f0103f00() {
    // Thumb encoding (32): 0xF0103F00
    // Test aarch32_TST_i_T1_A field imm3 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rn=0, i=0, imm3=3, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0103F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch32_tst_i_t1_a_field_imm3_7_max_f00_f0107f00() {
    // Thumb encoding (32): 0xF0107F00
    // Test aarch32_TST_i_T1_A field imm3 = 7 (Max)
    // ISET: T32
    // Fields: Rn=0, imm3=7, i=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0107F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_tst_i_t1_a_field_imm8_0_zero_f00_f0100f00() {
    // Thumb encoding (32): 0xF0100F00
    // Test aarch32_TST_i_T1_A field imm8 = 0 (Zero)
    // ISET: T32
    // Fields: Rn=0, imm3=0, i=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0100F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_tst_i_t1_a_field_imm8_1_poweroftwo_f00_f0100f01() {
    // Thumb encoding (32): 0xF0100F01
    // Test aarch32_TST_i_T1_A field imm8 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: i=0, imm8=1, Rn=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0100F01;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_tst_i_t1_a_field_imm8_3_poweroftwominusone_f00_f0100f03() {
    // Thumb encoding (32): 0xF0100F03
    // Test aarch32_TST_i_T1_A field imm8 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: i=0, imm3=0, imm8=3, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0100F03;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_tst_i_t1_a_field_imm8_4_poweroftwo_f00_f0100f04() {
    // Thumb encoding (32): 0xF0100F04
    // Test aarch32_TST_i_T1_A field imm8 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: i=0, imm3=0, Rn=0, imm8=4
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0100F04;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_tst_i_t1_a_field_imm8_7_poweroftwominusone_f00_f0100f07() {
    // Thumb encoding (32): 0xF0100F07
    // Test aarch32_TST_i_T1_A field imm8 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm3=0, Rn=0, i=0, imm8=7
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0100F07;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_tst_i_t1_a_field_imm8_8_poweroftwo_f00_f0100f08() {
    // Thumb encoding (32): 0xF0100F08
    // Test aarch32_TST_i_T1_A field imm8 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, imm8=8, i=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0100F08;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_tst_i_t1_a_field_imm8_15_poweroftwominusone_f00_f0100f0f() {
    // Thumb encoding (32): 0xF0100F0F
    // Test aarch32_TST_i_T1_A field imm8 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm3=0, i=0, Rn=0, imm8=15
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0100F0F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_tst_i_t1_a_field_imm8_16_poweroftwo_f00_f0100f10() {
    // Thumb encoding (32): 0xF0100F10
    // Test aarch32_TST_i_T1_A field imm8 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=0, Rn=0, i=0, imm8=16
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0100F10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_tst_i_t1_a_field_imm8_31_poweroftwominusone_f00_f0100f1f() {
    // Thumb encoding (32): 0xF0100F1F
    // Test aarch32_TST_i_T1_A field imm8 = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm3=0, Rn=0, i=0, imm8=31
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0100F1F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_tst_i_t1_a_field_imm8_32_poweroftwo_f00_f0100f20() {
    // Thumb encoding (32): 0xF0100F20
    // Test aarch32_TST_i_T1_A field imm8 = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: i=0, Rn=0, imm3=0, imm8=32
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0100F20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_tst_i_t1_a_field_imm8_63_poweroftwominusone_f00_f0100f3f() {
    // Thumb encoding (32): 0xF0100F3F
    // Test aarch32_TST_i_T1_A field imm8 = 63 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm3=0, i=0, Rn=0, imm8=63
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0100F3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_tst_i_t1_a_field_imm8_64_poweroftwo_f00_f0100f40() {
    // Thumb encoding (32): 0xF0100F40
    // Test aarch32_TST_i_T1_A field imm8 = 64 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, i=0, imm3=0, imm8=64
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0100F40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 127, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (127)
#[test]
fn test_aarch32_tst_i_t1_a_field_imm8_127_poweroftwominusone_f00_f0100f7f() {
    // Thumb encoding (32): 0xF0100F7F
    // Test aarch32_TST_i_T1_A field imm8 = 127 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: i=0, imm3=0, imm8=127, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0100F7F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_tst_i_t1_a_field_imm8_128_poweroftwo_f00_f0100f80() {
    // Thumb encoding (32): 0xF0100F80
    // Test aarch32_TST_i_T1_A field imm8 = 128 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=128, Rn=0, imm3=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0100F80;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 255, boundary: Max }
/// maximum immediate (255)
#[test]
fn test_aarch32_tst_i_t1_a_field_imm8_255_max_f00_f0100fff() {
    // Thumb encoding (32): 0xF0100FFF
    // Test aarch32_TST_i_T1_A field imm8 = 255 (Max)
    // ISET: T32
    // Fields: imm3=0, Rn=0, imm8=255, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0100FFF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// i=0 (minimum value)
#[test]
fn test_aarch32_tst_i_t1_a_combo_0_f00_f0100f00() {
    // Thumb encoding (32): 0xF0100F00
    // Test aarch32_TST_i_T1_A field combination: i=0, Rn=0, imm3=0, imm8=0
    // ISET: T32
    // Fields: Rn=0, i=0, imm3=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0100F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }), rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_tst_i_t1_a_invalid_0_f00_f0100f00() {
    // Thumb encoding (32): 0xF0100F00
    // Test aarch32_TST_i_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: LitInt(15) }
    // ISET: T32
    // Fields: imm3=0, imm8=0, i=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0100F00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_tst_i_t1_a_invalid_1_f00_f0100f00() {
    // Thumb encoding (32): 0xF0100F00
    // Test aarch32_TST_i_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: i=0, Rn=0, imm3=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0100F00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// zero AND zero (32-bit)
#[test]
fn test_aarch32_tst_i_a1_a_oracle_32_0_6a02003f() {
    // Test TST 32-bit: zero AND zero (oracle)
    // Encoding: 0x6A02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// zero AND zero (64-bit)
#[test]
fn test_aarch32_tst_i_a1_a_oracle_64_0_ea02003f() {
    // Test TST 64-bit: zero AND zero (oracle)
    // Encoding: 0xEA02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// partial overlap (32-bit)
#[test]
fn test_aarch32_tst_i_a1_a_oracle_32_1_6a02003f() {
    // Test TST 32-bit: partial overlap (oracle)
    // Encoding: 0x6A02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xF);
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// partial overlap (64-bit)
#[test]
fn test_aarch32_tst_i_a1_a_oracle_64_1_ea02003f() {
    // Test TST 64-bit: partial overlap (oracle)
    // Encoding: 0xEA02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xF);
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// no overlap (32-bit)
#[test]
fn test_aarch32_tst_i_a1_a_oracle_32_2_6a02003f() {
    // Test TST 32-bit: no overlap (oracle)
    // Encoding: 0x6A02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// no overlap (64-bit)
#[test]
fn test_aarch32_tst_i_a1_a_oracle_64_2_ea02003f() {
    // Test TST 64-bit: no overlap (oracle)
    // Encoding: 0xEA02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// MSB set (32-bit)
#[test]
fn test_aarch32_tst_i_a1_a_oracle_32_3_6a02003f() {
    // Test TST 32-bit: MSB set (oracle)
    // Encoding: 0x6A02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// MSB set (64-bit)
#[test]
fn test_aarch32_tst_i_a1_a_oracle_64_3_ea02003f() {
    // Test TST 64-bit: MSB set (oracle)
    // Encoding: 0xEA02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// all ones (32-bit)
#[test]
fn test_aarch32_tst_i_a1_a_oracle_32_4_6a02003f() {
    // Test TST 32-bit: all ones (oracle)
    // Encoding: 0x6A02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// all ones (64-bit)
#[test]
fn test_aarch32_tst_i_a1_a_oracle_64_4_ea02003f() {
    // Test TST 64-bit: all ones (oracle)
    // Encoding: 0xEA02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// alternating (no match) (32-bit)
#[test]
fn test_aarch32_tst_i_a1_a_oracle_32_5_6a02003f() {
    // Test TST 32-bit: alternating (no match) (oracle)
    // Encoding: 0x6A02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x55555555);
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// alternating (no match) (64-bit)
#[test]
fn test_aarch32_tst_i_a1_a_oracle_64_5_ea02003f() {
    // Test TST 64-bit: alternating (no match) (oracle)
    // Encoding: 0xEA02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x55555555);
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_tst_i_a1_a_flags_zeroresult_0_03110000() {
    // Test aarch32_TST_i_A1_A flag computation: ZeroResult
    // Encoding: 0x03110000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x03110000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_tst_i_a1_a_flags_zeroresult_1_03110000() {
    // Test aarch32_TST_i_A1_A flag computation: ZeroResult
    // Encoding: 0x03110000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x03110000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_tst_i_a1_a_flags_negativeresult_2_03110000() {
    // Test aarch32_TST_i_A1_A flag computation: NegativeResult
    // Encoding: 0x03110000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x03110000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_tst_i_a1_a_flags_unsignedoverflow_3_03110000() {
    // Test aarch32_TST_i_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x03110000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x03110000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_tst_i_a1_a_flags_unsignedoverflow_4_03110000() {
    // Test aarch32_TST_i_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x03110000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x2);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x03110000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_tst_i_a1_a_flags_signedoverflow_5_03110000() {
    // Test aarch32_TST_i_A1_A flag computation: SignedOverflow
    // Encoding: 0x03110000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x03110000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_tst_i_a1_a_flags_signedoverflow_6_03110000() {
    // Test aarch32_TST_i_A1_A flag computation: SignedOverflow
    // Encoding: 0x03110000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x03110000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TST_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_tst_i_a1_a_flags_positiveresult_7_03110000() {
    // Test aarch32_TST_i_A1_A flag computation: PositiveResult
    // Encoding: 0x03110000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x32);
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x03110000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// zero AND zero (32-bit)
#[test]
fn test_aarch32_tst_i_t1_a_oracle_32_0_6a02003f() {
    // Test TST 32-bit: zero AND zero (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// zero AND zero (64-bit)
#[test]
fn test_aarch32_tst_i_t1_a_oracle_64_0_ea02003f() {
    // Test TST 64-bit: zero AND zero (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// partial overlap (32-bit)
#[test]
fn test_aarch32_tst_i_t1_a_oracle_32_1_6a02003f() {
    // Test TST 32-bit: partial overlap (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    set_w(&mut cpu, 2, 0xF);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// partial overlap (64-bit)
#[test]
fn test_aarch32_tst_i_t1_a_oracle_64_1_ea02003f() {
    // Test TST 64-bit: partial overlap (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    set_w(&mut cpu, 2, 0xF);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// no overlap (32-bit)
#[test]
fn test_aarch32_tst_i_t1_a_oracle_32_2_6a02003f() {
    // Test TST 32-bit: no overlap (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// no overlap (64-bit)
#[test]
fn test_aarch32_tst_i_t1_a_oracle_64_2_ea02003f() {
    // Test TST 64-bit: no overlap (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// MSB set (32-bit)
#[test]
fn test_aarch32_tst_i_t1_a_oracle_32_3_6a02003f() {
    // Test TST 32-bit: MSB set (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// MSB set (64-bit)
#[test]
fn test_aarch32_tst_i_t1_a_oracle_64_3_ea02003f() {
    // Test TST 64-bit: MSB set (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// all ones (32-bit)
#[test]
fn test_aarch32_tst_i_t1_a_oracle_32_4_6a02003f() {
    // Test TST 32-bit: all ones (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// all ones (64-bit)
#[test]
fn test_aarch32_tst_i_t1_a_oracle_64_4_ea02003f() {
    // Test TST 64-bit: all ones (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// alternating (no match) (32-bit)
#[test]
fn test_aarch32_tst_i_t1_a_oracle_32_5_6a02003f() {
    // Test TST 32-bit: alternating (no match) (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    set_w(&mut cpu, 2, 0x55555555);
    let encoding: u32 = 0x6A02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// alternating (no match) (64-bit)
#[test]
fn test_aarch32_tst_i_t1_a_oracle_64_5_ea02003f() {
    // Test TST 64-bit: alternating (no match) (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    set_w(&mut cpu, 2, 0x55555555);
    let encoding: u32 = 0xEA02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z flag should be true");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be 0");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be 0");
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_tst_i_t1_a_flags_zeroresult_0_f0110f00() {
    // Test aarch32_TST_i_T1_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xF0110F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_tst_i_t1_a_flags_zeroresult_1_f0110f00() {
    // Test aarch32_TST_i_T1_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF0110F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_tst_i_t1_a_flags_negativeresult_2_f0110f00() {
    // Test aarch32_TST_i_T1_A flag computation: NegativeResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF0110F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_tst_i_t1_a_flags_unsignedoverflow_3_f0110f00() {
    // Test aarch32_TST_i_T1_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF0110F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_tst_i_t1_a_flags_unsignedoverflow_4_f0110f00() {
    // Test aarch32_TST_i_T1_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u32 = 0xF0110F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_tst_i_t1_a_flags_signedoverflow_5_f0110f00() {
    // Test aarch32_TST_i_T1_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xF0110F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_tst_i_t1_a_flags_signedoverflow_6_f0110f00() {
    // Test aarch32_TST_i_T1_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF0110F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TST_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_tst_i_t1_a_flags_positiveresult_7_f0110f00() {
    // Test aarch32_TST_i_T1_A flag computation: PositiveResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u32 = 0xF0110F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

// ============================================================================
// aarch32_CMP_rr_A Tests
// ============================================================================

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_cmp_rr_a1_a_field_cond_0_min_10_01500010() {
    // Encoding: 0x01500010
    // Test aarch32_CMP_rr_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: type1=0, Rm=0, Rs=0, cond=0, Rn=0
    let encoding: u32 = 0x01500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_cmp_rr_a1_a_field_cond_1_poweroftwo_10_11500010() {
    // Encoding: 0x11500010
    // Test aarch32_CMP_rr_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rs=0, Rm=0, cond=1, Rn=0
    let encoding: u32 = 0x11500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_cmp_rr_a1_a_field_cond_2_poweroftwo_10_21500010() {
    // Encoding: 0x21500010
    // Test aarch32_CMP_rr_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rs=0, type1=0, cond=2, Rn=0, Rm=0
    let encoding: u32 = 0x21500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_cmp_rr_a1_a_field_cond_3_poweroftwo_10_31500010() {
    // Encoding: 0x31500010
    // Test aarch32_CMP_rr_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rs=0, cond=3, Rn=0, type1=0
    let encoding: u32 = 0x31500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_cmp_rr_a1_a_field_cond_4_poweroftwo_10_41500010() {
    // Encoding: 0x41500010
    // Test aarch32_CMP_rr_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rs=0, type1=0, cond=4, Rm=0, Rn=0
    let encoding: u32 = 0x41500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_cmp_rr_a1_a_field_cond_5_poweroftwo_10_51500010() {
    // Encoding: 0x51500010
    // Test aarch32_CMP_rr_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rm=0, cond=5, Rs=0, Rn=0
    let encoding: u32 = 0x51500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_cmp_rr_a1_a_field_cond_6_poweroftwo_10_61500010() {
    // Encoding: 0x61500010
    // Test aarch32_CMP_rr_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=6, Rs=0, type1=0, Rm=0
    let encoding: u32 = 0x61500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_cmp_rr_a1_a_field_cond_7_poweroftwo_10_71500010() {
    // Encoding: 0x71500010
    // Test aarch32_CMP_rr_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rs=0, cond=7, type1=0, Rn=0, Rm=0
    let encoding: u32 = 0x71500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_cmp_rr_a1_a_field_cond_8_poweroftwo_10_81500010() {
    // Encoding: 0x81500010
    // Test aarch32_CMP_rr_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rs=0, cond=8, type1=0, Rn=0
    let encoding: u32 = 0x81500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_cmp_rr_a1_a_field_cond_9_poweroftwo_10_91500010() {
    // Encoding: 0x91500010
    // Test aarch32_CMP_rr_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, Rn=0, Rm=0, type1=0, Rs=0
    let encoding: u32 = 0x91500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_cmp_rr_a1_a_field_cond_10_poweroftwo_10_a1500010() {
    // Encoding: 0xA1500010
    // Test aarch32_CMP_rr_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=10, Rs=0, type1=0
    let encoding: u32 = 0xA1500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_cmp_rr_a1_a_field_cond_11_poweroftwo_10_b1500010() {
    // Encoding: 0xB1500010
    // Test aarch32_CMP_rr_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, cond=11, Rs=0, Rm=0, Rn=0
    let encoding: u32 = 0xB1500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_cmp_rr_a1_a_field_cond_12_poweroftwo_10_c1500010() {
    // Encoding: 0xC1500010
    // Test aarch32_CMP_rr_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, type1=0, cond=12, Rs=0, Rm=0
    let encoding: u32 = 0xC1500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_cmp_rr_a1_a_field_cond_13_poweroftwo_10_d1500010() {
    // Encoding: 0xD1500010
    // Test aarch32_CMP_rr_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, type1=0, Rm=0, cond=13, Rs=0
    let encoding: u32 = 0xD1500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_cmp_rr_a1_a_field_cond_14_poweroftwo_10_e1500010() {
    // Encoding: 0xE1500010
    // Test aarch32_CMP_rr_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, Rn=0, Rm=0, type1=0, Rs=0
    let encoding: u32 = 0xE1500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_cmp_rr_a1_a_field_cond_15_max_10_f1500010() {
    // Encoding: 0xF1500010
    // Test aarch32_CMP_rr_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, type1=0, Rm=0, Rn=0, Rs=0
    let encoding: u32 = 0xF1500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmp_rr_a1_a_field_rn_0_min_10_01500010() {
    // Encoding: 0x01500010
    // Test aarch32_CMP_rr_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rs=0, cond=0, Rn=0, type1=0, Rm=0
    let encoding: u32 = 0x01500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmp_rr_a1_a_field_rn_1_poweroftwo_10_01510010() {
    // Encoding: 0x01510010
    // Test aarch32_CMP_rr_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rm=0, cond=0, Rs=0, Rn=1
    let encoding: u32 = 0x01510010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field Rs 8 +: 4`
/// Requirement: FieldBoundary { field: "Rs", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmp_rr_a1_a_field_rs_0_min_10_01500010() {
    // Encoding: 0x01500010
    // Test aarch32_CMP_rr_A1_A field Rs = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=0, Rs=0, type1=0
    let encoding: u32 = 0x01500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field Rs 8 +: 4`
/// Requirement: FieldBoundary { field: "Rs", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmp_rr_a1_a_field_rs_1_poweroftwo_10_01500110() {
    // Encoding: 0x01500110
    // Test aarch32_CMP_rr_A1_A field Rs = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rn=0, Rs=1, cond=0, Rm=0
    let encoding: u32 = 0x01500110;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_cmp_rr_a1_a_field_type1_0_min_10_01500010() {
    // Encoding: 0x01500010
    // Test aarch32_CMP_rr_A1_A field type1 = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rs=0, type1=0, cond=0, Rm=0
    let encoding: u32 = 0x01500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_cmp_rr_a1_a_field_type1_1_poweroftwo_10_01500030() {
    // Encoding: 0x01500030
    // Test aarch32_CMP_rr_A1_A field type1 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, type1=1, Rs=0, Rm=0, cond=0
    let encoding: u32 = 0x01500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_cmp_rr_a1_a_field_type1_3_max_10_01500070() {
    // Encoding: 0x01500070
    // Test aarch32_CMP_rr_A1_A field type1 = 3 (Max)
    // ISET: A32
    // Fields: Rm=0, Rs=0, type1=3, cond=0, Rn=0
    let encoding: u32 = 0x01500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmp_rr_a1_a_field_rm_0_min_10_01500010() {
    // Encoding: 0x01500010
    // Test aarch32_CMP_rr_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: cond=0, type1=0, Rm=0, Rn=0, Rs=0
    let encoding: u32 = 0x01500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmp_rr_a1_a_field_rm_1_poweroftwo_10_01500011() {
    // Encoding: 0x01500011
    // Test aarch32_CMP_rr_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rm=1, Rs=0, cond=0, Rn=0
    let encoding: u32 = 0x01500011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_cmp_rr_a1_a_combo_0_10_01500010() {
    // Encoding: 0x01500010
    // Test aarch32_CMP_rr_A1_A field combination: cond=0, Rn=0, Rs=0, type1=0, Rm=0
    // ISET: A32
    // Fields: type1=0, Rn=0, Rs=0, Rm=0, cond=0
    let encoding: u32 = 0x01500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_cmp_rr_a1_a_special_cond_0_condition_eq_16_01500010() {
    // Encoding: 0x01500010
    // Test aarch32_CMP_rr_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rn=0, Rs=0, cond=0, type1=0, Rm=0
    let encoding: u32 = 0x01500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_cmp_rr_a1_a_special_cond_1_condition_ne_16_11500010() {
    // Encoding: 0x11500010
    // Test aarch32_CMP_rr_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rs=0, type1=0, Rn=0, Rm=0, cond=1
    let encoding: u32 = 0x11500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_cmp_rr_a1_a_special_cond_2_condition_cs_hs_16_21500010() {
    // Encoding: 0x21500010
    // Test aarch32_CMP_rr_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rm=0, type1=0, cond=2, Rn=0, Rs=0
    let encoding: u32 = 0x21500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_cmp_rr_a1_a_special_cond_3_condition_cc_lo_16_31500010() {
    // Encoding: 0x31500010
    // Test aarch32_CMP_rr_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rn=0, type1=0, cond=3, Rm=0, Rs=0
    let encoding: u32 = 0x31500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_cmp_rr_a1_a_special_cond_4_condition_mi_16_41500010() {
    // Encoding: 0x41500010
    // Test aarch32_CMP_rr_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rn=0, Rs=0, cond=4, type1=0, Rm=0
    let encoding: u32 = 0x41500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_cmp_rr_a1_a_special_cond_5_condition_pl_16_51500010() {
    // Encoding: 0x51500010
    // Test aarch32_CMP_rr_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rn=0, Rs=0, Rm=0, cond=5, type1=0
    let encoding: u32 = 0x51500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_cmp_rr_a1_a_special_cond_6_condition_vs_16_61500010() {
    // Encoding: 0x61500010
    // Test aarch32_CMP_rr_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: type1=0, Rs=0, cond=6, Rn=0, Rm=0
    let encoding: u32 = 0x61500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_cmp_rr_a1_a_special_cond_7_condition_vc_16_71500010() {
    // Encoding: 0x71500010
    // Test aarch32_CMP_rr_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rs=0, cond=7, type1=0
    let encoding: u32 = 0x71500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_cmp_rr_a1_a_special_cond_8_condition_hi_16_81500010() {
    // Encoding: 0x81500010
    // Test aarch32_CMP_rr_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=8, Rs=0, type1=0
    let encoding: u32 = 0x81500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_cmp_rr_a1_a_special_cond_9_condition_ls_16_91500010() {
    // Encoding: 0x91500010
    // Test aarch32_CMP_rr_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rs=0, Rm=0, type1=0, cond=9, Rn=0
    let encoding: u32 = 0x91500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_cmp_rr_a1_a_special_cond_10_condition_ge_16_a1500010() {
    // Encoding: 0xA1500010
    // Test aarch32_CMP_rr_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, Rs=0, Rn=0, type1=0, Rm=0
    let encoding: u32 = 0xA1500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_cmp_rr_a1_a_special_cond_11_condition_lt_16_b1500010() {
    // Encoding: 0xB1500010
    // Test aarch32_CMP_rr_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rs=0, type1=0, Rn=0, cond=11, Rm=0
    let encoding: u32 = 0xB1500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_cmp_rr_a1_a_special_cond_12_condition_gt_16_c1500010() {
    // Encoding: 0xC1500010
    // Test aarch32_CMP_rr_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: cond=12, Rn=0, Rs=0, type1=0, Rm=0
    let encoding: u32 = 0xC1500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_cmp_rr_a1_a_special_cond_13_condition_le_16_d1500010() {
    // Encoding: 0xD1500010
    // Test aarch32_CMP_rr_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: cond=13, Rm=0, Rn=0, type1=0, Rs=0
    let encoding: u32 = 0xD1500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_cmp_rr_a1_a_special_cond_14_condition_al_16_e1500010() {
    // Encoding: 0xE1500010
    // Test aarch32_CMP_rr_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rs=0, type1=0, Rn=0, Rm=0, cond=14
    let encoding: u32 = 0xE1500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_cmp_rr_a1_a_special_cond_15_condition_nv_16_f1500010() {
    // Encoding: 0xF1500010
    // Test aarch32_CMP_rr_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rn=0, cond=15, type1=0, Rm=0, Rs=0
    let encoding: u32 = 0xF1500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "s" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"s\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_cmp_rr_a1_a_invalid_0_10_01500010() {
    // Encoding: 0x01500010
    // Test aarch32_CMP_rr_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "s" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: type1=0, Rm=0, cond=0, Rn=0, Rs=0
    let encoding: u32 = 0x01500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_cmp_rr_a1_a_invalid_1_10_01500010() {
    // Encoding: 0x01500010
    // Test aarch32_CMP_rr_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rn=0, cond=0, Rs=0, type1=0, Rm=0
    let encoding: u32 = 0x01500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_cmp_rr_a1_a_flags_zeroresult_0_01510012() {
    // Test aarch32_CMP_rr_A1_A flag computation: ZeroResult
    // Encoding: 0x01510012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x01510012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_cmp_rr_a1_a_flags_zeroresult_1_01510012() {
    // Test aarch32_CMP_rr_A1_A flag computation: ZeroResult
    // Encoding: 0x01510012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x01510012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_cmp_rr_a1_a_flags_negativeresult_2_01510012() {
    // Test aarch32_CMP_rr_A1_A flag computation: NegativeResult
    // Encoding: 0x01510012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x01510012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_cmp_rr_a1_a_flags_unsignedoverflow_3_01510012() {
    // Test aarch32_CMP_rr_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x01510012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x01510012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_cmp_rr_a1_a_flags_unsignedoverflow_4_01510012() {
    // Test aarch32_CMP_rr_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x01510012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x01510012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_cmp_rr_a1_a_flags_signedoverflow_5_01510012() {
    // Test aarch32_CMP_rr_A1_A flag computation: SignedOverflow
    // Encoding: 0x01510012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x01510012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_cmp_rr_a1_a_flags_signedoverflow_6_01510012() {
    // Test aarch32_CMP_rr_A1_A flag computation: SignedOverflow
    // Encoding: 0x01510012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x01510012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMP_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_cmp_rr_a1_a_flags_positiveresult_7_01510012() {
    // Test aarch32_CMP_rr_A1_A flag computation: PositiveResult
    // Encoding: 0x01510012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x32);
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x01510012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

// ============================================================================
// aarch32_TEQ_rr_A Tests
// ============================================================================

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_teq_rr_a1_a_field_cond_0_min_10_01300010() {
    // Encoding: 0x01300010
    // Test aarch32_TEQ_rr_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rs=0, Rm=0, Rn=0, cond=0, type1=0
    let encoding: u32 = 0x01300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_teq_rr_a1_a_field_cond_1_poweroftwo_10_11300010() {
    // Encoding: 0x11300010
    // Test aarch32_TEQ_rr_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rn=0, Rm=0, Rs=0, cond=1
    let encoding: u32 = 0x11300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_teq_rr_a1_a_field_cond_2_poweroftwo_10_21300010() {
    // Encoding: 0x21300010
    // Test aarch32_TEQ_rr_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=2, type1=0, Rm=0, Rs=0, Rn=0
    let encoding: u32 = 0x21300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_teq_rr_a1_a_field_cond_3_poweroftwo_10_31300010() {
    // Encoding: 0x31300010
    // Test aarch32_TEQ_rr_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=3, Rs=0, type1=0, Rm=0
    let encoding: u32 = 0x31300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_teq_rr_a1_a_field_cond_4_poweroftwo_10_41300010() {
    // Encoding: 0x41300010
    // Test aarch32_TEQ_rr_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rs=0, type1=0, cond=4, Rn=0
    let encoding: u32 = 0x41300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_teq_rr_a1_a_field_cond_5_poweroftwo_10_51300010() {
    // Encoding: 0x51300010
    // Test aarch32_TEQ_rr_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, type1=0, cond=5, Rs=0, Rn=0
    let encoding: u32 = 0x51300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_teq_rr_a1_a_field_cond_6_poweroftwo_10_61300010() {
    // Encoding: 0x61300010
    // Test aarch32_TEQ_rr_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, Rn=0, type1=0, Rs=0, Rm=0
    let encoding: u32 = 0x61300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_teq_rr_a1_a_field_cond_7_poweroftwo_10_71300010() {
    // Encoding: 0x71300010
    // Test aarch32_TEQ_rr_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rn=0, Rs=0, Rm=0, cond=7
    let encoding: u32 = 0x71300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_teq_rr_a1_a_field_cond_8_poweroftwo_10_81300010() {
    // Encoding: 0x81300010
    // Test aarch32_TEQ_rr_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rs=0, cond=8, Rm=0, type1=0, Rn=0
    let encoding: u32 = 0x81300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_teq_rr_a1_a_field_cond_9_poweroftwo_10_91300010() {
    // Encoding: 0x91300010
    // Test aarch32_TEQ_rr_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rn=0, Rs=0, Rm=0, cond=9
    let encoding: u32 = 0x91300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_teq_rr_a1_a_field_cond_10_poweroftwo_10_a1300010() {
    // Encoding: 0xA1300010
    // Test aarch32_TEQ_rr_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=10, Rs=0, Rn=0, type1=0
    let encoding: u32 = 0xA1300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_teq_rr_a1_a_field_cond_11_poweroftwo_10_b1300010() {
    // Encoding: 0xB1300010
    // Test aarch32_TEQ_rr_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, type1=0, Rs=0, Rm=0, cond=11
    let encoding: u32 = 0xB1300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_teq_rr_a1_a_field_cond_12_poweroftwo_10_c1300010() {
    // Encoding: 0xC1300010
    // Test aarch32_TEQ_rr_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rs=0, type1=0, Rm=0, cond=12
    let encoding: u32 = 0xC1300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_teq_rr_a1_a_field_cond_13_poweroftwo_10_d1300010() {
    // Encoding: 0xD1300010
    // Test aarch32_TEQ_rr_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, type1=0, cond=13, Rn=0, Rs=0
    let encoding: u32 = 0xD1300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_teq_rr_a1_a_field_cond_14_poweroftwo_10_e1300010() {
    // Encoding: 0xE1300010
    // Test aarch32_TEQ_rr_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, Rm=0, Rn=0, Rs=0, type1=0
    let encoding: u32 = 0xE1300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_teq_rr_a1_a_field_cond_15_max_10_f1300010() {
    // Encoding: 0xF1300010
    // Test aarch32_TEQ_rr_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, Rs=0, Rn=0, type1=0, Rm=0
    let encoding: u32 = 0xF1300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_teq_rr_a1_a_field_rn_0_min_10_01300010() {
    // Encoding: 0x01300010
    // Test aarch32_TEQ_rr_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=0, Rs=0, type1=0
    let encoding: u32 = 0x01300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_teq_rr_a1_a_field_rn_1_poweroftwo_10_01310010() {
    // Encoding: 0x01310010
    // Test aarch32_TEQ_rr_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=1, cond=0, Rs=0, type1=0
    let encoding: u32 = 0x01310010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field Rs 8 +: 4`
/// Requirement: FieldBoundary { field: "Rs", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_teq_rr_a1_a_field_rs_0_min_10_01300010() {
    // Encoding: 0x01300010
    // Test aarch32_TEQ_rr_A1_A field Rs = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rs=0, type1=0, Rn=0
    let encoding: u32 = 0x01300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field Rs 8 +: 4`
/// Requirement: FieldBoundary { field: "Rs", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_teq_rr_a1_a_field_rs_1_poweroftwo_10_01300110() {
    // Encoding: 0x01300110
    // Test aarch32_TEQ_rr_A1_A field Rs = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rs=1, cond=0, Rm=0, Rn=0, type1=0
    let encoding: u32 = 0x01300110;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_teq_rr_a1_a_field_type1_0_min_10_01300010() {
    // Encoding: 0x01300010
    // Test aarch32_TEQ_rr_A1_A field type1 = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rs=0, cond=0, type1=0
    let encoding: u32 = 0x01300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_teq_rr_a1_a_field_type1_1_poweroftwo_10_01300030() {
    // Encoding: 0x01300030
    // Test aarch32_TEQ_rr_A1_A field type1 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=0, Rs=0, type1=1, Rm=0
    let encoding: u32 = 0x01300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_teq_rr_a1_a_field_type1_3_max_10_01300070() {
    // Encoding: 0x01300070
    // Test aarch32_TEQ_rr_A1_A field type1 = 3 (Max)
    // ISET: A32
    // Fields: Rn=0, type1=3, Rm=0, cond=0, Rs=0
    let encoding: u32 = 0x01300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_teq_rr_a1_a_field_rm_0_min_10_01300010() {
    // Encoding: 0x01300010
    // Test aarch32_TEQ_rr_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rs=0, type1=0, Rm=0, Rn=0
    let encoding: u32 = 0x01300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_teq_rr_a1_a_field_rm_1_poweroftwo_10_01300011() {
    // Encoding: 0x01300011
    // Test aarch32_TEQ_rr_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rs=0, Rn=0, type1=0, cond=0, Rm=1
    let encoding: u32 = 0x01300011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_teq_rr_a1_a_combo_0_10_01300010() {
    // Encoding: 0x01300010
    // Test aarch32_TEQ_rr_A1_A field combination: cond=0, Rn=0, Rs=0, type1=0, Rm=0
    // ISET: A32
    // Fields: type1=0, Rm=0, cond=0, Rs=0, Rn=0
    let encoding: u32 = 0x01300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_teq_rr_a1_a_special_cond_0_condition_eq_16_01300010() {
    // Encoding: 0x01300010
    // Test aarch32_TEQ_rr_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rs=0, Rn=0, type1=0, Rm=0, cond=0
    let encoding: u32 = 0x01300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_teq_rr_a1_a_special_cond_1_condition_ne_16_11300010() {
    // Encoding: 0x11300010
    // Test aarch32_TEQ_rr_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rn=0, cond=1, type1=0, Rm=0, Rs=0
    let encoding: u32 = 0x11300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_teq_rr_a1_a_special_cond_2_condition_cs_hs_16_21300010() {
    // Encoding: 0x21300010
    // Test aarch32_TEQ_rr_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: type1=0, Rm=0, Rn=0, cond=2, Rs=0
    let encoding: u32 = 0x21300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_teq_rr_a1_a_special_cond_3_condition_cc_lo_16_31300010() {
    // Encoding: 0x31300010
    // Test aarch32_TEQ_rr_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: type1=0, Rn=0, Rs=0, cond=3, Rm=0
    let encoding: u32 = 0x31300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_teq_rr_a1_a_special_cond_4_condition_mi_16_41300010() {
    // Encoding: 0x41300010
    // Test aarch32_TEQ_rr_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rm=0, type1=0, Rn=0, cond=4, Rs=0
    let encoding: u32 = 0x41300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_teq_rr_a1_a_special_cond_5_condition_pl_16_51300010() {
    // Encoding: 0x51300010
    // Test aarch32_TEQ_rr_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rn=0, cond=5, type1=0, Rm=0, Rs=0
    let encoding: u32 = 0x51300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_teq_rr_a1_a_special_cond_6_condition_vs_16_61300010() {
    // Encoding: 0x61300010
    // Test aarch32_TEQ_rr_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, Rs=0, type1=0, Rm=0, Rn=0
    let encoding: u32 = 0x61300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_teq_rr_a1_a_special_cond_7_condition_vc_16_71300010() {
    // Encoding: 0x71300010
    // Test aarch32_TEQ_rr_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rm=0, cond=7, Rs=0, Rn=0, type1=0
    let encoding: u32 = 0x71300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_teq_rr_a1_a_special_cond_8_condition_hi_16_81300010() {
    // Encoding: 0x81300010
    // Test aarch32_TEQ_rr_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: type1=0, Rn=0, cond=8, Rm=0, Rs=0
    let encoding: u32 = 0x81300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_teq_rr_a1_a_special_cond_9_condition_ls_16_91300010() {
    // Encoding: 0x91300010
    // Test aarch32_TEQ_rr_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rs=0, cond=9, Rm=0, type1=0, Rn=0
    let encoding: u32 = 0x91300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_teq_rr_a1_a_special_cond_10_condition_ge_16_a1300010() {
    // Encoding: 0xA1300010
    // Test aarch32_TEQ_rr_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rs=0, cond=10, type1=0, Rm=0, Rn=0
    let encoding: u32 = 0xA1300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_teq_rr_a1_a_special_cond_11_condition_lt_16_b1300010() {
    // Encoding: 0xB1300010
    // Test aarch32_TEQ_rr_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=11, type1=0, Rs=0
    let encoding: u32 = 0xB1300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_teq_rr_a1_a_special_cond_12_condition_gt_16_c1300010() {
    // Encoding: 0xC1300010
    // Test aarch32_TEQ_rr_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rn=0, Rs=0, type1=0, cond=12, Rm=0
    let encoding: u32 = 0xC1300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_teq_rr_a1_a_special_cond_13_condition_le_16_d1300010() {
    // Encoding: 0xD1300010
    // Test aarch32_TEQ_rr_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: cond=13, Rn=0, Rm=0, type1=0, Rs=0
    let encoding: u32 = 0xD1300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_teq_rr_a1_a_special_cond_14_condition_al_16_e1300010() {
    // Encoding: 0xE1300010
    // Test aarch32_TEQ_rr_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: type1=0, Rn=0, cond=14, Rs=0, Rm=0
    let encoding: u32 = 0xE1300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_teq_rr_a1_a_special_cond_15_condition_nv_16_f1300010() {
    // Encoding: 0xF1300010
    // Test aarch32_TEQ_rr_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: type1=0, cond=15, Rn=0, Rm=0, Rs=0
    let encoding: u32 = 0xF1300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "s" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"s\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_teq_rr_a1_a_invalid_0_10_01300010() {
    // Encoding: 0x01300010
    // Test aarch32_TEQ_rr_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "s" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rm=0, type1=0, Rs=0, cond=0, Rn=0
    let encoding: u32 = 0x01300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_teq_rr_a1_a_invalid_1_10_01300010() {
    // Encoding: 0x01300010
    // Test aarch32_TEQ_rr_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=0, Rs=0, type1=0
    let encoding: u32 = 0x01300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_teq_rr_a1_a_flags_zeroresult_0_01310012() {
    // Test aarch32_TEQ_rr_A1_A flag computation: ZeroResult
    // Encoding: 0x01310012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x01310012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_teq_rr_a1_a_flags_zeroresult_1_01310012() {
    // Test aarch32_TEQ_rr_A1_A flag computation: ZeroResult
    // Encoding: 0x01310012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x01310012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_teq_rr_a1_a_flags_negativeresult_2_01310012() {
    // Test aarch32_TEQ_rr_A1_A flag computation: NegativeResult
    // Encoding: 0x01310012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x01310012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_teq_rr_a1_a_flags_unsignedoverflow_3_01310012() {
    // Test aarch32_TEQ_rr_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x01310012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x01310012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_teq_rr_a1_a_flags_unsignedoverflow_4_01310012() {
    // Test aarch32_TEQ_rr_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x01310012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x01310012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_teq_rr_a1_a_flags_signedoverflow_5_01310012() {
    // Test aarch32_TEQ_rr_A1_A flag computation: SignedOverflow
    // Encoding: 0x01310012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x01310012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_teq_rr_a1_a_flags_signedoverflow_6_01310012() {
    // Test aarch32_TEQ_rr_A1_A flag computation: SignedOverflow
    // Encoding: 0x01310012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x01310012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TEQ_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_teq_rr_a1_a_flags_positiveresult_7_01310012() {
    // Test aarch32_TEQ_rr_A1_A flag computation: PositiveResult
    // Encoding: 0x01310012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u32 = 0x01310012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

// ============================================================================
// aarch32_CMP_r_A Tests
// ============================================================================

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_cmp_r_a1_a_field_cond_0_min_0_01500000() {
    // Encoding: 0x01500000
    // Test aarch32_CMP_r_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, type1=0, Rn=0, imm5=0, Rm=0
    let encoding: u32 = 0x01500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_cmp_r_a1_a_field_cond_1_poweroftwo_0_11500000() {
    // Encoding: 0x11500000
    // Test aarch32_CMP_r_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=1, Rm=0, imm5=0, type1=0
    let encoding: u32 = 0x11500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_cmp_r_a1_a_field_cond_2_poweroftwo_0_21500000() {
    // Encoding: 0x21500000
    // Test aarch32_CMP_r_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, imm5=0, cond=2, Rm=0, Rn=0
    let encoding: u32 = 0x21500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_cmp_r_a1_a_field_cond_3_poweroftwo_0_31500000() {
    // Encoding: 0x31500000
    // Test aarch32_CMP_r_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, Rn=0, imm5=0, Rm=0, type1=0
    let encoding: u32 = 0x31500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_cmp_r_a1_a_field_cond_4_poweroftwo_0_41500000() {
    // Encoding: 0x41500000
    // Test aarch32_CMP_r_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rn=0, Rm=0, imm5=0, cond=4
    let encoding: u32 = 0x41500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_cmp_r_a1_a_field_cond_5_poweroftwo_0_51500000() {
    // Encoding: 0x51500000
    // Test aarch32_CMP_r_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=0, Rn=0, type1=0, Rm=0, cond=5
    let encoding: u32 = 0x51500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_cmp_r_a1_a_field_cond_6_poweroftwo_0_61500000() {
    // Encoding: 0x61500000
    // Test aarch32_CMP_r_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, Rn=0, imm5=0, type1=0, Rm=0
    let encoding: u32 = 0x61500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_cmp_r_a1_a_field_cond_7_poweroftwo_0_71500000() {
    // Encoding: 0x71500000
    // Test aarch32_CMP_r_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rm=0, imm5=0, Rn=0, cond=7
    let encoding: u32 = 0x71500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_cmp_r_a1_a_field_cond_8_poweroftwo_0_81500000() {
    // Encoding: 0x81500000
    // Test aarch32_CMP_r_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=8, type1=0, imm5=0, Rm=0, Rn=0
    let encoding: u32 = 0x81500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_cmp_r_a1_a_field_cond_9_poweroftwo_0_91500000() {
    // Encoding: 0x91500000
    // Test aarch32_CMP_r_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=0, type1=0, cond=9, Rn=0, Rm=0
    let encoding: u32 = 0x91500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_cmp_r_a1_a_field_cond_10_poweroftwo_0_a1500000() {
    // Encoding: 0xA1500000
    // Test aarch32_CMP_r_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, imm5=0, Rm=0, type1=0, Rn=0
    let encoding: u32 = 0xA1500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_cmp_r_a1_a_field_cond_11_poweroftwo_0_b1500000() {
    // Encoding: 0xB1500000
    // Test aarch32_CMP_r_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=0, Rn=0, type1=0, Rm=0, cond=11
    let encoding: u32 = 0xB1500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_cmp_r_a1_a_field_cond_12_poweroftwo_0_c1500000() {
    // Encoding: 0xC1500000
    // Test aarch32_CMP_r_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, imm5=0, cond=12, Rn=0, Rm=0
    let encoding: u32 = 0xC1500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_cmp_r_a1_a_field_cond_13_poweroftwo_0_d1500000() {
    // Encoding: 0xD1500000
    // Test aarch32_CMP_r_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm5=0, cond=13, Rm=0, type1=0
    let encoding: u32 = 0xD1500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_cmp_r_a1_a_field_cond_14_poweroftwo_0_e1500000() {
    // Encoding: 0xE1500000
    // Test aarch32_CMP_r_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm5=0, type1=0, Rm=0, cond=14
    let encoding: u32 = 0xE1500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_cmp_r_a1_a_field_cond_15_max_0_f1500000() {
    // Encoding: 0xF1500000
    // Test aarch32_CMP_r_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, type1=0, Rn=0, Rm=0, imm5=0
    let encoding: u32 = 0xF1500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmp_r_a1_a_field_rn_0_min_0_01500000() {
    // Encoding: 0x01500000
    // Test aarch32_CMP_r_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, imm5=0, Rm=0, cond=0, type1=0
    let encoding: u32 = 0x01500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmp_r_a1_a_field_rn_1_poweroftwo_0_01510000() {
    // Encoding: 0x01510000
    // Test aarch32_CMP_r_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=0, cond=0, Rn=1, type1=0, Rm=0
    let encoding: u32 = 0x01510000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_cmp_r_a1_a_field_imm5_0_zero_0_01500000() {
    // Encoding: 0x01500000
    // Test aarch32_CMP_r_A1_A field imm5 = 0 (Zero)
    // ISET: A32
    // Fields: Rn=0, type1=0, Rm=0, cond=0, imm5=0
    let encoding: u32 = 0x01500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_cmp_r_a1_a_field_imm5_1_poweroftwo_0_01500080() {
    // Encoding: 0x01500080
    // Test aarch32_CMP_r_A1_A field imm5 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=1, type1=0, Rm=0, Rn=0, cond=0
    let encoding: u32 = 0x01500080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_cmp_r_a1_a_field_imm5_3_poweroftwominusone_0_01500180() {
    // Encoding: 0x01500180
    // Test aarch32_CMP_r_A1_A field imm5 = 3 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: type1=0, Rn=0, Rm=0, imm5=3, cond=0
    let encoding: u32 = 0x01500180;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_cmp_r_a1_a_field_imm5_4_poweroftwo_0_01500200() {
    // Encoding: 0x01500200
    // Test aarch32_CMP_r_A1_A field imm5 = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm5=4, cond=0, Rm=0, type1=0
    let encoding: u32 = 0x01500200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_cmp_r_a1_a_field_imm5_7_poweroftwominusone_0_01500380() {
    // Encoding: 0x01500380
    // Test aarch32_CMP_r_A1_A field imm5 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: type1=0, cond=0, imm5=7, Rm=0, Rn=0
    let encoding: u32 = 0x01500380;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_cmp_r_a1_a_field_imm5_8_poweroftwo_0_01500400() {
    // Encoding: 0x01500400
    // Test aarch32_CMP_r_A1_A field imm5 = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=0, Rm=0, imm5=8, type1=0
    let encoding: u32 = 0x01500400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 15, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (15)
#[test]
fn test_aarch32_cmp_r_a1_a_field_imm5_15_poweroftwominusone_0_01500780() {
    // Encoding: 0x01500780
    // Test aarch32_CMP_r_A1_A field imm5 = 15 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rn=0, type1=0, cond=0, imm5=15, Rm=0
    let encoding: u32 = 0x01500780;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_cmp_r_a1_a_field_imm5_16_poweroftwo_0_01500800() {
    // Encoding: 0x01500800
    // Test aarch32_CMP_r_A1_A field imm5 = 16 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, type1=0, cond=0, imm5=16
    let encoding: u32 = 0x01500800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 31, boundary: Max }
/// maximum immediate (31)
#[test]
fn test_aarch32_cmp_r_a1_a_field_imm5_31_max_0_01500f80() {
    // Encoding: 0x01500F80
    // Test aarch32_CMP_r_A1_A field imm5 = 31 (Max)
    // ISET: A32
    // Fields: type1=0, imm5=31, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x01500F80;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_cmp_r_a1_a_field_type1_0_min_0_01500000() {
    // Encoding: 0x01500000
    // Test aarch32_CMP_r_A1_A field type1 = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, cond=0, imm5=0, Rm=0, type1=0
    let encoding: u32 = 0x01500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_cmp_r_a1_a_field_type1_1_poweroftwo_0_01500020() {
    // Encoding: 0x01500020
    // Test aarch32_CMP_r_A1_A field type1 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=1, cond=0, Rm=0, Rn=0, imm5=0
    let encoding: u32 = 0x01500020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_cmp_r_a1_a_field_type1_3_max_0_01500060() {
    // Encoding: 0x01500060
    // Test aarch32_CMP_r_A1_A field type1 = 3 (Max)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=0, imm5=0, type1=3
    let encoding: u32 = 0x01500060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmp_r_a1_a_field_rm_0_min_0_01500000() {
    // Encoding: 0x01500000
    // Test aarch32_CMP_r_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: cond=0, type1=0, Rm=0, Rn=0, imm5=0
    let encoding: u32 = 0x01500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmp_r_a1_a_field_rm_1_poweroftwo_0_01500001() {
    // Encoding: 0x01500001
    // Test aarch32_CMP_r_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=0, cond=0, type1=0, Rm=1, Rn=0
    let encoding: u32 = 0x01500001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_cmp_r_a1_a_combo_0_0_01500000() {
    // Encoding: 0x01500000
    // Test aarch32_CMP_r_A1_A field combination: cond=0, Rn=0, imm5=0, type1=0, Rm=0
    // ISET: A32
    // Fields: type1=0, imm5=0, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x01500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_cmp_r_a1_a_special_cond_0_condition_eq_0_01500000() {
    // Encoding: 0x01500000
    // Test aarch32_CMP_r_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rn=0, imm5=0, Rm=0, type1=0, cond=0
    let encoding: u32 = 0x01500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_cmp_r_a1_a_special_cond_1_condition_ne_0_11500000() {
    // Encoding: 0x11500000
    // Test aarch32_CMP_r_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rn=0, imm5=0, type1=0, cond=1, Rm=0
    let encoding: u32 = 0x11500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_cmp_r_a1_a_special_cond_2_condition_cs_hs_0_21500000() {
    // Encoding: 0x21500000
    // Test aarch32_CMP_r_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, imm5=0, Rn=0, type1=0, Rm=0
    let encoding: u32 = 0x21500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_cmp_r_a1_a_special_cond_3_condition_cc_lo_0_31500000() {
    // Encoding: 0x31500000
    // Test aarch32_CMP_r_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: type1=0, Rm=0, Rn=0, cond=3, imm5=0
    let encoding: u32 = 0x31500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_cmp_r_a1_a_special_cond_4_condition_mi_0_41500000() {
    // Encoding: 0x41500000
    // Test aarch32_CMP_r_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, Rn=0, imm5=0, type1=0, Rm=0
    let encoding: u32 = 0x41500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_cmp_r_a1_a_special_cond_5_condition_pl_0_51500000() {
    // Encoding: 0x51500000
    // Test aarch32_CMP_r_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rm=0, type1=0, imm5=0, cond=5, Rn=0
    let encoding: u32 = 0x51500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_cmp_r_a1_a_special_cond_6_condition_vs_0_61500000() {
    // Encoding: 0x61500000
    // Test aarch32_CMP_r_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, Rm=0, imm5=0, Rn=0, type1=0
    let encoding: u32 = 0x61500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_cmp_r_a1_a_special_cond_7_condition_vc_0_71500000() {
    // Encoding: 0x71500000
    // Test aarch32_CMP_r_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: type1=0, cond=7, imm5=0, Rm=0, Rn=0
    let encoding: u32 = 0x71500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_cmp_r_a1_a_special_cond_8_condition_hi_0_81500000() {
    // Encoding: 0x81500000
    // Test aarch32_CMP_r_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=8, imm5=0, type1=0
    let encoding: u32 = 0x81500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_cmp_r_a1_a_special_cond_9_condition_ls_0_91500000() {
    // Encoding: 0x91500000
    // Test aarch32_CMP_r_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rn=0, imm5=0, Rm=0, type1=0, cond=9
    let encoding: u32 = 0x91500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_cmp_r_a1_a_special_cond_10_condition_ge_0_a1500000() {
    // Encoding: 0xA1500000
    // Test aarch32_CMP_r_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rn=0, cond=10, imm5=0, type1=0, Rm=0
    let encoding: u32 = 0xA1500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_cmp_r_a1_a_special_cond_11_condition_lt_0_b1500000() {
    // Encoding: 0xB1500000
    // Test aarch32_CMP_r_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: imm5=0, Rm=0, type1=0, cond=11, Rn=0
    let encoding: u32 = 0xB1500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_cmp_r_a1_a_special_cond_12_condition_gt_0_c1500000() {
    // Encoding: 0xC1500000
    // Test aarch32_CMP_r_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: cond=12, type1=0, Rm=0, Rn=0, imm5=0
    let encoding: u32 = 0xC1500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_cmp_r_a1_a_special_cond_13_condition_le_0_d1500000() {
    // Encoding: 0xD1500000
    // Test aarch32_CMP_r_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: type1=0, imm5=0, Rm=0, Rn=0, cond=13
    let encoding: u32 = 0xD1500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_cmp_r_a1_a_special_cond_14_condition_al_0_e1500000() {
    // Encoding: 0xE1500000
    // Test aarch32_CMP_r_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: cond=14, imm5=0, Rn=0, Rm=0, type1=0
    let encoding: u32 = 0xE1500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_cmp_r_a1_a_special_cond_15_condition_nv_0_f1500000() {
    // Encoding: 0xF1500000
    // Test aarch32_CMP_r_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, imm5=0, type1=0, Rm=0, Rn=0
    let encoding: u32 = 0xF1500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `field Rm 19 +: 3`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmp_r_t1_a_field_rm_0_min_0_42800000() {
    // Thumb encoding (32): 0x42800000
    // Test aarch32_CMP_r_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x42800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `field Rm 19 +: 3`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmp_r_t1_a_field_rm_1_poweroftwo_0_42880000() {
    // Thumb encoding (32): 0x42880000
    // Test aarch32_CMP_r_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x42880000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `field Rn 16 +: 3`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmp_r_t1_a_field_rn_0_min_0_42800000() {
    // Thumb encoding (32): 0x42800000
    // Test aarch32_CMP_r_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x42800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `field Rn 16 +: 3`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmp_r_t1_a_field_rn_1_poweroftwo_0_42810000() {
    // Thumb encoding (32): 0x42810000
    // Test aarch32_CMP_r_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x42810000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch32_cmp_r_t1_a_combo_0_0_42800000() {
    // Thumb encoding (32): 0x42800000
    // Test aarch32_CMP_r_T1_A field combination: Rm=0, Rn=0
    // ISET: T32
    // Fields: Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x42800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `field N 23 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_cmp_r_t2_a_field_n_0_min_0_45000000() {
    // Thumb encoding (32): 0x45000000
    // Test aarch32_CMP_r_T2_A field N = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rn=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x45000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `field N 23 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_cmp_r_t2_a_field_n_1_max_0_45800000() {
    // Thumb encoding (32): 0x45800000
    // Test aarch32_CMP_r_T2_A field N = 1 (Max)
    // ISET: T32
    // Fields: N=1, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x45800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `field Rm 19 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmp_r_t2_a_field_rm_0_min_0_45000000() {
    // Thumb encoding (32): 0x45000000
    // Test aarch32_CMP_r_T2_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, N=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x45000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `field Rm 19 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmp_r_t2_a_field_rm_1_poweroftwo_0_45080000() {
    // Thumb encoding (32): 0x45080000
    // Test aarch32_CMP_r_T2_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: N=0, Rm=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x45080000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `field Rn 16 +: 3`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmp_r_t2_a_field_rn_0_min_0_45000000() {
    // Thumb encoding (32): 0x45000000
    // Test aarch32_CMP_r_T2_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rn=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x45000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `field Rn 16 +: 3`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmp_r_t2_a_field_rn_1_poweroftwo_0_45010000() {
    // Thumb encoding (32): 0x45010000
    // Test aarch32_CMP_r_T2_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, N=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x45010000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// N=0 (minimum value)
#[test]
fn test_aarch32_cmp_r_t2_a_combo_0_0_45000000() {
    // Thumb encoding (32): 0x45000000
    // Test aarch32_CMP_r_T2_A field combination: N=0, Rm=0, Rn=0
    // ISET: T32
    // Fields: Rm=0, Rn=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x45000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `Binary { op: And, lhs: Binary { op: Lt, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: LitInt(8) }, rhs: Binary { op: Lt, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }), rhs: LitInt(8) } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: And, lhs: Binary { op: Lt, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }), rhs: LitInt(8) }, rhs: Binary { op: Lt, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }), rhs: LitInt(8) } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_cmp_r_t2_a_invalid_0_0_45000000() {
    // Thumb encoding (32): 0x45000000
    // Test aarch32_CMP_r_T2_A invalid encoding: Binary { op: And, lhs: Binary { op: Lt, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: LitInt(8) }, rhs: Binary { op: Lt, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }), rhs: LitInt(8) } }
    // ISET: T32
    // Fields: N=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x45000000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_cmp_r_t2_a_invalid_1_0_45000000() {
    // Thumb encoding (32): 0x45000000
    // Test aarch32_CMP_r_T2_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: N=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x45000000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_cmp_r_t2_a_invalid_2_0_45000000() {
    // Thumb encoding (32): 0x45000000
    // Test aarch32_CMP_r_T2_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rm=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x45000000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_cmp_r_t2_a_invalid_3_0_45000000() {
    // Thumb encoding (32): 0x45000000
    // Test aarch32_CMP_r_T2_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rm=0, Rn=0, N=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x45000000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmp_r_t3_a_field_rn_0_min_f00_ebb00f00() {
    // Thumb encoding (32): 0xEBB00F00
    // Test aarch32_CMP_r_T3_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: imm3=0, imm2=0, Rn=0, type1=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEBB00F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmp_r_t3_a_field_rn_1_poweroftwo_f00_ebb10f00() {
    // Thumb encoding (32): 0xEBB10F00
    // Test aarch32_CMP_r_T3_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, type1=0, Rn=1, imm2=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEBB10F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_cmp_r_t3_a_field_imm3_0_zero_f00_ebb00f00() {
    // Thumb encoding (32): 0xEBB00F00
    // Test aarch32_CMP_r_T3_A field imm3 = 0 (Zero)
    // ISET: T32
    // Fields: imm3=0, imm2=0, type1=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEBB00F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_cmp_r_t3_a_field_imm3_1_poweroftwo_f00_ebb01f00() {
    // Thumb encoding (32): 0xEBB01F00
    // Test aarch32_CMP_r_T3_A field imm3 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, imm2=0, Rn=0, type1=0, imm3=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEBB01F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch32_cmp_r_t3_a_field_imm3_3_poweroftwominusone_f00_ebb03f00() {
    // Thumb encoding (32): 0xEBB03F00
    // Test aarch32_CMP_r_T3_A field imm3 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: type1=0, Rn=0, imm2=0, Rm=0, imm3=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEBB03F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch32_cmp_r_t3_a_field_imm3_7_max_f00_ebb07f00() {
    // Thumb encoding (32): 0xEBB07F00
    // Test aarch32_CMP_r_T3_A field imm3 = 7 (Max)
    // ISET: T32
    // Fields: Rn=0, type1=0, Rm=0, imm3=7, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEBB07F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `field imm2 6 +: 2`
/// Requirement: FieldBoundary { field: "imm2", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_cmp_r_t3_a_field_imm2_0_zero_f00_ebb00f00() {
    // Thumb encoding (32): 0xEBB00F00
    // Test aarch32_CMP_r_T3_A field imm2 = 0 (Zero)
    // ISET: T32
    // Fields: imm3=0, type1=0, Rm=0, imm2=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEBB00F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `field imm2 6 +: 2`
/// Requirement: FieldBoundary { field: "imm2", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_cmp_r_t3_a_field_imm2_1_poweroftwo_f00_ebb00f40() {
    // Thumb encoding (32): 0xEBB00F40
    // Test aarch32_CMP_r_T3_A field imm2 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: type1=0, imm2=1, Rn=0, Rm=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEBB00F40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `field imm2 6 +: 2`
/// Requirement: FieldBoundary { field: "imm2", value: 3, boundary: Max }
/// maximum immediate (3)
#[test]
fn test_aarch32_cmp_r_t3_a_field_imm2_3_max_f00_ebb00fc0() {
    // Thumb encoding (32): 0xEBB00FC0
    // Test aarch32_CMP_r_T3_A field imm2 = 3 (Max)
    // ISET: T32
    // Fields: type1=0, Rn=0, Rm=0, imm3=0, imm2=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEBB00FC0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `field type1 4 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_cmp_r_t3_a_field_type1_0_min_f00_ebb00f00() {
    // Thumb encoding (32): 0xEBB00F00
    // Test aarch32_CMP_r_T3_A field type1 = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, imm3=0, imm2=0, Rm=0, type1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEBB00F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `field type1 4 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_cmp_r_t3_a_field_type1_1_poweroftwo_f00_ebb00f10() {
    // Thumb encoding (32): 0xEBB00F10
    // Test aarch32_CMP_r_T3_A field type1 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm2=0, type1=1, imm3=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEBB00F10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `field type1 4 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_cmp_r_t3_a_field_type1_3_max_f00_ebb00f30() {
    // Thumb encoding (32): 0xEBB00F30
    // Test aarch32_CMP_r_T3_A field type1 = 3 (Max)
    // ISET: T32
    // Fields: imm3=0, type1=3, Rm=0, Rn=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEBB00F30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmp_r_t3_a_field_rm_0_min_f00_ebb00f00() {
    // Thumb encoding (32): 0xEBB00F00
    // Test aarch32_CMP_r_T3_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rm=0, imm2=0, imm3=0, type1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEBB00F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmp_r_t3_a_field_rm_1_poweroftwo_f00_ebb00f01() {
    // Thumb encoding (32): 0xEBB00F01
    // Test aarch32_CMP_r_T3_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: type1=0, Rm=1, imm3=0, imm2=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEBB00F01;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_cmp_r_t3_a_combo_0_f00_ebb00f00() {
    // Thumb encoding (32): 0xEBB00F00
    // Test aarch32_CMP_r_T3_A field combination: Rn=0, imm3=0, imm2=0, type1=0, Rm=0
    // ISET: T32
    // Fields: Rn=0, imm2=0, imm3=0, Rm=0, type1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEBB00F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_cmp_r_t3_a_invalid_0_f00_ebb00f00() {
    // Thumb encoding (32): 0xEBB00F00
    // Test aarch32_CMP_r_T3_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: imm3=0, Rm=0, type1=0, imm2=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEBB00F00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_cmp_r_t3_a_invalid_1_f00_ebb00f00() {
    // Thumb encoding (32): 0xEBB00F00
    // Test aarch32_CMP_r_T3_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: imm3=0, Rm=0, imm2=0, Rn=0, type1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEBB00F00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_cmp_r_a1_a_flags_zeroresult_0_01510002() {
    // Test aarch32_CMP_r_A1_A flag computation: ZeroResult
    // Encoding: 0x01510002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x01510002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_cmp_r_a1_a_flags_zeroresult_1_01510002() {
    // Test aarch32_CMP_r_A1_A flag computation: ZeroResult
    // Encoding: 0x01510002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x01510002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_cmp_r_a1_a_flags_negativeresult_2_01510002() {
    // Test aarch32_CMP_r_A1_A flag computation: NegativeResult
    // Encoding: 0x01510002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x01510002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_cmp_r_a1_a_flags_unsignedoverflow_3_01510002() {
    // Test aarch32_CMP_r_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x01510002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x01510002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_cmp_r_a1_a_flags_unsignedoverflow_4_01510002() {
    // Test aarch32_CMP_r_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x01510002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x2);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x01510002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_cmp_r_a1_a_flags_signedoverflow_5_01510002() {
    // Test aarch32_CMP_r_A1_A flag computation: SignedOverflow
    // Encoding: 0x01510002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x01510002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_cmp_r_a1_a_flags_signedoverflow_6_01510002() {
    // Test aarch32_CMP_r_A1_A flag computation: SignedOverflow
    // Encoding: 0x01510002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x01510002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMP_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_cmp_r_a1_a_flags_positiveresult_7_01510002() {
    // Test aarch32_CMP_r_A1_A flag computation: PositiveResult
    // Encoding: 0x01510002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x32);
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x01510002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 0 (32)
#[test]
fn test_aarch32_cmp_r_t1_a_lslv_oracle_32_0_42820020() {
    // Test LSLV 32-bit: shift by 0 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x12345678, "W0 should be 0x12345678");
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 0 (64)
#[test]
fn test_aarch32_cmp_r_t1_a_lslv_oracle_64_0_c2820020() {
    // Test LSLV 64-bit: shift by 0 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x12345678,
        "X0 should be 0x0000000012345678"
    );
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4 (32)
#[test]
fn test_aarch32_cmp_r_t1_a_lslv_oracle_32_1_42820020() {
    // Test LSLV 32-bit: shift by 4 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x4);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x23456780, "W0 should be 0x23456780");
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 4 (64)
#[test]
fn test_aarch32_cmp_r_t1_a_lslv_oracle_64_1_c2820020() {
    // Test LSLV 64-bit: shift by 4 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x4);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x23456780,
        "X0 should be 0x0000000123456780"
    );
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 8 (32)
#[test]
fn test_aarch32_cmp_r_t1_a_lslv_oracle_32_2_42820020() {
    // Test LSLV 32-bit: shift by 8 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x8);
    set_w(&mut cpu, 1, 0x12345678);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x34567800, "W0 should be 0x34567800");
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 8 (64)
#[test]
fn test_aarch32_cmp_r_t1_a_lslv_oracle_64_2_c2820020() {
    // Test LSLV 64-bit: shift by 8 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x8);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x34567800,
        "X0 should be 0x0000001234567800"
    );
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1 (32)
#[test]
fn test_aarch32_cmp_r_t1_a_lslv_oracle_32_3_42820020() {
    // Test LSLV 32-bit: MSB set, shift 1 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x00000000");
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// MSB set, shift 1 (64)
#[test]
fn test_aarch32_cmp_r_t1_a_lslv_oracle_64_3_c2820020() {
    // Test LSLV 64-bit: MSB set, shift 1 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSB set, max shift (32)
#[test]
fn test_aarch32_cmp_r_t1_a_lslv_oracle_32_4_42820020() {
    // Test LSLV 32-bit: LSB set, max shift (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0x3F);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x80000000, "W0 should be 0x80000000");
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSB set, max shift (64)
#[test]
fn test_aarch32_cmp_r_t1_a_lslv_oracle_64_4_c2820020() {
    // Test LSLV 64-bit: LSB set, max shift (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0x3F);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x8000000000000000");
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// all ones, shift 32 (32)
#[test]
fn test_aarch32_cmp_r_t1_a_lslv_oracle_32_5_42820020() {
    // Test LSLV 32-bit: all ones, shift 32 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x20);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "W0 should be 0xFFFFFFFF");
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all ones, shift 32 (64)
#[test]
fn test_aarch32_cmp_r_t1_a_lslv_oracle_64_5_c2820020() {
    // Test LSLV 64-bit: all ones, shift 32 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x20);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0xFFFFFFFF00000000");
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift
#[test]
fn test_aarch32_cmp_r_t1_a_t16_oracle_0_42910000() {
    // Test T16 LSLS: no shift (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF, "R0 should be 0x000000FF");
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4
#[test]
fn test_aarch32_cmp_r_t1_a_t16_oracle_1_42910000() {
    // Test T16 LSLS: shift by 4 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    set_w(&mut cpu, 2, 0x4);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF0, "R0 should be 0x00000FF0");
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1
#[test]
fn test_aarch32_cmp_r_t1_a_t16_oracle_2_42910000() {
    // Test T16 LSLS: MSB set, shift 1 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x80000000);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift to MSB
#[test]
fn test_aarch32_cmp_r_t1_a_t16_oracle_3_42910000() {
    // Test T16 LSLS: shift to MSB (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0x1F);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x80000000, "R0 should be 0x80000000");
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_cmp_r_t1_a_flags_zeroresult_0_42910000() {
    // Test aarch32_CMP_r_T1_A flag computation: ZeroResult
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_cmp_r_t1_a_flags_zeroresult_1_42910000() {
    // Test aarch32_CMP_r_T1_A flag computation: ZeroResult
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_cmp_r_t1_a_flags_negativeresult_2_42910000() {
    // Test aarch32_CMP_r_T1_A flag computation: NegativeResult
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_cmp_r_t1_a_flags_unsignedoverflow_3_42910000() {
    // Test aarch32_CMP_r_T1_A flag computation: UnsignedOverflow
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_cmp_r_t1_a_flags_unsignedoverflow_4_42910000() {
    // Test aarch32_CMP_r_T1_A flag computation: UnsignedOverflow
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_cmp_r_t1_a_flags_signedoverflow_5_42910000() {
    // Test aarch32_CMP_r_T1_A flag computation: SignedOverflow
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_cmp_r_t1_a_flags_signedoverflow_6_42910000() {
    // Test aarch32_CMP_r_T1_A flag computation: SignedOverflow
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMP_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_cmp_r_t1_a_flags_positiveresult_7_42910000() {
    // Test aarch32_CMP_r_T1_A flag computation: PositiveResult
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 0 (32)
#[test]
fn test_aarch32_cmp_r_t2_a_lslv_oracle_32_0_45020020() {
    // Test LSLV 32-bit: shift by 0 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x12345678);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x12345678, "W0 should be 0x12345678");
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 0 (64)
#[test]
fn test_aarch32_cmp_r_t2_a_lslv_oracle_64_0_c5020020() {
    // Test LSLV 64-bit: shift by 0 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x12345678,
        "X0 should be 0x0000000012345678"
    );
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4 (32)
#[test]
fn test_aarch32_cmp_r_t2_a_lslv_oracle_32_1_45020020() {
    // Test LSLV 32-bit: shift by 4 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x4);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x23456780, "W0 should be 0x23456780");
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 4 (64)
#[test]
fn test_aarch32_cmp_r_t2_a_lslv_oracle_64_1_c5020020() {
    // Test LSLV 64-bit: shift by 4 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x4);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x23456780,
        "X0 should be 0x0000000123456780"
    );
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 8 (32)
#[test]
fn test_aarch32_cmp_r_t2_a_lslv_oracle_32_2_45020020() {
    // Test LSLV 32-bit: shift by 8 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x8);
    set_w(&mut cpu, 1, 0x12345678);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x34567800, "W0 should be 0x34567800");
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 8 (64)
#[test]
fn test_aarch32_cmp_r_t2_a_lslv_oracle_64_2_c5020020() {
    // Test LSLV 64-bit: shift by 8 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x8);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x34567800,
        "X0 should be 0x0000001234567800"
    );
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1 (32)
#[test]
fn test_aarch32_cmp_r_t2_a_lslv_oracle_32_3_45020020() {
    // Test LSLV 32-bit: MSB set, shift 1 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x00000000");
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// MSB set, shift 1 (64)
#[test]
fn test_aarch32_cmp_r_t2_a_lslv_oracle_64_3_c5020020() {
    // Test LSLV 64-bit: MSB set, shift 1 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSB set, max shift (32)
#[test]
fn test_aarch32_cmp_r_t2_a_lslv_oracle_32_4_45020020() {
    // Test LSLV 32-bit: LSB set, max shift (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0x3F);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x80000000, "W0 should be 0x80000000");
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSB set, max shift (64)
#[test]
fn test_aarch32_cmp_r_t2_a_lslv_oracle_64_4_c5020020() {
    // Test LSLV 64-bit: LSB set, max shift (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0x3F);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x8000000000000000");
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// all ones, shift 32 (32)
#[test]
fn test_aarch32_cmp_r_t2_a_lslv_oracle_32_5_45020020() {
    // Test LSLV 32-bit: all ones, shift 32 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x20);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "W0 should be 0xFFFFFFFF");
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all ones, shift 32 (64)
#[test]
fn test_aarch32_cmp_r_t2_a_lslv_oracle_64_5_c5020020() {
    // Test LSLV 64-bit: all ones, shift 32 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x20);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0xFFFFFFFF00000000");
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift
#[test]
fn test_aarch32_cmp_r_t2_a_t16_oracle_0_45110000() {
    // Test T16 LSLS: no shift (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF, "R0 should be 0x000000FF");
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4
#[test]
fn test_aarch32_cmp_r_t2_a_t16_oracle_1_45110000() {
    // Test T16 LSLS: shift by 4 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x4);
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF0, "R0 should be 0x00000FF0");
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1
#[test]
fn test_aarch32_cmp_r_t2_a_t16_oracle_2_45110000() {
    // Test T16 LSLS: MSB set, shift 1 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0x80000000);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift to MSB
#[test]
fn test_aarch32_cmp_r_t2_a_t16_oracle_3_45110000() {
    // Test T16 LSLS: shift to MSB (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1F);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x80000000, "R0 should be 0x80000000");
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_cmp_r_t2_a_flags_zeroresult_0_45110000() {
    // Test aarch32_CMP_r_T2_A flag computation: ZeroResult
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_cmp_r_t2_a_flags_zeroresult_1_45110000() {
    // Test aarch32_CMP_r_T2_A flag computation: ZeroResult
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_cmp_r_t2_a_flags_negativeresult_2_45110000() {
    // Test aarch32_CMP_r_T2_A flag computation: NegativeResult
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_cmp_r_t2_a_flags_unsignedoverflow_3_45110000() {
    // Test aarch32_CMP_r_T2_A flag computation: UnsignedOverflow
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_cmp_r_t2_a_flags_unsignedoverflow_4_45110000() {
    // Test aarch32_CMP_r_T2_A flag computation: UnsignedOverflow
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x2);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_cmp_r_t2_a_flags_signedoverflow_5_45110000() {
    // Test aarch32_CMP_r_T2_A flag computation: SignedOverflow
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_cmp_r_t2_a_flags_signedoverflow_6_45110000() {
    // Test aarch32_CMP_r_T2_A flag computation: SignedOverflow
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMP_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_cmp_r_t2_a_flags_positiveresult_7_45110000() {
    // Test aarch32_CMP_r_T2_A flag computation: PositiveResult
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_cmp_r_t3_a_flags_zeroresult_0_ebb10f02() {
    // Test aarch32_CMP_r_T3_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xEBB10F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_cmp_r_t3_a_flags_zeroresult_1_ebb10f02() {
    // Test aarch32_CMP_r_T3_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0xEBB10F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_cmp_r_t3_a_flags_negativeresult_2_ebb10f02() {
    // Test aarch32_CMP_r_T3_A flag computation: NegativeResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xEBB10F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_cmp_r_t3_a_flags_unsignedoverflow_3_ebb10f02() {
    // Test aarch32_CMP_r_T3_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xEBB10F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_cmp_r_t3_a_flags_unsignedoverflow_4_ebb10f02() {
    // Test aarch32_CMP_r_T3_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u32 = 0xEBB10F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_cmp_r_t3_a_flags_signedoverflow_5_ebb10f02() {
    // Test aarch32_CMP_r_T3_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xEBB10F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_cmp_r_t3_a_flags_signedoverflow_6_ebb10f02() {
    // Test aarch32_CMP_r_T3_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xEBB10F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMP_r_T3_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_cmp_r_t3_a_flags_positiveresult_7_ebb10f02() {
    // Test aarch32_CMP_r_T3_A flag computation: PositiveResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u32 = 0xEBB10F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

// ============================================================================
// aarch32_TST_r_A Tests
// ============================================================================

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_tst_r_a1_a_field_cond_0_min_0_01100000() {
    // Encoding: 0x01100000
    // Test aarch32_TST_r_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: imm5=0, type1=0, Rn=0, cond=0, Rm=0
    let encoding: u32 = 0x01100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_tst_r_a1_a_field_cond_1_poweroftwo_0_11100000() {
    // Encoding: 0x11100000
    // Test aarch32_TST_r_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, type1=0, imm5=0, Rn=0, Rm=0
    let encoding: u32 = 0x11100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_tst_r_a1_a_field_cond_2_poweroftwo_0_21100000() {
    // Encoding: 0x21100000
    // Test aarch32_TST_r_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, imm5=0, cond=2, Rn=0, type1=0
    let encoding: u32 = 0x21100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_tst_r_a1_a_field_cond_3_poweroftwo_0_31100000() {
    // Encoding: 0x31100000
    // Test aarch32_TST_r_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=3, imm5=0, type1=0, Rm=0
    let encoding: u32 = 0x31100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_tst_r_a1_a_field_cond_4_poweroftwo_0_41100000() {
    // Encoding: 0x41100000
    // Test aarch32_TST_r_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=0, Rn=0, type1=0, Rm=0, cond=4
    let encoding: u32 = 0x41100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_tst_r_a1_a_field_cond_5_poweroftwo_0_51100000() {
    // Encoding: 0x51100000
    // Test aarch32_TST_r_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=5, imm5=0, Rn=0, type1=0, Rm=0
    let encoding: u32 = 0x51100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_tst_r_a1_a_field_cond_6_poweroftwo_0_61100000() {
    // Encoding: 0x61100000
    // Test aarch32_TST_r_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=0, Rm=0, type1=0, Rn=0, cond=6
    let encoding: u32 = 0x61100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_tst_r_a1_a_field_cond_7_poweroftwo_0_71100000() {
    // Encoding: 0x71100000
    // Test aarch32_TST_r_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rm=0, Rn=0, cond=7, imm5=0
    let encoding: u32 = 0x71100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_tst_r_a1_a_field_cond_8_poweroftwo_0_81100000() {
    // Encoding: 0x81100000
    // Test aarch32_TST_r_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, type1=0, cond=8, imm5=0
    let encoding: u32 = 0x81100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_tst_r_a1_a_field_cond_9_poweroftwo_0_91100000() {
    // Encoding: 0x91100000
    // Test aarch32_TST_r_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, imm5=0, cond=9, type1=0
    let encoding: u32 = 0x91100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_tst_r_a1_a_field_cond_10_poweroftwo_0_a1100000() {
    // Encoding: 0xA1100000
    // Test aarch32_TST_r_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rm=0, imm5=0, cond=10, Rn=0
    let encoding: u32 = 0xA1100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_tst_r_a1_a_field_cond_11_poweroftwo_0_b1100000() {
    // Encoding: 0xB1100000
    // Test aarch32_TST_r_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=0, cond=11, Rn=0, Rm=0, type1=0
    let encoding: u32 = 0xB1100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_tst_r_a1_a_field_cond_12_poweroftwo_0_c1100000() {
    // Encoding: 0xC1100000
    // Test aarch32_TST_r_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=0, Rn=0, cond=12, Rm=0, type1=0
    let encoding: u32 = 0xC1100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_tst_r_a1_a_field_cond_13_poweroftwo_0_d1100000() {
    // Encoding: 0xD1100000
    // Test aarch32_TST_r_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=13, Rm=0, imm5=0, type1=0
    let encoding: u32 = 0xD1100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_tst_r_a1_a_field_cond_14_poweroftwo_0_e1100000() {
    // Encoding: 0xE1100000
    // Test aarch32_TST_r_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, imm5=0, type1=0, Rn=0, Rm=0
    let encoding: u32 = 0xE1100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_tst_r_a1_a_field_cond_15_max_0_f1100000() {
    // Encoding: 0xF1100000
    // Test aarch32_TST_r_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rm=0, cond=15, Rn=0, imm5=0, type1=0
    let encoding: u32 = 0xF1100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_tst_r_a1_a_field_rn_0_min_0_01100000() {
    // Encoding: 0x01100000
    // Test aarch32_TST_r_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rn=0, type1=0, imm5=0, cond=0
    let encoding: u32 = 0x01100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_tst_r_a1_a_field_rn_1_poweroftwo_0_01110000() {
    // Encoding: 0x01110000
    // Test aarch32_TST_r_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=1, type1=0, imm5=0
    let encoding: u32 = 0x01110000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_tst_r_a1_a_field_imm5_0_zero_0_01100000() {
    // Encoding: 0x01100000
    // Test aarch32_TST_r_A1_A field imm5 = 0 (Zero)
    // ISET: A32
    // Fields: Rm=0, imm5=0, cond=0, Rn=0, type1=0
    let encoding: u32 = 0x01100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_tst_r_a1_a_field_imm5_1_poweroftwo_0_01100080() {
    // Encoding: 0x01100080
    // Test aarch32_TST_r_A1_A field imm5 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=1, Rn=0, type1=0, cond=0, Rm=0
    let encoding: u32 = 0x01100080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_tst_r_a1_a_field_imm5_3_poweroftwominusone_0_01100180() {
    // Encoding: 0x01100180
    // Test aarch32_TST_r_A1_A field imm5 = 3 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rm=0, type1=0, cond=0, imm5=3, Rn=0
    let encoding: u32 = 0x01100180;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_tst_r_a1_a_field_imm5_4_poweroftwo_0_01100200() {
    // Encoding: 0x01100200
    // Test aarch32_TST_r_A1_A field imm5 = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rn=0, Rm=0, cond=0, imm5=4
    let encoding: u32 = 0x01100200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_tst_r_a1_a_field_imm5_7_poweroftwominusone_0_01100380() {
    // Encoding: 0x01100380
    // Test aarch32_TST_r_A1_A field imm5 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm5=7, cond=0, type1=0, Rm=0, Rn=0
    let encoding: u32 = 0x01100380;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_tst_r_a1_a_field_imm5_8_poweroftwo_0_01100400() {
    // Encoding: 0x01100400
    // Test aarch32_TST_r_A1_A field imm5 = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=8, cond=0, type1=0, Rn=0, Rm=0
    let encoding: u32 = 0x01100400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 15, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (15)
#[test]
fn test_aarch32_tst_r_a1_a_field_imm5_15_poweroftwominusone_0_01100780() {
    // Encoding: 0x01100780
    // Test aarch32_TST_r_A1_A field imm5 = 15 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rn=0, type1=0, Rm=0, cond=0, imm5=15
    let encoding: u32 = 0x01100780;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_tst_r_a1_a_field_imm5_16_poweroftwo_0_01100800() {
    // Encoding: 0x01100800
    // Test aarch32_TST_r_A1_A field imm5 = 16 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=16, Rm=0, cond=0, Rn=0, type1=0
    let encoding: u32 = 0x01100800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 31, boundary: Max }
/// maximum immediate (31)
#[test]
fn test_aarch32_tst_r_a1_a_field_imm5_31_max_0_01100f80() {
    // Encoding: 0x01100F80
    // Test aarch32_TST_r_A1_A field imm5 = 31 (Max)
    // ISET: A32
    // Fields: type1=0, imm5=31, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x01100F80;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_tst_r_a1_a_field_type1_0_min_0_01100000() {
    // Encoding: 0x01100000
    // Test aarch32_TST_r_A1_A field type1 = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, cond=0, type1=0, Rm=0, imm5=0
    let encoding: u32 = 0x01100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_tst_r_a1_a_field_type1_1_poweroftwo_0_01100020() {
    // Encoding: 0x01100020
    // Test aarch32_TST_r_A1_A field type1 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm5=0, Rm=0, cond=0, type1=1
    let encoding: u32 = 0x01100020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_tst_r_a1_a_field_type1_3_max_0_01100060() {
    // Encoding: 0x01100060
    // Test aarch32_TST_r_A1_A field type1 = 3 (Max)
    // ISET: A32
    // Fields: Rn=0, cond=0, Rm=0, imm5=0, type1=3
    let encoding: u32 = 0x01100060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_tst_r_a1_a_field_rm_0_min_0_01100000() {
    // Encoding: 0x01100000
    // Test aarch32_TST_r_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0, imm5=0, Rn=0, type1=0
    let encoding: u32 = 0x01100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_tst_r_a1_a_field_rm_1_poweroftwo_0_01100001() {
    // Encoding: 0x01100001
    // Test aarch32_TST_r_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, type1=0, Rm=1, imm5=0, cond=0
    let encoding: u32 = 0x01100001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_tst_r_a1_a_combo_0_0_01100000() {
    // Encoding: 0x01100000
    // Test aarch32_TST_r_A1_A field combination: cond=0, Rn=0, imm5=0, type1=0, Rm=0
    // ISET: A32
    // Fields: type1=0, cond=0, Rn=0, imm5=0, Rm=0
    let encoding: u32 = 0x01100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_tst_r_a1_a_special_cond_0_condition_eq_0_01100000() {
    // Encoding: 0x01100000
    // Test aarch32_TST_r_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm5=0, type1=0, Rm=0
    let encoding: u32 = 0x01100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_tst_r_a1_a_special_cond_1_condition_ne_0_11100000() {
    // Encoding: 0x11100000
    // Test aarch32_TST_r_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, imm5=0, type1=0, Rm=0, Rn=0
    let encoding: u32 = 0x11100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_tst_r_a1_a_special_cond_2_condition_cs_hs_0_21100000() {
    // Encoding: 0x21100000
    // Test aarch32_TST_r_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, Rn=0, imm5=0, type1=0, Rm=0
    let encoding: u32 = 0x21100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_tst_r_a1_a_special_cond_3_condition_cc_lo_0_31100000() {
    // Encoding: 0x31100000
    // Test aarch32_TST_r_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: type1=0, imm5=0, Rn=0, cond=3, Rm=0
    let encoding: u32 = 0x31100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_tst_r_a1_a_special_cond_4_condition_mi_0_41100000() {
    // Encoding: 0x41100000
    // Test aarch32_TST_r_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rm=0, imm5=0, cond=4, type1=0, Rn=0
    let encoding: u32 = 0x41100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_tst_r_a1_a_special_cond_5_condition_pl_0_51100000() {
    // Encoding: 0x51100000
    // Test aarch32_TST_r_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rm=0, Rn=0, type1=0, cond=5, imm5=0
    let encoding: u32 = 0x51100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_tst_r_a1_a_special_cond_6_condition_vs_0_61100000() {
    // Encoding: 0x61100000
    // Test aarch32_TST_r_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rm=0, cond=6, type1=0, imm5=0, Rn=0
    let encoding: u32 = 0x61100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_tst_r_a1_a_special_cond_7_condition_vc_0_71100000() {
    // Encoding: 0x71100000
    // Test aarch32_TST_r_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rm=0, imm5=0, type1=0, cond=7, Rn=0
    let encoding: u32 = 0x71100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_tst_r_a1_a_special_cond_8_condition_hi_0_81100000() {
    // Encoding: 0x81100000
    // Test aarch32_TST_r_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: imm5=0, cond=8, Rn=0, Rm=0, type1=0
    let encoding: u32 = 0x81100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_tst_r_a1_a_special_cond_9_condition_ls_0_91100000() {
    // Encoding: 0x91100000
    // Test aarch32_TST_r_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rn=0, type1=0, cond=9, Rm=0, imm5=0
    let encoding: u32 = 0x91100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_tst_r_a1_a_special_cond_10_condition_ge_0_a1100000() {
    // Encoding: 0xA1100000
    // Test aarch32_TST_r_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, type1=0, Rn=0, Rm=0, imm5=0
    let encoding: u32 = 0xA1100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_tst_r_a1_a_special_cond_11_condition_lt_0_b1100000() {
    // Encoding: 0xB1100000
    // Test aarch32_TST_r_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rn=0, cond=11, imm5=0, type1=0, Rm=0
    let encoding: u32 = 0xB1100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_tst_r_a1_a_special_cond_12_condition_gt_0_c1100000() {
    // Encoding: 0xC1100000
    // Test aarch32_TST_r_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: cond=12, type1=0, Rn=0, Rm=0, imm5=0
    let encoding: u32 = 0xC1100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_tst_r_a1_a_special_cond_13_condition_le_0_d1100000() {
    // Encoding: 0xD1100000
    // Test aarch32_TST_r_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: type1=0, cond=13, Rn=0, Rm=0, imm5=0
    let encoding: u32 = 0xD1100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_tst_r_a1_a_special_cond_14_condition_al_0_e1100000() {
    // Encoding: 0xE1100000
    // Test aarch32_TST_r_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: type1=0, cond=14, Rm=0, imm5=0, Rn=0
    let encoding: u32 = 0xE1100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_tst_r_a1_a_special_cond_15_condition_nv_0_f1100000() {
    // Encoding: 0xF1100000
    // Test aarch32_TST_r_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rn=0, imm5=0, type1=0, Rm=0, cond=15
    let encoding: u32 = 0xF1100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `field Rm 19 +: 3`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_tst_r_t1_a_field_rm_0_min_0_42000000() {
    // Thumb encoding (32): 0x42000000
    // Test aarch32_TST_r_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x42000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `field Rm 19 +: 3`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_tst_r_t1_a_field_rm_1_poweroftwo_0_42080000() {
    // Thumb encoding (32): 0x42080000
    // Test aarch32_TST_r_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x42080000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `field Rn 16 +: 3`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_tst_r_t1_a_field_rn_0_min_0_42000000() {
    // Thumb encoding (32): 0x42000000
    // Test aarch32_TST_r_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x42000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `field Rn 16 +: 3`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_tst_r_t1_a_field_rn_1_poweroftwo_0_42010000() {
    // Thumb encoding (32): 0x42010000
    // Test aarch32_TST_r_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x42010000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch32_tst_r_t1_a_combo_0_0_42000000() {
    // Thumb encoding (32): 0x42000000
    // Test aarch32_TST_r_T1_A field combination: Rm=0, Rn=0
    // ISET: T32
    // Fields: Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x42000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_tst_r_t2_a_field_rn_0_min_f00_ea100f00() {
    // Thumb encoding (32): 0xEA100F00
    // Test aarch32_TST_r_T2_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: imm3=0, Rn=0, imm2=0, Rm=0, type1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA100F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_tst_r_t2_a_field_rn_1_poweroftwo_f00_ea110f00() {
    // Thumb encoding (32): 0xEA110F00
    // Test aarch32_TST_r_T2_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm2=0, imm3=0, Rm=0, Rn=1, type1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA110F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_tst_r_t2_a_field_imm3_0_zero_f00_ea100f00() {
    // Thumb encoding (32): 0xEA100F00
    // Test aarch32_TST_r_T2_A field imm3 = 0 (Zero)
    // ISET: T32
    // Fields: Rm=0, imm3=0, imm2=0, Rn=0, type1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA100F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_tst_r_t2_a_field_imm3_1_poweroftwo_f00_ea101f00() {
    // Thumb encoding (32): 0xEA101F00
    // Test aarch32_TST_r_T2_A field imm3 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=1, imm2=0, Rm=0, type1=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA101F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch32_tst_r_t2_a_field_imm3_3_poweroftwominusone_f00_ea103f00() {
    // Thumb encoding (32): 0xEA103F00
    // Test aarch32_TST_r_T2_A field imm3 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm2=0, Rm=0, imm3=3, type1=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA103F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch32_tst_r_t2_a_field_imm3_7_max_f00_ea107f00() {
    // Thumb encoding (32): 0xEA107F00
    // Test aarch32_TST_r_T2_A field imm3 = 7 (Max)
    // ISET: T32
    // Fields: Rm=0, Rn=0, type1=0, imm3=7, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA107F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `field imm2 6 +: 2`
/// Requirement: FieldBoundary { field: "imm2", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_tst_r_t2_a_field_imm2_0_zero_f00_ea100f00() {
    // Thumb encoding (32): 0xEA100F00
    // Test aarch32_TST_r_T2_A field imm2 = 0 (Zero)
    // ISET: T32
    // Fields: imm2=0, imm3=0, type1=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA100F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `field imm2 6 +: 2`
/// Requirement: FieldBoundary { field: "imm2", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_tst_r_t2_a_field_imm2_1_poweroftwo_f00_ea100f40() {
    // Thumb encoding (32): 0xEA100F40
    // Test aarch32_TST_r_T2_A field imm2 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, imm3=0, imm2=1, type1=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA100F40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `field imm2 6 +: 2`
/// Requirement: FieldBoundary { field: "imm2", value: 3, boundary: Max }
/// maximum immediate (3)
#[test]
fn test_aarch32_tst_r_t2_a_field_imm2_3_max_f00_ea100fc0() {
    // Thumb encoding (32): 0xEA100FC0
    // Test aarch32_TST_r_T2_A field imm2 = 3 (Max)
    // ISET: T32
    // Fields: imm3=0, Rn=0, imm2=3, type1=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA100FC0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `field type1 4 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_tst_r_t2_a_field_type1_0_min_f00_ea100f00() {
    // Thumb encoding (32): 0xEA100F00
    // Test aarch32_TST_r_T2_A field type1 = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, imm2=0, Rn=0, type1=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA100F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `field type1 4 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_tst_r_t2_a_field_type1_1_poweroftwo_f00_ea100f10() {
    // Thumb encoding (32): 0xEA100F10
    // Test aarch32_TST_r_T2_A field type1 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, imm2=0, imm3=0, type1=1, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA100F10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `field type1 4 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_tst_r_t2_a_field_type1_3_max_f00_ea100f30() {
    // Thumb encoding (32): 0xEA100F30
    // Test aarch32_TST_r_T2_A field type1 = 3 (Max)
    // ISET: T32
    // Fields: imm3=0, Rn=0, imm2=0, type1=3, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA100F30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_tst_r_t2_a_field_rm_0_min_f00_ea100f00() {
    // Thumb encoding (32): 0xEA100F00
    // Test aarch32_TST_r_T2_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, type1=0, imm2=0, imm3=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA100F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_tst_r_t2_a_field_rm_1_poweroftwo_f00_ea100f01() {
    // Thumb encoding (32): 0xEA100F01
    // Test aarch32_TST_r_T2_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, type1=0, Rm=1, imm2=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA100F01;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_tst_r_t2_a_combo_0_f00_ea100f00() {
    // Thumb encoding (32): 0xEA100F00
    // Test aarch32_TST_r_T2_A field combination: Rn=0, imm3=0, imm2=0, type1=0, Rm=0
    // ISET: T32
    // Fields: Rm=0, type1=0, imm2=0, imm3=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA100F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_tst_r_t2_a_invalid_0_f00_ea100f00() {
    // Thumb encoding (32): 0xEA100F00
    // Test aarch32_TST_r_T2_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: imm3=0, type1=0, Rn=0, Rm=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA100F00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_tst_r_t2_a_invalid_1_f00_ea100f00() {
    // Thumb encoding (32): 0xEA100F00
    // Test aarch32_TST_r_T2_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: imm2=0, imm3=0, Rn=0, type1=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA100F00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_tst_r_a1_a_flags_zeroresult_0_01110002() {
    // Test aarch32_TST_r_A1_A flag computation: ZeroResult
    // Encoding: 0x01110002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x01110002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_tst_r_a1_a_flags_zeroresult_1_01110002() {
    // Test aarch32_TST_r_A1_A flag computation: ZeroResult
    // Encoding: 0x01110002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x01110002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_tst_r_a1_a_flags_negativeresult_2_01110002() {
    // Test aarch32_TST_r_A1_A flag computation: NegativeResult
    // Encoding: 0x01110002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x01110002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_tst_r_a1_a_flags_unsignedoverflow_3_01110002() {
    // Test aarch32_TST_r_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x01110002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x01110002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_tst_r_a1_a_flags_unsignedoverflow_4_01110002() {
    // Test aarch32_TST_r_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x01110002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x01110002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_tst_r_a1_a_flags_signedoverflow_5_01110002() {
    // Test aarch32_TST_r_A1_A flag computation: SignedOverflow
    // Encoding: 0x01110002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x01110002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_tst_r_a1_a_flags_signedoverflow_6_01110002() {
    // Test aarch32_TST_r_A1_A flag computation: SignedOverflow
    // Encoding: 0x01110002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x01110002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TST_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_tst_r_a1_a_flags_positiveresult_7_01110002() {
    // Test aarch32_TST_r_A1_A flag computation: PositiveResult
    // Encoding: 0x01110002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u32 = 0x01110002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 0 (32)
#[test]
fn test_aarch32_tst_r_t1_a_lslv_oracle_32_0_42020020() {
    // Test LSLV 32-bit: shift by 0 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x12345678, "W0 should be 0x12345678");
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 0 (64)
#[test]
fn test_aarch32_tst_r_t1_a_lslv_oracle_64_0_c2020020() {
    // Test LSLV 64-bit: shift by 0 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x12345678);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x12345678,
        "X0 should be 0x0000000012345678"
    );
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4 (32)
#[test]
fn test_aarch32_tst_r_t1_a_lslv_oracle_32_1_42020020() {
    // Test LSLV 32-bit: shift by 4 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x4);
    set_w(&mut cpu, 1, 0x12345678);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x23456780, "W0 should be 0x23456780");
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 4 (64)
#[test]
fn test_aarch32_tst_r_t1_a_lslv_oracle_64_1_c2020020() {
    // Test LSLV 64-bit: shift by 4 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x4);
    set_w(&mut cpu, 1, 0x12345678);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x23456780,
        "X0 should be 0x0000000123456780"
    );
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 8 (32)
#[test]
fn test_aarch32_tst_r_t1_a_lslv_oracle_32_2_42020020() {
    // Test LSLV 32-bit: shift by 8 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x8);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x34567800, "W0 should be 0x34567800");
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 8 (64)
#[test]
fn test_aarch32_tst_r_t1_a_lslv_oracle_64_2_c2020020() {
    // Test LSLV 64-bit: shift by 8 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x8);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x34567800,
        "X0 should be 0x0000001234567800"
    );
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1 (32)
#[test]
fn test_aarch32_tst_r_t1_a_lslv_oracle_32_3_42020020() {
    // Test LSLV 32-bit: MSB set, shift 1 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x00000000");
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// MSB set, shift 1 (64)
#[test]
fn test_aarch32_tst_r_t1_a_lslv_oracle_64_3_c2020020() {
    // Test LSLV 64-bit: MSB set, shift 1 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSB set, max shift (32)
#[test]
fn test_aarch32_tst_r_t1_a_lslv_oracle_32_4_42020020() {
    // Test LSLV 32-bit: LSB set, max shift (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0x3F);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x80000000, "W0 should be 0x80000000");
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSB set, max shift (64)
#[test]
fn test_aarch32_tst_r_t1_a_lslv_oracle_64_4_c2020020() {
    // Test LSLV 64-bit: LSB set, max shift (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x3F);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x8000000000000000");
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// all ones, shift 32 (32)
#[test]
fn test_aarch32_tst_r_t1_a_lslv_oracle_32_5_42020020() {
    // Test LSLV 32-bit: all ones, shift 32 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x20);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "W0 should be 0xFFFFFFFF");
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all ones, shift 32 (64)
#[test]
fn test_aarch32_tst_r_t1_a_lslv_oracle_64_5_c2020020() {
    // Test LSLV 64-bit: all ones, shift 32 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x20);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0xFFFFFFFF00000000");
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift
#[test]
fn test_aarch32_tst_r_t1_a_t16_oracle_0_42110000() {
    // Test T16 LSLS: no shift (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF, "R0 should be 0x000000FF");
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4
#[test]
fn test_aarch32_tst_r_t1_a_t16_oracle_1_42110000() {
    // Test T16 LSLS: shift by 4 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFF);
    set_w(&mut cpu, 2, 0x4);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF0, "R0 should be 0x00000FF0");
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1
#[test]
fn test_aarch32_tst_r_t1_a_t16_oracle_2_42110000() {
    // Test T16 LSLS: MSB set, shift 1 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x80000000);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift to MSB
#[test]
fn test_aarch32_tst_r_t1_a_t16_oracle_3_42110000() {
    // Test T16 LSLS: shift to MSB (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0x1F);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x80000000, "R0 should be 0x80000000");
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_tst_r_t1_a_flags_zeroresult_0_42110000() {
    // Test aarch32_TST_r_T1_A flag computation: ZeroResult
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_tst_r_t1_a_flags_zeroresult_1_42110000() {
    // Test aarch32_TST_r_T1_A flag computation: ZeroResult
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_tst_r_t1_a_flags_negativeresult_2_42110000() {
    // Test aarch32_TST_r_T1_A flag computation: NegativeResult
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_tst_r_t1_a_flags_unsignedoverflow_3_42110000() {
    // Test aarch32_TST_r_T1_A flag computation: UnsignedOverflow
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_tst_r_t1_a_flags_unsignedoverflow_4_42110000() {
    // Test aarch32_TST_r_T1_A flag computation: UnsignedOverflow
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_tst_r_t1_a_flags_signedoverflow_5_42110000() {
    // Test aarch32_TST_r_T1_A flag computation: SignedOverflow
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_tst_r_t1_a_flags_signedoverflow_6_42110000() {
    // Test aarch32_TST_r_T1_A flag computation: SignedOverflow
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TST_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_tst_r_t1_a_flags_positiveresult_7_42110000() {
    // Test aarch32_TST_r_T1_A flag computation: PositiveResult
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x32);
    set_w(&mut cpu, 1, 0x64);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_tst_r_t2_a_flags_zeroresult_0_ea110f02() {
    // Test aarch32_TST_r_T2_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xEA110F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_tst_r_t2_a_flags_zeroresult_1_ea110f02() {
    // Test aarch32_TST_r_T2_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xEA110F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_tst_r_t2_a_flags_negativeresult_2_ea110f02() {
    // Test aarch32_TST_r_T2_A flag computation: NegativeResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xEA110F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_tst_r_t2_a_flags_unsignedoverflow_3_ea110f02() {
    // Test aarch32_TST_r_T2_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xEA110F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_tst_r_t2_a_flags_unsignedoverflow_4_ea110f02() {
    // Test aarch32_TST_r_T2_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u32 = 0xEA110F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_tst_r_t2_a_flags_signedoverflow_5_ea110f02() {
    // Test aarch32_TST_r_T2_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xEA110F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_tst_r_t2_a_flags_signedoverflow_6_ea110f02() {
    // Test aarch32_TST_r_T2_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0xEA110F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TST_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_tst_r_t2_a_flags_positiveresult_7_ea110f02() {
    // Test aarch32_TST_r_T2_A flag computation: PositiveResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x32);
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xEA110F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

// ============================================================================
// aarch32_CMN_r_A Tests
// ============================================================================

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_cmn_r_a1_a_field_cond_0_min_0_01700000() {
    // Encoding: 0x01700000
    // Test aarch32_CMN_r_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, imm5=0, type1=0, Rm=0, Rn=0
    let encoding: u32 = 0x01700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_cmn_r_a1_a_field_cond_1_poweroftwo_0_11700000() {
    // Encoding: 0x11700000
    // Test aarch32_CMN_r_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, type1=0, Rm=0, cond=1, imm5=0
    let encoding: u32 = 0x11700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_cmn_r_a1_a_field_cond_2_poweroftwo_0_21700000() {
    // Encoding: 0x21700000
    // Test aarch32_CMN_r_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm5=0, type1=0, Rm=0, cond=2
    let encoding: u32 = 0x21700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_cmn_r_a1_a_field_cond_3_poweroftwo_0_31700000() {
    // Encoding: 0x31700000
    // Test aarch32_CMN_r_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=3, imm5=0, type1=0, Rm=0
    let encoding: u32 = 0x31700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_cmn_r_a1_a_field_cond_4_poweroftwo_0_41700000() {
    // Encoding: 0x41700000
    // Test aarch32_CMN_r_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, imm5=0, Rn=0, cond=4, Rm=0
    let encoding: u32 = 0x41700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_cmn_r_a1_a_field_cond_5_poweroftwo_0_51700000() {
    // Encoding: 0x51700000
    // Test aarch32_CMN_r_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, imm5=0, type1=0, cond=5
    let encoding: u32 = 0x51700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_cmn_r_a1_a_field_cond_6_poweroftwo_0_61700000() {
    // Encoding: 0x61700000
    // Test aarch32_CMN_r_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, type1=0, cond=6, imm5=0
    let encoding: u32 = 0x61700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_cmn_r_a1_a_field_cond_7_poweroftwo_0_71700000() {
    // Encoding: 0x71700000
    // Test aarch32_CMN_r_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, imm5=0, cond=7, type1=0
    let encoding: u32 = 0x71700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_cmn_r_a1_a_field_cond_8_poweroftwo_0_81700000() {
    // Encoding: 0x81700000
    // Test aarch32_CMN_r_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=8, Rn=0, imm5=0, type1=0
    let encoding: u32 = 0x81700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_cmn_r_a1_a_field_cond_9_poweroftwo_0_91700000() {
    // Encoding: 0x91700000
    // Test aarch32_CMN_r_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, type1=0, imm5=0, Rn=0, Rm=0
    let encoding: u32 = 0x91700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_cmn_r_a1_a_field_cond_10_poweroftwo_0_a1700000() {
    // Encoding: 0xA1700000
    // Test aarch32_CMN_r_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=10, Rm=0, type1=0, imm5=0
    let encoding: u32 = 0xA1700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_cmn_r_a1_a_field_cond_11_poweroftwo_0_b1700000() {
    // Encoding: 0xB1700000
    // Test aarch32_CMN_r_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, imm5=0, type1=0, Rn=0, cond=11
    let encoding: u32 = 0xB1700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_cmn_r_a1_a_field_cond_12_poweroftwo_0_c1700000() {
    // Encoding: 0xC1700000
    // Test aarch32_CMN_r_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=0, Rn=0, cond=12, Rm=0, type1=0
    let encoding: u32 = 0xC1700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_cmn_r_a1_a_field_cond_13_poweroftwo_0_d1700000() {
    // Encoding: 0xD1700000
    // Test aarch32_CMN_r_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=13, Rn=0, imm5=0, type1=0
    let encoding: u32 = 0xD1700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_cmn_r_a1_a_field_cond_14_poweroftwo_0_e1700000() {
    // Encoding: 0xE1700000
    // Test aarch32_CMN_r_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=14, Rn=0, type1=0, imm5=0
    let encoding: u32 = 0xE1700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_cmn_r_a1_a_field_cond_15_max_0_f1700000() {
    // Encoding: 0xF1700000
    // Test aarch32_CMN_r_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, Rn=0, Rm=0, imm5=0, type1=0
    let encoding: u32 = 0xF1700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmn_r_a1_a_field_rn_0_min_0_01700000() {
    // Encoding: 0x01700000
    // Test aarch32_CMN_r_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, type1=0, Rn=0, imm5=0, Rm=0
    let encoding: u32 = 0x01700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmn_r_a1_a_field_rn_1_poweroftwo_0_01710000() {
    // Encoding: 0x01710000
    // Test aarch32_CMN_r_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=0, type1=0, Rn=1, cond=0, Rm=0
    let encoding: u32 = 0x01710000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_cmn_r_a1_a_field_imm5_0_zero_0_01700000() {
    // Encoding: 0x01700000
    // Test aarch32_CMN_r_A1_A field imm5 = 0 (Zero)
    // ISET: A32
    // Fields: Rn=0, type1=0, imm5=0, Rm=0, cond=0
    let encoding: u32 = 0x01700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_cmn_r_a1_a_field_imm5_1_poweroftwo_0_01700080() {
    // Encoding: 0x01700080
    // Test aarch32_CMN_r_A1_A field imm5 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, imm5=1, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x01700080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_cmn_r_a1_a_field_imm5_3_poweroftwominusone_0_01700180() {
    // Encoding: 0x01700180
    // Test aarch32_CMN_r_A1_A field imm5 = 3 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm5=3, Rm=0, type1=0, Rn=0, cond=0
    let encoding: u32 = 0x01700180;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_cmn_r_a1_a_field_imm5_4_poweroftwo_0_01700200() {
    // Encoding: 0x01700200
    // Test aarch32_CMN_r_A1_A field imm5 = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm5=4, Rm=0, cond=0, type1=0
    let encoding: u32 = 0x01700200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_cmn_r_a1_a_field_imm5_7_poweroftwominusone_0_01700380() {
    // Encoding: 0x01700380
    // Test aarch32_CMN_r_A1_A field imm5 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm5=7, type1=0, cond=0, Rm=0, Rn=0
    let encoding: u32 = 0x01700380;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_cmn_r_a1_a_field_imm5_8_poweroftwo_0_01700400() {
    // Encoding: 0x01700400
    // Test aarch32_CMN_r_A1_A field imm5 = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=0, imm5=8, type1=0, Rn=0
    let encoding: u32 = 0x01700400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 15, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (15)
#[test]
fn test_aarch32_cmn_r_a1_a_field_imm5_15_poweroftwominusone_0_01700780() {
    // Encoding: 0x01700780
    // Test aarch32_CMN_r_A1_A field imm5 = 15 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rn=0, imm5=15, type1=0, cond=0, Rm=0
    let encoding: u32 = 0x01700780;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_cmn_r_a1_a_field_imm5_16_poweroftwo_0_01700800() {
    // Encoding: 0x01700800
    // Test aarch32_CMN_r_A1_A field imm5 = 16 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=16, type1=0, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x01700800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 31, boundary: Max }
/// maximum immediate (31)
#[test]
fn test_aarch32_cmn_r_a1_a_field_imm5_31_max_0_01700f80() {
    // Encoding: 0x01700F80
    // Test aarch32_CMN_r_A1_A field imm5 = 31 (Max)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, imm5=31, type1=0
    let encoding: u32 = 0x01700F80;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_cmn_r_a1_a_field_type1_0_min_0_01700000() {
    // Encoding: 0x01700000
    // Test aarch32_CMN_r_A1_A field type1 = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rm=0, type1=0, imm5=0, cond=0
    let encoding: u32 = 0x01700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_cmn_r_a1_a_field_type1_1_poweroftwo_0_01700020() {
    // Encoding: 0x01700020
    // Test aarch32_CMN_r_A1_A field type1 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, type1=1, Rm=0, imm5=0
    let encoding: u32 = 0x01700020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_cmn_r_a1_a_field_type1_3_max_0_01700060() {
    // Encoding: 0x01700060
    // Test aarch32_CMN_r_A1_A field type1 = 3 (Max)
    // ISET: A32
    // Fields: Rn=0, Rm=0, imm5=0, cond=0, type1=3
    let encoding: u32 = 0x01700060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmn_r_a1_a_field_rm_0_min_0_01700000() {
    // Encoding: 0x01700000
    // Test aarch32_CMN_r_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, imm5=0, type1=0, cond=0, Rm=0
    let encoding: u32 = 0x01700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmn_r_a1_a_field_rm_1_poweroftwo_0_01700001() {
    // Encoding: 0x01700001
    // Test aarch32_CMN_r_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=0, cond=0, Rn=0, type1=0, Rm=1
    let encoding: u32 = 0x01700001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_cmn_r_a1_a_combo_0_0_01700000() {
    // Encoding: 0x01700000
    // Test aarch32_CMN_r_A1_A field combination: cond=0, Rn=0, imm5=0, type1=0, Rm=0
    // ISET: A32
    // Fields: imm5=0, type1=0, cond=0, Rn=0, Rm=0
    let encoding: u32 = 0x01700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_cmn_r_a1_a_special_cond_0_condition_eq_0_01700000() {
    // Encoding: 0x01700000
    // Test aarch32_CMN_r_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: imm5=0, cond=0, type1=0, Rn=0, Rm=0
    let encoding: u32 = 0x01700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_cmn_r_a1_a_special_cond_1_condition_ne_0_11700000() {
    // Encoding: 0x11700000
    // Test aarch32_CMN_r_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rn=0, imm5=0, cond=1, type1=0, Rm=0
    let encoding: u32 = 0x11700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_cmn_r_a1_a_special_cond_2_condition_cs_hs_0_21700000() {
    // Encoding: 0x21700000
    // Test aarch32_CMN_r_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: type1=0, Rn=0, cond=2, imm5=0, Rm=0
    let encoding: u32 = 0x21700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_cmn_r_a1_a_special_cond_3_condition_cc_lo_0_31700000() {
    // Encoding: 0x31700000
    // Test aarch32_CMN_r_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rn=0, imm5=0, Rm=0, cond=3, type1=0
    let encoding: u32 = 0x31700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_cmn_r_a1_a_special_cond_4_condition_mi_0_41700000() {
    // Encoding: 0x41700000
    // Test aarch32_CMN_r_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rn=0, imm5=0, Rm=0, cond=4, type1=0
    let encoding: u32 = 0x41700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_cmn_r_a1_a_special_cond_5_condition_pl_0_51700000() {
    // Encoding: 0x51700000
    // Test aarch32_CMN_r_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: imm5=0, Rn=0, Rm=0, type1=0, cond=5
    let encoding: u32 = 0x51700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_cmn_r_a1_a_special_cond_6_condition_vs_0_61700000() {
    // Encoding: 0x61700000
    // Test aarch32_CMN_r_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: imm5=0, Rn=0, type1=0, Rm=0, cond=6
    let encoding: u32 = 0x61700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_cmn_r_a1_a_special_cond_7_condition_vc_0_71700000() {
    // Encoding: 0x71700000
    // Test aarch32_CMN_r_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: imm5=0, Rn=0, type1=0, Rm=0, cond=7
    let encoding: u32 = 0x71700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_cmn_r_a1_a_special_cond_8_condition_hi_0_81700000() {
    // Encoding: 0x81700000
    // Test aarch32_CMN_r_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: type1=0, cond=8, Rn=0, Rm=0, imm5=0
    let encoding: u32 = 0x81700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_cmn_r_a1_a_special_cond_9_condition_ls_0_91700000() {
    // Encoding: 0x91700000
    // Test aarch32_CMN_r_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: type1=0, Rm=0, imm5=0, cond=9, Rn=0
    let encoding: u32 = 0x91700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_cmn_r_a1_a_special_cond_10_condition_ge_0_a1700000() {
    // Encoding: 0xA1700000
    // Test aarch32_CMN_r_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rn=0, type1=0, Rm=0, cond=10, imm5=0
    let encoding: u32 = 0xA1700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_cmn_r_a1_a_special_cond_11_condition_lt_0_b1700000() {
    // Encoding: 0xB1700000
    // Test aarch32_CMN_r_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, Rn=0, type1=0, Rm=0, imm5=0
    let encoding: u32 = 0xB1700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_cmn_r_a1_a_special_cond_12_condition_gt_0_c1700000() {
    // Encoding: 0xC1700000
    // Test aarch32_CMN_r_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: cond=12, imm5=0, Rn=0, type1=0, Rm=0
    let encoding: u32 = 0xC1700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_cmn_r_a1_a_special_cond_13_condition_le_0_d1700000() {
    // Encoding: 0xD1700000
    // Test aarch32_CMN_r_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rn=0, type1=0, cond=13, Rm=0, imm5=0
    let encoding: u32 = 0xD1700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_cmn_r_a1_a_special_cond_14_condition_al_0_e1700000() {
    // Encoding: 0xE1700000
    // Test aarch32_CMN_r_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rm=0, imm5=0, Rn=0, cond=14, type1=0
    let encoding: u32 = 0xE1700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_cmn_r_a1_a_special_cond_15_condition_nv_0_f1700000() {
    // Encoding: 0xF1700000
    // Test aarch32_CMN_r_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: imm5=0, Rm=0, cond=15, type1=0, Rn=0
    let encoding: u32 = 0xF1700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `field Rm 19 +: 3`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmn_r_t1_a_field_rm_0_min_0_42c00000() {
    // Thumb encoding (32): 0x42C00000
    // Test aarch32_CMN_r_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x42C00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `field Rm 19 +: 3`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmn_r_t1_a_field_rm_1_poweroftwo_0_42c80000() {
    // Thumb encoding (32): 0x42C80000
    // Test aarch32_CMN_r_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x42C80000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `field Rn 16 +: 3`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmn_r_t1_a_field_rn_0_min_0_42c00000() {
    // Thumb encoding (32): 0x42C00000
    // Test aarch32_CMN_r_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x42C00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `field Rn 16 +: 3`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmn_r_t1_a_field_rn_1_poweroftwo_0_42c10000() {
    // Thumb encoding (32): 0x42C10000
    // Test aarch32_CMN_r_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x42C10000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch32_cmn_r_t1_a_combo_0_0_42c00000() {
    // Thumb encoding (32): 0x42C00000
    // Test aarch32_CMN_r_T1_A field combination: Rm=0, Rn=0
    // ISET: T32
    // Fields: Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x42C00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmn_r_t2_a_field_rn_0_min_f00_eb100f00() {
    // Thumb encoding (32): 0xEB100F00
    // Test aarch32_CMN_r_T2_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: imm2=0, imm3=0, Rn=0, type1=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEB100F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmn_r_t2_a_field_rn_1_poweroftwo_f00_eb110f00() {
    // Thumb encoding (32): 0xEB110F00
    // Test aarch32_CMN_r_T2_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, imm3=0, Rm=0, imm2=0, type1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEB110F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_cmn_r_t2_a_field_imm3_0_zero_f00_eb100f00() {
    // Thumb encoding (32): 0xEB100F00
    // Test aarch32_CMN_r_T2_A field imm3 = 0 (Zero)
    // ISET: T32
    // Fields: Rn=0, imm3=0, type1=0, Rm=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEB100F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_cmn_r_t2_a_field_imm3_1_poweroftwo_f00_eb101f00() {
    // Thumb encoding (32): 0xEB101F00
    // Test aarch32_CMN_r_T2_A field imm3 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=1, Rn=0, Rm=0, imm2=0, type1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEB101F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch32_cmn_r_t2_a_field_imm3_3_poweroftwominusone_f00_eb103f00() {
    // Thumb encoding (32): 0xEB103F00
    // Test aarch32_CMN_r_T2_A field imm3 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rn=0, imm2=0, type1=0, imm3=3, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEB103F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch32_cmn_r_t2_a_field_imm3_7_max_f00_eb107f00() {
    // Thumb encoding (32): 0xEB107F00
    // Test aarch32_CMN_r_T2_A field imm3 = 7 (Max)
    // ISET: T32
    // Fields: type1=0, imm2=0, Rn=0, imm3=7, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEB107F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `field imm2 6 +: 2`
/// Requirement: FieldBoundary { field: "imm2", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_cmn_r_t2_a_field_imm2_0_zero_f00_eb100f00() {
    // Thumb encoding (32): 0xEB100F00
    // Test aarch32_CMN_r_T2_A field imm2 = 0 (Zero)
    // ISET: T32
    // Fields: type1=0, Rm=0, imm3=0, imm2=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEB100F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `field imm2 6 +: 2`
/// Requirement: FieldBoundary { field: "imm2", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_cmn_r_t2_a_field_imm2_1_poweroftwo_f00_eb100f40() {
    // Thumb encoding (32): 0xEB100F40
    // Test aarch32_CMN_r_T2_A field imm2 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=0, Rm=0, Rn=0, type1=0, imm2=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEB100F40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `field imm2 6 +: 2`
/// Requirement: FieldBoundary { field: "imm2", value: 3, boundary: Max }
/// maximum immediate (3)
#[test]
fn test_aarch32_cmn_r_t2_a_field_imm2_3_max_f00_eb100fc0() {
    // Thumb encoding (32): 0xEB100FC0
    // Test aarch32_CMN_r_T2_A field imm2 = 3 (Max)
    // ISET: T32
    // Fields: imm2=3, Rn=0, imm3=0, Rm=0, type1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEB100FC0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `field type1 4 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_cmn_r_t2_a_field_type1_0_min_f00_eb100f00() {
    // Thumb encoding (32): 0xEB100F00
    // Test aarch32_CMN_r_T2_A field type1 = 0 (Min)
    // ISET: T32
    // Fields: imm2=0, type1=0, imm3=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEB100F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `field type1 4 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_cmn_r_t2_a_field_type1_1_poweroftwo_f00_eb100f10() {
    // Thumb encoding (32): 0xEB100F10
    // Test aarch32_CMN_r_T2_A field type1 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=0, type1=1, Rn=0, Rm=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEB100F10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `field type1 4 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_cmn_r_t2_a_field_type1_3_max_f00_eb100f30() {
    // Thumb encoding (32): 0xEB100F30
    // Test aarch32_CMN_r_T2_A field type1 = 3 (Max)
    // ISET: T32
    // Fields: imm2=0, Rm=0, imm3=0, type1=3, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEB100F30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmn_r_t2_a_field_rm_0_min_f00_eb100f00() {
    // Thumb encoding (32): 0xEB100F00
    // Test aarch32_CMN_r_T2_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rm=0, type1=0, imm3=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEB100F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmn_r_t2_a_field_rm_1_poweroftwo_f00_eb100f01() {
    // Thumb encoding (32): 0xEB100F01
    // Test aarch32_CMN_r_T2_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=0, Rm=1, Rn=0, imm2=0, type1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEB100F01;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_cmn_r_t2_a_combo_0_f00_eb100f00() {
    // Thumb encoding (32): 0xEB100F00
    // Test aarch32_CMN_r_T2_A field combination: Rn=0, imm3=0, imm2=0, type1=0, Rm=0
    // ISET: T32
    // Fields: imm2=0, Rn=0, type1=0, imm3=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEB100F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_cmn_r_t2_a_invalid_0_f00_eb100f00() {
    // Thumb encoding (32): 0xEB100F00
    // Test aarch32_CMN_r_T2_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: imm3=0, type1=0, Rm=0, Rn=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEB100F00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_cmn_r_t2_a_invalid_1_f00_eb100f00() {
    // Thumb encoding (32): 0xEB100F00
    // Test aarch32_CMN_r_T2_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rn=0, Rm=0, imm3=0, type1=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEB100F00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_cmn_r_a1_a_flags_zeroresult_0_01710002() {
    // Test aarch32_CMN_r_A1_A flag computation: ZeroResult
    // Encoding: 0x01710002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x01710002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_cmn_r_a1_a_flags_zeroresult_1_01710002() {
    // Test aarch32_CMN_r_A1_A flag computation: ZeroResult
    // Encoding: 0x01710002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x01710002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_cmn_r_a1_a_flags_negativeresult_2_01710002() {
    // Test aarch32_CMN_r_A1_A flag computation: NegativeResult
    // Encoding: 0x01710002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x01710002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_cmn_r_a1_a_flags_unsignedoverflow_3_01710002() {
    // Test aarch32_CMN_r_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x01710002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x01710002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_cmn_r_a1_a_flags_unsignedoverflow_4_01710002() {
    // Test aarch32_CMN_r_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x01710002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x01710002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_cmn_r_a1_a_flags_signedoverflow_5_01710002() {
    // Test aarch32_CMN_r_A1_A flag computation: SignedOverflow
    // Encoding: 0x01710002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x01710002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_cmn_r_a1_a_flags_signedoverflow_6_01710002() {
    // Test aarch32_CMN_r_A1_A flag computation: SignedOverflow
    // Encoding: 0x01710002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x01710002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMN_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_cmn_r_a1_a_flags_positiveresult_7_01710002() {
    // Test aarch32_CMN_r_A1_A flag computation: PositiveResult
    // Encoding: 0x01710002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u32 = 0x01710002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 0 (32)
#[test]
fn test_aarch32_cmn_r_t1_a_lslv_oracle_32_0_42c20020() {
    // Test LSLV 32-bit: shift by 0 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x12345678, "W0 should be 0x12345678");
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 0 (64)
#[test]
fn test_aarch32_cmn_r_t1_a_lslv_oracle_64_0_c2c20020() {
    // Test LSLV 64-bit: shift by 0 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x12345678);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x12345678,
        "X0 should be 0x0000000012345678"
    );
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4 (32)
#[test]
fn test_aarch32_cmn_r_t1_a_lslv_oracle_32_1_42c20020() {
    // Test LSLV 32-bit: shift by 4 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x4);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x23456780, "W0 should be 0x23456780");
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 4 (64)
#[test]
fn test_aarch32_cmn_r_t1_a_lslv_oracle_64_1_c2c20020() {
    // Test LSLV 64-bit: shift by 4 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x4);
    set_w(&mut cpu, 1, 0x12345678);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x23456780,
        "X0 should be 0x0000000123456780"
    );
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 8 (32)
#[test]
fn test_aarch32_cmn_r_t1_a_lslv_oracle_32_2_42c20020() {
    // Test LSLV 32-bit: shift by 8 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x8);
    set_w(&mut cpu, 1, 0x12345678);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x34567800, "W0 should be 0x34567800");
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 8 (64)
#[test]
fn test_aarch32_cmn_r_t1_a_lslv_oracle_64_2_c2c20020() {
    // Test LSLV 64-bit: shift by 8 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x8);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x34567800,
        "X0 should be 0x0000001234567800"
    );
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1 (32)
#[test]
fn test_aarch32_cmn_r_t1_a_lslv_oracle_32_3_42c20020() {
    // Test LSLV 32-bit: MSB set, shift 1 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x00000000");
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// MSB set, shift 1 (64)
#[test]
fn test_aarch32_cmn_r_t1_a_lslv_oracle_64_3_c2c20020() {
    // Test LSLV 64-bit: MSB set, shift 1 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSB set, max shift (32)
#[test]
fn test_aarch32_cmn_r_t1_a_lslv_oracle_32_4_42c20020() {
    // Test LSLV 32-bit: LSB set, max shift (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0x3F);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x80000000, "W0 should be 0x80000000");
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSB set, max shift (64)
#[test]
fn test_aarch32_cmn_r_t1_a_lslv_oracle_64_4_c2c20020() {
    // Test LSLV 64-bit: LSB set, max shift (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0x3F);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x8000000000000000");
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// all ones, shift 32 (32)
#[test]
fn test_aarch32_cmn_r_t1_a_lslv_oracle_32_5_42c20020() {
    // Test LSLV 32-bit: all ones, shift 32 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x20);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "W0 should be 0xFFFFFFFF");
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all ones, shift 32 (64)
#[test]
fn test_aarch32_cmn_r_t1_a_lslv_oracle_64_5_c2c20020() {
    // Test LSLV 64-bit: all ones, shift 32 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x20);
    let encoding: u16 = 0x0020;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0xFFFFFFFF00000000");
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift
#[test]
fn test_aarch32_cmn_r_t1_a_t16_oracle_0_42d10000() {
    // Test T16 LSLS: no shift (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF, "R0 should be 0x000000FF");
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4
#[test]
fn test_aarch32_cmn_r_t1_a_t16_oracle_1_42d10000() {
    // Test T16 LSLS: shift by 4 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x4);
    set_w(&mut cpu, 1, 0xFF);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF0, "R0 should be 0x00000FF0");
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1
#[test]
fn test_aarch32_cmn_r_t1_a_t16_oracle_2_42d10000() {
    // Test T16 LSLS: MSB set, shift 1 (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0x80000000);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift to MSB
#[test]
fn test_aarch32_cmn_r_t1_a_t16_oracle_3_42d10000() {
    // Test T16 LSLS: shift to MSB (oracle)
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0x1F);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x80000000, "R0 should be 0x80000000");
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_cmn_r_t1_a_flags_zeroresult_0_42d10000() {
    // Test aarch32_CMN_r_T1_A flag computation: ZeroResult
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_cmn_r_t1_a_flags_zeroresult_1_42d10000() {
    // Test aarch32_CMN_r_T1_A flag computation: ZeroResult
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_cmn_r_t1_a_flags_negativeresult_2_42d10000() {
    // Test aarch32_CMN_r_T1_A flag computation: NegativeResult
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_cmn_r_t1_a_flags_unsignedoverflow_3_42d10000() {
    // Test aarch32_CMN_r_T1_A flag computation: UnsignedOverflow
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_cmn_r_t1_a_flags_unsignedoverflow_4_42d10000() {
    // Test aarch32_CMN_r_T1_A flag computation: UnsignedOverflow
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x2);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_cmn_r_t1_a_flags_signedoverflow_5_42d10000() {
    // Test aarch32_CMN_r_T1_A flag computation: SignedOverflow
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_cmn_r_t1_a_flags_signedoverflow_6_42d10000() {
    // Test aarch32_CMN_r_T1_A flag computation: SignedOverflow
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMN_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_cmn_r_t1_a_flags_positiveresult_7_42d10000() {
    // Test aarch32_CMN_r_T1_A flag computation: PositiveResult
    // ISET: T16
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u16 = 0x0000;
    // T16: Write 16-bit instruction
    cpu.write_memory(0, &(encoding as u32).to_le_bytes())
        .unwrap();
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_cmn_r_t2_a_flags_zeroresult_0_eb110f02() {
    // Test aarch32_CMN_r_T2_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xEB110F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_cmn_r_t2_a_flags_zeroresult_1_eb110f02() {
    // Test aarch32_CMN_r_T2_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0xEB110F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_cmn_r_t2_a_flags_negativeresult_2_eb110f02() {
    // Test aarch32_CMN_r_T2_A flag computation: NegativeResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xEB110F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_cmn_r_t2_a_flags_unsignedoverflow_3_eb110f02() {
    // Test aarch32_CMN_r_T2_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xEB110F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_cmn_r_t2_a_flags_unsignedoverflow_4_eb110f02() {
    // Test aarch32_CMN_r_T2_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x2);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xEB110F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_cmn_r_t2_a_flags_signedoverflow_5_eb110f02() {
    // Test aarch32_CMN_r_T2_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xEB110F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_cmn_r_t2_a_flags_signedoverflow_6_eb110f02() {
    // Test aarch32_CMN_r_T2_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0xEB110F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMN_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_cmn_r_t2_a_flags_positiveresult_7_eb110f02() {
    // Test aarch32_CMN_r_T2_A flag computation: PositiveResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x32);
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xEB110F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

// ============================================================================
// aarch32_CMN_rr_A Tests
// ============================================================================

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_cmn_rr_a1_a_field_cond_0_min_10_01700010() {
    // Encoding: 0x01700010
    // Test aarch32_CMN_rr_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rs=0, type1=0, Rn=0, Rm=0
    let encoding: u32 = 0x01700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_cmn_rr_a1_a_field_cond_1_poweroftwo_10_11700010() {
    // Encoding: 0x11700010
    // Test aarch32_CMN_rr_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rs=0, Rn=0, cond=1, type1=0, Rm=0
    let encoding: u32 = 0x11700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_cmn_rr_a1_a_field_cond_2_poweroftwo_10_21700010() {
    // Encoding: 0x21700010
    // Test aarch32_CMN_rr_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rs=0, type1=0, cond=2
    let encoding: u32 = 0x21700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_cmn_rr_a1_a_field_cond_3_poweroftwo_10_31700010() {
    // Encoding: 0x31700010
    // Test aarch32_CMN_rr_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=3, type1=0, Rs=0, Rm=0
    let encoding: u32 = 0x31700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_cmn_rr_a1_a_field_cond_4_poweroftwo_10_41700010() {
    // Encoding: 0x41700010
    // Test aarch32_CMN_rr_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rs=0, Rm=0, cond=4, Rn=0, type1=0
    let encoding: u32 = 0x41700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_cmn_rr_a1_a_field_cond_5_poweroftwo_10_51700010() {
    // Encoding: 0x51700010
    // Test aarch32_CMN_rr_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, type1=0, Rm=0, cond=5, Rs=0
    let encoding: u32 = 0x51700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_cmn_rr_a1_a_field_cond_6_poweroftwo_10_61700010() {
    // Encoding: 0x61700010
    // Test aarch32_CMN_rr_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rn=0, cond=6, Rm=0, Rs=0
    let encoding: u32 = 0x61700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_cmn_rr_a1_a_field_cond_7_poweroftwo_10_71700010() {
    // Encoding: 0x71700010
    // Test aarch32_CMN_rr_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=7, Rs=0, Rn=0, Rm=0, type1=0
    let encoding: u32 = 0x71700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_cmn_rr_a1_a_field_cond_8_poweroftwo_10_81700010() {
    // Encoding: 0x81700010
    // Test aarch32_CMN_rr_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=8, Rn=0, Rs=0, Rm=0, type1=0
    let encoding: u32 = 0x81700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_cmn_rr_a1_a_field_cond_9_poweroftwo_10_91700010() {
    // Encoding: 0x91700010
    // Test aarch32_CMN_rr_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rs=0, type1=0, Rn=0, cond=9
    let encoding: u32 = 0x91700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_cmn_rr_a1_a_field_cond_10_poweroftwo_10_a1700010() {
    // Encoding: 0xA1700010
    // Test aarch32_CMN_rr_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, type1=0, Rm=0, Rs=0, Rn=0
    let encoding: u32 = 0xA1700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_cmn_rr_a1_a_field_cond_11_poweroftwo_10_b1700010() {
    // Encoding: 0xB1700010
    // Test aarch32_CMN_rr_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, cond=11, Rn=0, Rm=0, Rs=0
    let encoding: u32 = 0xB1700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_cmn_rr_a1_a_field_cond_12_poweroftwo_10_c1700010() {
    // Encoding: 0xC1700010
    // Test aarch32_CMN_rr_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, Rs=0, Rm=0, type1=0, Rn=0
    let encoding: u32 = 0xC1700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_cmn_rr_a1_a_field_cond_13_poweroftwo_10_d1700010() {
    // Encoding: 0xD1700010
    // Test aarch32_CMN_rr_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rs=0, Rn=0, cond=13, Rm=0
    let encoding: u32 = 0xD1700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_cmn_rr_a1_a_field_cond_14_poweroftwo_10_e1700010() {
    // Encoding: 0xE1700010
    // Test aarch32_CMN_rr_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rs=0, Rm=0, cond=14, Rn=0, type1=0
    let encoding: u32 = 0xE1700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_cmn_rr_a1_a_field_cond_15_max_10_f1700010() {
    // Encoding: 0xF1700010
    // Test aarch32_CMN_rr_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: type1=0, Rm=0, cond=15, Rn=0, Rs=0
    let encoding: u32 = 0xF1700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmn_rr_a1_a_field_rn_0_min_10_01700010() {
    // Encoding: 0x01700010
    // Test aarch32_CMN_rr_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, type1=0, Rs=0, Rm=0
    let encoding: u32 = 0x01700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmn_rr_a1_a_field_rn_1_poweroftwo_10_01710010() {
    // Encoding: 0x01710010
    // Test aarch32_CMN_rr_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=1, Rm=0, Rs=0, type1=0
    let encoding: u32 = 0x01710010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field Rs 8 +: 4`
/// Requirement: FieldBoundary { field: "Rs", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmn_rr_a1_a_field_rs_0_min_10_01700010() {
    // Encoding: 0x01700010
    // Test aarch32_CMN_rr_A1_A field Rs = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=0, type1=0, Rs=0
    let encoding: u32 = 0x01700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field Rs 8 +: 4`
/// Requirement: FieldBoundary { field: "Rs", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmn_rr_a1_a_field_rs_1_poweroftwo_10_01700110() {
    // Encoding: 0x01700110
    // Test aarch32_CMN_rr_A1_A field Rs = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, type1=0, Rm=0, Rs=1
    let encoding: u32 = 0x01700110;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_cmn_rr_a1_a_field_type1_0_min_10_01700010() {
    // Encoding: 0x01700010
    // Test aarch32_CMN_rr_A1_A field type1 = 0 (Min)
    // ISET: A32
    // Fields: type1=0, Rm=0, Rn=0, Rs=0, cond=0
    let encoding: u32 = 0x01700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_cmn_rr_a1_a_field_type1_1_poweroftwo_10_01700030() {
    // Encoding: 0x01700030
    // Test aarch32_CMN_rr_A1_A field type1 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rs=0, cond=0, Rn=0, type1=1, Rm=0
    let encoding: u32 = 0x01700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_cmn_rr_a1_a_field_type1_3_max_10_01700070() {
    // Encoding: 0x01700070
    // Test aarch32_CMN_rr_A1_A field type1 = 3 (Max)
    // ISET: A32
    // Fields: cond=0, Rs=0, Rn=0, Rm=0, type1=3
    let encoding: u32 = 0x01700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_cmn_rr_a1_a_field_rm_0_min_10_01700010() {
    // Encoding: 0x01700010
    // Test aarch32_CMN_rr_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rs=0, type1=0, Rm=0, cond=0
    let encoding: u32 = 0x01700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_cmn_rr_a1_a_field_rm_1_poweroftwo_10_01700011() {
    // Encoding: 0x01700011
    // Test aarch32_CMN_rr_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rm=1, Rs=0, Rn=0, type1=0
    let encoding: u32 = 0x01700011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_cmn_rr_a1_a_combo_0_10_01700010() {
    // Encoding: 0x01700010
    // Test aarch32_CMN_rr_A1_A field combination: cond=0, Rn=0, Rs=0, type1=0, Rm=0
    // ISET: A32
    // Fields: cond=0, Rs=0, Rm=0, Rn=0, type1=0
    let encoding: u32 = 0x01700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_cmn_rr_a1_a_special_cond_0_condition_eq_16_01700010() {
    // Encoding: 0x01700010
    // Test aarch32_CMN_rr_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: cond=0, Rs=0, Rm=0, type1=0, Rn=0
    let encoding: u32 = 0x01700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_cmn_rr_a1_a_special_cond_1_condition_ne_16_11700010() {
    // Encoding: 0x11700010
    // Test aarch32_CMN_rr_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rm=0, cond=1, Rs=0, type1=0, Rn=0
    let encoding: u32 = 0x11700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_cmn_rr_a1_a_special_cond_2_condition_cs_hs_16_21700010() {
    // Encoding: 0x21700010
    // Test aarch32_CMN_rr_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: type1=0, cond=2, Rm=0, Rn=0, Rs=0
    let encoding: u32 = 0x21700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_cmn_rr_a1_a_special_cond_3_condition_cc_lo_16_31700010() {
    // Encoding: 0x31700010
    // Test aarch32_CMN_rr_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rn=0, Rs=0, type1=0, Rm=0, cond=3
    let encoding: u32 = 0x31700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_cmn_rr_a1_a_special_cond_4_condition_mi_16_41700010() {
    // Encoding: 0x41700010
    // Test aarch32_CMN_rr_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: type1=0, Rs=0, Rm=0, cond=4, Rn=0
    let encoding: u32 = 0x41700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_cmn_rr_a1_a_special_cond_5_condition_pl_16_51700010() {
    // Encoding: 0x51700010
    // Test aarch32_CMN_rr_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rn=0, type1=0, cond=5, Rs=0, Rm=0
    let encoding: u32 = 0x51700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_cmn_rr_a1_a_special_cond_6_condition_vs_16_61700010() {
    // Encoding: 0x61700010
    // Test aarch32_CMN_rr_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: type1=0, Rn=0, Rm=0, Rs=0, cond=6
    let encoding: u32 = 0x61700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_cmn_rr_a1_a_special_cond_7_condition_vc_16_71700010() {
    // Encoding: 0x71700010
    // Test aarch32_CMN_rr_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, Rs=0, Rm=0, Rn=0, type1=0
    let encoding: u32 = 0x71700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_cmn_rr_a1_a_special_cond_8_condition_hi_16_81700010() {
    // Encoding: 0x81700010
    // Test aarch32_CMN_rr_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rs=0, Rn=0, Rm=0, type1=0, cond=8
    let encoding: u32 = 0x81700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_cmn_rr_a1_a_special_cond_9_condition_ls_16_91700010() {
    // Encoding: 0x91700010
    // Test aarch32_CMN_rr_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rn=0, cond=9, Rs=0, Rm=0, type1=0
    let encoding: u32 = 0x91700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_cmn_rr_a1_a_special_cond_10_condition_ge_16_a1700010() {
    // Encoding: 0xA1700010
    // Test aarch32_CMN_rr_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: type1=0, Rm=0, cond=10, Rs=0, Rn=0
    let encoding: u32 = 0xA1700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_cmn_rr_a1_a_special_cond_11_condition_lt_16_b1700010() {
    // Encoding: 0xB1700010
    // Test aarch32_CMN_rr_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rs=0, cond=11, type1=0, Rm=0, Rn=0
    let encoding: u32 = 0xB1700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_cmn_rr_a1_a_special_cond_12_condition_gt_16_c1700010() {
    // Encoding: 0xC1700010
    // Test aarch32_CMN_rr_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: cond=12, Rs=0, Rn=0, type1=0, Rm=0
    let encoding: u32 = 0xC1700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_cmn_rr_a1_a_special_cond_13_condition_le_16_d1700010() {
    // Encoding: 0xD1700010
    // Test aarch32_CMN_rr_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rm=0, Rn=0, type1=0, Rs=0, cond=13
    let encoding: u32 = 0xD1700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_cmn_rr_a1_a_special_cond_14_condition_al_16_e1700010() {
    // Encoding: 0xE1700010
    // Test aarch32_CMN_rr_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: cond=14, Rs=0, type1=0, Rn=0, Rm=0
    let encoding: u32 = 0xE1700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_cmn_rr_a1_a_special_cond_15_condition_nv_16_f1700010() {
    // Encoding: 0xF1700010
    // Test aarch32_CMN_rr_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rn=0, Rs=0, type1=0, cond=15, Rm=0
    let encoding: u32 = 0xF1700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "s" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"s\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_cmn_rr_a1_a_invalid_0_10_01700010() {
    // Encoding: 0x01700010
    // Test aarch32_CMN_rr_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "s" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rm=0, Rs=0, Rn=0, type1=0, cond=0
    let encoding: u32 = 0x01700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_cmn_rr_a1_a_invalid_1_10_01700010() {
    // Encoding: 0x01700010
    // Test aarch32_CMN_rr_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: cond=0, Rm=0, Rn=0, type1=0, Rs=0
    let encoding: u32 = 0x01700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_cmn_rr_a1_a_flags_zeroresult_0_01710012() {
    // Test aarch32_CMN_rr_A1_A flag computation: ZeroResult
    // Encoding: 0x01710012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x01710012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_cmn_rr_a1_a_flags_zeroresult_1_01710012() {
    // Test aarch32_CMN_rr_A1_A flag computation: ZeroResult
    // Encoding: 0x01710012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x01710012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_cmn_rr_a1_a_flags_negativeresult_2_01710012() {
    // Test aarch32_CMN_rr_A1_A flag computation: NegativeResult
    // Encoding: 0x01710012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x01710012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_cmn_rr_a1_a_flags_unsignedoverflow_3_01710012() {
    // Test aarch32_CMN_rr_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x01710012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x01710012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_cmn_rr_a1_a_flags_unsignedoverflow_4_01710012() {
    // Test aarch32_CMN_rr_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x01710012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x01710012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_cmn_rr_a1_a_flags_signedoverflow_5_01710012() {
    // Test aarch32_CMN_rr_A1_A flag computation: SignedOverflow
    // Encoding: 0x01710012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x01710012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_cmn_rr_a1_a_flags_signedoverflow_6_01710012() {
    // Test aarch32_CMN_rr_A1_A flag computation: SignedOverflow
    // Encoding: 0x01710012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x01710012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_CMN_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_cmn_rr_a1_a_flags_positiveresult_7_01710012() {
    // Test aarch32_CMN_rr_A1_A flag computation: PositiveResult
    // Encoding: 0x01710012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u32 = 0x01710012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

// ============================================================================
// aarch32_TEQ_r_A Tests
// ============================================================================

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_teq_r_a1_a_field_cond_0_min_0_01300000() {
    // Encoding: 0x01300000
    // Test aarch32_TEQ_r_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: type1=0, cond=0, imm5=0, Rm=0, Rn=0
    let encoding: u32 = 0x01300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_teq_r_a1_a_field_cond_1_poweroftwo_0_11300000() {
    // Encoding: 0x11300000
    // Test aarch32_TEQ_r_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, type1=0, Rm=0, imm5=0, Rn=0
    let encoding: u32 = 0x11300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_teq_r_a1_a_field_cond_2_poweroftwo_0_21300000() {
    // Encoding: 0x21300000
    // Test aarch32_TEQ_r_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=0, Rm=0, Rn=0, type1=0, cond=2
    let encoding: u32 = 0x21300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_teq_r_a1_a_field_cond_3_poweroftwo_0_31300000() {
    // Encoding: 0x31300000
    // Test aarch32_TEQ_r_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, imm5=0, Rn=0, cond=3, type1=0
    let encoding: u32 = 0x31300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_teq_r_a1_a_field_cond_4_poweroftwo_0_41300000() {
    // Encoding: 0x41300000
    // Test aarch32_TEQ_r_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=4, type1=0, imm5=0
    let encoding: u32 = 0x41300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_teq_r_a1_a_field_cond_5_poweroftwo_0_51300000() {
    // Encoding: 0x51300000
    // Test aarch32_TEQ_r_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=0, type1=0, cond=5, Rm=0, Rn=0
    let encoding: u32 = 0x51300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_teq_r_a1_a_field_cond_6_poweroftwo_0_61300000() {
    // Encoding: 0x61300000
    // Test aarch32_TEQ_r_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, cond=6, Rn=0, imm5=0, Rm=0
    let encoding: u32 = 0x61300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_teq_r_a1_a_field_cond_7_poweroftwo_0_71300000() {
    // Encoding: 0x71300000
    // Test aarch32_TEQ_r_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=7, type1=0, imm5=0, Rm=0, Rn=0
    let encoding: u32 = 0x71300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_teq_r_a1_a_field_cond_8_poweroftwo_0_81300000() {
    // Encoding: 0x81300000
    // Test aarch32_TEQ_r_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=0, type1=0, Rn=0, cond=8, Rm=0
    let encoding: u32 = 0x81300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_teq_r_a1_a_field_cond_9_poweroftwo_0_91300000() {
    // Encoding: 0x91300000
    // Test aarch32_TEQ_r_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, type1=0, imm5=0, Rm=0, cond=9
    let encoding: u32 = 0x91300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_teq_r_a1_a_field_cond_10_poweroftwo_0_a1300000() {
    // Encoding: 0xA1300000
    // Test aarch32_TEQ_r_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, type1=0, cond=10, Rm=0, imm5=0
    let encoding: u32 = 0xA1300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_teq_r_a1_a_field_cond_11_poweroftwo_0_b1300000() {
    // Encoding: 0xB1300000
    // Test aarch32_TEQ_r_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rn=0, imm5=0, cond=11, Rm=0
    let encoding: u32 = 0xB1300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_teq_r_a1_a_field_cond_12_poweroftwo_0_c1300000() {
    // Encoding: 0xC1300000
    // Test aarch32_TEQ_r_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=12, Rn=0, imm5=0, type1=0
    let encoding: u32 = 0xC1300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_teq_r_a1_a_field_cond_13_poweroftwo_0_d1300000() {
    // Encoding: 0xD1300000
    // Test aarch32_TEQ_r_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rm=0, Rn=0, imm5=0, cond=13
    let encoding: u32 = 0xD1300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_teq_r_a1_a_field_cond_14_poweroftwo_0_e1300000() {
    // Encoding: 0xE1300000
    // Test aarch32_TEQ_r_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=14, type1=0, Rm=0, imm5=0
    let encoding: u32 = 0xE1300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_teq_r_a1_a_field_cond_15_max_0_f1300000() {
    // Encoding: 0xF1300000
    // Test aarch32_TEQ_r_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rm=0, imm5=0, cond=15, Rn=0, type1=0
    let encoding: u32 = 0xF1300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_teq_r_a1_a_field_rn_0_min_0_01300000() {
    // Encoding: 0x01300000
    // Test aarch32_TEQ_r_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, imm5=0, type1=0, Rm=0
    let encoding: u32 = 0x01300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_teq_r_a1_a_field_rn_1_poweroftwo_0_01310000() {
    // Encoding: 0x01310000
    // Test aarch32_TEQ_r_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=1, cond=0, Rm=0, type1=0, imm5=0
    let encoding: u32 = 0x01310000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_teq_r_a1_a_field_imm5_0_zero_0_01300000() {
    // Encoding: 0x01300000
    // Test aarch32_TEQ_r_A1_A field imm5 = 0 (Zero)
    // ISET: A32
    // Fields: imm5=0, type1=0, Rn=0, Rm=0, cond=0
    let encoding: u32 = 0x01300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_teq_r_a1_a_field_imm5_1_poweroftwo_0_01300080() {
    // Encoding: 0x01300080
    // Test aarch32_TEQ_r_A1_A field imm5 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, imm5=1, type1=0, Rn=0, cond=0
    let encoding: u32 = 0x01300080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_teq_r_a1_a_field_imm5_3_poweroftwominusone_0_01300180() {
    // Encoding: 0x01300180
    // Test aarch32_TEQ_r_A1_A field imm5 = 3 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rn=0, imm5=3, type1=0, Rm=0, cond=0
    let encoding: u32 = 0x01300180;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_teq_r_a1_a_field_imm5_4_poweroftwo_0_01300200() {
    // Encoding: 0x01300200
    // Test aarch32_TEQ_r_A1_A field imm5 = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm5=4, Rm=0, type1=0, cond=0
    let encoding: u32 = 0x01300200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_teq_r_a1_a_field_imm5_7_poweroftwominusone_0_01300380() {
    // Encoding: 0x01300380
    // Test aarch32_TEQ_r_A1_A field imm5 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm5=7, cond=0, type1=0, Rm=0, Rn=0
    let encoding: u32 = 0x01300380;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_teq_r_a1_a_field_imm5_8_poweroftwo_0_01300400() {
    // Encoding: 0x01300400
    // Test aarch32_TEQ_r_A1_A field imm5 = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, type1=0, cond=0, imm5=8
    let encoding: u32 = 0x01300400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 15, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (15)
#[test]
fn test_aarch32_teq_r_a1_a_field_imm5_15_poweroftwominusone_0_01300780() {
    // Encoding: 0x01300780
    // Test aarch32_TEQ_r_A1_A field imm5 = 15 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm5=15, Rn=0, type1=0, cond=0, Rm=0
    let encoding: u32 = 0x01300780;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_teq_r_a1_a_field_imm5_16_poweroftwo_0_01300800() {
    // Encoding: 0x01300800
    // Test aarch32_TEQ_r_A1_A field imm5 = 16 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm5=16, cond=0, type1=0, Rm=0
    let encoding: u32 = 0x01300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 31, boundary: Max }
/// maximum immediate (31)
#[test]
fn test_aarch32_teq_r_a1_a_field_imm5_31_max_0_01300f80() {
    // Encoding: 0x01300F80
    // Test aarch32_TEQ_r_A1_A field imm5 = 31 (Max)
    // ISET: A32
    // Fields: Rn=0, type1=0, cond=0, Rm=0, imm5=31
    let encoding: u32 = 0x01300F80;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_teq_r_a1_a_field_type1_0_min_0_01300000() {
    // Encoding: 0x01300000
    // Test aarch32_TEQ_r_A1_A field type1 = 0 (Min)
    // ISET: A32
    // Fields: cond=0, imm5=0, Rn=0, type1=0, Rm=0
    let encoding: u32 = 0x01300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_teq_r_a1_a_field_type1_1_poweroftwo_0_01300020() {
    // Encoding: 0x01300020
    // Test aarch32_TEQ_r_A1_A field type1 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=0, type1=1, cond=0, Rn=0, Rm=0
    let encoding: u32 = 0x01300020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_teq_r_a1_a_field_type1_3_max_0_01300060() {
    // Encoding: 0x01300060
    // Test aarch32_TEQ_r_A1_A field type1 = 3 (Max)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rm=0, type1=3, imm5=0
    let encoding: u32 = 0x01300060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_teq_r_a1_a_field_rm_0_min_0_01300000() {
    // Encoding: 0x01300000
    // Test aarch32_TEQ_r_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rn=0, imm5=0, type1=0, cond=0
    let encoding: u32 = 0x01300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_teq_r_a1_a_field_rm_1_poweroftwo_0_01300001() {
    // Encoding: 0x01300001
    // Test aarch32_TEQ_r_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=0, Rm=1, type1=0, Rn=0, cond=0
    let encoding: u32 = 0x01300001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_teq_r_a1_a_combo_0_0_01300000() {
    // Encoding: 0x01300000
    // Test aarch32_TEQ_r_A1_A field combination: cond=0, Rn=0, imm5=0, type1=0, Rm=0
    // ISET: A32
    // Fields: cond=0, imm5=0, type1=0, Rn=0, Rm=0
    let encoding: u32 = 0x01300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_teq_r_a1_a_special_cond_0_condition_eq_0_01300000() {
    // Encoding: 0x01300000
    // Test aarch32_TEQ_r_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rn=0, type1=0, Rm=0, imm5=0, cond=0
    let encoding: u32 = 0x01300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_teq_r_a1_a_special_cond_1_condition_ne_0_11300000() {
    // Encoding: 0x11300000
    // Test aarch32_TEQ_r_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rn=0, Rm=0, imm5=0, cond=1, type1=0
    let encoding: u32 = 0x11300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_teq_r_a1_a_special_cond_2_condition_cs_hs_0_21300000() {
    // Encoding: 0x21300000
    // Test aarch32_TEQ_r_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rm=0, type1=0, imm5=0, cond=2, Rn=0
    let encoding: u32 = 0x21300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_teq_r_a1_a_special_cond_3_condition_cc_lo_0_31300000() {
    // Encoding: 0x31300000
    // Test aarch32_TEQ_r_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, imm5=0, Rn=0, type1=0, Rm=0
    let encoding: u32 = 0x31300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_teq_r_a1_a_special_cond_4_condition_mi_0_41300000() {
    // Encoding: 0x41300000
    // Test aarch32_TEQ_r_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rn=0, Rm=0, imm5=0, type1=0, cond=4
    let encoding: u32 = 0x41300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_teq_r_a1_a_special_cond_5_condition_pl_0_51300000() {
    // Encoding: 0x51300000
    // Test aarch32_TEQ_r_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, Rm=0, imm5=0, Rn=0, type1=0
    let encoding: u32 = 0x51300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_teq_r_a1_a_special_cond_6_condition_vs_0_61300000() {
    // Encoding: 0x61300000
    // Test aarch32_TEQ_r_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, imm5=0, Rn=0, Rm=0, type1=0
    let encoding: u32 = 0x61300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_teq_r_a1_a_special_cond_7_condition_vc_0_71300000() {
    // Encoding: 0x71300000
    // Test aarch32_TEQ_r_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, type1=0, Rm=0, Rn=0, imm5=0
    let encoding: u32 = 0x71300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_teq_r_a1_a_special_cond_8_condition_hi_0_81300000() {
    // Encoding: 0x81300000
    // Test aarch32_TEQ_r_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: type1=0, imm5=0, cond=8, Rm=0, Rn=0
    let encoding: u32 = 0x81300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_teq_r_a1_a_special_cond_9_condition_ls_0_91300000() {
    // Encoding: 0x91300000
    // Test aarch32_TEQ_r_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rn=0, cond=9, imm5=0, Rm=0, type1=0
    let encoding: u32 = 0x91300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_teq_r_a1_a_special_cond_10_condition_ge_0_a1300000() {
    // Encoding: 0xA1300000
    // Test aarch32_TEQ_r_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: imm5=0, cond=10, Rm=0, Rn=0, type1=0
    let encoding: u32 = 0xA1300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_teq_r_a1_a_special_cond_11_condition_lt_0_b1300000() {
    // Encoding: 0xB1300000
    // Test aarch32_TEQ_r_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, type1=0, Rn=0, imm5=0, Rm=0
    let encoding: u32 = 0xB1300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_teq_r_a1_a_special_cond_12_condition_gt_0_c1300000() {
    // Encoding: 0xC1300000
    // Test aarch32_TEQ_r_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rn=0, type1=0, cond=12, imm5=0, Rm=0
    let encoding: u32 = 0xC1300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_teq_r_a1_a_special_cond_13_condition_le_0_d1300000() {
    // Encoding: 0xD1300000
    // Test aarch32_TEQ_r_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rn=0, cond=13, imm5=0, type1=0, Rm=0
    let encoding: u32 = 0xD1300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_teq_r_a1_a_special_cond_14_condition_al_0_e1300000() {
    // Encoding: 0xE1300000
    // Test aarch32_TEQ_r_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: type1=0, Rn=0, Rm=0, imm5=0, cond=14
    let encoding: u32 = 0xE1300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_teq_r_a1_a_special_cond_15_condition_nv_0_f1300000() {
    // Encoding: 0xF1300000
    // Test aarch32_TEQ_r_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: imm5=0, Rn=0, Rm=0, type1=0, cond=15
    let encoding: u32 = 0xF1300000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_teq_r_t1_a_field_rn_0_min_f00_ea900f00() {
    // Thumb encoding (32): 0xEA900F00
    // Test aarch32_TEQ_r_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: imm3=0, Rm=0, Rn=0, type1=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA900F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_teq_r_t1_a_field_rn_1_poweroftwo_f00_ea910f00() {
    // Thumb encoding (32): 0xEA910F00
    // Test aarch32_TEQ_r_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=0, Rm=0, Rn=1, type1=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA910F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_teq_r_t1_a_field_imm3_0_zero_f00_ea900f00() {
    // Thumb encoding (32): 0xEA900F00
    // Test aarch32_TEQ_r_T1_A field imm3 = 0 (Zero)
    // ISET: T32
    // Fields: Rn=0, imm2=0, type1=0, Rm=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA900F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_teq_r_t1_a_field_imm3_1_poweroftwo_f00_ea901f00() {
    // Thumb encoding (32): 0xEA901F00
    // Test aarch32_TEQ_r_T1_A field imm3 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=1, Rm=0, imm2=0, type1=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA901F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch32_teq_r_t1_a_field_imm3_3_poweroftwominusone_f00_ea903f00() {
    // Thumb encoding (32): 0xEA903F00
    // Test aarch32_TEQ_r_T1_A field imm3 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm3=3, type1=0, Rm=0, Rn=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA903F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch32_teq_r_t1_a_field_imm3_7_max_f00_ea907f00() {
    // Thumb encoding (32): 0xEA907F00
    // Test aarch32_TEQ_r_T1_A field imm3 = 7 (Max)
    // ISET: T32
    // Fields: Rn=0, imm3=7, imm2=0, type1=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA907F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `field imm2 6 +: 2`
/// Requirement: FieldBoundary { field: "imm2", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_teq_r_t1_a_field_imm2_0_zero_f00_ea900f00() {
    // Thumb encoding (32): 0xEA900F00
    // Test aarch32_TEQ_r_T1_A field imm2 = 0 (Zero)
    // ISET: T32
    // Fields: Rn=0, imm3=0, type1=0, Rm=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA900F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `field imm2 6 +: 2`
/// Requirement: FieldBoundary { field: "imm2", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_teq_r_t1_a_field_imm2_1_poweroftwo_f00_ea900f40() {
    // Thumb encoding (32): 0xEA900F40
    // Test aarch32_TEQ_r_T1_A field imm2 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=0, Rn=0, type1=0, Rm=0, imm2=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA900F40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `field imm2 6 +: 2`
/// Requirement: FieldBoundary { field: "imm2", value: 3, boundary: Max }
/// maximum immediate (3)
#[test]
fn test_aarch32_teq_r_t1_a_field_imm2_3_max_f00_ea900fc0() {
    // Thumb encoding (32): 0xEA900FC0
    // Test aarch32_TEQ_r_T1_A field imm2 = 3 (Max)
    // ISET: T32
    // Fields: Rn=0, Rm=0, imm2=3, imm3=0, type1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA900FC0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `field type1 4 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_teq_r_t1_a_field_type1_0_min_f00_ea900f00() {
    // Thumb encoding (32): 0xEA900F00
    // Test aarch32_TEQ_r_T1_A field type1 = 0 (Min)
    // ISET: T32
    // Fields: imm2=0, Rm=0, type1=0, Rn=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA900F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `field type1 4 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_teq_r_t1_a_field_type1_1_poweroftwo_f00_ea900f10() {
    // Thumb encoding (32): 0xEA900F10
    // Test aarch32_TEQ_r_T1_A field type1 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm2=0, Rn=0, imm3=0, type1=1, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA900F10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `field type1 4 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_teq_r_t1_a_field_type1_3_max_f00_ea900f30() {
    // Thumb encoding (32): 0xEA900F30
    // Test aarch32_TEQ_r_T1_A field type1 = 3 (Max)
    // ISET: T32
    // Fields: Rn=0, type1=3, imm2=0, imm3=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA900F30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_teq_r_t1_a_field_rm_0_min_f00_ea900f00() {
    // Thumb encoding (32): 0xEA900F00
    // Test aarch32_TEQ_r_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, type1=0, imm3=0, Rm=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA900F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_teq_r_t1_a_field_rm_1_poweroftwo_f00_ea900f01() {
    // Thumb encoding (32): 0xEA900F01
    // Test aarch32_TEQ_r_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1, imm2=0, type1=0, imm3=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA900F01;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_teq_r_t1_a_combo_0_f00_ea900f00() {
    // Thumb encoding (32): 0xEA900F00
    // Test aarch32_TEQ_r_T1_A field combination: Rn=0, imm3=0, imm2=0, type1=0, Rm=0
    // ISET: T32
    // Fields: Rn=0, imm3=0, Rm=0, type1=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA900F00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_teq_r_t1_a_invalid_0_f00_ea900f00() {
    // Thumb encoding (32): 0xEA900F00
    // Test aarch32_TEQ_r_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: imm2=0, Rn=0, imm3=0, type1=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA900F00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_teq_r_t1_a_invalid_1_f00_ea900f00() {
    // Thumb encoding (32): 0xEA900F00
    // Test aarch32_TEQ_r_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rn=0, imm3=0, type1=0, Rm=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA900F00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_teq_r_a1_a_flags_zeroresult_0_01310002() {
    // Test aarch32_TEQ_r_A1_A flag computation: ZeroResult
    // Encoding: 0x01310002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x01310002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_teq_r_a1_a_flags_zeroresult_1_01310002() {
    // Test aarch32_TEQ_r_A1_A flag computation: ZeroResult
    // Encoding: 0x01310002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x01310002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_teq_r_a1_a_flags_negativeresult_2_01310002() {
    // Test aarch32_TEQ_r_A1_A flag computation: NegativeResult
    // Encoding: 0x01310002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x01310002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_teq_r_a1_a_flags_unsignedoverflow_3_01310002() {
    // Test aarch32_TEQ_r_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x01310002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x01310002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_teq_r_a1_a_flags_unsignedoverflow_4_01310002() {
    // Test aarch32_TEQ_r_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x01310002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x01310002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_teq_r_a1_a_flags_signedoverflow_5_01310002() {
    // Test aarch32_TEQ_r_A1_A flag computation: SignedOverflow
    // Encoding: 0x01310002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x01310002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_teq_r_a1_a_flags_signedoverflow_6_01310002() {
    // Test aarch32_TEQ_r_A1_A flag computation: SignedOverflow
    // Encoding: 0x01310002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x01310002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TEQ_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_teq_r_a1_a_flags_positiveresult_7_01310002() {
    // Test aarch32_TEQ_r_A1_A flag computation: PositiveResult
    // Encoding: 0x01310002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x32);
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x01310002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_teq_r_t1_a_flags_zeroresult_0_ea910f02() {
    // Test aarch32_TEQ_r_T1_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xEA910F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_teq_r_t1_a_flags_zeroresult_1_ea910f02() {
    // Test aarch32_TEQ_r_T1_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xEA910F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_teq_r_t1_a_flags_negativeresult_2_ea910f02() {
    // Test aarch32_TEQ_r_T1_A flag computation: NegativeResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xEA910F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_teq_r_t1_a_flags_unsignedoverflow_3_ea910f02() {
    // Test aarch32_TEQ_r_T1_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xEA910F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_teq_r_t1_a_flags_unsignedoverflow_4_ea910f02() {
    // Test aarch32_TEQ_r_T1_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x2);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xEA910F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_teq_r_t1_a_flags_signedoverflow_5_ea910f02() {
    // Test aarch32_TEQ_r_T1_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xEA910F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_teq_r_t1_a_flags_signedoverflow_6_ea910f02() {
    // Test aarch32_TEQ_r_T1_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0xEA910F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_TEQ_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_teq_r_t1_a_flags_positiveresult_7_ea910f02() {
    // Test aarch32_TEQ_r_T1_A flag computation: PositiveResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x32);
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xEA910F02;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}
