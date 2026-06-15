//! A32 data_processing saturating tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers_32::*;

// ============================================================================
// aarch32_QDSUB_A Tests
// ============================================================================

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_qdsub_a1_a_field_cond_0_min_50_01600050() {
    // Encoding: 0x01600050
    // Test aarch32_QDSUB_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x01600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_qdsub_a1_a_field_cond_1_poweroftwo_50_11600050() {
    // Encoding: 0x11600050
    // Test aarch32_QDSUB_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=1
    let encoding: u32 = 0x11600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_qdsub_a1_a_field_cond_2_poweroftwo_50_21600050() {
    // Encoding: 0x21600050
    // Test aarch32_QDSUB_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=2, Rm=0, Rn=0
    let encoding: u32 = 0x21600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_qdsub_a1_a_field_cond_3_poweroftwo_50_31600050() {
    // Encoding: 0x31600050
    // Test aarch32_QDSUB_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x31600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_qdsub_a1_a_field_cond_4_poweroftwo_50_41600050() {
    // Encoding: 0x41600050
    // Test aarch32_QDSUB_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=4, Rm=0
    let encoding: u32 = 0x41600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_qdsub_a1_a_field_cond_5_poweroftwo_50_51600050() {
    // Encoding: 0x51600050
    // Test aarch32_QDSUB_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=5, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x51600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_qdsub_a1_a_field_cond_6_poweroftwo_50_61600050() {
    // Encoding: 0x61600050
    // Test aarch32_QDSUB_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x61600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_qdsub_a1_a_field_cond_7_poweroftwo_50_71600050() {
    // Encoding: 0x71600050
    // Test aarch32_QDSUB_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=7, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x71600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_qdsub_a1_a_field_cond_8_poweroftwo_50_81600050() {
    // Encoding: 0x81600050
    // Test aarch32_QDSUB_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=8
    let encoding: u32 = 0x81600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_qdsub_a1_a_field_cond_9_poweroftwo_50_91600050() {
    // Encoding: 0x91600050
    // Test aarch32_QDSUB_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=9
    let encoding: u32 = 0x91600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_qdsub_a1_a_field_cond_10_poweroftwo_50_a1600050() {
    // Encoding: 0xA1600050
    // Test aarch32_QDSUB_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=10, Rm=0
    let encoding: u32 = 0xA1600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_qdsub_a1_a_field_cond_11_poweroftwo_50_b1600050() {
    // Encoding: 0xB1600050
    // Test aarch32_QDSUB_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=11, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0xB1600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_qdsub_a1_a_field_cond_12_poweroftwo_50_c1600050() {
    // Encoding: 0xC1600050
    // Test aarch32_QDSUB_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=12
    let encoding: u32 = 0xC1600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_qdsub_a1_a_field_cond_13_poweroftwo_50_d1600050() {
    // Encoding: 0xD1600050
    // Test aarch32_QDSUB_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=13, Rd=0
    let encoding: u32 = 0xD1600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_qdsub_a1_a_field_cond_14_poweroftwo_50_e1600050() {
    // Encoding: 0xE1600050
    // Test aarch32_QDSUB_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=14
    let encoding: u32 = 0xE1600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_qdsub_a1_a_field_cond_15_max_50_f1600050() {
    // Encoding: 0xF1600050
    // Test aarch32_QDSUB_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=15, Rd=0
    let encoding: u32 = 0xF1600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qdsub_a1_a_field_rn_0_min_50_01600050() {
    // Encoding: 0x01600050
    // Test aarch32_QDSUB_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, cond=0, Rd=0, Rm=0
    let encoding: u32 = 0x01600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qdsub_a1_a_field_rn_1_poweroftwo_50_01610050() {
    // Encoding: 0x01610050
    // Test aarch32_QDSUB_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=1, cond=0
    let encoding: u32 = 0x01610050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qdsub_a1_a_field_rd_0_min_50_01600050() {
    // Encoding: 0x01600050
    // Test aarch32_QDSUB_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, cond=0, Rn=0, Rm=0
    let encoding: u32 = 0x01600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qdsub_a1_a_field_rd_1_poweroftwo_50_01601050() {
    // Encoding: 0x01601050
    // Test aarch32_QDSUB_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=1, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x01601050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qdsub_a1_a_field_rm_0_min_50_01600050() {
    // Encoding: 0x01600050
    // Test aarch32_QDSUB_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=0
    let encoding: u32 = 0x01600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qdsub_a1_a_field_rm_1_poweroftwo_50_01600051() {
    // Encoding: 0x01600051
    // Test aarch32_QDSUB_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=0, Rn=0, Rm=1
    let encoding: u32 = 0x01600051;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_qdsub_a1_a_combo_0_50_01600050() {
    // Encoding: 0x01600050
    // Test aarch32_QDSUB_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=0, Rd=0
    let encoding: u32 = 0x01600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_qdsub_a1_a_special_cond_0_condition_eq_80_01600050() {
    // Encoding: 0x01600050
    // Test aarch32_QDSUB_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x01600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_qdsub_a1_a_special_cond_1_condition_ne_80_11600050() {
    // Encoding: 0x11600050
    // Test aarch32_QDSUB_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=1, Rm=0
    let encoding: u32 = 0x11600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_qdsub_a1_a_special_cond_2_condition_cs_hs_80_21600050() {
    // Encoding: 0x21600050
    // Test aarch32_QDSUB_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=2
    let encoding: u32 = 0x21600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_qdsub_a1_a_special_cond_3_condition_cc_lo_80_31600050() {
    // Encoding: 0x31600050
    // Test aarch32_QDSUB_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x31600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_qdsub_a1_a_special_cond_4_condition_mi_80_41600050() {
    // Encoding: 0x41600050
    // Test aarch32_QDSUB_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x41600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_qdsub_a1_a_special_cond_5_condition_pl_80_51600050() {
    // Encoding: 0x51600050
    // Test aarch32_QDSUB_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x51600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_qdsub_a1_a_special_cond_6_condition_vs_80_61600050() {
    // Encoding: 0x61600050
    // Test aarch32_QDSUB_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=6
    let encoding: u32 = 0x61600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_qdsub_a1_a_special_cond_7_condition_vc_80_71600050() {
    // Encoding: 0x71600050
    // Test aarch32_QDSUB_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x71600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_qdsub_a1_a_special_cond_8_condition_hi_80_81600050() {
    // Encoding: 0x81600050
    // Test aarch32_QDSUB_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x81600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_qdsub_a1_a_special_cond_9_condition_ls_80_91600050() {
    // Encoding: 0x91600050
    // Test aarch32_QDSUB_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x91600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_qdsub_a1_a_special_cond_10_condition_ge_80_a1600050() {
    // Encoding: 0xA1600050
    // Test aarch32_QDSUB_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=10, Rd=0
    let encoding: u32 = 0xA1600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_qdsub_a1_a_special_cond_11_condition_lt_80_b1600050() {
    // Encoding: 0xB1600050
    // Test aarch32_QDSUB_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=11
    let encoding: u32 = 0xB1600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_qdsub_a1_a_special_cond_12_condition_gt_80_c1600050() {
    // Encoding: 0xC1600050
    // Test aarch32_QDSUB_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rd=0, cond=12, Rn=0, Rm=0
    let encoding: u32 = 0xC1600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_qdsub_a1_a_special_cond_13_condition_le_80_d1600050() {
    // Encoding: 0xD1600050
    // Test aarch32_QDSUB_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rm=0, cond=13, Rn=0, Rd=0
    let encoding: u32 = 0xD1600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_qdsub_a1_a_special_cond_14_condition_al_80_e1600050() {
    // Encoding: 0xE1600050
    // Test aarch32_QDSUB_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=14
    let encoding: u32 = 0xE1600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_qdsub_a1_a_special_cond_15_condition_nv_80_f1600050() {
    // Encoding: 0xF1600050
    // Test aarch32_QDSUB_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=15, Rm=0
    let encoding: u32 = 0xF1600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qdsub_a1_a_invalid_0_50_01600050() {
    // Encoding: 0x01600050
    // Test aarch32_QDSUB_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rm=0, cond=0, Rd=0, Rn=0
    let encoding: u32 = 0x01600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_QDSUB_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qdsub_a1_a_invalid_1_50_01600050() {
    // Encoding: 0x01600050
    // Test aarch32_QDSUB_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: cond=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x01600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_QDSUB_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qdsub_t1_a_field_rn_0_min_f0b0_fa80f0b0() {
    // Thumb encoding (32): 0xFA80F0B0
    // Test aarch32_QDSUB_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F0B0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QDSUB_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qdsub_t1_a_field_rn_1_poweroftwo_f0b0_fa81f0b0() {
    // Thumb encoding (32): 0xFA81F0B0
    // Test aarch32_QDSUB_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA81F0B0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QDSUB_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qdsub_t1_a_field_rd_0_min_f0b0_fa80f0b0() {
    // Thumb encoding (32): 0xFA80F0B0
    // Test aarch32_QDSUB_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F0B0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QDSUB_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qdsub_t1_a_field_rd_1_poweroftwo_f0b0_fa80f1b0() {
    // Thumb encoding (32): 0xFA80F1B0
    // Test aarch32_QDSUB_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rd=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F1B0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QDSUB_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qdsub_t1_a_field_rm_0_min_f0b0_fa80f0b0() {
    // Thumb encoding (32): 0xFA80F0B0
    // Test aarch32_QDSUB_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F0B0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QDSUB_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qdsub_t1_a_field_rm_1_poweroftwo_f0b0_fa80f0b1() {
    // Thumb encoding (32): 0xFA80F0B1
    // Test aarch32_QDSUB_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F0B1;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QDSUB_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_qdsub_t1_a_combo_0_f0b0_fa80f0b0() {
    // Thumb encoding (32): 0xFA80F0B0
    // Test aarch32_QDSUB_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F0B0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QDSUB_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qdsub_t1_a_invalid_0_f0b0_fa80f0b0() {
    // Thumb encoding (32): 0xFA80F0B0
    // Test aarch32_QDSUB_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F0B0;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_QDSUB_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qdsub_t1_a_invalid_1_f0b0_fa80f0b0() {
    // Thumb encoding (32): 0xFA80F0B0
    // Test aarch32_QDSUB_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F0B0;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_UQSUB8_A Tests
// ============================================================================

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_uqsub8_a1_a_field_cond_0_min_f0_066000f0() {
    // Encoding: 0x066000F0
    // Test aarch32_UQSUB8_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, cond=0, Rd=0, Rm=0
    let encoding: u32 = 0x066000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_uqsub8_a1_a_field_cond_1_poweroftwo_f0_166000f0() {
    // Encoding: 0x166000F0
    // Test aarch32_UQSUB8_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=1, Rn=0, Rm=0
    let encoding: u32 = 0x166000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_uqsub8_a1_a_field_cond_2_poweroftwo_f0_266000f0() {
    // Encoding: 0x266000F0
    // Test aarch32_UQSUB8_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=2, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x266000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_uqsub8_a1_a_field_cond_3_poweroftwo_f0_366000f0() {
    // Encoding: 0x366000F0
    // Test aarch32_UQSUB8_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x366000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_uqsub8_a1_a_field_cond_4_poweroftwo_f0_466000f0() {
    // Encoding: 0x466000F0
    // Test aarch32_UQSUB8_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=4, Rm=0
    let encoding: u32 = 0x466000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_uqsub8_a1_a_field_cond_5_poweroftwo_f0_566000f0() {
    // Encoding: 0x566000F0
    // Test aarch32_UQSUB8_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=5, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x566000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_uqsub8_a1_a_field_cond_6_poweroftwo_f0_666000f0() {
    // Encoding: 0x666000F0
    // Test aarch32_UQSUB8_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=6, Rm=0
    let encoding: u32 = 0x666000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_uqsub8_a1_a_field_cond_7_poweroftwo_f0_766000f0() {
    // Encoding: 0x766000F0
    // Test aarch32_UQSUB8_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=7
    let encoding: u32 = 0x766000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_uqsub8_a1_a_field_cond_8_poweroftwo_f0_866000f0() {
    // Encoding: 0x866000F0
    // Test aarch32_UQSUB8_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=8, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x866000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_uqsub8_a1_a_field_cond_9_poweroftwo_f0_966000f0() {
    // Encoding: 0x966000F0
    // Test aarch32_UQSUB8_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=9, Rn=0, Rm=0
    let encoding: u32 = 0x966000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_uqsub8_a1_a_field_cond_10_poweroftwo_f0_a66000f0() {
    // Encoding: 0xA66000F0
    // Test aarch32_UQSUB8_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0xA66000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_uqsub8_a1_a_field_cond_11_poweroftwo_f0_b66000f0() {
    // Encoding: 0xB66000F0
    // Test aarch32_UQSUB8_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=11, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0xB66000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_uqsub8_a1_a_field_cond_12_poweroftwo_f0_c66000f0() {
    // Encoding: 0xC66000F0
    // Test aarch32_UQSUB8_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=12, Rd=0, Rn=0
    let encoding: u32 = 0xC66000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_uqsub8_a1_a_field_cond_13_poweroftwo_f0_d66000f0() {
    // Encoding: 0xD66000F0
    // Test aarch32_UQSUB8_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=13, Rm=0
    let encoding: u32 = 0xD66000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_uqsub8_a1_a_field_cond_14_poweroftwo_f0_e66000f0() {
    // Encoding: 0xE66000F0
    // Test aarch32_UQSUB8_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=14
    let encoding: u32 = 0xE66000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_uqsub8_a1_a_field_cond_15_max_f0_f66000f0() {
    // Encoding: 0xF66000F0
    // Test aarch32_UQSUB8_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=15, Rn=0
    let encoding: u32 = 0xF66000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqsub8_a1_a_field_rn_0_min_f0_066000f0() {
    // Encoding: 0x066000F0
    // Test aarch32_UQSUB8_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=0, Rd=0
    let encoding: u32 = 0x066000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqsub8_a1_a_field_rn_1_poweroftwo_f0_066100f0() {
    // Encoding: 0x066100F0
    // Test aarch32_UQSUB8_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=0, Rn=1
    let encoding: u32 = 0x066100F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqsub8_a1_a_field_rd_0_min_f0_066000f0() {
    // Encoding: 0x066000F0
    // Test aarch32_UQSUB8_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x066000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqsub8_a1_a_field_rd_1_poweroftwo_f0_066010f0() {
    // Encoding: 0x066010F0
    // Test aarch32_UQSUB8_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rm=0, Rd=1
    let encoding: u32 = 0x066010F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqsub8_a1_a_field_rm_0_min_f0_066000f0() {
    // Encoding: 0x066000F0
    // Test aarch32_UQSUB8_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=0
    let encoding: u32 = 0x066000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqsub8_a1_a_field_rm_1_poweroftwo_f0_066000f1() {
    // Encoding: 0x066000F1
    // Test aarch32_UQSUB8_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=1, cond=0
    let encoding: u32 = 0x066000F1;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_uqsub8_a1_a_combo_0_f0_066000f0() {
    // Encoding: 0x066000F0
    // Test aarch32_UQSUB8_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=0
    let encoding: u32 = 0x066000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_uqsub8_a1_a_special_cond_0_condition_eq_240_066000f0() {
    // Encoding: 0x066000F0
    // Test aarch32_UQSUB8_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rn=0, cond=0, Rd=0, Rm=0
    let encoding: u32 = 0x066000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_uqsub8_a1_a_special_cond_1_condition_ne_240_166000f0() {
    // Encoding: 0x166000F0
    // Test aarch32_UQSUB8_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=1, Rm=0
    let encoding: u32 = 0x166000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_uqsub8_a1_a_special_cond_2_condition_cs_hs_240_266000f0() {
    // Encoding: 0x266000F0
    // Test aarch32_UQSUB8_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x266000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_uqsub8_a1_a_special_cond_3_condition_cc_lo_240_366000f0() {
    // Encoding: 0x366000F0
    // Test aarch32_UQSUB8_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=3, Rm=0
    let encoding: u32 = 0x366000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_uqsub8_a1_a_special_cond_4_condition_mi_240_466000f0() {
    // Encoding: 0x466000F0
    // Test aarch32_UQSUB8_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rd=0, cond=4, Rn=0, Rm=0
    let encoding: u32 = 0x466000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_uqsub8_a1_a_special_cond_5_condition_pl_240_566000f0() {
    // Encoding: 0x566000F0
    // Test aarch32_UQSUB8_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=5
    let encoding: u32 = 0x566000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_uqsub8_a1_a_special_cond_6_condition_vs_240_666000f0() {
    // Encoding: 0x666000F0
    // Test aarch32_UQSUB8_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=6, Rm=0
    let encoding: u32 = 0x666000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_uqsub8_a1_a_special_cond_7_condition_vc_240_766000f0() {
    // Encoding: 0x766000F0
    // Test aarch32_UQSUB8_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=7
    let encoding: u32 = 0x766000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_uqsub8_a1_a_special_cond_8_condition_hi_240_866000f0() {
    // Encoding: 0x866000F0
    // Test aarch32_UQSUB8_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=8, Rn=0
    let encoding: u32 = 0x866000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_uqsub8_a1_a_special_cond_9_condition_ls_240_966000f0() {
    // Encoding: 0x966000F0
    // Test aarch32_UQSUB8_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x966000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_uqsub8_a1_a_special_cond_10_condition_ge_240_a66000f0() {
    // Encoding: 0xA66000F0
    // Test aarch32_UQSUB8_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rm=0, cond=10, Rn=0, Rd=0
    let encoding: u32 = 0xA66000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_uqsub8_a1_a_special_cond_11_condition_lt_240_b66000f0() {
    // Encoding: 0xB66000F0
    // Test aarch32_UQSUB8_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0xB66000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_uqsub8_a1_a_special_cond_12_condition_gt_240_c66000f0() {
    // Encoding: 0xC66000F0
    // Test aarch32_UQSUB8_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=12
    let encoding: u32 = 0xC66000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_uqsub8_a1_a_special_cond_13_condition_le_240_d66000f0() {
    // Encoding: 0xD66000F0
    // Test aarch32_UQSUB8_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rm=0, cond=13, Rn=0, Rd=0
    let encoding: u32 = 0xD66000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_uqsub8_a1_a_special_cond_14_condition_al_240_e66000f0() {
    // Encoding: 0xE66000F0
    // Test aarch32_UQSUB8_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rn=0, cond=14, Rd=0, Rm=0
    let encoding: u32 = 0xE66000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_uqsub8_a1_a_special_cond_15_condition_nv_240_f66000f0() {
    // Encoding: 0xF66000F0
    // Test aarch32_UQSUB8_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rn=0, cond=15, Rd=0, Rm=0
    let encoding: u32 = 0xF66000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uqsub8_a1_a_invalid_0_f0_066000f0() {
    // Encoding: 0x066000F0
    // Test aarch32_UQSUB8_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rd=0, cond=0, Rn=0, Rm=0
    let encoding: u32 = 0x066000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UQSUB8_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uqsub8_a1_a_invalid_1_f0_066000f0() {
    // Encoding: 0x066000F0
    // Test aarch32_UQSUB8_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=0
    let encoding: u32 = 0x066000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UQSUB8_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqsub8_t1_a_field_rn_0_min_f050_fac0f050() {
    // Thumb encoding (32): 0xFAC0F050
    // Test aarch32_UQSUB8_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQSUB8_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqsub8_t1_a_field_rn_1_poweroftwo_f050_fac1f050() {
    // Thumb encoding (32): 0xFAC1F050
    // Test aarch32_UQSUB8_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC1F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQSUB8_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqsub8_t1_a_field_rd_0_min_f050_fac0f050() {
    // Thumb encoding (32): 0xFAC0F050
    // Test aarch32_UQSUB8_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQSUB8_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqsub8_t1_a_field_rd_1_poweroftwo_f050_fac0f150() {
    // Thumb encoding (32): 0xFAC0F150
    // Test aarch32_UQSUB8_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rd=1, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F150;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQSUB8_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqsub8_t1_a_field_rm_0_min_f050_fac0f050() {
    // Thumb encoding (32): 0xFAC0F050
    // Test aarch32_UQSUB8_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQSUB8_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqsub8_t1_a_field_rm_1_poweroftwo_f050_fac0f051() {
    // Thumb encoding (32): 0xFAC0F051
    // Test aarch32_UQSUB8_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F051;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQSUB8_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_uqsub8_t1_a_combo_0_f050_fac0f050() {
    // Thumb encoding (32): 0xFAC0F050
    // Test aarch32_UQSUB8_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQSUB8_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uqsub8_t1_a_invalid_0_f050_fac0f050() {
    // Thumb encoding (32): 0xFAC0F050
    // Test aarch32_UQSUB8_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F050;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_UQSUB8_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uqsub8_t1_a_invalid_1_f050_fac0f050() {
    // Thumb encoding (32): 0xFAC0F050
    // Test aarch32_UQSUB8_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F050;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_QDADD_A Tests
// ============================================================================

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_qdadd_a1_a_field_cond_0_min_50_01400050() {
    // Encoding: 0x01400050
    // Test aarch32_QDADD_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x01400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_qdadd_a1_a_field_cond_1_poweroftwo_50_11400050() {
    // Encoding: 0x11400050
    // Test aarch32_QDADD_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=1, Rd=0, Rm=0
    let encoding: u32 = 0x11400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_qdadd_a1_a_field_cond_2_poweroftwo_50_21400050() {
    // Encoding: 0x21400050
    // Test aarch32_QDADD_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=2, Rm=0
    let encoding: u32 = 0x21400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_qdadd_a1_a_field_cond_3_poweroftwo_50_31400050() {
    // Encoding: 0x31400050
    // Test aarch32_QDADD_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=3, Rd=0, Rm=0
    let encoding: u32 = 0x31400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_qdadd_a1_a_field_cond_4_poweroftwo_50_41400050() {
    // Encoding: 0x41400050
    // Test aarch32_QDADD_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=4
    let encoding: u32 = 0x41400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_qdadd_a1_a_field_cond_5_poweroftwo_50_51400050() {
    // Encoding: 0x51400050
    // Test aarch32_QDADD_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=5, Rm=0, Rn=0
    let encoding: u32 = 0x51400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_qdadd_a1_a_field_cond_6_poweroftwo_50_61400050() {
    // Encoding: 0x61400050
    // Test aarch32_QDADD_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x61400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_qdadd_a1_a_field_cond_7_poweroftwo_50_71400050() {
    // Encoding: 0x71400050
    // Test aarch32_QDADD_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=7, Rd=0
    let encoding: u32 = 0x71400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_qdadd_a1_a_field_cond_8_poweroftwo_50_81400050() {
    // Encoding: 0x81400050
    // Test aarch32_QDADD_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=8
    let encoding: u32 = 0x81400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_qdadd_a1_a_field_cond_9_poweroftwo_50_91400050() {
    // Encoding: 0x91400050
    // Test aarch32_QDADD_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=9, Rn=0, Rm=0
    let encoding: u32 = 0x91400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_qdadd_a1_a_field_cond_10_poweroftwo_50_a1400050() {
    // Encoding: 0xA1400050
    // Test aarch32_QDADD_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xA1400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_qdadd_a1_a_field_cond_11_poweroftwo_50_b1400050() {
    // Encoding: 0xB1400050
    // Test aarch32_QDADD_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=11, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0xB1400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_qdadd_a1_a_field_cond_12_poweroftwo_50_c1400050() {
    // Encoding: 0xC1400050
    // Test aarch32_QDADD_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=12, Rd=0
    let encoding: u32 = 0xC1400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_qdadd_a1_a_field_cond_13_poweroftwo_50_d1400050() {
    // Encoding: 0xD1400050
    // Test aarch32_QDADD_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=13, Rn=0, Rd=0
    let encoding: u32 = 0xD1400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_qdadd_a1_a_field_cond_14_poweroftwo_50_e1400050() {
    // Encoding: 0xE1400050
    // Test aarch32_QDADD_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0xE1400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_qdadd_a1_a_field_cond_15_max_50_f1400050() {
    // Encoding: 0xF1400050
    // Test aarch32_QDADD_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=15, Rd=0
    let encoding: u32 = 0xF1400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qdadd_a1_a_field_rn_0_min_50_01400050() {
    // Encoding: 0x01400050
    // Test aarch32_QDADD_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x01400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qdadd_a1_a_field_rn_1_poweroftwo_50_01410050() {
    // Encoding: 0x01410050
    // Test aarch32_QDADD_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rd=0, Rn=1
    let encoding: u32 = 0x01410050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qdadd_a1_a_field_rd_0_min_50_01400050() {
    // Encoding: 0x01400050
    // Test aarch32_QDADD_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x01400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qdadd_a1_a_field_rd_1_poweroftwo_50_01401050() {
    // Encoding: 0x01401050
    // Test aarch32_QDADD_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rd=1, Rn=0
    let encoding: u32 = 0x01401050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qdadd_a1_a_field_rm_0_min_50_01400050() {
    // Encoding: 0x01400050
    // Test aarch32_QDADD_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=0, Rm=0
    let encoding: u32 = 0x01400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qdadd_a1_a_field_rm_1_poweroftwo_50_01400051() {
    // Encoding: 0x01400051
    // Test aarch32_QDADD_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=0, Rn=0, Rm=1
    let encoding: u32 = 0x01400051;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_qdadd_a1_a_combo_0_50_01400050() {
    // Encoding: 0x01400050
    // Test aarch32_QDADD_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rd=0, cond=0, Rn=0, Rm=0
    let encoding: u32 = 0x01400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_qdadd_a1_a_special_cond_0_condition_eq_80_01400050() {
    // Encoding: 0x01400050
    // Test aarch32_QDADD_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x01400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_qdadd_a1_a_special_cond_1_condition_ne_80_11400050() {
    // Encoding: 0x11400050
    // Test aarch32_QDADD_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=1
    let encoding: u32 = 0x11400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_qdadd_a1_a_special_cond_2_condition_cs_hs_80_21400050() {
    // Encoding: 0x21400050
    // Test aarch32_QDADD_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=2, Rd=0
    let encoding: u32 = 0x21400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_qdadd_a1_a_special_cond_3_condition_cc_lo_80_31400050() {
    // Encoding: 0x31400050
    // Test aarch32_QDADD_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x31400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_qdadd_a1_a_special_cond_4_condition_mi_80_41400050() {
    // Encoding: 0x41400050
    // Test aarch32_QDADD_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=4
    let encoding: u32 = 0x41400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_qdadd_a1_a_special_cond_5_condition_pl_80_51400050() {
    // Encoding: 0x51400050
    // Test aarch32_QDADD_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rm=0, cond=5, Rd=0, Rn=0
    let encoding: u32 = 0x51400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_qdadd_a1_a_special_cond_6_condition_vs_80_61400050() {
    // Encoding: 0x61400050
    // Test aarch32_QDADD_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=6
    let encoding: u32 = 0x61400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_qdadd_a1_a_special_cond_7_condition_vc_80_71400050() {
    // Encoding: 0x71400050
    // Test aarch32_QDADD_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=7
    let encoding: u32 = 0x71400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_qdadd_a1_a_special_cond_8_condition_hi_80_81400050() {
    // Encoding: 0x81400050
    // Test aarch32_QDADD_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=8, Rm=0
    let encoding: u32 = 0x81400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_qdadd_a1_a_special_cond_9_condition_ls_80_91400050() {
    // Encoding: 0x91400050
    // Test aarch32_QDADD_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rn=0, cond=9, Rd=0, Rm=0
    let encoding: u32 = 0x91400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_qdadd_a1_a_special_cond_10_condition_ge_80_a1400050() {
    // Encoding: 0xA1400050
    // Test aarch32_QDADD_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0xA1400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_qdadd_a1_a_special_cond_11_condition_lt_80_b1400050() {
    // Encoding: 0xB1400050
    // Test aarch32_QDADD_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0xB1400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_qdadd_a1_a_special_cond_12_condition_gt_80_c1400050() {
    // Encoding: 0xC1400050
    // Test aarch32_QDADD_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=12, Rm=0
    let encoding: u32 = 0xC1400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_qdadd_a1_a_special_cond_13_condition_le_80_d1400050() {
    // Encoding: 0xD1400050
    // Test aarch32_QDADD_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=13, Rm=0
    let encoding: u32 = 0xD1400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_qdadd_a1_a_special_cond_14_condition_al_80_e1400050() {
    // Encoding: 0xE1400050
    // Test aarch32_QDADD_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=14
    let encoding: u32 = 0xE1400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_qdadd_a1_a_special_cond_15_condition_nv_80_f1400050() {
    // Encoding: 0xF1400050
    // Test aarch32_QDADD_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0xF1400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qdadd_a1_a_invalid_0_50_01400050() {
    // Encoding: 0x01400050
    // Test aarch32_QDADD_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=0
    let encoding: u32 = 0x01400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_QDADD_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qdadd_a1_a_invalid_1_50_01400050() {
    // Encoding: 0x01400050
    // Test aarch32_QDADD_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x01400050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_QDADD_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qdadd_t1_a_field_rn_0_min_f090_fa80f090() {
    // Thumb encoding (32): 0xFA80F090
    // Test aarch32_QDADD_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F090;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QDADD_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qdadd_t1_a_field_rn_1_poweroftwo_f090_fa81f090() {
    // Thumb encoding (32): 0xFA81F090
    // Test aarch32_QDADD_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA81F090;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QDADD_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qdadd_t1_a_field_rd_0_min_f090_fa80f090() {
    // Thumb encoding (32): 0xFA80F090
    // Test aarch32_QDADD_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F090;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QDADD_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qdadd_t1_a_field_rd_1_poweroftwo_f090_fa80f190() {
    // Thumb encoding (32): 0xFA80F190
    // Test aarch32_QDADD_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rd=1, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F190;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QDADD_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qdadd_t1_a_field_rm_0_min_f090_fa80f090() {
    // Thumb encoding (32): 0xFA80F090
    // Test aarch32_QDADD_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F090;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QDADD_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qdadd_t1_a_field_rm_1_poweroftwo_f090_fa80f091() {
    // Thumb encoding (32): 0xFA80F091
    // Test aarch32_QDADD_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rm=1, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F091;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QDADD_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_qdadd_t1_a_combo_0_f090_fa80f090() {
    // Thumb encoding (32): 0xFA80F090
    // Test aarch32_QDADD_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F090;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QDADD_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qdadd_t1_a_invalid_0_f090_fa80f090() {
    // Thumb encoding (32): 0xFA80F090
    // Test aarch32_QDADD_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F090;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_QDADD_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qdadd_t1_a_invalid_1_f090_fa80f090() {
    // Thumb encoding (32): 0xFA80F090
    // Test aarch32_QDADD_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F090;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_QADD16_A Tests
// ============================================================================

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_qadd16_a1_a_field_cond_0_min_10_06200010() {
    // Encoding: 0x06200010
    // Test aarch32_QADD16_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x06200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_qadd16_a1_a_field_cond_1_poweroftwo_10_16200010() {
    // Encoding: 0x16200010
    // Test aarch32_QADD16_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=1, Rn=0, Rm=0
    let encoding: u32 = 0x16200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_qadd16_a1_a_field_cond_2_poweroftwo_10_26200010() {
    // Encoding: 0x26200010
    // Test aarch32_QADD16_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=2
    let encoding: u32 = 0x26200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_qadd16_a1_a_field_cond_3_poweroftwo_10_36200010() {
    // Encoding: 0x36200010
    // Test aarch32_QADD16_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x36200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_qadd16_a1_a_field_cond_4_poweroftwo_10_46200010() {
    // Encoding: 0x46200010
    // Test aarch32_QADD16_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=4, Rn=0, Rd=0
    let encoding: u32 = 0x46200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_qadd16_a1_a_field_cond_5_poweroftwo_10_56200010() {
    // Encoding: 0x56200010
    // Test aarch32_QADD16_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=5, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x56200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_qadd16_a1_a_field_cond_6_poweroftwo_10_66200010() {
    // Encoding: 0x66200010
    // Test aarch32_QADD16_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=6
    let encoding: u32 = 0x66200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_qadd16_a1_a_field_cond_7_poweroftwo_10_76200010() {
    // Encoding: 0x76200010
    // Test aarch32_QADD16_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=7, Rn=0
    let encoding: u32 = 0x76200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_qadd16_a1_a_field_cond_8_poweroftwo_10_86200010() {
    // Encoding: 0x86200010
    // Test aarch32_QADD16_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=8, Rm=0, Rn=0
    let encoding: u32 = 0x86200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_qadd16_a1_a_field_cond_9_poweroftwo_10_96200010() {
    // Encoding: 0x96200010
    // Test aarch32_QADD16_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=9, Rm=0, Rd=0
    let encoding: u32 = 0x96200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_qadd16_a1_a_field_cond_10_poweroftwo_10_a6200010() {
    // Encoding: 0xA6200010
    // Test aarch32_QADD16_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=10
    let encoding: u32 = 0xA6200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_qadd16_a1_a_field_cond_11_poweroftwo_10_b6200010() {
    // Encoding: 0xB6200010
    // Test aarch32_QADD16_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=11, Rn=0
    let encoding: u32 = 0xB6200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_qadd16_a1_a_field_cond_12_poweroftwo_10_c6200010() {
    // Encoding: 0xC6200010
    // Test aarch32_QADD16_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=12, Rm=0, Rn=0
    let encoding: u32 = 0xC6200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_qadd16_a1_a_field_cond_13_poweroftwo_10_d6200010() {
    // Encoding: 0xD6200010
    // Test aarch32_QADD16_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=13
    let encoding: u32 = 0xD6200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_qadd16_a1_a_field_cond_14_poweroftwo_10_e6200010() {
    // Encoding: 0xE6200010
    // Test aarch32_QADD16_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=14, Rm=0, Rn=0
    let encoding: u32 = 0xE6200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_qadd16_a1_a_field_cond_15_max_10_f6200010() {
    // Encoding: 0xF6200010
    // Test aarch32_QADD16_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=15, Rm=0
    let encoding: u32 = 0xF6200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qadd16_a1_a_field_rn_0_min_10_06200010() {
    // Encoding: 0x06200010
    // Test aarch32_QADD16_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=0
    let encoding: u32 = 0x06200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qadd16_a1_a_field_rn_1_poweroftwo_10_06210010() {
    // Encoding: 0x06210010
    // Test aarch32_QADD16_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=1, Rm=0, Rd=0
    let encoding: u32 = 0x06210010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qadd16_a1_a_field_rd_0_min_10_06200010() {
    // Encoding: 0x06200010
    // Test aarch32_QADD16_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rd=0, Rn=0
    let encoding: u32 = 0x06200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qadd16_a1_a_field_rd_1_poweroftwo_10_06201010() {
    // Encoding: 0x06201010
    // Test aarch32_QADD16_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=1, cond=0, Rn=0
    let encoding: u32 = 0x06201010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qadd16_a1_a_field_rm_0_min_10_06200010() {
    // Encoding: 0x06200010
    // Test aarch32_QADD16_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x06200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qadd16_a1_a_field_rm_1_poweroftwo_10_06200011() {
    // Encoding: 0x06200011
    // Test aarch32_QADD16_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, Rm=1
    let encoding: u32 = 0x06200011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_qadd16_a1_a_combo_0_10_06200010() {
    // Encoding: 0x06200010
    // Test aarch32_QADD16_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=0, Rd=0
    let encoding: u32 = 0x06200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_qadd16_a1_a_special_cond_0_condition_eq_16_06200010() {
    // Encoding: 0x06200010
    // Test aarch32_QADD16_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=0, Rd=0
    let encoding: u32 = 0x06200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_qadd16_a1_a_special_cond_1_condition_ne_16_16200010() {
    // Encoding: 0x16200010
    // Test aarch32_QADD16_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rd=0, cond=1, Rm=0, Rn=0
    let encoding: u32 = 0x16200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_qadd16_a1_a_special_cond_2_condition_cs_hs_16_26200010() {
    // Encoding: 0x26200010
    // Test aarch32_QADD16_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=2, Rd=0
    let encoding: u32 = 0x26200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_qadd16_a1_a_special_cond_3_condition_cc_lo_16_36200010() {
    // Encoding: 0x36200010
    // Test aarch32_QADD16_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=3, Rd=0
    let encoding: u32 = 0x36200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_qadd16_a1_a_special_cond_4_condition_mi_16_46200010() {
    // Encoding: 0x46200010
    // Test aarch32_QADD16_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=4, Rm=0
    let encoding: u32 = 0x46200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_qadd16_a1_a_special_cond_5_condition_pl_16_56200010() {
    // Encoding: 0x56200010
    // Test aarch32_QADD16_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=5, Rm=0
    let encoding: u32 = 0x56200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_qadd16_a1_a_special_cond_6_condition_vs_16_66200010() {
    // Encoding: 0x66200010
    // Test aarch32_QADD16_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=6
    let encoding: u32 = 0x66200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_qadd16_a1_a_special_cond_7_condition_vc_16_76200010() {
    // Encoding: 0x76200010
    // Test aarch32_QADD16_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rd=0, cond=7, Rm=0, Rn=0
    let encoding: u32 = 0x76200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_qadd16_a1_a_special_cond_8_condition_hi_16_86200010() {
    // Encoding: 0x86200010
    // Test aarch32_QADD16_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=8
    let encoding: u32 = 0x86200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_qadd16_a1_a_special_cond_9_condition_ls_16_96200010() {
    // Encoding: 0x96200010
    // Test aarch32_QADD16_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=9, Rm=0
    let encoding: u32 = 0x96200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_qadd16_a1_a_special_cond_10_condition_ge_16_a6200010() {
    // Encoding: 0xA6200010
    // Test aarch32_QADD16_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rd=0, cond=10, Rm=0, Rn=0
    let encoding: u32 = 0xA6200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_qadd16_a1_a_special_cond_11_condition_lt_16_b6200010() {
    // Encoding: 0xB6200010
    // Test aarch32_QADD16_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=11, Rm=0
    let encoding: u32 = 0xB6200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_qadd16_a1_a_special_cond_12_condition_gt_16_c6200010() {
    // Encoding: 0xC6200010
    // Test aarch32_QADD16_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rd=0, cond=12, Rn=0, Rm=0
    let encoding: u32 = 0xC6200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_qadd16_a1_a_special_cond_13_condition_le_16_d6200010() {
    // Encoding: 0xD6200010
    // Test aarch32_QADD16_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=13, Rd=0
    let encoding: u32 = 0xD6200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_qadd16_a1_a_special_cond_14_condition_al_16_e6200010() {
    // Encoding: 0xE6200010
    // Test aarch32_QADD16_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=14
    let encoding: u32 = 0xE6200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_qadd16_a1_a_special_cond_15_condition_nv_16_f6200010() {
    // Encoding: 0xF6200010
    // Test aarch32_QADD16_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0xF6200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qadd16_a1_a_invalid_0_10_06200010() {
    // Encoding: 0x06200010
    // Test aarch32_QADD16_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=0, Rn=0
    let encoding: u32 = 0x06200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_QADD16_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qadd16_a1_a_invalid_1_10_06200010() {
    // Encoding: 0x06200010
    // Test aarch32_QADD16_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=0, Rd=0
    let encoding: u32 = 0x06200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_QADD16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qadd16_t1_a_field_rn_0_min_f010_fa90f010() {
    // Thumb encoding (32): 0xFA90F010
    // Test aarch32_QADD16_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QADD16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qadd16_t1_a_field_rn_1_poweroftwo_f010_fa91f010() {
    // Thumb encoding (32): 0xFA91F010
    // Test aarch32_QADD16_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rn=1, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA91F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QADD16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qadd16_t1_a_field_rd_0_min_f010_fa90f010() {
    // Thumb encoding (32): 0xFA90F010
    // Test aarch32_QADD16_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QADD16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qadd16_t1_a_field_rd_1_poweroftwo_f010_fa90f110() {
    // Thumb encoding (32): 0xFA90F110
    // Test aarch32_QADD16_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F110;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QADD16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qadd16_t1_a_field_rm_0_min_f010_fa90f010() {
    // Thumb encoding (32): 0xFA90F010
    // Test aarch32_QADD16_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QADD16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qadd16_t1_a_field_rm_1_poweroftwo_f010_fa90f011() {
    // Thumb encoding (32): 0xFA90F011
    // Test aarch32_QADD16_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F011;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QADD16_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_qadd16_t1_a_combo_0_f010_fa90f010() {
    // Thumb encoding (32): 0xFA90F010
    // Test aarch32_QADD16_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QADD16_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qadd16_t1_a_invalid_0_f010_fa90f010() {
    // Thumb encoding (32): 0xFA90F010
    // Test aarch32_QADD16_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F010;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_QADD16_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qadd16_t1_a_invalid_1_f010_fa90f010() {
    // Thumb encoding (32): 0xFA90F010
    // Test aarch32_QADD16_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F010;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_QSAX_A Tests
// ============================================================================

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_qsax_a1_a_field_cond_0_min_50_06200050() {
    // Encoding: 0x06200050
    // Test aarch32_QSAX_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=0
    let encoding: u32 = 0x06200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_qsax_a1_a_field_cond_1_poweroftwo_50_16200050() {
    // Encoding: 0x16200050
    // Test aarch32_QSAX_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x16200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_qsax_a1_a_field_cond_2_poweroftwo_50_26200050() {
    // Encoding: 0x26200050
    // Test aarch32_QSAX_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=2, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x26200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_qsax_a1_a_field_cond_3_poweroftwo_50_36200050() {
    // Encoding: 0x36200050
    // Test aarch32_QSAX_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x36200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_qsax_a1_a_field_cond_4_poweroftwo_50_46200050() {
    // Encoding: 0x46200050
    // Test aarch32_QSAX_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=4, Rn=0, Rm=0
    let encoding: u32 = 0x46200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_qsax_a1_a_field_cond_5_poweroftwo_50_56200050() {
    // Encoding: 0x56200050
    // Test aarch32_QSAX_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=5, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x56200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_qsax_a1_a_field_cond_6_poweroftwo_50_66200050() {
    // Encoding: 0x66200050
    // Test aarch32_QSAX_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=6
    let encoding: u32 = 0x66200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_qsax_a1_a_field_cond_7_poweroftwo_50_76200050() {
    // Encoding: 0x76200050
    // Test aarch32_QSAX_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=7, Rn=0, Rd=0
    let encoding: u32 = 0x76200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_qsax_a1_a_field_cond_8_poweroftwo_50_86200050() {
    // Encoding: 0x86200050
    // Test aarch32_QSAX_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=8, Rm=0, Rd=0
    let encoding: u32 = 0x86200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_qsax_a1_a_field_cond_9_poweroftwo_50_96200050() {
    // Encoding: 0x96200050
    // Test aarch32_QSAX_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x96200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_qsax_a1_a_field_cond_10_poweroftwo_50_a6200050() {
    // Encoding: 0xA6200050
    // Test aarch32_QSAX_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=10, Rd=0, Rn=0
    let encoding: u32 = 0xA6200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_qsax_a1_a_field_cond_11_poweroftwo_50_b6200050() {
    // Encoding: 0xB6200050
    // Test aarch32_QSAX_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=11, Rm=0, Rn=0
    let encoding: u32 = 0xB6200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_qsax_a1_a_field_cond_12_poweroftwo_50_c6200050() {
    // Encoding: 0xC6200050
    // Test aarch32_QSAX_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=12, Rm=0
    let encoding: u32 = 0xC6200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_qsax_a1_a_field_cond_13_poweroftwo_50_d6200050() {
    // Encoding: 0xD6200050
    // Test aarch32_QSAX_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=13, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xD6200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_qsax_a1_a_field_cond_14_poweroftwo_50_e6200050() {
    // Encoding: 0xE6200050
    // Test aarch32_QSAX_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=14, Rd=0
    let encoding: u32 = 0xE6200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_qsax_a1_a_field_cond_15_max_50_f6200050() {
    // Encoding: 0xF6200050
    // Test aarch32_QSAX_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0xF6200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qsax_a1_a_field_rn_0_min_50_06200050() {
    // Encoding: 0x06200050
    // Test aarch32_QSAX_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=0
    let encoding: u32 = 0x06200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qsax_a1_a_field_rn_1_poweroftwo_50_06210050() {
    // Encoding: 0x06210050
    // Test aarch32_QSAX_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=1, Rd=0, Rm=0, cond=0
    let encoding: u32 = 0x06210050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qsax_a1_a_field_rd_0_min_50_06200050() {
    // Encoding: 0x06200050
    // Test aarch32_QSAX_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x06200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qsax_a1_a_field_rd_1_poweroftwo_50_06201050() {
    // Encoding: 0x06201050
    // Test aarch32_QSAX_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=0, Rm=0, Rd=1
    let encoding: u32 = 0x06201050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qsax_a1_a_field_rm_0_min_50_06200050() {
    // Encoding: 0x06200050
    // Test aarch32_QSAX_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x06200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qsax_a1_a_field_rm_1_poweroftwo_50_06200051() {
    // Encoding: 0x06200051
    // Test aarch32_QSAX_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=0, Rn=0, Rm=1
    let encoding: u32 = 0x06200051;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_qsax_a1_a_combo_0_50_06200050() {
    // Encoding: 0x06200050
    // Test aarch32_QSAX_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=0
    let encoding: u32 = 0x06200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_qsax_a1_a_special_cond_0_condition_eq_80_06200050() {
    // Encoding: 0x06200050
    // Test aarch32_QSAX_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rn=0, cond=0, Rm=0, Rd=0
    let encoding: u32 = 0x06200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_qsax_a1_a_special_cond_1_condition_ne_80_16200050() {
    // Encoding: 0x16200050
    // Test aarch32_QSAX_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=1, Rd=0
    let encoding: u32 = 0x16200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_qsax_a1_a_special_cond_2_condition_cs_hs_80_26200050() {
    // Encoding: 0x26200050
    // Test aarch32_QSAX_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x26200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_qsax_a1_a_special_cond_3_condition_cc_lo_80_36200050() {
    // Encoding: 0x36200050
    // Test aarch32_QSAX_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x36200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_qsax_a1_a_special_cond_4_condition_mi_80_46200050() {
    // Encoding: 0x46200050
    // Test aarch32_QSAX_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x46200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_qsax_a1_a_special_cond_5_condition_pl_80_56200050() {
    // Encoding: 0x56200050
    // Test aarch32_QSAX_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rd=0, cond=5, Rm=0, Rn=0
    let encoding: u32 = 0x56200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_qsax_a1_a_special_cond_6_condition_vs_80_66200050() {
    // Encoding: 0x66200050
    // Test aarch32_QSAX_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x66200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_qsax_a1_a_special_cond_7_condition_vc_80_76200050() {
    // Encoding: 0x76200050
    // Test aarch32_QSAX_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x76200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_qsax_a1_a_special_cond_8_condition_hi_80_86200050() {
    // Encoding: 0x86200050
    // Test aarch32_QSAX_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rn=0, cond=8, Rd=0, Rm=0
    let encoding: u32 = 0x86200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_qsax_a1_a_special_cond_9_condition_ls_80_96200050() {
    // Encoding: 0x96200050
    // Test aarch32_QSAX_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x96200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_qsax_a1_a_special_cond_10_condition_ge_80_a6200050() {
    // Encoding: 0xA6200050
    // Test aarch32_QSAX_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=10, Rm=0
    let encoding: u32 = 0xA6200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_qsax_a1_a_special_cond_11_condition_lt_80_b6200050() {
    // Encoding: 0xB6200050
    // Test aarch32_QSAX_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=11
    let encoding: u32 = 0xB6200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_qsax_a1_a_special_cond_12_condition_gt_80_c6200050() {
    // Encoding: 0xC6200050
    // Test aarch32_QSAX_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=12, Rn=0
    let encoding: u32 = 0xC6200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_qsax_a1_a_special_cond_13_condition_le_80_d6200050() {
    // Encoding: 0xD6200050
    // Test aarch32_QSAX_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rd=0, cond=13, Rn=0, Rm=0
    let encoding: u32 = 0xD6200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_qsax_a1_a_special_cond_14_condition_al_80_e6200050() {
    // Encoding: 0xE6200050
    // Test aarch32_QSAX_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=14
    let encoding: u32 = 0xE6200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_qsax_a1_a_special_cond_15_condition_nv_80_f6200050() {
    // Encoding: 0xF6200050
    // Test aarch32_QSAX_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=15, Rd=0
    let encoding: u32 = 0xF6200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qsax_a1_a_invalid_0_50_06200050() {
    // Encoding: 0x06200050
    // Test aarch32_QSAX_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x06200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_QSAX_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qsax_a1_a_invalid_1_50_06200050() {
    // Encoding: 0x06200050
    // Test aarch32_QSAX_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=0
    let encoding: u32 = 0x06200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_QSAX_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qsax_t1_a_field_rn_0_min_f010_fae0f010() {
    // Thumb encoding (32): 0xFAE0F010
    // Test aarch32_QSAX_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSAX_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qsax_t1_a_field_rn_1_poweroftwo_f010_fae1f010() {
    // Thumb encoding (32): 0xFAE1F010
    // Test aarch32_QSAX_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rn=1, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE1F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSAX_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qsax_t1_a_field_rd_0_min_f010_fae0f010() {
    // Thumb encoding (32): 0xFAE0F010
    // Test aarch32_QSAX_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSAX_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qsax_t1_a_field_rd_1_poweroftwo_f010_fae0f110() {
    // Thumb encoding (32): 0xFAE0F110
    // Test aarch32_QSAX_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rd=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F110;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSAX_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qsax_t1_a_field_rm_0_min_f010_fae0f010() {
    // Thumb encoding (32): 0xFAE0F010
    // Test aarch32_QSAX_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSAX_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qsax_t1_a_field_rm_1_poweroftwo_f010_fae0f011() {
    // Thumb encoding (32): 0xFAE0F011
    // Test aarch32_QSAX_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F011;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSAX_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_qsax_t1_a_combo_0_f010_fae0f010() {
    // Thumb encoding (32): 0xFAE0F010
    // Test aarch32_QSAX_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSAX_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qsax_t1_a_invalid_0_f010_fae0f010() {
    // Thumb encoding (32): 0xFAE0F010
    // Test aarch32_QSAX_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F010;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_QSAX_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qsax_t1_a_invalid_1_f010_fae0f010() {
    // Thumb encoding (32): 0xFAE0F010
    // Test aarch32_QSAX_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F010;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_QSUB16_A Tests
// ============================================================================

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_qsub16_a1_a_field_cond_0_min_70_06200070() {
    // Encoding: 0x06200070
    // Test aarch32_QSUB16_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=0, Rm=0
    let encoding: u32 = 0x06200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_qsub16_a1_a_field_cond_1_poweroftwo_70_16200070() {
    // Encoding: 0x16200070
    // Test aarch32_QSUB16_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=1
    let encoding: u32 = 0x16200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_qsub16_a1_a_field_cond_2_poweroftwo_70_26200070() {
    // Encoding: 0x26200070
    // Test aarch32_QSUB16_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=2, Rn=0, Rd=0
    let encoding: u32 = 0x26200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_qsub16_a1_a_field_cond_3_poweroftwo_70_36200070() {
    // Encoding: 0x36200070
    // Test aarch32_QSUB16_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=3, Rm=0
    let encoding: u32 = 0x36200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_qsub16_a1_a_field_cond_4_poweroftwo_70_46200070() {
    // Encoding: 0x46200070
    // Test aarch32_QSUB16_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=4, Rn=0
    let encoding: u32 = 0x46200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_qsub16_a1_a_field_cond_5_poweroftwo_70_56200070() {
    // Encoding: 0x56200070
    // Test aarch32_QSUB16_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=5, Rn=0, Rm=0
    let encoding: u32 = 0x56200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_qsub16_a1_a_field_cond_6_poweroftwo_70_66200070() {
    // Encoding: 0x66200070
    // Test aarch32_QSUB16_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=6, Rd=0, Rm=0
    let encoding: u32 = 0x66200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_qsub16_a1_a_field_cond_7_poweroftwo_70_76200070() {
    // Encoding: 0x76200070
    // Test aarch32_QSUB16_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=7, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x76200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_qsub16_a1_a_field_cond_8_poweroftwo_70_86200070() {
    // Encoding: 0x86200070
    // Test aarch32_QSUB16_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=8, Rn=0
    let encoding: u32 = 0x86200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_qsub16_a1_a_field_cond_9_poweroftwo_70_96200070() {
    // Encoding: 0x96200070
    // Test aarch32_QSUB16_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=9, Rn=0, Rm=0
    let encoding: u32 = 0x96200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_qsub16_a1_a_field_cond_10_poweroftwo_70_a6200070() {
    // Encoding: 0xA6200070
    // Test aarch32_QSUB16_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=10, Rd=0, Rm=0
    let encoding: u32 = 0xA6200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_qsub16_a1_a_field_cond_11_poweroftwo_70_b6200070() {
    // Encoding: 0xB6200070
    // Test aarch32_QSUB16_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=11, Rm=0, Rn=0
    let encoding: u32 = 0xB6200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_qsub16_a1_a_field_cond_12_poweroftwo_70_c6200070() {
    // Encoding: 0xC6200070
    // Test aarch32_QSUB16_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=12, Rd=0, Rn=0
    let encoding: u32 = 0xC6200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_qsub16_a1_a_field_cond_13_poweroftwo_70_d6200070() {
    // Encoding: 0xD6200070
    // Test aarch32_QSUB16_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=13, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xD6200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_qsub16_a1_a_field_cond_14_poweroftwo_70_e6200070() {
    // Encoding: 0xE6200070
    // Test aarch32_QSUB16_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0xE6200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_qsub16_a1_a_field_cond_15_max_70_f6200070() {
    // Encoding: 0xF6200070
    // Test aarch32_QSUB16_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0xF6200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qsub16_a1_a_field_rn_0_min_70_06200070() {
    // Encoding: 0x06200070
    // Test aarch32_QSUB16_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, cond=0, Rd=0, Rm=0
    let encoding: u32 = 0x06200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qsub16_a1_a_field_rn_1_poweroftwo_70_06210070() {
    // Encoding: 0x06210070
    // Test aarch32_QSUB16_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=1, cond=0, Rd=0
    let encoding: u32 = 0x06210070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qsub16_a1_a_field_rd_0_min_70_06200070() {
    // Encoding: 0x06200070
    // Test aarch32_QSUB16_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=0, Rd=0
    let encoding: u32 = 0x06200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qsub16_a1_a_field_rd_1_poweroftwo_70_06201070() {
    // Encoding: 0x06201070
    // Test aarch32_QSUB16_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rd=1, Rn=0
    let encoding: u32 = 0x06201070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qsub16_a1_a_field_rm_0_min_70_06200070() {
    // Encoding: 0x06200070
    // Test aarch32_QSUB16_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x06200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qsub16_a1_a_field_rm_1_poweroftwo_70_06200071() {
    // Encoding: 0x06200071
    // Test aarch32_QSUB16_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, Rm=1
    let encoding: u32 = 0x06200071;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_qsub16_a1_a_combo_0_70_06200070() {
    // Encoding: 0x06200070
    // Test aarch32_QSUB16_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x06200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_qsub16_a1_a_special_cond_0_condition_eq_112_06200070() {
    // Encoding: 0x06200070
    // Test aarch32_QSUB16_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x06200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_qsub16_a1_a_special_cond_1_condition_ne_112_16200070() {
    // Encoding: 0x16200070
    // Test aarch32_QSUB16_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x16200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_qsub16_a1_a_special_cond_2_condition_cs_hs_112_26200070() {
    // Encoding: 0x26200070
    // Test aarch32_QSUB16_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x26200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_qsub16_a1_a_special_cond_3_condition_cc_lo_112_36200070() {
    // Encoding: 0x36200070
    // Test aarch32_QSUB16_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x36200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_qsub16_a1_a_special_cond_4_condition_mi_112_46200070() {
    // Encoding: 0x46200070
    // Test aarch32_QSUB16_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rm=0, cond=4, Rn=0, Rd=0
    let encoding: u32 = 0x46200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_qsub16_a1_a_special_cond_5_condition_pl_112_56200070() {
    // Encoding: 0x56200070
    // Test aarch32_QSUB16_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x56200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_qsub16_a1_a_special_cond_6_condition_vs_112_66200070() {
    // Encoding: 0x66200070
    // Test aarch32_QSUB16_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=6
    let encoding: u32 = 0x66200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_qsub16_a1_a_special_cond_7_condition_vc_112_76200070() {
    // Encoding: 0x76200070
    // Test aarch32_QSUB16_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=7
    let encoding: u32 = 0x76200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_qsub16_a1_a_special_cond_8_condition_hi_112_86200070() {
    // Encoding: 0x86200070
    // Test aarch32_QSUB16_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x86200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_qsub16_a1_a_special_cond_9_condition_ls_112_96200070() {
    // Encoding: 0x96200070
    // Test aarch32_QSUB16_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=9
    let encoding: u32 = 0x96200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_qsub16_a1_a_special_cond_10_condition_ge_112_a6200070() {
    // Encoding: 0xA6200070
    // Test aarch32_QSUB16_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rd=0, cond=10, Rn=0, Rm=0
    let encoding: u32 = 0xA6200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_qsub16_a1_a_special_cond_11_condition_lt_112_b6200070() {
    // Encoding: 0xB6200070
    // Test aarch32_QSUB16_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xB6200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_qsub16_a1_a_special_cond_12_condition_gt_112_c6200070() {
    // Encoding: 0xC6200070
    // Test aarch32_QSUB16_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=12
    let encoding: u32 = 0xC6200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_qsub16_a1_a_special_cond_13_condition_le_112_d6200070() {
    // Encoding: 0xD6200070
    // Test aarch32_QSUB16_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rn=0, cond=13, Rm=0, Rd=0
    let encoding: u32 = 0xD6200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_qsub16_a1_a_special_cond_14_condition_al_112_e6200070() {
    // Encoding: 0xE6200070
    // Test aarch32_QSUB16_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=14, Rn=0
    let encoding: u32 = 0xE6200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_qsub16_a1_a_special_cond_15_condition_nv_112_f6200070() {
    // Encoding: 0xF6200070
    // Test aarch32_QSUB16_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=15
    let encoding: u32 = 0xF6200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qsub16_a1_a_invalid_0_70_06200070() {
    // Encoding: 0x06200070
    // Test aarch32_QSUB16_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x06200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_QSUB16_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qsub16_a1_a_invalid_1_70_06200070() {
    // Encoding: 0x06200070
    // Test aarch32_QSUB16_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x06200070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_QSUB16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qsub16_t1_a_field_rn_0_min_f010_fad0f010() {
    // Thumb encoding (32): 0xFAD0F010
    // Test aarch32_QSUB16_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSUB16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qsub16_t1_a_field_rn_1_poweroftwo_f010_fad1f010() {
    // Thumb encoding (32): 0xFAD1F010
    // Test aarch32_QSUB16_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rn=1, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD1F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSUB16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qsub16_t1_a_field_rd_0_min_f010_fad0f010() {
    // Thumb encoding (32): 0xFAD0F010
    // Test aarch32_QSUB16_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSUB16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qsub16_t1_a_field_rd_1_poweroftwo_f010_fad0f110() {
    // Thumb encoding (32): 0xFAD0F110
    // Test aarch32_QSUB16_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rd=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F110;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSUB16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qsub16_t1_a_field_rm_0_min_f010_fad0f010() {
    // Thumb encoding (32): 0xFAD0F010
    // Test aarch32_QSUB16_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSUB16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qsub16_t1_a_field_rm_1_poweroftwo_f010_fad0f011() {
    // Thumb encoding (32): 0xFAD0F011
    // Test aarch32_QSUB16_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F011;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSUB16_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_qsub16_t1_a_combo_0_f010_fad0f010() {
    // Thumb encoding (32): 0xFAD0F010
    // Test aarch32_QSUB16_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSUB16_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qsub16_t1_a_invalid_0_f010_fad0f010() {
    // Thumb encoding (32): 0xFAD0F010
    // Test aarch32_QSUB16_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F010;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_QSUB16_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qsub16_t1_a_invalid_1_f010_fad0f010() {
    // Thumb encoding (32): 0xFAD0F010
    // Test aarch32_QSUB16_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F010;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_UQASX_A Tests
// ============================================================================

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_uqasx_a1_a_field_cond_0_min_30_06600030() {
    // Encoding: 0x06600030
    // Test aarch32_UQASX_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=0, Rm=0
    let encoding: u32 = 0x06600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_uqasx_a1_a_field_cond_1_poweroftwo_30_16600030() {
    // Encoding: 0x16600030
    // Test aarch32_UQASX_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=1
    let encoding: u32 = 0x16600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_uqasx_a1_a_field_cond_2_poweroftwo_30_26600030() {
    // Encoding: 0x26600030
    // Test aarch32_UQASX_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=2
    let encoding: u32 = 0x26600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_uqasx_a1_a_field_cond_3_poweroftwo_30_36600030() {
    // Encoding: 0x36600030
    // Test aarch32_UQASX_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x36600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_uqasx_a1_a_field_cond_4_poweroftwo_30_46600030() {
    // Encoding: 0x46600030
    // Test aarch32_UQASX_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=4, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x46600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_uqasx_a1_a_field_cond_5_poweroftwo_30_56600030() {
    // Encoding: 0x56600030
    // Test aarch32_UQASX_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=5, Rn=0, Rm=0
    let encoding: u32 = 0x56600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_uqasx_a1_a_field_cond_6_poweroftwo_30_66600030() {
    // Encoding: 0x66600030
    // Test aarch32_UQASX_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=6
    let encoding: u32 = 0x66600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_uqasx_a1_a_field_cond_7_poweroftwo_30_76600030() {
    // Encoding: 0x76600030
    // Test aarch32_UQASX_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=7, Rn=0, Rd=0
    let encoding: u32 = 0x76600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_uqasx_a1_a_field_cond_8_poweroftwo_30_86600030() {
    // Encoding: 0x86600030
    // Test aarch32_UQASX_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=8, Rn=0, Rm=0
    let encoding: u32 = 0x86600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_uqasx_a1_a_field_cond_9_poweroftwo_30_96600030() {
    // Encoding: 0x96600030
    // Test aarch32_UQASX_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=9, Rm=0
    let encoding: u32 = 0x96600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_uqasx_a1_a_field_cond_10_poweroftwo_30_a6600030() {
    // Encoding: 0xA6600030
    // Test aarch32_UQASX_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=10
    let encoding: u32 = 0xA6600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_uqasx_a1_a_field_cond_11_poweroftwo_30_b6600030() {
    // Encoding: 0xB6600030
    // Test aarch32_UQASX_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=11, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0xB6600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_uqasx_a1_a_field_cond_12_poweroftwo_30_c6600030() {
    // Encoding: 0xC6600030
    // Test aarch32_UQASX_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=12, Rm=0
    let encoding: u32 = 0xC6600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_uqasx_a1_a_field_cond_13_poweroftwo_30_d6600030() {
    // Encoding: 0xD6600030
    // Test aarch32_UQASX_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=13, Rn=0, Rd=0
    let encoding: u32 = 0xD6600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_uqasx_a1_a_field_cond_14_poweroftwo_30_e6600030() {
    // Encoding: 0xE6600030
    // Test aarch32_UQASX_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0xE6600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_uqasx_a1_a_field_cond_15_max_30_f6600030() {
    // Encoding: 0xF6600030
    // Test aarch32_UQASX_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rm=0, cond=15, Rn=0, Rd=0
    let encoding: u32 = 0xF6600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqasx_a1_a_field_rn_0_min_30_06600030() {
    // Encoding: 0x06600030
    // Test aarch32_UQASX_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x06600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqasx_a1_a_field_rn_1_poweroftwo_30_06610030() {
    // Encoding: 0x06610030
    // Test aarch32_UQASX_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=1, Rd=0, cond=0
    let encoding: u32 = 0x06610030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqasx_a1_a_field_rd_0_min_30_06600030() {
    // Encoding: 0x06600030
    // Test aarch32_UQASX_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, cond=0, Rn=0, Rm=0
    let encoding: u32 = 0x06600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqasx_a1_a_field_rd_1_poweroftwo_30_06601030() {
    // Encoding: 0x06601030
    // Test aarch32_UQASX_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=1, Rn=0, cond=0, Rm=0
    let encoding: u32 = 0x06601030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqasx_a1_a_field_rm_0_min_30_06600030() {
    // Encoding: 0x06600030
    // Test aarch32_UQASX_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=0
    let encoding: u32 = 0x06600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqasx_a1_a_field_rm_1_poweroftwo_30_06600031() {
    // Encoding: 0x06600031
    // Test aarch32_UQASX_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=1, cond=0, Rn=0, Rd=0
    let encoding: u32 = 0x06600031;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_uqasx_a1_a_combo_0_30_06600030() {
    // Encoding: 0x06600030
    // Test aarch32_UQASX_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=0, Rn=0
    let encoding: u32 = 0x06600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_uqasx_a1_a_special_cond_0_condition_eq_48_06600030() {
    // Encoding: 0x06600030
    // Test aarch32_UQASX_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=0
    let encoding: u32 = 0x06600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_uqasx_a1_a_special_cond_1_condition_ne_48_16600030() {
    // Encoding: 0x16600030
    // Test aarch32_UQASX_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x16600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_uqasx_a1_a_special_cond_2_condition_cs_hs_48_26600030() {
    // Encoding: 0x26600030
    // Test aarch32_UQASX_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=2, Rn=0
    let encoding: u32 = 0x26600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_uqasx_a1_a_special_cond_3_condition_cc_lo_48_36600030() {
    // Encoding: 0x36600030
    // Test aarch32_UQASX_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=3, Rn=0
    let encoding: u32 = 0x36600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_uqasx_a1_a_special_cond_4_condition_mi_48_46600030() {
    // Encoding: 0x46600030
    // Test aarch32_UQASX_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=4, Rd=0
    let encoding: u32 = 0x46600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_uqasx_a1_a_special_cond_5_condition_pl_48_56600030() {
    // Encoding: 0x56600030
    // Test aarch32_UQASX_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x56600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_uqasx_a1_a_special_cond_6_condition_vs_48_66600030() {
    // Encoding: 0x66600030
    // Test aarch32_UQASX_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rn=0, cond=6, Rd=0, Rm=0
    let encoding: u32 = 0x66600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_uqasx_a1_a_special_cond_7_condition_vc_48_76600030() {
    // Encoding: 0x76600030
    // Test aarch32_UQASX_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=7, Rd=0
    let encoding: u32 = 0x76600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_uqasx_a1_a_special_cond_8_condition_hi_48_86600030() {
    // Encoding: 0x86600030
    // Test aarch32_UQASX_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x86600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_uqasx_a1_a_special_cond_9_condition_ls_48_96600030() {
    // Encoding: 0x96600030
    // Test aarch32_UQASX_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x96600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_uqasx_a1_a_special_cond_10_condition_ge_48_a6600030() {
    // Encoding: 0xA6600030
    // Test aarch32_UQASX_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0xA6600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_uqasx_a1_a_special_cond_11_condition_lt_48_b6600030() {
    // Encoding: 0xB6600030
    // Test aarch32_UQASX_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=11, Rn=0
    let encoding: u32 = 0xB6600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_uqasx_a1_a_special_cond_12_condition_gt_48_c6600030() {
    // Encoding: 0xC6600030
    // Test aarch32_UQASX_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: cond=12, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0xC6600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_uqasx_a1_a_special_cond_13_condition_le_48_d6600030() {
    // Encoding: 0xD6600030
    // Test aarch32_UQASX_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: cond=13, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0xD6600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_uqasx_a1_a_special_cond_14_condition_al_48_e6600030() {
    // Encoding: 0xE6600030
    // Test aarch32_UQASX_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: cond=14, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0xE6600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_uqasx_a1_a_special_cond_15_condition_nv_48_f6600030() {
    // Encoding: 0xF6600030
    // Test aarch32_UQASX_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=15
    let encoding: u32 = 0xF6600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uqasx_a1_a_invalid_0_30_06600030() {
    // Encoding: 0x06600030
    // Test aarch32_UQASX_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rn=0, cond=0, Rd=0, Rm=0
    let encoding: u32 = 0x06600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UQASX_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uqasx_a1_a_invalid_1_30_06600030() {
    // Encoding: 0x06600030
    // Test aarch32_UQASX_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x06600030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UQASX_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqasx_t1_a_field_rn_0_min_f050_faa0f050() {
    // Thumb encoding (32): 0xFAA0F050
    // Test aarch32_UQASX_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQASX_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqasx_t1_a_field_rn_1_poweroftwo_f050_faa1f050() {
    // Thumb encoding (32): 0xFAA1F050
    // Test aarch32_UQASX_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA1F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQASX_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqasx_t1_a_field_rd_0_min_f050_faa0f050() {
    // Thumb encoding (32): 0xFAA0F050
    // Test aarch32_UQASX_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQASX_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqasx_t1_a_field_rd_1_poweroftwo_f050_faa0f150() {
    // Thumb encoding (32): 0xFAA0F150
    // Test aarch32_UQASX_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rd=1, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F150;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQASX_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqasx_t1_a_field_rm_0_min_f050_faa0f050() {
    // Thumb encoding (32): 0xFAA0F050
    // Test aarch32_UQASX_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQASX_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqasx_t1_a_field_rm_1_poweroftwo_f050_faa0f051() {
    // Thumb encoding (32): 0xFAA0F051
    // Test aarch32_UQASX_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F051;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQASX_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_uqasx_t1_a_combo_0_f050_faa0f050() {
    // Thumb encoding (32): 0xFAA0F050
    // Test aarch32_UQASX_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQASX_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uqasx_t1_a_invalid_0_f050_faa0f050() {
    // Thumb encoding (32): 0xFAA0F050
    // Test aarch32_UQASX_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F050;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_UQASX_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uqasx_t1_a_invalid_1_f050_faa0f050() {
    // Thumb encoding (32): 0xFAA0F050
    // Test aarch32_UQASX_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F050;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_QADD8_A Tests
// ============================================================================

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_qadd8_a1_a_field_cond_0_min_90_06200090() {
    // Encoding: 0x06200090
    // Test aarch32_QADD8_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=0
    let encoding: u32 = 0x06200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_qadd8_a1_a_field_cond_1_poweroftwo_90_16200090() {
    // Encoding: 0x16200090
    // Test aarch32_QADD8_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=1, Rn=0, Rm=0
    let encoding: u32 = 0x16200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_qadd8_a1_a_field_cond_2_poweroftwo_90_26200090() {
    // Encoding: 0x26200090
    // Test aarch32_QADD8_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=2, Rd=0
    let encoding: u32 = 0x26200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_qadd8_a1_a_field_cond_3_poweroftwo_90_36200090() {
    // Encoding: 0x36200090
    // Test aarch32_QADD8_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x36200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_qadd8_a1_a_field_cond_4_poweroftwo_90_46200090() {
    // Encoding: 0x46200090
    // Test aarch32_QADD8_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=4, Rd=0
    let encoding: u32 = 0x46200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_qadd8_a1_a_field_cond_5_poweroftwo_90_56200090() {
    // Encoding: 0x56200090
    // Test aarch32_QADD8_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=5, Rd=0, Rm=0
    let encoding: u32 = 0x56200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_qadd8_a1_a_field_cond_6_poweroftwo_90_66200090() {
    // Encoding: 0x66200090
    // Test aarch32_QADD8_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=6, Rm=0
    let encoding: u32 = 0x66200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_qadd8_a1_a_field_cond_7_poweroftwo_90_76200090() {
    // Encoding: 0x76200090
    // Test aarch32_QADD8_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=7
    let encoding: u32 = 0x76200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_qadd8_a1_a_field_cond_8_poweroftwo_90_86200090() {
    // Encoding: 0x86200090
    // Test aarch32_QADD8_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=8, Rd=0
    let encoding: u32 = 0x86200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_qadd8_a1_a_field_cond_9_poweroftwo_90_96200090() {
    // Encoding: 0x96200090
    // Test aarch32_QADD8_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x96200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_qadd8_a1_a_field_cond_10_poweroftwo_90_a6200090() {
    // Encoding: 0xA6200090
    // Test aarch32_QADD8_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=10
    let encoding: u32 = 0xA6200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_qadd8_a1_a_field_cond_11_poweroftwo_90_b6200090() {
    // Encoding: 0xB6200090
    // Test aarch32_QADD8_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=11
    let encoding: u32 = 0xB6200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_qadd8_a1_a_field_cond_12_poweroftwo_90_c6200090() {
    // Encoding: 0xC6200090
    // Test aarch32_QADD8_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=12
    let encoding: u32 = 0xC6200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_qadd8_a1_a_field_cond_13_poweroftwo_90_d6200090() {
    // Encoding: 0xD6200090
    // Test aarch32_QADD8_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=13, Rd=0
    let encoding: u32 = 0xD6200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_qadd8_a1_a_field_cond_14_poweroftwo_90_e6200090() {
    // Encoding: 0xE6200090
    // Test aarch32_QADD8_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xE6200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_qadd8_a1_a_field_cond_15_max_90_f6200090() {
    // Encoding: 0xF6200090
    // Test aarch32_QADD8_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0xF6200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qadd8_a1_a_field_rn_0_min_90_06200090() {
    // Encoding: 0x06200090
    // Test aarch32_QADD8_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x06200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qadd8_a1_a_field_rn_1_poweroftwo_90_06210090() {
    // Encoding: 0x06210090
    // Test aarch32_QADD8_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rn=1, Rd=0
    let encoding: u32 = 0x06210090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qadd8_a1_a_field_rd_0_min_90_06200090() {
    // Encoding: 0x06200090
    // Test aarch32_QADD8_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x06200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qadd8_a1_a_field_rd_1_poweroftwo_90_06201090() {
    // Encoding: 0x06201090
    // Test aarch32_QADD8_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=1, cond=0
    let encoding: u32 = 0x06201090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qadd8_a1_a_field_rm_0_min_90_06200090() {
    // Encoding: 0x06200090
    // Test aarch32_QADD8_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x06200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qadd8_a1_a_field_rm_1_poweroftwo_90_06200091() {
    // Encoding: 0x06200091
    // Test aarch32_QADD8_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=0, Rm=1
    let encoding: u32 = 0x06200091;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_qadd8_a1_a_combo_0_90_06200090() {
    // Encoding: 0x06200090
    // Test aarch32_QADD8_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: cond=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x06200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_qadd8_a1_a_special_cond_0_condition_eq_144_06200090() {
    // Encoding: 0x06200090
    // Test aarch32_QADD8_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x06200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_qadd8_a1_a_special_cond_1_condition_ne_144_16200090() {
    // Encoding: 0x16200090
    // Test aarch32_QADD8_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=1
    let encoding: u32 = 0x16200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_qadd8_a1_a_special_cond_2_condition_cs_hs_144_26200090() {
    // Encoding: 0x26200090
    // Test aarch32_QADD8_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=2, Rd=0
    let encoding: u32 = 0x26200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_qadd8_a1_a_special_cond_3_condition_cc_lo_144_36200090() {
    // Encoding: 0x36200090
    // Test aarch32_QADD8_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rn=0, cond=3, Rm=0, Rd=0
    let encoding: u32 = 0x36200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_qadd8_a1_a_special_cond_4_condition_mi_144_46200090() {
    // Encoding: 0x46200090
    // Test aarch32_QADD8_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=4
    let encoding: u32 = 0x46200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_qadd8_a1_a_special_cond_5_condition_pl_144_56200090() {
    // Encoding: 0x56200090
    // Test aarch32_QADD8_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rn=0, cond=5, Rd=0, Rm=0
    let encoding: u32 = 0x56200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_qadd8_a1_a_special_cond_6_condition_vs_144_66200090() {
    // Encoding: 0x66200090
    // Test aarch32_QADD8_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x66200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_qadd8_a1_a_special_cond_7_condition_vc_144_76200090() {
    // Encoding: 0x76200090
    // Test aarch32_QADD8_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x76200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_qadd8_a1_a_special_cond_8_condition_hi_144_86200090() {
    // Encoding: 0x86200090
    // Test aarch32_QADD8_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rd=0, cond=8, Rm=0, Rn=0
    let encoding: u32 = 0x86200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_qadd8_a1_a_special_cond_9_condition_ls_144_96200090() {
    // Encoding: 0x96200090
    // Test aarch32_QADD8_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=9, Rn=0
    let encoding: u32 = 0x96200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_qadd8_a1_a_special_cond_10_condition_ge_144_a6200090() {
    // Encoding: 0xA6200090
    // Test aarch32_QADD8_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rn=0, cond=10, Rm=0, Rd=0
    let encoding: u32 = 0xA6200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_qadd8_a1_a_special_cond_11_condition_lt_144_b6200090() {
    // Encoding: 0xB6200090
    // Test aarch32_QADD8_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=11
    let encoding: u32 = 0xB6200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_qadd8_a1_a_special_cond_12_condition_gt_144_c6200090() {
    // Encoding: 0xC6200090
    // Test aarch32_QADD8_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rd=0, cond=12, Rn=0, Rm=0
    let encoding: u32 = 0xC6200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_qadd8_a1_a_special_cond_13_condition_le_144_d6200090() {
    // Encoding: 0xD6200090
    // Test aarch32_QADD8_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=13, Rd=0
    let encoding: u32 = 0xD6200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_qadd8_a1_a_special_cond_14_condition_al_144_e6200090() {
    // Encoding: 0xE6200090
    // Test aarch32_QADD8_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: cond=14, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0xE6200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_qadd8_a1_a_special_cond_15_condition_nv_144_f6200090() {
    // Encoding: 0xF6200090
    // Test aarch32_QADD8_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0xF6200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qadd8_a1_a_invalid_0_90_06200090() {
    // Encoding: 0x06200090
    // Test aarch32_QADD8_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x06200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_QADD8_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qadd8_a1_a_invalid_1_90_06200090() {
    // Encoding: 0x06200090
    // Test aarch32_QADD8_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=0, Rn=0
    let encoding: u32 = 0x06200090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_QADD8_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qadd8_t1_a_field_rn_0_min_f010_fa80f010() {
    // Thumb encoding (32): 0xFA80F010
    // Test aarch32_QADD8_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QADD8_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qadd8_t1_a_field_rn_1_poweroftwo_f010_fa81f010() {
    // Thumb encoding (32): 0xFA81F010
    // Test aarch32_QADD8_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA81F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QADD8_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qadd8_t1_a_field_rd_0_min_f010_fa80f010() {
    // Thumb encoding (32): 0xFA80F010
    // Test aarch32_QADD8_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QADD8_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qadd8_t1_a_field_rd_1_poweroftwo_f010_fa80f110() {
    // Thumb encoding (32): 0xFA80F110
    // Test aarch32_QADD8_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F110;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QADD8_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qadd8_t1_a_field_rm_0_min_f010_fa80f010() {
    // Thumb encoding (32): 0xFA80F010
    // Test aarch32_QADD8_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QADD8_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qadd8_t1_a_field_rm_1_poweroftwo_f010_fa80f011() {
    // Thumb encoding (32): 0xFA80F011
    // Test aarch32_QADD8_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F011;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QADD8_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_qadd8_t1_a_combo_0_f010_fa80f010() {
    // Thumb encoding (32): 0xFA80F010
    // Test aarch32_QADD8_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QADD8_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qadd8_t1_a_invalid_0_f010_fa80f010() {
    // Thumb encoding (32): 0xFA80F010
    // Test aarch32_QADD8_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F010;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_QADD8_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qadd8_t1_a_invalid_1_f010_fa80f010() {
    // Thumb encoding (32): 0xFA80F010
    // Test aarch32_QADD8_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F010;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_QSUB_A Tests
// ============================================================================

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_qsub_a1_a_field_cond_0_min_50_01200050() {
    // Encoding: 0x01200050
    // Test aarch32_QSUB_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x01200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_qsub_a1_a_field_cond_1_poweroftwo_50_11200050() {
    // Encoding: 0x11200050
    // Test aarch32_QSUB_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=1, Rm=0, Rn=0
    let encoding: u32 = 0x11200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_qsub_a1_a_field_cond_2_poweroftwo_50_21200050() {
    // Encoding: 0x21200050
    // Test aarch32_QSUB_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=2, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x21200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_qsub_a1_a_field_cond_3_poweroftwo_50_31200050() {
    // Encoding: 0x31200050
    // Test aarch32_QSUB_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=3, Rd=0, Rm=0
    let encoding: u32 = 0x31200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_qsub_a1_a_field_cond_4_poweroftwo_50_41200050() {
    // Encoding: 0x41200050
    // Test aarch32_QSUB_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=4, Rm=0, Rn=0
    let encoding: u32 = 0x41200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_qsub_a1_a_field_cond_5_poweroftwo_50_51200050() {
    // Encoding: 0x51200050
    // Test aarch32_QSUB_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=5
    let encoding: u32 = 0x51200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_qsub_a1_a_field_cond_6_poweroftwo_50_61200050() {
    // Encoding: 0x61200050
    // Test aarch32_QSUB_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x61200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_qsub_a1_a_field_cond_7_poweroftwo_50_71200050() {
    // Encoding: 0x71200050
    // Test aarch32_QSUB_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=7, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x71200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_qsub_a1_a_field_cond_8_poweroftwo_50_81200050() {
    // Encoding: 0x81200050
    // Test aarch32_QSUB_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=8, Rn=0
    let encoding: u32 = 0x81200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_qsub_a1_a_field_cond_9_poweroftwo_50_91200050() {
    // Encoding: 0x91200050
    // Test aarch32_QSUB_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x91200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_qsub_a1_a_field_cond_10_poweroftwo_50_a1200050() {
    // Encoding: 0xA1200050
    // Test aarch32_QSUB_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=10
    let encoding: u32 = 0xA1200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_qsub_a1_a_field_cond_11_poweroftwo_50_b1200050() {
    // Encoding: 0xB1200050
    // Test aarch32_QSUB_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=11, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0xB1200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_qsub_a1_a_field_cond_12_poweroftwo_50_c1200050() {
    // Encoding: 0xC1200050
    // Test aarch32_QSUB_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=12, Rd=0, Rm=0
    let encoding: u32 = 0xC1200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_qsub_a1_a_field_cond_13_poweroftwo_50_d1200050() {
    // Encoding: 0xD1200050
    // Test aarch32_QSUB_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=13, Rm=0, Rd=0
    let encoding: u32 = 0xD1200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_qsub_a1_a_field_cond_14_poweroftwo_50_e1200050() {
    // Encoding: 0xE1200050
    // Test aarch32_QSUB_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=14, Rm=0
    let encoding: u32 = 0xE1200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_qsub_a1_a_field_cond_15_max_50_f1200050() {
    // Encoding: 0xF1200050
    // Test aarch32_QSUB_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=15, Rn=0
    let encoding: u32 = 0xF1200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qsub_a1_a_field_rn_0_min_50_01200050() {
    // Encoding: 0x01200050
    // Test aarch32_QSUB_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x01200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qsub_a1_a_field_rn_1_poweroftwo_50_01210050() {
    // Encoding: 0x01210050
    // Test aarch32_QSUB_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=1, Rd=0, cond=0, Rm=0
    let encoding: u32 = 0x01210050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qsub_a1_a_field_rd_0_min_50_01200050() {
    // Encoding: 0x01200050
    // Test aarch32_QSUB_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=0, Rm=0
    let encoding: u32 = 0x01200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qsub_a1_a_field_rd_1_poweroftwo_50_01201050() {
    // Encoding: 0x01201050
    // Test aarch32_QSUB_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rd=1, Rn=0
    let encoding: u32 = 0x01201050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qsub_a1_a_field_rm_0_min_50_01200050() {
    // Encoding: 0x01200050
    // Test aarch32_QSUB_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x01200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qsub_a1_a_field_rm_1_poweroftwo_50_01200051() {
    // Encoding: 0x01200051
    // Test aarch32_QSUB_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=1, cond=0, Rd=0, Rn=0
    let encoding: u32 = 0x01200051;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_qsub_a1_a_combo_0_50_01200050() {
    // Encoding: 0x01200050
    // Test aarch32_QSUB_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=0
    let encoding: u32 = 0x01200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_qsub_a1_a_special_cond_0_condition_eq_80_01200050() {
    // Encoding: 0x01200050
    // Test aarch32_QSUB_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=0, Rn=0
    let encoding: u32 = 0x01200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_qsub_a1_a_special_cond_1_condition_ne_80_11200050() {
    // Encoding: 0x11200050
    // Test aarch32_QSUB_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=1
    let encoding: u32 = 0x11200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_qsub_a1_a_special_cond_2_condition_cs_hs_80_21200050() {
    // Encoding: 0x21200050
    // Test aarch32_QSUB_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x21200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_qsub_a1_a_special_cond_3_condition_cc_lo_80_31200050() {
    // Encoding: 0x31200050
    // Test aarch32_QSUB_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x31200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_qsub_a1_a_special_cond_4_condition_mi_80_41200050() {
    // Encoding: 0x41200050
    // Test aarch32_QSUB_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=4, Rn=0
    let encoding: u32 = 0x41200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_qsub_a1_a_special_cond_5_condition_pl_80_51200050() {
    // Encoding: 0x51200050
    // Test aarch32_QSUB_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x51200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_qsub_a1_a_special_cond_6_condition_vs_80_61200050() {
    // Encoding: 0x61200050
    // Test aarch32_QSUB_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x61200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_qsub_a1_a_special_cond_7_condition_vc_80_71200050() {
    // Encoding: 0x71200050
    // Test aarch32_QSUB_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x71200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_qsub_a1_a_special_cond_8_condition_hi_80_81200050() {
    // Encoding: 0x81200050
    // Test aarch32_QSUB_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=8
    let encoding: u32 = 0x81200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_qsub_a1_a_special_cond_9_condition_ls_80_91200050() {
    // Encoding: 0x91200050
    // Test aarch32_QSUB_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=9, Rd=0
    let encoding: u32 = 0x91200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_qsub_a1_a_special_cond_10_condition_ge_80_a1200050() {
    // Encoding: 0xA1200050
    // Test aarch32_QSUB_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rd=0, cond=10, Rn=0, Rm=0
    let encoding: u32 = 0xA1200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_qsub_a1_a_special_cond_11_condition_lt_80_b1200050() {
    // Encoding: 0xB1200050
    // Test aarch32_QSUB_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=11, Rm=0
    let encoding: u32 = 0xB1200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_qsub_a1_a_special_cond_12_condition_gt_80_c1200050() {
    // Encoding: 0xC1200050
    // Test aarch32_QSUB_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=12, Rd=0
    let encoding: u32 = 0xC1200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_qsub_a1_a_special_cond_13_condition_le_80_d1200050() {
    // Encoding: 0xD1200050
    // Test aarch32_QSUB_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rn=0, cond=13, Rd=0, Rm=0
    let encoding: u32 = 0xD1200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_qsub_a1_a_special_cond_14_condition_al_80_e1200050() {
    // Encoding: 0xE1200050
    // Test aarch32_QSUB_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: cond=14, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xE1200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_qsub_a1_a_special_cond_15_condition_nv_80_f1200050() {
    // Encoding: 0xF1200050
    // Test aarch32_QSUB_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0xF1200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qsub_a1_a_invalid_0_50_01200050() {
    // Encoding: 0x01200050
    // Test aarch32_QSUB_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x01200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_QSUB_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qsub_a1_a_invalid_1_50_01200050() {
    // Encoding: 0x01200050
    // Test aarch32_QSUB_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=0, Rm=0
    let encoding: u32 = 0x01200050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_QSUB_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qsub_t1_a_field_rn_0_min_f0a0_fa80f0a0() {
    // Thumb encoding (32): 0xFA80F0A0
    // Test aarch32_QSUB_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F0A0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSUB_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qsub_t1_a_field_rn_1_poweroftwo_f0a0_fa81f0a0() {
    // Thumb encoding (32): 0xFA81F0A0
    // Test aarch32_QSUB_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rn=1, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA81F0A0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSUB_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qsub_t1_a_field_rd_0_min_f0a0_fa80f0a0() {
    // Thumb encoding (32): 0xFA80F0A0
    // Test aarch32_QSUB_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F0A0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSUB_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qsub_t1_a_field_rd_1_poweroftwo_f0a0_fa80f1a0() {
    // Thumb encoding (32): 0xFA80F1A0
    // Test aarch32_QSUB_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rd=1, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F1A0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSUB_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qsub_t1_a_field_rm_0_min_f0a0_fa80f0a0() {
    // Thumb encoding (32): 0xFA80F0A0
    // Test aarch32_QSUB_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F0A0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSUB_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qsub_t1_a_field_rm_1_poweroftwo_f0a0_fa80f0a1() {
    // Thumb encoding (32): 0xFA80F0A1
    // Test aarch32_QSUB_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F0A1;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSUB_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_qsub_t1_a_combo_0_f0a0_fa80f0a0() {
    // Thumb encoding (32): 0xFA80F0A0
    // Test aarch32_QSUB_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F0A0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSUB_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qsub_t1_a_invalid_0_f0a0_fa80f0a0() {
    // Thumb encoding (32): 0xFA80F0A0
    // Test aarch32_QSUB_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F0A0;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_QSUB_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qsub_t1_a_invalid_1_f0a0_fa80f0a0() {
    // Thumb encoding (32): 0xFA80F0A0
    // Test aarch32_QSUB_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F0A0;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_UQADD16_A Tests
// ============================================================================

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_uqadd16_a1_a_field_cond_0_min_10_06600010() {
    // Encoding: 0x06600010
    // Test aarch32_UQADD16_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, cond=0, Rm=0, Rd=0
    let encoding: u32 = 0x06600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_uqadd16_a1_a_field_cond_1_poweroftwo_10_16600010() {
    // Encoding: 0x16600010
    // Test aarch32_UQADD16_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=1
    let encoding: u32 = 0x16600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_uqadd16_a1_a_field_cond_2_poweroftwo_10_26600010() {
    // Encoding: 0x26600010
    // Test aarch32_UQADD16_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=2, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x26600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_uqadd16_a1_a_field_cond_3_poweroftwo_10_36600010() {
    // Encoding: 0x36600010
    // Test aarch32_UQADD16_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=3
    let encoding: u32 = 0x36600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_uqadd16_a1_a_field_cond_4_poweroftwo_10_46600010() {
    // Encoding: 0x46600010
    // Test aarch32_UQADD16_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=4, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x46600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_uqadd16_a1_a_field_cond_5_poweroftwo_10_56600010() {
    // Encoding: 0x56600010
    // Test aarch32_UQADD16_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=5, Rn=0, Rd=0
    let encoding: u32 = 0x56600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_uqadd16_a1_a_field_cond_6_poweroftwo_10_66600010() {
    // Encoding: 0x66600010
    // Test aarch32_UQADD16_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=6, Rm=0, Rn=0
    let encoding: u32 = 0x66600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_uqadd16_a1_a_field_cond_7_poweroftwo_10_76600010() {
    // Encoding: 0x76600010
    // Test aarch32_UQADD16_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=7, Rd=0, Rm=0
    let encoding: u32 = 0x76600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_uqadd16_a1_a_field_cond_8_poweroftwo_10_86600010() {
    // Encoding: 0x86600010
    // Test aarch32_UQADD16_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=8
    let encoding: u32 = 0x86600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_uqadd16_a1_a_field_cond_9_poweroftwo_10_96600010() {
    // Encoding: 0x96600010
    // Test aarch32_UQADD16_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=9, Rd=0
    let encoding: u32 = 0x96600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_uqadd16_a1_a_field_cond_10_poweroftwo_10_a6600010() {
    // Encoding: 0xA6600010
    // Test aarch32_UQADD16_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xA6600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_uqadd16_a1_a_field_cond_11_poweroftwo_10_b6600010() {
    // Encoding: 0xB6600010
    // Test aarch32_UQADD16_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=11, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0xB6600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_uqadd16_a1_a_field_cond_12_poweroftwo_10_c6600010() {
    // Encoding: 0xC6600010
    // Test aarch32_UQADD16_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0xC6600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_uqadd16_a1_a_field_cond_13_poweroftwo_10_d6600010() {
    // Encoding: 0xD6600010
    // Test aarch32_UQADD16_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=13
    let encoding: u32 = 0xD6600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_uqadd16_a1_a_field_cond_14_poweroftwo_10_e6600010() {
    // Encoding: 0xE6600010
    // Test aarch32_UQADD16_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0xE6600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_uqadd16_a1_a_field_cond_15_max_10_f6600010() {
    // Encoding: 0xF6600010
    // Test aarch32_UQADD16_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0xF6600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqadd16_a1_a_field_rn_0_min_10_06600010() {
    // Encoding: 0x06600010
    // Test aarch32_UQADD16_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=0
    let encoding: u32 = 0x06600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqadd16_a1_a_field_rn_1_poweroftwo_10_06610010() {
    // Encoding: 0x06610010
    // Test aarch32_UQADD16_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=0, Rn=1
    let encoding: u32 = 0x06610010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqadd16_a1_a_field_rd_0_min_10_06600010() {
    // Encoding: 0x06600010
    // Test aarch32_UQADD16_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x06600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqadd16_a1_a_field_rd_1_poweroftwo_10_06601010() {
    // Encoding: 0x06601010
    // Test aarch32_UQADD16_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=1, Rn=0, Rm=0, cond=0
    let encoding: u32 = 0x06601010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqadd16_a1_a_field_rm_0_min_10_06600010() {
    // Encoding: 0x06600010
    // Test aarch32_UQADD16_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=0
    let encoding: u32 = 0x06600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqadd16_a1_a_field_rm_1_poweroftwo_10_06600011() {
    // Encoding: 0x06600011
    // Test aarch32_UQADD16_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=0, Rm=1, Rn=0
    let encoding: u32 = 0x06600011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_uqadd16_a1_a_combo_0_10_06600010() {
    // Encoding: 0x06600010
    // Test aarch32_UQADD16_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x06600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_uqadd16_a1_a_special_cond_0_condition_eq_16_06600010() {
    // Encoding: 0x06600010
    // Test aarch32_UQADD16_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x06600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_uqadd16_a1_a_special_cond_1_condition_ne_16_16600010() {
    // Encoding: 0x16600010
    // Test aarch32_UQADD16_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=1, Rd=0
    let encoding: u32 = 0x16600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_uqadd16_a1_a_special_cond_2_condition_cs_hs_16_26600010() {
    // Encoding: 0x26600010
    // Test aarch32_UQADD16_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x26600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_uqadd16_a1_a_special_cond_3_condition_cc_lo_16_36600010() {
    // Encoding: 0x36600010
    // Test aarch32_UQADD16_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rm=0, cond=3, Rn=0, Rd=0
    let encoding: u32 = 0x36600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_uqadd16_a1_a_special_cond_4_condition_mi_16_46600010() {
    // Encoding: 0x46600010
    // Test aarch32_UQADD16_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x46600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_uqadd16_a1_a_special_cond_5_condition_pl_16_56600010() {
    // Encoding: 0x56600010
    // Test aarch32_UQADD16_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rd=0, cond=5, Rm=0, Rn=0
    let encoding: u32 = 0x56600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_uqadd16_a1_a_special_cond_6_condition_vs_16_66600010() {
    // Encoding: 0x66600010
    // Test aarch32_UQADD16_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=6, Rm=0
    let encoding: u32 = 0x66600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_uqadd16_a1_a_special_cond_7_condition_vc_16_76600010() {
    // Encoding: 0x76600010
    // Test aarch32_UQADD16_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rm=0, cond=7, Rd=0, Rn=0
    let encoding: u32 = 0x76600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_uqadd16_a1_a_special_cond_8_condition_hi_16_86600010() {
    // Encoding: 0x86600010
    // Test aarch32_UQADD16_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x86600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_uqadd16_a1_a_special_cond_9_condition_ls_16_96600010() {
    // Encoding: 0x96600010
    // Test aarch32_UQADD16_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=9, Rn=0
    let encoding: u32 = 0x96600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_uqadd16_a1_a_special_cond_10_condition_ge_16_a6600010() {
    // Encoding: 0xA6600010
    // Test aarch32_UQADD16_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=10, Rn=0
    let encoding: u32 = 0xA6600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_uqadd16_a1_a_special_cond_11_condition_lt_16_b6600010() {
    // Encoding: 0xB6600010
    // Test aarch32_UQADD16_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0xB6600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_uqadd16_a1_a_special_cond_12_condition_gt_16_c6600010() {
    // Encoding: 0xC6600010
    // Test aarch32_UQADD16_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rn=0, cond=12, Rm=0, Rd=0
    let encoding: u32 = 0xC6600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_uqadd16_a1_a_special_cond_13_condition_le_16_d6600010() {
    // Encoding: 0xD6600010
    // Test aarch32_UQADD16_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rn=0, cond=13, Rm=0, Rd=0
    let encoding: u32 = 0xD6600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_uqadd16_a1_a_special_cond_14_condition_al_16_e6600010() {
    // Encoding: 0xE6600010
    // Test aarch32_UQADD16_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=14, Rd=0
    let encoding: u32 = 0xE6600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_uqadd16_a1_a_special_cond_15_condition_nv_16_f6600010() {
    // Encoding: 0xF6600010
    // Test aarch32_UQADD16_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=15, Rm=0
    let encoding: u32 = 0xF6600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uqadd16_a1_a_invalid_0_10_06600010() {
    // Encoding: 0x06600010
    // Test aarch32_UQADD16_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rn=0, cond=0, Rd=0, Rm=0
    let encoding: u32 = 0x06600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UQADD16_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uqadd16_a1_a_invalid_1_10_06600010() {
    // Encoding: 0x06600010
    // Test aarch32_UQADD16_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rm=0, cond=0, Rd=0, Rn=0
    let encoding: u32 = 0x06600010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UQADD16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqadd16_t1_a_field_rn_0_min_f050_fa90f050() {
    // Thumb encoding (32): 0xFA90F050
    // Test aarch32_UQADD16_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQADD16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqadd16_t1_a_field_rn_1_poweroftwo_f050_fa91f050() {
    // Thumb encoding (32): 0xFA91F050
    // Test aarch32_UQADD16_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rn=1, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA91F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQADD16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqadd16_t1_a_field_rd_0_min_f050_fa90f050() {
    // Thumb encoding (32): 0xFA90F050
    // Test aarch32_UQADD16_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQADD16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqadd16_t1_a_field_rd_1_poweroftwo_f050_fa90f150() {
    // Thumb encoding (32): 0xFA90F150
    // Test aarch32_UQADD16_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rd=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F150;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQADD16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqadd16_t1_a_field_rm_0_min_f050_fa90f050() {
    // Thumb encoding (32): 0xFA90F050
    // Test aarch32_UQADD16_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQADD16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqadd16_t1_a_field_rm_1_poweroftwo_f050_fa90f051() {
    // Thumb encoding (32): 0xFA90F051
    // Test aarch32_UQADD16_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F051;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQADD16_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_uqadd16_t1_a_combo_0_f050_fa90f050() {
    // Thumb encoding (32): 0xFA90F050
    // Test aarch32_UQADD16_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQADD16_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uqadd16_t1_a_invalid_0_f050_fa90f050() {
    // Thumb encoding (32): 0xFA90F050
    // Test aarch32_UQADD16_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F050;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_UQADD16_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uqadd16_t1_a_invalid_1_f050_fa90f050() {
    // Thumb encoding (32): 0xFA90F050
    // Test aarch32_UQADD16_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F050;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_UQSAX_A Tests
// ============================================================================

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_uqsax_a1_a_field_cond_0_min_50_06600050() {
    // Encoding: 0x06600050
    // Test aarch32_UQSAX_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x06600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_uqsax_a1_a_field_cond_1_poweroftwo_50_16600050() {
    // Encoding: 0x16600050
    // Test aarch32_UQSAX_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=1, Rm=0, Rn=0
    let encoding: u32 = 0x16600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_uqsax_a1_a_field_cond_2_poweroftwo_50_26600050() {
    // Encoding: 0x26600050
    // Test aarch32_UQSAX_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=2
    let encoding: u32 = 0x26600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_uqsax_a1_a_field_cond_3_poweroftwo_50_36600050() {
    // Encoding: 0x36600050
    // Test aarch32_UQSAX_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=3, Rn=0
    let encoding: u32 = 0x36600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_uqsax_a1_a_field_cond_4_poweroftwo_50_46600050() {
    // Encoding: 0x46600050
    // Test aarch32_UQSAX_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=4
    let encoding: u32 = 0x46600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_uqsax_a1_a_field_cond_5_poweroftwo_50_56600050() {
    // Encoding: 0x56600050
    // Test aarch32_UQSAX_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=5, Rd=0
    let encoding: u32 = 0x56600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_uqsax_a1_a_field_cond_6_poweroftwo_50_66600050() {
    // Encoding: 0x66600050
    // Test aarch32_UQSAX_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x66600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_uqsax_a1_a_field_cond_7_poweroftwo_50_76600050() {
    // Encoding: 0x76600050
    // Test aarch32_UQSAX_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=7, Rn=0, Rd=0
    let encoding: u32 = 0x76600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_uqsax_a1_a_field_cond_8_poweroftwo_50_86600050() {
    // Encoding: 0x86600050
    // Test aarch32_UQSAX_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=8, Rm=0, Rn=0
    let encoding: u32 = 0x86600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_uqsax_a1_a_field_cond_9_poweroftwo_50_96600050() {
    // Encoding: 0x96600050
    // Test aarch32_UQSAX_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=9
    let encoding: u32 = 0x96600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_uqsax_a1_a_field_cond_10_poweroftwo_50_a6600050() {
    // Encoding: 0xA6600050
    // Test aarch32_UQSAX_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=10, Rd=0
    let encoding: u32 = 0xA6600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_uqsax_a1_a_field_cond_11_poweroftwo_50_b6600050() {
    // Encoding: 0xB6600050
    // Test aarch32_UQSAX_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=11
    let encoding: u32 = 0xB6600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_uqsax_a1_a_field_cond_12_poweroftwo_50_c6600050() {
    // Encoding: 0xC6600050
    // Test aarch32_UQSAX_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=12, Rn=0
    let encoding: u32 = 0xC6600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_uqsax_a1_a_field_cond_13_poweroftwo_50_d6600050() {
    // Encoding: 0xD6600050
    // Test aarch32_UQSAX_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=13, Rn=0
    let encoding: u32 = 0xD6600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_uqsax_a1_a_field_cond_14_poweroftwo_50_e6600050() {
    // Encoding: 0xE6600050
    // Test aarch32_UQSAX_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xE6600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_uqsax_a1_a_field_cond_15_max_50_f6600050() {
    // Encoding: 0xF6600050
    // Test aarch32_UQSAX_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0xF6600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqsax_a1_a_field_rn_0_min_50_06600050() {
    // Encoding: 0x06600050
    // Test aarch32_UQSAX_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x06600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqsax_a1_a_field_rn_1_poweroftwo_50_06610050() {
    // Encoding: 0x06610050
    // Test aarch32_UQSAX_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rn=1, Rd=0
    let encoding: u32 = 0x06610050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqsax_a1_a_field_rd_0_min_50_06600050() {
    // Encoding: 0x06600050
    // Test aarch32_UQSAX_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, cond=0, Rm=0, Rn=0
    let encoding: u32 = 0x06600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqsax_a1_a_field_rd_1_poweroftwo_50_06601050() {
    // Encoding: 0x06601050
    // Test aarch32_UQSAX_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=1, cond=0, Rn=0
    let encoding: u32 = 0x06601050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqsax_a1_a_field_rm_0_min_50_06600050() {
    // Encoding: 0x06600050
    // Test aarch32_UQSAX_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=0
    let encoding: u32 = 0x06600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqsax_a1_a_field_rm_1_poweroftwo_50_06600051() {
    // Encoding: 0x06600051
    // Test aarch32_UQSAX_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=1, Rd=0, cond=0, Rn=0
    let encoding: u32 = 0x06600051;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_uqsax_a1_a_combo_0_50_06600050() {
    // Encoding: 0x06600050
    // Test aarch32_UQSAX_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=0
    let encoding: u32 = 0x06600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_uqsax_a1_a_special_cond_0_condition_eq_80_06600050() {
    // Encoding: 0x06600050
    // Test aarch32_UQSAX_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=0
    let encoding: u32 = 0x06600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_uqsax_a1_a_special_cond_1_condition_ne_80_16600050() {
    // Encoding: 0x16600050
    // Test aarch32_UQSAX_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rn=0, cond=1, Rd=0, Rm=0
    let encoding: u32 = 0x16600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_uqsax_a1_a_special_cond_2_condition_cs_hs_80_26600050() {
    // Encoding: 0x26600050
    // Test aarch32_UQSAX_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x26600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_uqsax_a1_a_special_cond_3_condition_cc_lo_80_36600050() {
    // Encoding: 0x36600050
    // Test aarch32_UQSAX_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=3
    let encoding: u32 = 0x36600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_uqsax_a1_a_special_cond_4_condition_mi_80_46600050() {
    // Encoding: 0x46600050
    // Test aarch32_UQSAX_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=4, Rn=0
    let encoding: u32 = 0x46600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_uqsax_a1_a_special_cond_5_condition_pl_80_56600050() {
    // Encoding: 0x56600050
    // Test aarch32_UQSAX_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rn=0, cond=5, Rd=0, Rm=0
    let encoding: u32 = 0x56600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_uqsax_a1_a_special_cond_6_condition_vs_80_66600050() {
    // Encoding: 0x66600050
    // Test aarch32_UQSAX_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x66600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_uqsax_a1_a_special_cond_7_condition_vc_80_76600050() {
    // Encoding: 0x76600050
    // Test aarch32_UQSAX_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x76600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_uqsax_a1_a_special_cond_8_condition_hi_80_86600050() {
    // Encoding: 0x86600050
    // Test aarch32_UQSAX_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=8, Rm=0
    let encoding: u32 = 0x86600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_uqsax_a1_a_special_cond_9_condition_ls_80_96600050() {
    // Encoding: 0x96600050
    // Test aarch32_UQSAX_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x96600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_uqsax_a1_a_special_cond_10_condition_ge_80_a6600050() {
    // Encoding: 0xA6600050
    // Test aarch32_UQSAX_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0xA6600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_uqsax_a1_a_special_cond_11_condition_lt_80_b6600050() {
    // Encoding: 0xB6600050
    // Test aarch32_UQSAX_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rd=0, cond=11, Rm=0, Rn=0
    let encoding: u32 = 0xB6600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_uqsax_a1_a_special_cond_12_condition_gt_80_c6600050() {
    // Encoding: 0xC6600050
    // Test aarch32_UQSAX_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=12, Rd=0
    let encoding: u32 = 0xC6600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_uqsax_a1_a_special_cond_13_condition_le_80_d6600050() {
    // Encoding: 0xD6600050
    // Test aarch32_UQSAX_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: cond=13, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xD6600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_uqsax_a1_a_special_cond_14_condition_al_80_e6600050() {
    // Encoding: 0xE6600050
    // Test aarch32_UQSAX_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: cond=14, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0xE6600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_uqsax_a1_a_special_cond_15_condition_nv_80_f6600050() {
    // Encoding: 0xF6600050
    // Test aarch32_UQSAX_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rn=0, cond=15, Rm=0, Rd=0
    let encoding: u32 = 0xF6600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uqsax_a1_a_invalid_0_50_06600050() {
    // Encoding: 0x06600050
    // Test aarch32_UQSAX_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=0, Rm=0
    let encoding: u32 = 0x06600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UQSAX_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uqsax_a1_a_invalid_1_50_06600050() {
    // Encoding: 0x06600050
    // Test aarch32_UQSAX_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=0
    let encoding: u32 = 0x06600050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UQSAX_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqsax_t1_a_field_rn_0_min_f050_fae0f050() {
    // Thumb encoding (32): 0xFAE0F050
    // Test aarch32_UQSAX_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQSAX_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqsax_t1_a_field_rn_1_poweroftwo_f050_fae1f050() {
    // Thumb encoding (32): 0xFAE1F050
    // Test aarch32_UQSAX_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rn=1, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE1F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQSAX_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqsax_t1_a_field_rd_0_min_f050_fae0f050() {
    // Thumb encoding (32): 0xFAE0F050
    // Test aarch32_UQSAX_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQSAX_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqsax_t1_a_field_rd_1_poweroftwo_f050_fae0f150() {
    // Thumb encoding (32): 0xFAE0F150
    // Test aarch32_UQSAX_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F150;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQSAX_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqsax_t1_a_field_rm_0_min_f050_fae0f050() {
    // Thumb encoding (32): 0xFAE0F050
    // Test aarch32_UQSAX_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQSAX_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqsax_t1_a_field_rm_1_poweroftwo_f050_fae0f051() {
    // Thumb encoding (32): 0xFAE0F051
    // Test aarch32_UQSAX_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F051;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQSAX_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_uqsax_t1_a_combo_0_f050_fae0f050() {
    // Thumb encoding (32): 0xFAE0F050
    // Test aarch32_UQSAX_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQSAX_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uqsax_t1_a_invalid_0_f050_fae0f050() {
    // Thumb encoding (32): 0xFAE0F050
    // Test aarch32_UQSAX_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F050;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_UQSAX_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uqsax_t1_a_invalid_1_f050_fae0f050() {
    // Thumb encoding (32): 0xFAE0F050
    // Test aarch32_UQSAX_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F050;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_QADD_A Tests
// ============================================================================

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_qadd_a1_a_field_cond_0_min_50_01000050() {
    // Encoding: 0x01000050
    // Test aarch32_QADD_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x01000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_qadd_a1_a_field_cond_1_poweroftwo_50_11000050() {
    // Encoding: 0x11000050
    // Test aarch32_QADD_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=1, Rn=0, Rd=0
    let encoding: u32 = 0x11000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_qadd_a1_a_field_cond_2_poweroftwo_50_21000050() {
    // Encoding: 0x21000050
    // Test aarch32_QADD_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=2, Rm=0
    let encoding: u32 = 0x21000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_qadd_a1_a_field_cond_3_poweroftwo_50_31000050() {
    // Encoding: 0x31000050
    // Test aarch32_QADD_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=3
    let encoding: u32 = 0x31000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_qadd_a1_a_field_cond_4_poweroftwo_50_41000050() {
    // Encoding: 0x41000050
    // Test aarch32_QADD_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=4, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x41000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_qadd_a1_a_field_cond_5_poweroftwo_50_51000050() {
    // Encoding: 0x51000050
    // Test aarch32_QADD_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=5, Rd=0, Rn=0
    let encoding: u32 = 0x51000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_qadd_a1_a_field_cond_6_poweroftwo_50_61000050() {
    // Encoding: 0x61000050
    // Test aarch32_QADD_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=6
    let encoding: u32 = 0x61000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_qadd_a1_a_field_cond_7_poweroftwo_50_71000050() {
    // Encoding: 0x71000050
    // Test aarch32_QADD_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=7, Rd=0
    let encoding: u32 = 0x71000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_qadd_a1_a_field_cond_8_poweroftwo_50_81000050() {
    // Encoding: 0x81000050
    // Test aarch32_QADD_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=8
    let encoding: u32 = 0x81000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_qadd_a1_a_field_cond_9_poweroftwo_50_91000050() {
    // Encoding: 0x91000050
    // Test aarch32_QADD_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=9, Rn=0
    let encoding: u32 = 0x91000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_qadd_a1_a_field_cond_10_poweroftwo_50_a1000050() {
    // Encoding: 0xA1000050
    // Test aarch32_QADD_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0xA1000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_qadd_a1_a_field_cond_11_poweroftwo_50_b1000050() {
    // Encoding: 0xB1000050
    // Test aarch32_QADD_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=11, Rn=0
    let encoding: u32 = 0xB1000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_qadd_a1_a_field_cond_12_poweroftwo_50_c1000050() {
    // Encoding: 0xC1000050
    // Test aarch32_QADD_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0xC1000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_qadd_a1_a_field_cond_13_poweroftwo_50_d1000050() {
    // Encoding: 0xD1000050
    // Test aarch32_QADD_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=13
    let encoding: u32 = 0xD1000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_qadd_a1_a_field_cond_14_poweroftwo_50_e1000050() {
    // Encoding: 0xE1000050
    // Test aarch32_QADD_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0xE1000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_qadd_a1_a_field_cond_15_max_50_f1000050() {
    // Encoding: 0xF1000050
    // Test aarch32_QADD_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0xF1000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qadd_a1_a_field_rn_0_min_50_01000050() {
    // Encoding: 0x01000050
    // Test aarch32_QADD_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=0
    let encoding: u32 = 0x01000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qadd_a1_a_field_rn_1_poweroftwo_50_01010050() {
    // Encoding: 0x01010050
    // Test aarch32_QADD_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=1, Rm=0, Rd=0
    let encoding: u32 = 0x01010050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qadd_a1_a_field_rd_0_min_50_01000050() {
    // Encoding: 0x01000050
    // Test aarch32_QADD_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x01000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qadd_a1_a_field_rd_1_poweroftwo_50_01001050() {
    // Encoding: 0x01001050
    // Test aarch32_QADD_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=1, Rm=0, Rn=0
    let encoding: u32 = 0x01001050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qadd_a1_a_field_rm_0_min_50_01000050() {
    // Encoding: 0x01000050
    // Test aarch32_QADD_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=0
    let encoding: u32 = 0x01000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qadd_a1_a_field_rm_1_poweroftwo_50_01000051() {
    // Encoding: 0x01000051
    // Test aarch32_QADD_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=1, Rd=0, cond=0
    let encoding: u32 = 0x01000051;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_qadd_a1_a_combo_0_50_01000050() {
    // Encoding: 0x01000050
    // Test aarch32_QADD_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=0
    let encoding: u32 = 0x01000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_qadd_a1_a_special_cond_0_condition_eq_80_01000050() {
    // Encoding: 0x01000050
    // Test aarch32_QADD_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x01000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_qadd_a1_a_special_cond_1_condition_ne_80_11000050() {
    // Encoding: 0x11000050
    // Test aarch32_QADD_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=1, Rd=0
    let encoding: u32 = 0x11000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_qadd_a1_a_special_cond_2_condition_cs_hs_80_21000050() {
    // Encoding: 0x21000050
    // Test aarch32_QADD_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=2
    let encoding: u32 = 0x21000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_qadd_a1_a_special_cond_3_condition_cc_lo_80_31000050() {
    // Encoding: 0x31000050
    // Test aarch32_QADD_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=3, Rm=0
    let encoding: u32 = 0x31000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_qadd_a1_a_special_cond_4_condition_mi_80_41000050() {
    // Encoding: 0x41000050
    // Test aarch32_QADD_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x41000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_qadd_a1_a_special_cond_5_condition_pl_80_51000050() {
    // Encoding: 0x51000050
    // Test aarch32_QADD_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=5
    let encoding: u32 = 0x51000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_qadd_a1_a_special_cond_6_condition_vs_80_61000050() {
    // Encoding: 0x61000050
    // Test aarch32_QADD_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x61000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_qadd_a1_a_special_cond_7_condition_vc_80_71000050() {
    // Encoding: 0x71000050
    // Test aarch32_QADD_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=7, Rd=0
    let encoding: u32 = 0x71000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_qadd_a1_a_special_cond_8_condition_hi_80_81000050() {
    // Encoding: 0x81000050
    // Test aarch32_QADD_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=8, Rd=0
    let encoding: u32 = 0x81000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_qadd_a1_a_special_cond_9_condition_ls_80_91000050() {
    // Encoding: 0x91000050
    // Test aarch32_QADD_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=9, Rn=0
    let encoding: u32 = 0x91000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_qadd_a1_a_special_cond_10_condition_ge_80_a1000050() {
    // Encoding: 0xA1000050
    // Test aarch32_QADD_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rd=0, cond=10, Rm=0, Rn=0
    let encoding: u32 = 0xA1000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_qadd_a1_a_special_cond_11_condition_lt_80_b1000050() {
    // Encoding: 0xB1000050
    // Test aarch32_QADD_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xB1000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_qadd_a1_a_special_cond_12_condition_gt_80_c1000050() {
    // Encoding: 0xC1000050
    // Test aarch32_QADD_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=12, Rd=0
    let encoding: u32 = 0xC1000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_qadd_a1_a_special_cond_13_condition_le_80_d1000050() {
    // Encoding: 0xD1000050
    // Test aarch32_QADD_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=13
    let encoding: u32 = 0xD1000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_qadd_a1_a_special_cond_14_condition_al_80_e1000050() {
    // Encoding: 0xE1000050
    // Test aarch32_QADD_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: cond=14, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0xE1000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_qadd_a1_a_special_cond_15_condition_nv_80_f1000050() {
    // Encoding: 0xF1000050
    // Test aarch32_QADD_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rm=0, cond=15, Rn=0, Rd=0
    let encoding: u32 = 0xF1000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qadd_a1_a_invalid_0_50_01000050() {
    // Encoding: 0x01000050
    // Test aarch32_QADD_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=0, Rd=0
    let encoding: u32 = 0x01000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_QADD_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qadd_a1_a_invalid_1_50_01000050() {
    // Encoding: 0x01000050
    // Test aarch32_QADD_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rd=0, cond=0, Rn=0, Rm=0
    let encoding: u32 = 0x01000050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_QADD_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qadd_t1_a_field_rn_0_min_f080_fa80f080() {
    // Thumb encoding (32): 0xFA80F080
    // Test aarch32_QADD_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QADD_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qadd_t1_a_field_rn_1_poweroftwo_f080_fa81f080() {
    // Thumb encoding (32): 0xFA81F080
    // Test aarch32_QADD_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA81F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QADD_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qadd_t1_a_field_rd_0_min_f080_fa80f080() {
    // Thumb encoding (32): 0xFA80F080
    // Test aarch32_QADD_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QADD_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qadd_t1_a_field_rd_1_poweroftwo_f080_fa80f180() {
    // Thumb encoding (32): 0xFA80F180
    // Test aarch32_QADD_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rd=1, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QADD_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qadd_t1_a_field_rm_0_min_f080_fa80f080() {
    // Thumb encoding (32): 0xFA80F080
    // Test aarch32_QADD_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QADD_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qadd_t1_a_field_rm_1_poweroftwo_f080_fa80f081() {
    // Thumb encoding (32): 0xFA80F081
    // Test aarch32_QADD_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F081;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QADD_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_qadd_t1_a_combo_0_f080_fa80f080() {
    // Thumb encoding (32): 0xFA80F080
    // Test aarch32_QADD_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QADD_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qadd_t1_a_invalid_0_f080_fa80f080() {
    // Thumb encoding (32): 0xFA80F080
    // Test aarch32_QADD_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_QADD_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qadd_t1_a_invalid_1_f080_fa80f080() {
    // Thumb encoding (32): 0xFA80F080
    // Test aarch32_QADD_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_QSUB8_A Tests
// ============================================================================

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_qsub8_a1_a_field_cond_0_min_f0_062000f0() {
    // Encoding: 0x062000F0
    // Test aarch32_QSUB8_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=0, Rn=0
    let encoding: u32 = 0x062000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_qsub8_a1_a_field_cond_1_poweroftwo_f0_162000f0() {
    // Encoding: 0x162000F0
    // Test aarch32_QSUB8_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x162000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_qsub8_a1_a_field_cond_2_poweroftwo_f0_262000f0() {
    // Encoding: 0x262000F0
    // Test aarch32_QSUB8_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=2, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x262000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_qsub8_a1_a_field_cond_3_poweroftwo_f0_362000f0() {
    // Encoding: 0x362000F0
    // Test aarch32_QSUB8_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=3, Rn=0
    let encoding: u32 = 0x362000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_qsub8_a1_a_field_cond_4_poweroftwo_f0_462000f0() {
    // Encoding: 0x462000F0
    // Test aarch32_QSUB8_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=4, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x462000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_qsub8_a1_a_field_cond_5_poweroftwo_f0_562000f0() {
    // Encoding: 0x562000F0
    // Test aarch32_QSUB8_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=5
    let encoding: u32 = 0x562000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_qsub8_a1_a_field_cond_6_poweroftwo_f0_662000f0() {
    // Encoding: 0x662000F0
    // Test aarch32_QSUB8_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=6, Rn=0, Rm=0
    let encoding: u32 = 0x662000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_qsub8_a1_a_field_cond_7_poweroftwo_f0_762000f0() {
    // Encoding: 0x762000F0
    // Test aarch32_QSUB8_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=7
    let encoding: u32 = 0x762000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_qsub8_a1_a_field_cond_8_poweroftwo_f0_862000f0() {
    // Encoding: 0x862000F0
    // Test aarch32_QSUB8_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=8, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x862000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_qsub8_a1_a_field_cond_9_poweroftwo_f0_962000f0() {
    // Encoding: 0x962000F0
    // Test aarch32_QSUB8_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=9, Rn=0
    let encoding: u32 = 0x962000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_qsub8_a1_a_field_cond_10_poweroftwo_f0_a62000f0() {
    // Encoding: 0xA62000F0
    // Test aarch32_QSUB8_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=10, Rm=0
    let encoding: u32 = 0xA62000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_qsub8_a1_a_field_cond_11_poweroftwo_f0_b62000f0() {
    // Encoding: 0xB62000F0
    // Test aarch32_QSUB8_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=11
    let encoding: u32 = 0xB62000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_qsub8_a1_a_field_cond_12_poweroftwo_f0_c62000f0() {
    // Encoding: 0xC62000F0
    // Test aarch32_QSUB8_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=12, Rn=0, Rd=0
    let encoding: u32 = 0xC62000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_qsub8_a1_a_field_cond_13_poweroftwo_f0_d62000f0() {
    // Encoding: 0xD62000F0
    // Test aarch32_QSUB8_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=13
    let encoding: u32 = 0xD62000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_qsub8_a1_a_field_cond_14_poweroftwo_f0_e62000f0() {
    // Encoding: 0xE62000F0
    // Test aarch32_QSUB8_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0xE62000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_qsub8_a1_a_field_cond_15_max_f0_f62000f0() {
    // Encoding: 0xF62000F0
    // Test aarch32_QSUB8_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=15, Rm=0
    let encoding: u32 = 0xF62000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qsub8_a1_a_field_rn_0_min_f0_062000f0() {
    // Encoding: 0x062000F0
    // Test aarch32_QSUB8_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=0, Rm=0
    let encoding: u32 = 0x062000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qsub8_a1_a_field_rn_1_poweroftwo_f0_062100f0() {
    // Encoding: 0x062100F0
    // Test aarch32_QSUB8_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=1, cond=0, Rd=0, Rm=0
    let encoding: u32 = 0x062100F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qsub8_a1_a_field_rd_0_min_f0_062000f0() {
    // Encoding: 0x062000F0
    // Test aarch32_QSUB8_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=0
    let encoding: u32 = 0x062000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qsub8_a1_a_field_rd_1_poweroftwo_f0_062010f0() {
    // Encoding: 0x062010F0
    // Test aarch32_QSUB8_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=1, Rm=0, Rn=0
    let encoding: u32 = 0x062010F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qsub8_a1_a_field_rm_0_min_f0_062000f0() {
    // Encoding: 0x062000F0
    // Test aarch32_QSUB8_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x062000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qsub8_a1_a_field_rm_1_poweroftwo_f0_062000f1() {
    // Encoding: 0x062000F1
    // Test aarch32_QSUB8_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=1, cond=0
    let encoding: u32 = 0x062000F1;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_qsub8_a1_a_combo_0_f0_062000f0() {
    // Encoding: 0x062000F0
    // Test aarch32_QSUB8_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=0, Rd=0
    let encoding: u32 = 0x062000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_qsub8_a1_a_special_cond_0_condition_eq_240_062000f0() {
    // Encoding: 0x062000F0
    // Test aarch32_QSUB8_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x062000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_qsub8_a1_a_special_cond_1_condition_ne_240_162000f0() {
    // Encoding: 0x162000F0
    // Test aarch32_QSUB8_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x162000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_qsub8_a1_a_special_cond_2_condition_cs_hs_240_262000f0() {
    // Encoding: 0x262000F0
    // Test aarch32_QSUB8_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=2
    let encoding: u32 = 0x262000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_qsub8_a1_a_special_cond_3_condition_cc_lo_240_362000f0() {
    // Encoding: 0x362000F0
    // Test aarch32_QSUB8_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=3, Rd=0
    let encoding: u32 = 0x362000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_qsub8_a1_a_special_cond_4_condition_mi_240_462000f0() {
    // Encoding: 0x462000F0
    // Test aarch32_QSUB8_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=4
    let encoding: u32 = 0x462000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_qsub8_a1_a_special_cond_5_condition_pl_240_562000f0() {
    // Encoding: 0x562000F0
    // Test aarch32_QSUB8_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=5, Rd=0
    let encoding: u32 = 0x562000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_qsub8_a1_a_special_cond_6_condition_vs_240_662000f0() {
    // Encoding: 0x662000F0
    // Test aarch32_QSUB8_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=6
    let encoding: u32 = 0x662000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_qsub8_a1_a_special_cond_7_condition_vc_240_762000f0() {
    // Encoding: 0x762000F0
    // Test aarch32_QSUB8_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=7, Rm=0
    let encoding: u32 = 0x762000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_qsub8_a1_a_special_cond_8_condition_hi_240_862000f0() {
    // Encoding: 0x862000F0
    // Test aarch32_QSUB8_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x862000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_qsub8_a1_a_special_cond_9_condition_ls_240_962000f0() {
    // Encoding: 0x962000F0
    // Test aarch32_QSUB8_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=9
    let encoding: u32 = 0x962000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_qsub8_a1_a_special_cond_10_condition_ge_240_a62000f0() {
    // Encoding: 0xA62000F0
    // Test aarch32_QSUB8_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rm=0, cond=10, Rn=0, Rd=0
    let encoding: u32 = 0xA62000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_qsub8_a1_a_special_cond_11_condition_lt_240_b62000f0() {
    // Encoding: 0xB62000F0
    // Test aarch32_QSUB8_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xB62000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_qsub8_a1_a_special_cond_12_condition_gt_240_c62000f0() {
    // Encoding: 0xC62000F0
    // Test aarch32_QSUB8_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=12
    let encoding: u32 = 0xC62000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_qsub8_a1_a_special_cond_13_condition_le_240_d62000f0() {
    // Encoding: 0xD62000F0
    // Test aarch32_QSUB8_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=13
    let encoding: u32 = 0xD62000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_qsub8_a1_a_special_cond_14_condition_al_240_e62000f0() {
    // Encoding: 0xE62000F0
    // Test aarch32_QSUB8_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=14, Rm=0
    let encoding: u32 = 0xE62000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_qsub8_a1_a_special_cond_15_condition_nv_240_f62000f0() {
    // Encoding: 0xF62000F0
    // Test aarch32_QSUB8_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xF62000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qsub8_a1_a_invalid_0_f0_062000f0() {
    // Encoding: 0x062000F0
    // Test aarch32_QSUB8_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=0, Rm=0
    let encoding: u32 = 0x062000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_QSUB8_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qsub8_a1_a_invalid_1_f0_062000f0() {
    // Encoding: 0x062000F0
    // Test aarch32_QSUB8_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x062000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_QSUB8_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qsub8_t1_a_field_rn_0_min_f010_fac0f010() {
    // Thumb encoding (32): 0xFAC0F010
    // Test aarch32_QSUB8_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSUB8_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qsub8_t1_a_field_rn_1_poweroftwo_f010_fac1f010() {
    // Thumb encoding (32): 0xFAC1F010
    // Test aarch32_QSUB8_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC1F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSUB8_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qsub8_t1_a_field_rd_0_min_f010_fac0f010() {
    // Thumb encoding (32): 0xFAC0F010
    // Test aarch32_QSUB8_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSUB8_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qsub8_t1_a_field_rd_1_poweroftwo_f010_fac0f110() {
    // Thumb encoding (32): 0xFAC0F110
    // Test aarch32_QSUB8_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=1, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F110;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSUB8_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qsub8_t1_a_field_rm_0_min_f010_fac0f010() {
    // Thumb encoding (32): 0xFAC0F010
    // Test aarch32_QSUB8_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSUB8_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qsub8_t1_a_field_rm_1_poweroftwo_f010_fac0f011() {
    // Thumb encoding (32): 0xFAC0F011
    // Test aarch32_QSUB8_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F011;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSUB8_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_qsub8_t1_a_combo_0_f010_fac0f010() {
    // Thumb encoding (32): 0xFAC0F010
    // Test aarch32_QSUB8_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QSUB8_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qsub8_t1_a_invalid_0_f010_fac0f010() {
    // Thumb encoding (32): 0xFAC0F010
    // Test aarch32_QSUB8_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F010;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_QSUB8_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qsub8_t1_a_invalid_1_f010_fac0f010() {
    // Thumb encoding (32): 0xFAC0F010
    // Test aarch32_QSUB8_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F010;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_QASX_A Tests
// ============================================================================

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_qasx_a1_a_field_cond_0_min_30_06200030() {
    // Encoding: 0x06200030
    // Test aarch32_QASX_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x06200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_qasx_a1_a_field_cond_1_poweroftwo_30_16200030() {
    // Encoding: 0x16200030
    // Test aarch32_QASX_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=1, Rd=0, Rn=0
    let encoding: u32 = 0x16200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_qasx_a1_a_field_cond_2_poweroftwo_30_26200030() {
    // Encoding: 0x26200030
    // Test aarch32_QASX_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=2, Rn=0, Rm=0
    let encoding: u32 = 0x26200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_qasx_a1_a_field_cond_3_poweroftwo_30_36200030() {
    // Encoding: 0x36200030
    // Test aarch32_QASX_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x36200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_qasx_a1_a_field_cond_4_poweroftwo_30_46200030() {
    // Encoding: 0x46200030
    // Test aarch32_QASX_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=4, Rm=0
    let encoding: u32 = 0x46200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_qasx_a1_a_field_cond_5_poweroftwo_30_56200030() {
    // Encoding: 0x56200030
    // Test aarch32_QASX_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=5, Rn=0
    let encoding: u32 = 0x56200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_qasx_a1_a_field_cond_6_poweroftwo_30_66200030() {
    // Encoding: 0x66200030
    // Test aarch32_QASX_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=6
    let encoding: u32 = 0x66200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_qasx_a1_a_field_cond_7_poweroftwo_30_76200030() {
    // Encoding: 0x76200030
    // Test aarch32_QASX_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=7, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x76200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_qasx_a1_a_field_cond_8_poweroftwo_30_86200030() {
    // Encoding: 0x86200030
    // Test aarch32_QASX_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=8, Rd=0
    let encoding: u32 = 0x86200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_qasx_a1_a_field_cond_9_poweroftwo_30_96200030() {
    // Encoding: 0x96200030
    // Test aarch32_QASX_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x96200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_qasx_a1_a_field_cond_10_poweroftwo_30_a6200030() {
    // Encoding: 0xA6200030
    // Test aarch32_QASX_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=10
    let encoding: u32 = 0xA6200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_qasx_a1_a_field_cond_11_poweroftwo_30_b6200030() {
    // Encoding: 0xB6200030
    // Test aarch32_QASX_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=11, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0xB6200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_qasx_a1_a_field_cond_12_poweroftwo_30_c6200030() {
    // Encoding: 0xC6200030
    // Test aarch32_QASX_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=12
    let encoding: u32 = 0xC6200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_qasx_a1_a_field_cond_13_poweroftwo_30_d6200030() {
    // Encoding: 0xD6200030
    // Test aarch32_QASX_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=13, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xD6200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_qasx_a1_a_field_cond_14_poweroftwo_30_e6200030() {
    // Encoding: 0xE6200030
    // Test aarch32_QASX_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=14, Rd=0, Rm=0
    let encoding: u32 = 0xE6200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_qasx_a1_a_field_cond_15_max_30_f6200030() {
    // Encoding: 0xF6200030
    // Test aarch32_QASX_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xF6200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qasx_a1_a_field_rn_0_min_30_06200030() {
    // Encoding: 0x06200030
    // Test aarch32_QASX_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x06200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qasx_a1_a_field_rn_1_poweroftwo_30_06210030() {
    // Encoding: 0x06210030
    // Test aarch32_QASX_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=1, cond=0, Rd=0, Rm=0
    let encoding: u32 = 0x06210030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qasx_a1_a_field_rd_0_min_30_06200030() {
    // Encoding: 0x06200030
    // Test aarch32_QASX_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, cond=0, Rm=0, Rn=0
    let encoding: u32 = 0x06200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qasx_a1_a_field_rd_1_poweroftwo_30_06201030() {
    // Encoding: 0x06201030
    // Test aarch32_QASX_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rm=0, Rd=1
    let encoding: u32 = 0x06201030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qasx_a1_a_field_rm_0_min_30_06200030() {
    // Encoding: 0x06200030
    // Test aarch32_QASX_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x06200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qasx_a1_a_field_rm_1_poweroftwo_30_06200031() {
    // Encoding: 0x06200031
    // Test aarch32_QASX_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=1, cond=0, Rn=0
    let encoding: u32 = 0x06200031;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_qasx_a1_a_combo_0_30_06200030() {
    // Encoding: 0x06200030
    // Test aarch32_QASX_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rn=0, cond=0, Rm=0, Rd=0
    let encoding: u32 = 0x06200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_qasx_a1_a_special_cond_0_condition_eq_48_06200030() {
    // Encoding: 0x06200030
    // Test aarch32_QASX_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x06200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_qasx_a1_a_special_cond_1_condition_ne_48_16200030() {
    // Encoding: 0x16200030
    // Test aarch32_QASX_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rd=0, cond=1, Rm=0, Rn=0
    let encoding: u32 = 0x16200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_qasx_a1_a_special_cond_2_condition_cs_hs_48_26200030() {
    // Encoding: 0x26200030
    // Test aarch32_QASX_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x26200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_qasx_a1_a_special_cond_3_condition_cc_lo_48_36200030() {
    // Encoding: 0x36200030
    // Test aarch32_QASX_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=3
    let encoding: u32 = 0x36200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_qasx_a1_a_special_cond_4_condition_mi_48_46200030() {
    // Encoding: 0x46200030
    // Test aarch32_QASX_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rm=0, cond=4, Rd=0, Rn=0
    let encoding: u32 = 0x46200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_qasx_a1_a_special_cond_5_condition_pl_48_56200030() {
    // Encoding: 0x56200030
    // Test aarch32_QASX_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=5, Rn=0
    let encoding: u32 = 0x56200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_qasx_a1_a_special_cond_6_condition_vs_48_66200030() {
    // Encoding: 0x66200030
    // Test aarch32_QASX_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=6
    let encoding: u32 = 0x66200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_qasx_a1_a_special_cond_7_condition_vc_48_76200030() {
    // Encoding: 0x76200030
    // Test aarch32_QASX_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=7
    let encoding: u32 = 0x76200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_qasx_a1_a_special_cond_8_condition_hi_48_86200030() {
    // Encoding: 0x86200030
    // Test aarch32_QASX_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=8
    let encoding: u32 = 0x86200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_qasx_a1_a_special_cond_9_condition_ls_48_96200030() {
    // Encoding: 0x96200030
    // Test aarch32_QASX_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rd=0, cond=9, Rn=0, Rm=0
    let encoding: u32 = 0x96200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_qasx_a1_a_special_cond_10_condition_ge_48_a6200030() {
    // Encoding: 0xA6200030
    // Test aarch32_QASX_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rd=0, cond=10, Rn=0, Rm=0
    let encoding: u32 = 0xA6200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_qasx_a1_a_special_cond_11_condition_lt_48_b6200030() {
    // Encoding: 0xB6200030
    // Test aarch32_QASX_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0xB6200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_qasx_a1_a_special_cond_12_condition_gt_48_c6200030() {
    // Encoding: 0xC6200030
    // Test aarch32_QASX_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: cond=12, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xC6200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_qasx_a1_a_special_cond_13_condition_le_48_d6200030() {
    // Encoding: 0xD6200030
    // Test aarch32_QASX_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rd=0, cond=13, Rn=0, Rm=0
    let encoding: u32 = 0xD6200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_qasx_a1_a_special_cond_14_condition_al_48_e6200030() {
    // Encoding: 0xE6200030
    // Test aarch32_QASX_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: cond=14, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0xE6200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_qasx_a1_a_special_cond_15_condition_nv_48_f6200030() {
    // Encoding: 0xF6200030
    // Test aarch32_QASX_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=15
    let encoding: u32 = 0xF6200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qasx_a1_a_invalid_0_30_06200030() {
    // Encoding: 0x06200030
    // Test aarch32_QASX_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x06200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_QASX_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qasx_a1_a_invalid_1_30_06200030() {
    // Encoding: 0x06200030
    // Test aarch32_QASX_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=0
    let encoding: u32 = 0x06200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_QASX_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qasx_t1_a_field_rn_0_min_f010_faa0f010() {
    // Thumb encoding (32): 0xFAA0F010
    // Test aarch32_QASX_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QASX_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qasx_t1_a_field_rn_1_poweroftwo_f010_faa1f010() {
    // Thumb encoding (32): 0xFAA1F010
    // Test aarch32_QASX_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rn=1, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA1F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QASX_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qasx_t1_a_field_rd_0_min_f010_faa0f010() {
    // Thumb encoding (32): 0xFAA0F010
    // Test aarch32_QASX_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QASX_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qasx_t1_a_field_rd_1_poweroftwo_f010_faa0f110() {
    // Thumb encoding (32): 0xFAA0F110
    // Test aarch32_QASX_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rd=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F110;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QASX_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_qasx_t1_a_field_rm_0_min_f010_faa0f010() {
    // Thumb encoding (32): 0xFAA0F010
    // Test aarch32_QASX_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QASX_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_qasx_t1_a_field_rm_1_poweroftwo_f010_faa0f011() {
    // Thumb encoding (32): 0xFAA0F011
    // Test aarch32_QASX_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F011;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QASX_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_qasx_t1_a_combo_0_f010_faa0f010() {
    // Thumb encoding (32): 0xFAA0F010
    // Test aarch32_QASX_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_QASX_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qasx_t1_a_invalid_0_f010_faa0f010() {
    // Thumb encoding (32): 0xFAA0F010
    // Test aarch32_QASX_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F010;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_QASX_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_qasx_t1_a_invalid_1_f010_faa0f010() {
    // Thumb encoding (32): 0xFAA0F010
    // Test aarch32_QASX_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F010;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_UQADD8_A Tests
// ============================================================================

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_uqadd8_a1_a_field_cond_0_min_90_06600090() {
    // Encoding: 0x06600090
    // Test aarch32_UQADD8_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=0, Rd=0
    let encoding: u32 = 0x06600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_uqadd8_a1_a_field_cond_1_poweroftwo_90_16600090() {
    // Encoding: 0x16600090
    // Test aarch32_UQADD8_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=1
    let encoding: u32 = 0x16600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_uqadd8_a1_a_field_cond_2_poweroftwo_90_26600090() {
    // Encoding: 0x26600090
    // Test aarch32_UQADD8_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=2, Rn=0, Rm=0
    let encoding: u32 = 0x26600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_uqadd8_a1_a_field_cond_3_poweroftwo_90_36600090() {
    // Encoding: 0x36600090
    // Test aarch32_UQADD8_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x36600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_uqadd8_a1_a_field_cond_4_poweroftwo_90_46600090() {
    // Encoding: 0x46600090
    // Test aarch32_UQADD8_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=4
    let encoding: u32 = 0x46600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_uqadd8_a1_a_field_cond_5_poweroftwo_90_56600090() {
    // Encoding: 0x56600090
    // Test aarch32_UQADD8_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=5, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x56600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_uqadd8_a1_a_field_cond_6_poweroftwo_90_66600090() {
    // Encoding: 0x66600090
    // Test aarch32_UQADD8_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x66600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_uqadd8_a1_a_field_cond_7_poweroftwo_90_76600090() {
    // Encoding: 0x76600090
    // Test aarch32_UQADD8_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=7, Rn=0
    let encoding: u32 = 0x76600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_uqadd8_a1_a_field_cond_8_poweroftwo_90_86600090() {
    // Encoding: 0x86600090
    // Test aarch32_UQADD8_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=8, Rn=0, Rm=0
    let encoding: u32 = 0x86600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_uqadd8_a1_a_field_cond_9_poweroftwo_90_96600090() {
    // Encoding: 0x96600090
    // Test aarch32_UQADD8_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=9
    let encoding: u32 = 0x96600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_uqadd8_a1_a_field_cond_10_poweroftwo_90_a6600090() {
    // Encoding: 0xA6600090
    // Test aarch32_UQADD8_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=10, Rn=0, Rd=0
    let encoding: u32 = 0xA6600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_uqadd8_a1_a_field_cond_11_poweroftwo_90_b6600090() {
    // Encoding: 0xB6600090
    // Test aarch32_UQADD8_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=11, Rd=0
    let encoding: u32 = 0xB6600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_uqadd8_a1_a_field_cond_12_poweroftwo_90_c6600090() {
    // Encoding: 0xC6600090
    // Test aarch32_UQADD8_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=12, Rm=0
    let encoding: u32 = 0xC6600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_uqadd8_a1_a_field_cond_13_poweroftwo_90_d6600090() {
    // Encoding: 0xD6600090
    // Test aarch32_UQADD8_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=13, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xD6600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_uqadd8_a1_a_field_cond_14_poweroftwo_90_e6600090() {
    // Encoding: 0xE6600090
    // Test aarch32_UQADD8_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xE6600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_uqadd8_a1_a_field_cond_15_max_90_f6600090() {
    // Encoding: 0xF6600090
    // Test aarch32_UQADD8_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rm=0, cond=15, Rd=0, Rn=0
    let encoding: u32 = 0xF6600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqadd8_a1_a_field_rn_0_min_90_06600090() {
    // Encoding: 0x06600090
    // Test aarch32_UQADD8_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x06600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqadd8_a1_a_field_rn_1_poweroftwo_90_06610090() {
    // Encoding: 0x06610090
    // Test aarch32_UQADD8_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=1, cond=0, Rm=0
    let encoding: u32 = 0x06610090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqadd8_a1_a_field_rd_0_min_90_06600090() {
    // Encoding: 0x06600090
    // Test aarch32_UQADD8_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x06600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqadd8_a1_a_field_rd_1_poweroftwo_90_06601090() {
    // Encoding: 0x06601090
    // Test aarch32_UQADD8_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=1, cond=0, Rm=0
    let encoding: u32 = 0x06601090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqadd8_a1_a_field_rm_0_min_90_06600090() {
    // Encoding: 0x06600090
    // Test aarch32_UQADD8_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=0, Rd=0
    let encoding: u32 = 0x06600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqadd8_a1_a_field_rm_1_poweroftwo_90_06600091() {
    // Encoding: 0x06600091
    // Test aarch32_UQADD8_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=1, cond=0, Rn=0
    let encoding: u32 = 0x06600091;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_uqadd8_a1_a_combo_0_90_06600090() {
    // Encoding: 0x06600090
    // Test aarch32_UQADD8_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: cond=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x06600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_uqadd8_a1_a_special_cond_0_condition_eq_144_06600090() {
    // Encoding: 0x06600090
    // Test aarch32_UQADD8_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=0, Rm=0
    let encoding: u32 = 0x06600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_uqadd8_a1_a_special_cond_1_condition_ne_144_16600090() {
    // Encoding: 0x16600090
    // Test aarch32_UQADD8_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=1, Rm=0
    let encoding: u32 = 0x16600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_uqadd8_a1_a_special_cond_2_condition_cs_hs_144_26600090() {
    // Encoding: 0x26600090
    // Test aarch32_UQADD8_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=2, Rm=0
    let encoding: u32 = 0x26600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_uqadd8_a1_a_special_cond_3_condition_cc_lo_144_36600090() {
    // Encoding: 0x36600090
    // Test aarch32_UQADD8_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rm=0, cond=3, Rn=0, Rd=0
    let encoding: u32 = 0x36600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_uqadd8_a1_a_special_cond_4_condition_mi_144_46600090() {
    // Encoding: 0x46600090
    // Test aarch32_UQADD8_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x46600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_uqadd8_a1_a_special_cond_5_condition_pl_144_56600090() {
    // Encoding: 0x56600090
    // Test aarch32_UQADD8_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rn=0, cond=5, Rd=0, Rm=0
    let encoding: u32 = 0x56600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_uqadd8_a1_a_special_cond_6_condition_vs_144_66600090() {
    // Encoding: 0x66600090
    // Test aarch32_UQADD8_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x66600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_uqadd8_a1_a_special_cond_7_condition_vc_144_76600090() {
    // Encoding: 0x76600090
    // Test aarch32_UQADD8_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rd=0, cond=7, Rn=0, Rm=0
    let encoding: u32 = 0x76600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_uqadd8_a1_a_special_cond_8_condition_hi_144_86600090() {
    // Encoding: 0x86600090
    // Test aarch32_UQADD8_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=8, Rn=0
    let encoding: u32 = 0x86600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_uqadd8_a1_a_special_cond_9_condition_ls_144_96600090() {
    // Encoding: 0x96600090
    // Test aarch32_UQADD8_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rm=0, cond=9, Rn=0, Rd=0
    let encoding: u32 = 0x96600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_uqadd8_a1_a_special_cond_10_condition_ge_144_a6600090() {
    // Encoding: 0xA6600090
    // Test aarch32_UQADD8_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=10
    let encoding: u32 = 0xA6600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_uqadd8_a1_a_special_cond_11_condition_lt_144_b6600090() {
    // Encoding: 0xB6600090
    // Test aarch32_UQADD8_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0xB6600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_uqadd8_a1_a_special_cond_12_condition_gt_144_c6600090() {
    // Encoding: 0xC6600090
    // Test aarch32_UQADD8_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rm=0, cond=12, Rn=0, Rd=0
    let encoding: u32 = 0xC6600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_uqadd8_a1_a_special_cond_13_condition_le_144_d6600090() {
    // Encoding: 0xD6600090
    // Test aarch32_UQADD8_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rd=0, cond=13, Rn=0, Rm=0
    let encoding: u32 = 0xD6600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_uqadd8_a1_a_special_cond_14_condition_al_144_e6600090() {
    // Encoding: 0xE6600090
    // Test aarch32_UQADD8_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: cond=14, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0xE6600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_uqadd8_a1_a_special_cond_15_condition_nv_144_f6600090() {
    // Encoding: 0xF6600090
    // Test aarch32_UQADD8_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0xF6600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uqadd8_a1_a_invalid_0_90_06600090() {
    // Encoding: 0x06600090
    // Test aarch32_UQADD8_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rm=0, cond=0, Rd=0, Rn=0
    let encoding: u32 = 0x06600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UQADD8_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uqadd8_a1_a_invalid_1_90_06600090() {
    // Encoding: 0x06600090
    // Test aarch32_UQADD8_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=0
    let encoding: u32 = 0x06600090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UQADD8_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqadd8_t1_a_field_rn_0_min_f050_fa80f050() {
    // Thumb encoding (32): 0xFA80F050
    // Test aarch32_UQADD8_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQADD8_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqadd8_t1_a_field_rn_1_poweroftwo_f050_fa81f050() {
    // Thumb encoding (32): 0xFA81F050
    // Test aarch32_UQADD8_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rn=1, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA81F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQADD8_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqadd8_t1_a_field_rd_0_min_f050_fa80f050() {
    // Thumb encoding (32): 0xFA80F050
    // Test aarch32_UQADD8_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQADD8_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqadd8_t1_a_field_rd_1_poweroftwo_f050_fa80f150() {
    // Thumb encoding (32): 0xFA80F150
    // Test aarch32_UQADD8_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rd=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F150;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQADD8_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqadd8_t1_a_field_rm_0_min_f050_fa80f050() {
    // Thumb encoding (32): 0xFA80F050
    // Test aarch32_UQADD8_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQADD8_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqadd8_t1_a_field_rm_1_poweroftwo_f050_fa80f051() {
    // Thumb encoding (32): 0xFA80F051
    // Test aarch32_UQADD8_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F051;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQADD8_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_uqadd8_t1_a_combo_0_f050_fa80f050() {
    // Thumb encoding (32): 0xFA80F050
    // Test aarch32_UQADD8_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQADD8_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uqadd8_t1_a_invalid_0_f050_fa80f050() {
    // Thumb encoding (32): 0xFA80F050
    // Test aarch32_UQADD8_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F050;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_UQADD8_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uqadd8_t1_a_invalid_1_f050_fa80f050() {
    // Thumb encoding (32): 0xFA80F050
    // Test aarch32_UQADD8_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F050;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_UQSUB16_A Tests
// ============================================================================

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_uqsub16_a1_a_field_cond_0_min_70_06600070() {
    // Encoding: 0x06600070
    // Test aarch32_UQSUB16_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=0
    let encoding: u32 = 0x06600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_uqsub16_a1_a_field_cond_1_poweroftwo_70_16600070() {
    // Encoding: 0x16600070
    // Test aarch32_UQSUB16_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x16600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_uqsub16_a1_a_field_cond_2_poweroftwo_70_26600070() {
    // Encoding: 0x26600070
    // Test aarch32_UQSUB16_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=2, Rd=0
    let encoding: u32 = 0x26600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_uqsub16_a1_a_field_cond_3_poweroftwo_70_36600070() {
    // Encoding: 0x36600070
    // Test aarch32_UQSUB16_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=3, Rd=0
    let encoding: u32 = 0x36600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_uqsub16_a1_a_field_cond_4_poweroftwo_70_46600070() {
    // Encoding: 0x46600070
    // Test aarch32_UQSUB16_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=4, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x46600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_uqsub16_a1_a_field_cond_5_poweroftwo_70_56600070() {
    // Encoding: 0x56600070
    // Test aarch32_UQSUB16_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=5, Rn=0
    let encoding: u32 = 0x56600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_uqsub16_a1_a_field_cond_6_poweroftwo_70_66600070() {
    // Encoding: 0x66600070
    // Test aarch32_UQSUB16_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x66600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_uqsub16_a1_a_field_cond_7_poweroftwo_70_76600070() {
    // Encoding: 0x76600070
    // Test aarch32_UQSUB16_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=7, Rn=0, Rd=0
    let encoding: u32 = 0x76600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_uqsub16_a1_a_field_cond_8_poweroftwo_70_86600070() {
    // Encoding: 0x86600070
    // Test aarch32_UQSUB16_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=8, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x86600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_uqsub16_a1_a_field_cond_9_poweroftwo_70_96600070() {
    // Encoding: 0x96600070
    // Test aarch32_UQSUB16_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=9
    let encoding: u32 = 0x96600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_uqsub16_a1_a_field_cond_10_poweroftwo_70_a6600070() {
    // Encoding: 0xA6600070
    // Test aarch32_UQSUB16_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=10, Rn=0, Rm=0
    let encoding: u32 = 0xA6600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_uqsub16_a1_a_field_cond_11_poweroftwo_70_b6600070() {
    // Encoding: 0xB6600070
    // Test aarch32_UQSUB16_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=11
    let encoding: u32 = 0xB6600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_uqsub16_a1_a_field_cond_12_poweroftwo_70_c6600070() {
    // Encoding: 0xC6600070
    // Test aarch32_UQSUB16_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0xC6600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_uqsub16_a1_a_field_cond_13_poweroftwo_70_d6600070() {
    // Encoding: 0xD6600070
    // Test aarch32_UQSUB16_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=13, Rm=0
    let encoding: u32 = 0xD6600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_uqsub16_a1_a_field_cond_14_poweroftwo_70_e6600070() {
    // Encoding: 0xE6600070
    // Test aarch32_UQSUB16_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=14
    let encoding: u32 = 0xE6600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_uqsub16_a1_a_field_cond_15_max_70_f6600070() {
    // Encoding: 0xF6600070
    // Test aarch32_UQSUB16_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=15
    let encoding: u32 = 0xF6600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqsub16_a1_a_field_rn_0_min_70_06600070() {
    // Encoding: 0x06600070
    // Test aarch32_UQSUB16_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=0
    let encoding: u32 = 0x06600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqsub16_a1_a_field_rn_1_poweroftwo_70_06610070() {
    // Encoding: 0x06610070
    // Test aarch32_UQSUB16_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=1, Rm=0
    let encoding: u32 = 0x06610070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqsub16_a1_a_field_rd_0_min_70_06600070() {
    // Encoding: 0x06600070
    // Test aarch32_UQSUB16_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x06600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqsub16_a1_a_field_rd_1_poweroftwo_70_06601070() {
    // Encoding: 0x06601070
    // Test aarch32_UQSUB16_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=1, cond=0
    let encoding: u32 = 0x06601070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqsub16_a1_a_field_rm_0_min_70_06600070() {
    // Encoding: 0x06600070
    // Test aarch32_UQSUB16_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x06600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqsub16_a1_a_field_rm_1_poweroftwo_70_06600071() {
    // Encoding: 0x06600071
    // Test aarch32_UQSUB16_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, Rm=1
    let encoding: u32 = 0x06600071;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_uqsub16_a1_a_combo_0_70_06600070() {
    // Encoding: 0x06600070
    // Test aarch32_UQSUB16_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x06600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_uqsub16_a1_a_special_cond_0_condition_eq_112_06600070() {
    // Encoding: 0x06600070
    // Test aarch32_UQSUB16_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=0
    let encoding: u32 = 0x06600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_uqsub16_a1_a_special_cond_1_condition_ne_112_16600070() {
    // Encoding: 0x16600070
    // Test aarch32_UQSUB16_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=1, Rd=0
    let encoding: u32 = 0x16600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_uqsub16_a1_a_special_cond_2_condition_cs_hs_112_26600070() {
    // Encoding: 0x26600070
    // Test aarch32_UQSUB16_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=2, Rn=0
    let encoding: u32 = 0x26600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_uqsub16_a1_a_special_cond_3_condition_cc_lo_112_36600070() {
    // Encoding: 0x36600070
    // Test aarch32_UQSUB16_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=3, Rm=0
    let encoding: u32 = 0x36600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_uqsub16_a1_a_special_cond_4_condition_mi_112_46600070() {
    // Encoding: 0x46600070
    // Test aarch32_UQSUB16_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x46600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_uqsub16_a1_a_special_cond_5_condition_pl_112_56600070() {
    // Encoding: 0x56600070
    // Test aarch32_UQSUB16_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rd=0, cond=5, Rn=0, Rm=0
    let encoding: u32 = 0x56600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_uqsub16_a1_a_special_cond_6_condition_vs_112_66600070() {
    // Encoding: 0x66600070
    // Test aarch32_UQSUB16_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rn=0, cond=6, Rd=0, Rm=0
    let encoding: u32 = 0x66600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_uqsub16_a1_a_special_cond_7_condition_vc_112_76600070() {
    // Encoding: 0x76600070
    // Test aarch32_UQSUB16_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=7
    let encoding: u32 = 0x76600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_uqsub16_a1_a_special_cond_8_condition_hi_112_86600070() {
    // Encoding: 0x86600070
    // Test aarch32_UQSUB16_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rn=0, cond=8, Rd=0, Rm=0
    let encoding: u32 = 0x86600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_uqsub16_a1_a_special_cond_9_condition_ls_112_96600070() {
    // Encoding: 0x96600070
    // Test aarch32_UQSUB16_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=9
    let encoding: u32 = 0x96600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_uqsub16_a1_a_special_cond_10_condition_ge_112_a6600070() {
    // Encoding: 0xA6600070
    // Test aarch32_UQSUB16_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=10
    let encoding: u32 = 0xA6600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_uqsub16_a1_a_special_cond_11_condition_lt_112_b6600070() {
    // Encoding: 0xB6600070
    // Test aarch32_UQSUB16_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0xB6600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_uqsub16_a1_a_special_cond_12_condition_gt_112_c6600070() {
    // Encoding: 0xC6600070
    // Test aarch32_UQSUB16_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rn=0, cond=12, Rm=0, Rd=0
    let encoding: u32 = 0xC6600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_uqsub16_a1_a_special_cond_13_condition_le_112_d6600070() {
    // Encoding: 0xD6600070
    // Test aarch32_UQSUB16_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=13, Rn=0
    let encoding: u32 = 0xD6600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_uqsub16_a1_a_special_cond_14_condition_al_112_e6600070() {
    // Encoding: 0xE6600070
    // Test aarch32_UQSUB16_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: cond=14, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xE6600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_uqsub16_a1_a_special_cond_15_condition_nv_112_f6600070() {
    // Encoding: 0xF6600070
    // Test aarch32_UQSUB16_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=15
    let encoding: u32 = 0xF6600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uqsub16_a1_a_invalid_0_70_06600070() {
    // Encoding: 0x06600070
    // Test aarch32_UQSUB16_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rn=0, cond=0, Rd=0, Rm=0
    let encoding: u32 = 0x06600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UQSUB16_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uqsub16_a1_a_invalid_1_70_06600070() {
    // Encoding: 0x06600070
    // Test aarch32_UQSUB16_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rd=0, cond=0, Rn=0, Rm=0
    let encoding: u32 = 0x06600070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UQSUB16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqsub16_t1_a_field_rn_0_min_f050_fad0f050() {
    // Thumb encoding (32): 0xFAD0F050
    // Test aarch32_UQSUB16_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQSUB16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqsub16_t1_a_field_rn_1_poweroftwo_f050_fad1f050() {
    // Thumb encoding (32): 0xFAD1F050
    // Test aarch32_UQSUB16_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD1F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQSUB16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqsub16_t1_a_field_rd_0_min_f050_fad0f050() {
    // Thumb encoding (32): 0xFAD0F050
    // Test aarch32_UQSUB16_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQSUB16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqsub16_t1_a_field_rd_1_poweroftwo_f050_fad0f150() {
    // Thumb encoding (32): 0xFAD0F150
    // Test aarch32_UQSUB16_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rd=1, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F150;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQSUB16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uqsub16_t1_a_field_rm_0_min_f050_fad0f050() {
    // Thumb encoding (32): 0xFAD0F050
    // Test aarch32_UQSUB16_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQSUB16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uqsub16_t1_a_field_rm_1_poweroftwo_f050_fad0f051() {
    // Thumb encoding (32): 0xFAD0F051
    // Test aarch32_UQSUB16_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F051;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQSUB16_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_uqsub16_t1_a_combo_0_f050_fad0f050() {
    // Thumb encoding (32): 0xFAD0F050
    // Test aarch32_UQSUB16_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F050;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UQSUB16_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uqsub16_t1_a_invalid_0_f050_fad0f050() {
    // Thumb encoding (32): 0xFAD0F050
    // Test aarch32_UQSUB16_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F050;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_UQSUB16_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uqsub16_t1_a_invalid_1_f050_fad0f050() {
    // Thumb encoding (32): 0xFAD0F050
    // Test aarch32_UQSUB16_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F050;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}
