//! A32 data_processing extend tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers_32::*;

// ============================================================================
// aarch32_SXTB_A Tests
// ============================================================================

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_sxtb_a1_a_field_cond_0_min_70_06af0070() {
    // Encoding: 0x06AF0070
    // Test aarch32_SXTB_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=0, rotate=0
    let encoding: u32 = 0x06AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_sxtb_a1_a_field_cond_1_poweroftwo_70_16af0070() {
    // Encoding: 0x16AF0070
    // Test aarch32_SXTB_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, rotate=0, Rm=0, cond=1
    let encoding: u32 = 0x16AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_sxtb_a1_a_field_cond_2_poweroftwo_70_26af0070() {
    // Encoding: 0x26AF0070
    // Test aarch32_SXTB_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=2, rotate=0, Rm=0, Rd=0
    let encoding: u32 = 0x26AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_sxtb_a1_a_field_cond_3_poweroftwo_70_36af0070() {
    // Encoding: 0x36AF0070
    // Test aarch32_SXTB_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=3, rotate=0
    let encoding: u32 = 0x36AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_sxtb_a1_a_field_cond_4_poweroftwo_70_46af0070() {
    // Encoding: 0x46AF0070
    // Test aarch32_SXTB_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=4, rotate=0
    let encoding: u32 = 0x46AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_sxtb_a1_a_field_cond_5_poweroftwo_70_56af0070() {
    // Encoding: 0x56AF0070
    // Test aarch32_SXTB_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=5, rotate=0, Rd=0
    let encoding: u32 = 0x56AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_sxtb_a1_a_field_cond_6_poweroftwo_70_66af0070() {
    // Encoding: 0x66AF0070
    // Test aarch32_SXTB_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, rotate=0, Rm=0, Rd=0
    let encoding: u32 = 0x66AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_sxtb_a1_a_field_cond_7_poweroftwo_70_76af0070() {
    // Encoding: 0x76AF0070
    // Test aarch32_SXTB_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rd=0, Rm=0, cond=7
    let encoding: u32 = 0x76AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_sxtb_a1_a_field_cond_8_poweroftwo_70_86af0070() {
    // Encoding: 0x86AF0070
    // Test aarch32_SXTB_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, rotate=0, cond=8
    let encoding: u32 = 0x86AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_sxtb_a1_a_field_cond_9_poweroftwo_70_96af0070() {
    // Encoding: 0x96AF0070
    // Test aarch32_SXTB_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, rotate=0, cond=9
    let encoding: u32 = 0x96AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_sxtb_a1_a_field_cond_10_poweroftwo_70_a6af0070() {
    // Encoding: 0xA6AF0070
    // Test aarch32_SXTB_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, rotate=0, Rd=0, Rm=0
    let encoding: u32 = 0xA6AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_sxtb_a1_a_field_cond_11_poweroftwo_70_b6af0070() {
    // Encoding: 0xB6AF0070
    // Test aarch32_SXTB_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=11, Rd=0, rotate=0
    let encoding: u32 = 0xB6AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_sxtb_a1_a_field_cond_12_poweroftwo_70_c6af0070() {
    // Encoding: 0xC6AF0070
    // Test aarch32_SXTB_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=12, Rd=0, rotate=0
    let encoding: u32 = 0xC6AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_sxtb_a1_a_field_cond_13_poweroftwo_70_d6af0070() {
    // Encoding: 0xD6AF0070
    // Test aarch32_SXTB_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=13, Rm=0, Rd=0, rotate=0
    let encoding: u32 = 0xD6AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_sxtb_a1_a_field_cond_14_poweroftwo_70_e6af0070() {
    // Encoding: 0xE6AF0070
    // Test aarch32_SXTB_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, rotate=0, cond=14
    let encoding: u32 = 0xE6AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_sxtb_a1_a_field_cond_15_max_70_f6af0070() {
    // Encoding: 0xF6AF0070
    // Test aarch32_SXTB_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, rotate=0, Rm=0, Rd=0
    let encoding: u32 = 0xF6AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtb_a1_a_field_rd_0_min_70_06af0070() {
    // Encoding: 0x06AF0070
    // Test aarch32_SXTB_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, rotate=0, Rd=0, Rm=0
    let encoding: u32 = 0x06AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtb_a1_a_field_rd_1_poweroftwo_70_06af1070() {
    // Encoding: 0x06AF1070
    // Test aarch32_SXTB_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rm=0, cond=0, Rd=1
    let encoding: u32 = 0x06AF1070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_sxtb_a1_a_field_rotate_0_min_70_06af0070() {
    // Encoding: 0x06AF0070
    // Test aarch32_SXTB_A1_A field rotate = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rd=0, rotate=0, cond=0
    let encoding: u32 = 0x06AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_sxtb_a1_a_field_rotate_1_poweroftwo_70_06af0470() {
    // Encoding: 0x06AF0470
    // Test aarch32_SXTB_A1_A field rotate = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=1, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x06AF0470;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_sxtb_a1_a_field_rotate_3_max_70_06af0c70() {
    // Encoding: 0x06AF0C70
    // Test aarch32_SXTB_A1_A field rotate = 3 (Max)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=0, rotate=3
    let encoding: u32 = 0x06AF0C70;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtb_a1_a_field_rm_0_min_70_06af0070() {
    // Encoding: 0x06AF0070
    // Test aarch32_SXTB_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=0, rotate=0
    let encoding: u32 = 0x06AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtb_a1_a_field_rm_1_poweroftwo_70_06af0071() {
    // Encoding: 0x06AF0071
    // Test aarch32_SXTB_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=1, rotate=0
    let encoding: u32 = 0x06AF0071;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_sxtb_a1_a_combo_0_70_06af0070() {
    // Encoding: 0x06AF0070
    // Test aarch32_SXTB_A1_A field combination: cond=0, Rd=0, rotate=0, Rm=0
    // ISET: A32
    // Fields: cond=0, Rm=0, rotate=0, Rd=0
    let encoding: u32 = 0x06AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_sxtb_a1_a_special_cond_0_condition_eq_112_06af0070() {
    // Encoding: 0x06AF0070
    // Test aarch32_SXTB_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: rotate=0, Rm=0, Rd=0, cond=0
    let encoding: u32 = 0x06AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_sxtb_a1_a_special_cond_1_condition_ne_112_16af0070() {
    // Encoding: 0x16AF0070
    // Test aarch32_SXTB_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: rotate=0, Rm=0, cond=1, Rd=0
    let encoding: u32 = 0x16AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_sxtb_a1_a_special_cond_2_condition_cs_hs_112_26af0070() {
    // Encoding: 0x26AF0070
    // Test aarch32_SXTB_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, rotate=0, Rm=0, Rd=0
    let encoding: u32 = 0x26AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_sxtb_a1_a_special_cond_3_condition_cc_lo_112_36af0070() {
    // Encoding: 0x36AF0070
    // Test aarch32_SXTB_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, Rm=0, rotate=0, Rd=0
    let encoding: u32 = 0x36AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_sxtb_a1_a_special_cond_4_condition_mi_112_46af0070() {
    // Encoding: 0x46AF0070
    // Test aarch32_SXTB_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rd=0, rotate=0, cond=4, Rm=0
    let encoding: u32 = 0x46AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_sxtb_a1_a_special_cond_5_condition_pl_112_56af0070() {
    // Encoding: 0x56AF0070
    // Test aarch32_SXTB_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: rotate=0, cond=5, Rm=0, Rd=0
    let encoding: u32 = 0x56AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_sxtb_a1_a_special_cond_6_condition_vs_112_66af0070() {
    // Encoding: 0x66AF0070
    // Test aarch32_SXTB_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: rotate=0, cond=6, Rm=0, Rd=0
    let encoding: u32 = 0x66AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_sxtb_a1_a_special_cond_7_condition_vc_112_76af0070() {
    // Encoding: 0x76AF0070
    // Test aarch32_SXTB_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rm=0, cond=7, rotate=0, Rd=0
    let encoding: u32 = 0x76AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_sxtb_a1_a_special_cond_8_condition_hi_112_86af0070() {
    // Encoding: 0x86AF0070
    // Test aarch32_SXTB_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x86AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_sxtb_a1_a_special_cond_9_condition_ls_112_96af0070() {
    // Encoding: 0x96AF0070
    // Test aarch32_SXTB_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rm=0, cond=9, Rd=0, rotate=0
    let encoding: u32 = 0x96AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_sxtb_a1_a_special_cond_10_condition_ge_112_a6af0070() {
    // Encoding: 0xA6AF0070
    // Test aarch32_SXTB_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, rotate=0, Rm=0, Rd=0
    let encoding: u32 = 0xA6AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_sxtb_a1_a_special_cond_11_condition_lt_112_b6af0070() {
    // Encoding: 0xB6AF0070
    // Test aarch32_SXTB_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rd=0, rotate=0, cond=11, Rm=0
    let encoding: u32 = 0xB6AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_sxtb_a1_a_special_cond_12_condition_gt_112_c6af0070() {
    // Encoding: 0xC6AF0070
    // Test aarch32_SXTB_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rm=0, rotate=0, cond=12, Rd=0
    let encoding: u32 = 0xC6AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_sxtb_a1_a_special_cond_13_condition_le_112_d6af0070() {
    // Encoding: 0xD6AF0070
    // Test aarch32_SXTB_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: cond=13, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0xD6AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_sxtb_a1_a_special_cond_14_condition_al_112_e6af0070() {
    // Encoding: 0xE6AF0070
    // Test aarch32_SXTB_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: rotate=0, Rd=0, Rm=0, cond=14
    let encoding: u32 = 0xE6AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_sxtb_a1_a_special_cond_15_condition_nv_112_f6af0070() {
    // Encoding: 0xF6AF0070
    // Test aarch32_SXTB_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=15, rotate=0
    let encoding: u32 = 0xF6AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sxtb_a1_a_invalid_0_70_06af0070() {
    // Encoding: 0x06AF0070
    // Test aarch32_SXTB_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: cond=0, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x06AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SXTB_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sxtb_a1_a_invalid_1_70_06af0070() {
    // Encoding: 0x06AF0070
    // Test aarch32_SXTB_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rm=0, rotate=0, Rd=0, cond=0
    let encoding: u32 = 0x06AF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SXTB_T1_A
/// ASL: `field Rm 19 +: 3`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtb_t1_a_field_rm_0_min_0_b2400000() {
    // Thumb encoding (32): 0xB2400000
    // Test aarch32_SXTB_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB2400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTB_T1_A
/// ASL: `field Rm 19 +: 3`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtb_t1_a_field_rm_1_poweroftwo_0_b2480000() {
    // Thumb encoding (32): 0xB2480000
    // Test aarch32_SXTB_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB2480000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTB_T1_A
/// ASL: `field Rd 16 +: 3`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtb_t1_a_field_rd_0_min_0_b2400000() {
    // Thumb encoding (32): 0xB2400000
    // Test aarch32_SXTB_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB2400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTB_T1_A
/// ASL: `field Rd 16 +: 3`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtb_t1_a_field_rd_1_poweroftwo_0_b2410000() {
    // Thumb encoding (32): 0xB2410000
    // Test aarch32_SXTB_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=1, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB2410000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTB_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch32_sxtb_t1_a_combo_0_0_b2400000() {
    // Thumb encoding (32): 0xB2400000
    // Test aarch32_SXTB_T1_A field combination: Rm=0, Rd=0
    // ISET: T32
    // Fields: Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB2400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTB_T2_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtb_t2_a_field_rd_0_min_f080_fa4ff080() {
    // Thumb encoding (32): 0xFA4FF080
    // Test aarch32_SXTB_T2_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rd=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA4FF080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTB_T2_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtb_t2_a_field_rd_1_poweroftwo_f080_fa4ff180() {
    // Thumb encoding (32): 0xFA4FF180
    // Test aarch32_SXTB_T2_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rd=1, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA4FF180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTB_T2_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_sxtb_t2_a_field_rotate_0_min_f080_fa4ff080() {
    // Thumb encoding (32): 0xFA4FF080
    // Test aarch32_SXTB_T2_A field rotate = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, rotate=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA4FF080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTB_T2_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_sxtb_t2_a_field_rotate_1_poweroftwo_f080_fa4ff090() {
    // Thumb encoding (32): 0xFA4FF090
    // Test aarch32_SXTB_T2_A field rotate = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=0, rotate=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA4FF090;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTB_T2_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_sxtb_t2_a_field_rotate_3_max_f080_fa4ff0b0() {
    // Thumb encoding (32): 0xFA4FF0B0
    // Test aarch32_SXTB_T2_A field rotate = 3 (Max)
    // ISET: T32
    // Fields: Rm=0, Rd=0, rotate=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA4FF0B0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTB_T2_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtb_t2_a_field_rm_0_min_f080_fa4ff080() {
    // Thumb encoding (32): 0xFA4FF080
    // Test aarch32_SXTB_T2_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, rotate=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA4FF080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTB_T2_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtb_t2_a_field_rm_1_poweroftwo_f080_fa4ff081() {
    // Thumb encoding (32): 0xFA4FF081
    // Test aarch32_SXTB_T2_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: rotate=0, Rd=0, Rm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA4FF081;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTB_T2_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch32_sxtb_t2_a_combo_0_f080_fa4ff080() {
    // Thumb encoding (32): 0xFA4FF080
    // Test aarch32_SXTB_T2_A field combination: Rd=0, rotate=0, Rm=0
    // ISET: T32
    // Fields: rotate=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA4FF080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTB_T2_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sxtb_t2_a_invalid_0_f080_fa4ff080() {
    // Thumb encoding (32): 0xFA4FF080
    // Test aarch32_SXTB_T2_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: rotate=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA4FF080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_SXTB_T2_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sxtb_t2_a_invalid_1_f080_fa4ff080() {
    // Thumb encoding (32): 0xFA4FF080
    // Test aarch32_SXTB_T2_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rd=0, rotate=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA4FF080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_SXTB_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 0 (32)
#[test]
fn test_aarch32_sxtb_t1_a_lslv_oracle_32_0_b2420020() {
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

/// Provenance: aarch32_SXTB_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 0 (64)
#[test]
fn test_aarch32_sxtb_t1_a_lslv_oracle_64_0_b2420020() {
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

/// Provenance: aarch32_SXTB_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4 (32)
#[test]
fn test_aarch32_sxtb_t1_a_lslv_oracle_32_1_b2420020() {
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

/// Provenance: aarch32_SXTB_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 4 (64)
#[test]
fn test_aarch32_sxtb_t1_a_lslv_oracle_64_1_b2420020() {
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

/// Provenance: aarch32_SXTB_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 8 (32)
#[test]
fn test_aarch32_sxtb_t1_a_lslv_oracle_32_2_b2420020() {
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

/// Provenance: aarch32_SXTB_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 8 (64)
#[test]
fn test_aarch32_sxtb_t1_a_lslv_oracle_64_2_b2420020() {
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

/// Provenance: aarch32_SXTB_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1 (32)
#[test]
fn test_aarch32_sxtb_t1_a_lslv_oracle_32_3_b2420020() {
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

/// Provenance: aarch32_SXTB_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// MSB set, shift 1 (64)
#[test]
fn test_aarch32_sxtb_t1_a_lslv_oracle_64_3_b2420020() {
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

/// Provenance: aarch32_SXTB_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSB set, max shift (32)
#[test]
fn test_aarch32_sxtb_t1_a_lslv_oracle_32_4_b2420020() {
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

/// Provenance: aarch32_SXTB_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSB set, max shift (64)
#[test]
fn test_aarch32_sxtb_t1_a_lslv_oracle_64_4_b2420020() {
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

/// Provenance: aarch32_SXTB_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// all ones, shift 32 (32)
#[test]
fn test_aarch32_sxtb_t1_a_lslv_oracle_32_5_b2420020() {
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

/// Provenance: aarch32_SXTB_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all ones, shift 32 (64)
#[test]
fn test_aarch32_sxtb_t1_a_lslv_oracle_64_5_b2420020() {
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

/// Provenance: aarch32_SXTB_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift
#[test]
fn test_aarch32_sxtb_t1_a_t16_oracle_0_b2500000() {
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

/// Provenance: aarch32_SXTB_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4
#[test]
fn test_aarch32_sxtb_t1_a_t16_oracle_1_b2500000() {
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

/// Provenance: aarch32_SXTB_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1
#[test]
fn test_aarch32_sxtb_t1_a_t16_oracle_2_b2500000() {
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

/// Provenance: aarch32_SXTB_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift to MSB
#[test]
fn test_aarch32_sxtb_t1_a_t16_oracle_3_b2500000() {
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

// ============================================================================
// aarch32_UXTAH_A Tests
// ============================================================================

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_uxtah_a1_a_field_cond_0_min_70_06f00070() {
    // Encoding: 0x06F00070
    // Test aarch32_UXTAH_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, rotate=0, Rd=0
    let encoding: u32 = 0x06F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_uxtah_a1_a_field_cond_1_poweroftwo_70_16f00070() {
    // Encoding: 0x16F00070
    // Test aarch32_UXTAH_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, Rm=0, Rn=0, Rd=0, rotate=0
    let encoding: u32 = 0x16F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_uxtah_a1_a_field_cond_2_poweroftwo_70_26f00070() {
    // Encoding: 0x26F00070
    // Test aarch32_UXTAH_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=2, Rn=0, rotate=0, Rm=0
    let encoding: u32 = 0x26F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_uxtah_a1_a_field_cond_3_poweroftwo_70_36f00070() {
    // Encoding: 0x36F00070
    // Test aarch32_UXTAH_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=3, rotate=0, Rm=0
    let encoding: u32 = 0x36F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_uxtah_a1_a_field_cond_4_poweroftwo_70_46f00070() {
    // Encoding: 0x46F00070
    // Test aarch32_UXTAH_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rm=0, Rn=0, Rd=0, cond=4
    let encoding: u32 = 0x46F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_uxtah_a1_a_field_cond_5_poweroftwo_70_56f00070() {
    // Encoding: 0x56F00070
    // Test aarch32_UXTAH_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=5, rotate=0, Rn=0, Rm=0
    let encoding: u32 = 0x56F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_uxtah_a1_a_field_cond_6_poweroftwo_70_66f00070() {
    // Encoding: 0x66F00070
    // Test aarch32_UXTAH_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=6, Rn=0, rotate=0, Rd=0
    let encoding: u32 = 0x66F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_uxtah_a1_a_field_cond_7_poweroftwo_70_76f00070() {
    // Encoding: 0x76F00070
    // Test aarch32_UXTAH_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, rotate=0, cond=7
    let encoding: u32 = 0x76F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_uxtah_a1_a_field_cond_8_poweroftwo_70_86f00070() {
    // Encoding: 0x86F00070
    // Test aarch32_UXTAH_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=8, rotate=0
    let encoding: u32 = 0x86F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_uxtah_a1_a_field_cond_9_poweroftwo_70_96f00070() {
    // Encoding: 0x96F00070
    // Test aarch32_UXTAH_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=9, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x96F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_uxtah_a1_a_field_cond_10_poweroftwo_70_a6f00070() {
    // Encoding: 0xA6F00070
    // Test aarch32_UXTAH_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, rotate=0, Rn=0, cond=10, Rm=0
    let encoding: u32 = 0xA6F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_uxtah_a1_a_field_cond_11_poweroftwo_70_b6f00070() {
    // Encoding: 0xB6F00070
    // Test aarch32_UXTAH_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=11, Rn=0, rotate=0, Rm=0
    let encoding: u32 = 0xB6F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_uxtah_a1_a_field_cond_12_poweroftwo_70_c6f00070() {
    // Encoding: 0xC6F00070
    // Test aarch32_UXTAH_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, Rn=0, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0xC6F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_uxtah_a1_a_field_cond_13_poweroftwo_70_d6f00070() {
    // Encoding: 0xD6F00070
    // Test aarch32_UXTAH_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rd=0, cond=13, Rn=0, Rm=0
    let encoding: u32 = 0xD6F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_uxtah_a1_a_field_cond_14_poweroftwo_70_e6f00070() {
    // Encoding: 0xE6F00070
    // Test aarch32_UXTAH_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, rotate=0, Rd=0, cond=14, Rm=0
    let encoding: u32 = 0xE6F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_uxtah_a1_a_field_cond_15_max_70_f6f00070() {
    // Encoding: 0xF6F00070
    // Test aarch32_UXTAH_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, Rn=0, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0xF6F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtah_a1_a_field_rn_0_min_70_06f00070() {
    // Encoding: 0x06F00070
    // Test aarch32_UXTAH_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, rotate=0, cond=0, Rd=0, Rm=0
    let encoding: u32 = 0x06F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtah_a1_a_field_rn_1_poweroftwo_70_06f10070() {
    // Encoding: 0x06F10070
    // Test aarch32_UXTAH_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=1, cond=0, Rm=0, Rd=0, rotate=0
    let encoding: u32 = 0x06F10070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtah_a1_a_field_rd_0_min_70_06f00070() {
    // Encoding: 0x06F00070
    // Test aarch32_UXTAH_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rd=0, rotate=0, Rn=0
    let encoding: u32 = 0x06F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtah_a1_a_field_rd_1_poweroftwo_70_06f01070() {
    // Encoding: 0x06F01070
    // Test aarch32_UXTAH_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=1, cond=0, Rn=0, Rm=0, rotate=0
    let encoding: u32 = 0x06F01070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_uxtah_a1_a_field_rotate_0_min_70_06f00070() {
    // Encoding: 0x06F00070
    // Test aarch32_UXTAH_A1_A field rotate = 0 (Min)
    // ISET: A32
    // Fields: cond=0, rotate=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x06F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_uxtah_a1_a_field_rotate_1_poweroftwo_70_06f00470() {
    // Encoding: 0x06F00470
    // Test aarch32_UXTAH_A1_A field rotate = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rm=0, rotate=1, Rd=0, Rn=0
    let encoding: u32 = 0x06F00470;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_uxtah_a1_a_field_rotate_3_max_70_06f00c70() {
    // Encoding: 0x06F00C70
    // Test aarch32_UXTAH_A1_A field rotate = 3 (Max)
    // ISET: A32
    // Fields: rotate=3, Rd=0, Rn=0, cond=0, Rm=0
    let encoding: u32 = 0x06F00C70;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtah_a1_a_field_rm_0_min_70_06f00070() {
    // Encoding: 0x06F00070
    // Test aarch32_UXTAH_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, rotate=0, Rm=0, Rn=0, cond=0
    let encoding: u32 = 0x06F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtah_a1_a_field_rm_1_poweroftwo_70_06f00071() {
    // Encoding: 0x06F00071
    // Test aarch32_UXTAH_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=1, rotate=0, Rd=0, cond=0, Rn=0
    let encoding: u32 = 0x06F00071;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_uxtah_a1_a_combo_0_70_06f00070() {
    // Encoding: 0x06F00070
    // Test aarch32_UXTAH_A1_A field combination: cond=0, Rn=0, Rd=0, rotate=0, Rm=0
    // ISET: A32
    // Fields: rotate=0, Rd=0, Rn=0, cond=0, Rm=0
    let encoding: u32 = 0x06F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_uxtah_a1_a_special_cond_0_condition_eq_112_06f00070() {
    // Encoding: 0x06F00070
    // Test aarch32_UXTAH_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: rotate=0, Rm=0, cond=0, Rd=0, Rn=0
    let encoding: u32 = 0x06F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_uxtah_a1_a_special_cond_1_condition_ne_112_16f00070() {
    // Encoding: 0x16F00070
    // Test aarch32_UXTAH_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rn=0, cond=1, rotate=0, Rd=0, Rm=0
    let encoding: u32 = 0x16F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_uxtah_a1_a_special_cond_2_condition_cs_hs_112_26f00070() {
    // Encoding: 0x26F00070
    // Test aarch32_UXTAH_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rn=0, Rm=0, rotate=0, Rd=0, cond=2
    let encoding: u32 = 0x26F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_uxtah_a1_a_special_cond_3_condition_cc_lo_112_36f00070() {
    // Encoding: 0x36F00070
    // Test aarch32_UXTAH_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=3, rotate=0
    let encoding: u32 = 0x36F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_uxtah_a1_a_special_cond_4_condition_mi_112_46f00070() {
    // Encoding: 0x46F00070
    // Test aarch32_UXTAH_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rn=0, rotate=0, Rm=0, Rd=0, cond=4
    let encoding: u32 = 0x46F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_uxtah_a1_a_special_cond_5_condition_pl_112_56f00070() {
    // Encoding: 0x56F00070
    // Test aarch32_UXTAH_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rn=0, cond=5, Rm=0, Rd=0, rotate=0
    let encoding: u32 = 0x56F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_uxtah_a1_a_special_cond_6_condition_vs_112_66f00070() {
    // Encoding: 0x66F00070
    // Test aarch32_UXTAH_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, Rm=0, Rn=0, rotate=0, Rd=0
    let encoding: u32 = 0x66F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_uxtah_a1_a_special_cond_7_condition_vc_112_76f00070() {
    // Encoding: 0x76F00070
    // Test aarch32_UXTAH_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rn=0, rotate=0, Rd=0, Rm=0, cond=7
    let encoding: u32 = 0x76F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_uxtah_a1_a_special_cond_8_condition_hi_112_86f00070() {
    // Encoding: 0x86F00070
    // Test aarch32_UXTAH_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, Rd=0, rotate=0, Rm=0, Rn=0
    let encoding: u32 = 0x86F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_uxtah_a1_a_special_cond_9_condition_ls_112_96f00070() {
    // Encoding: 0x96F00070
    // Test aarch32_UXTAH_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=9, rotate=0
    let encoding: u32 = 0x96F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_uxtah_a1_a_special_cond_10_condition_ge_112_a6f00070() {
    // Encoding: 0xA6F00070
    // Test aarch32_UXTAH_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rd=0, cond=10, rotate=0, Rn=0, Rm=0
    let encoding: u32 = 0xA6F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_uxtah_a1_a_special_cond_11_condition_lt_112_b6f00070() {
    // Encoding: 0xB6F00070
    // Test aarch32_UXTAH_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: rotate=0, Rm=0, Rn=0, cond=11, Rd=0
    let encoding: u32 = 0xB6F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_uxtah_a1_a_special_cond_12_condition_gt_112_c6f00070() {
    // Encoding: 0xC6F00070
    // Test aarch32_UXTAH_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=12, rotate=0, Rm=0
    let encoding: u32 = 0xC6F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_uxtah_a1_a_special_cond_13_condition_le_112_d6f00070() {
    // Encoding: 0xD6F00070
    // Test aarch32_UXTAH_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rn=0, rotate=0, Rm=0, cond=13, Rd=0
    let encoding: u32 = 0xD6F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_uxtah_a1_a_special_cond_14_condition_al_112_e6f00070() {
    // Encoding: 0xE6F00070
    // Test aarch32_UXTAH_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rn=0, cond=14, Rd=0, Rm=0, rotate=0
    let encoding: u32 = 0xE6F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_uxtah_a1_a_special_cond_15_condition_nv_112_f6f00070() {
    // Encoding: 0xF6F00070
    // Test aarch32_UXTAH_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rm=0, cond=15, Rn=0, rotate=0, Rd=0
    let encoding: u32 = 0xF6F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uxtah_a1_a_invalid_0_70_06f00070() {
    // Encoding: 0x06F00070
    // Test aarch32_UXTAH_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: rotate=0, Rd=0, Rn=0, Rm=0, cond=0
    let encoding: u32 = 0x06F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UXTAH_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uxtah_a1_a_invalid_1_70_06f00070() {
    // Encoding: 0x06F00070
    // Test aarch32_UXTAH_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rd=0, rotate=0, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x06F00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UXTAH_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtah_t1_a_field_rn_0_min_f080_fa10f080() {
    // Thumb encoding (32): 0xFA10F080
    // Test aarch32_UXTAH_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, rotate=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA10F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAH_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtah_t1_a_field_rn_1_poweroftwo_f080_fa11f080() {
    // Thumb encoding (32): 0xFA11F080
    // Test aarch32_UXTAH_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, rotate=0, Rm=0, Rn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA11F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAH_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtah_t1_a_field_rd_0_min_f080_fa10f080() {
    // Thumb encoding (32): 0xFA10F080
    // Test aarch32_UXTAH_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: rotate=0, Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA10F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAH_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtah_t1_a_field_rd_1_poweroftwo_f080_fa10f180() {
    // Thumb encoding (32): 0xFA10F180
    // Test aarch32_UXTAH_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=1, rotate=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA10F180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAH_T1_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_uxtah_t1_a_field_rotate_0_min_f080_fa10f080() {
    // Thumb encoding (32): 0xFA10F080
    // Test aarch32_UXTAH_T1_A field rotate = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA10F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAH_T1_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_uxtah_t1_a_field_rotate_1_poweroftwo_f080_fa10f090() {
    // Thumb encoding (32): 0xFA10F090
    // Test aarch32_UXTAH_T1_A field rotate = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, rotate=1, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA10F090;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAH_T1_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_uxtah_t1_a_field_rotate_3_max_f080_fa10f0b0() {
    // Thumb encoding (32): 0xFA10F0B0
    // Test aarch32_UXTAH_T1_A field rotate = 3 (Max)
    // ISET: T32
    // Fields: Rm=0, rotate=3, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA10F0B0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAH_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtah_t1_a_field_rm_0_min_f080_fa10f080() {
    // Thumb encoding (32): 0xFA10F080
    // Test aarch32_UXTAH_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: rotate=0, Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA10F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAH_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtah_t1_a_field_rm_1_poweroftwo_f080_fa10f081() {
    // Thumb encoding (32): 0xFA10F081
    // Test aarch32_UXTAH_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, rotate=0, Rn=0, Rm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA10F081;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAH_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_uxtah_t1_a_combo_0_f080_fa10f080() {
    // Thumb encoding (32): 0xFA10F080
    // Test aarch32_UXTAH_T1_A field combination: Rn=0, Rd=0, rotate=0, Rm=0
    // ISET: T32
    // Fields: Rd=0, Rm=0, rotate=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA10F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAH_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uxtah_t1_a_invalid_0_f080_fa10f080() {
    // Thumb encoding (32): 0xFA10F080
    // Test aarch32_UXTAH_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rm=0, rotate=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA10F080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_UXTAH_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uxtah_t1_a_invalid_1_f080_fa10f080() {
    // Thumb encoding (32): 0xFA10F080
    // Test aarch32_UXTAH_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: rotate=0, Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA10F080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_SXTB16_A Tests
// ============================================================================

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_sxtb16_a1_a_field_cond_0_min_70_068f0070() {
    // Encoding: 0x068F0070
    // Test aarch32_SXTB16_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rd=0, rotate=0, cond=0
    let encoding: u32 = 0x068F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_sxtb16_a1_a_field_cond_1_poweroftwo_70_168f0070() {
    // Encoding: 0x168F0070
    // Test aarch32_SXTB16_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, rotate=0, Rd=0, Rm=0
    let encoding: u32 = 0x168F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_sxtb16_a1_a_field_cond_2_poweroftwo_70_268f0070() {
    // Encoding: 0x268F0070
    // Test aarch32_SXTB16_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=2, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x268F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_sxtb16_a1_a_field_cond_3_poweroftwo_70_368f0070() {
    // Encoding: 0x368F0070
    // Test aarch32_SXTB16_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, Rm=0, Rd=0, rotate=0
    let encoding: u32 = 0x368F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_sxtb16_a1_a_field_cond_4_poweroftwo_70_468f0070() {
    // Encoding: 0x468F0070
    // Test aarch32_SXTB16_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=4, Rm=0, Rd=0, rotate=0
    let encoding: u32 = 0x468F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_sxtb16_a1_a_field_cond_5_poweroftwo_70_568f0070() {
    // Encoding: 0x568F0070
    // Test aarch32_SXTB16_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=5, rotate=0, Rm=0
    let encoding: u32 = 0x568F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_sxtb16_a1_a_field_cond_6_poweroftwo_70_668f0070() {
    // Encoding: 0x668F0070
    // Test aarch32_SXTB16_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=6, rotate=0, Rm=0
    let encoding: u32 = 0x668F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_sxtb16_a1_a_field_cond_7_poweroftwo_70_768f0070() {
    // Encoding: 0x768F0070
    // Test aarch32_SXTB16_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=7, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x768F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_sxtb16_a1_a_field_cond_8_poweroftwo_70_868f0070() {
    // Encoding: 0x868F0070
    // Test aarch32_SXTB16_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rm=0, Rd=0, cond=8
    let encoding: u32 = 0x868F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_sxtb16_a1_a_field_cond_9_poweroftwo_70_968f0070() {
    // Encoding: 0x968F0070
    // Test aarch32_SXTB16_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rm=0, Rd=0, cond=9
    let encoding: u32 = 0x968F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_sxtb16_a1_a_field_cond_10_poweroftwo_70_a68f0070() {
    // Encoding: 0xA68F0070
    // Test aarch32_SXTB16_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, Rm=0, Rd=0, rotate=0
    let encoding: u32 = 0xA68F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_sxtb16_a1_a_field_cond_11_poweroftwo_70_b68f0070() {
    // Encoding: 0xB68F0070
    // Test aarch32_SXTB16_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=11, rotate=0, Rd=0, Rm=0
    let encoding: u32 = 0xB68F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_sxtb16_a1_a_field_cond_12_poweroftwo_70_c68f0070() {
    // Encoding: 0xC68F0070
    // Test aarch32_SXTB16_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, rotate=0, Rm=0, cond=12
    let encoding: u32 = 0xC68F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_sxtb16_a1_a_field_cond_13_poweroftwo_70_d68f0070() {
    // Encoding: 0xD68F0070
    // Test aarch32_SXTB16_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=13, Rm=0, rotate=0
    let encoding: u32 = 0xD68F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_sxtb16_a1_a_field_cond_14_poweroftwo_70_e68f0070() {
    // Encoding: 0xE68F0070
    // Test aarch32_SXTB16_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, rotate=0, Rm=0, Rd=0
    let encoding: u32 = 0xE68F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_sxtb16_a1_a_field_cond_15_max_70_f68f0070() {
    // Encoding: 0xF68F0070
    // Test aarch32_SXTB16_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: rotate=0, cond=15, Rm=0, Rd=0
    let encoding: u32 = 0xF68F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtb16_a1_a_field_rd_0_min_70_068f0070() {
    // Encoding: 0x068F0070
    // Test aarch32_SXTB16_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x068F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtb16_a1_a_field_rd_1_poweroftwo_70_068f1070() {
    // Encoding: 0x068F1070
    // Test aarch32_SXTB16_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=1, rotate=0, Rm=0
    let encoding: u32 = 0x068F1070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_sxtb16_a1_a_field_rotate_0_min_70_068f0070() {
    // Encoding: 0x068F0070
    // Test aarch32_SXTB16_A1_A field rotate = 0 (Min)
    // ISET: A32
    // Fields: rotate=0, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x068F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_sxtb16_a1_a_field_rotate_1_poweroftwo_70_068f0470() {
    // Encoding: 0x068F0470
    // Test aarch32_SXTB16_A1_A field rotate = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, rotate=1, Rm=0, Rd=0
    let encoding: u32 = 0x068F0470;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_sxtb16_a1_a_field_rotate_3_max_70_068f0c70() {
    // Encoding: 0x068F0C70
    // Test aarch32_SXTB16_A1_A field rotate = 3 (Max)
    // ISET: A32
    // Fields: rotate=3, cond=0, Rd=0, Rm=0
    let encoding: u32 = 0x068F0C70;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtb16_a1_a_field_rm_0_min_70_068f0070() {
    // Encoding: 0x068F0070
    // Test aarch32_SXTB16_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, cond=0, Rm=0, rotate=0
    let encoding: u32 = 0x068F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtb16_a1_a_field_rm_1_poweroftwo_70_068f0071() {
    // Encoding: 0x068F0071
    // Test aarch32_SXTB16_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=1, Rd=0, rotate=0, cond=0
    let encoding: u32 = 0x068F0071;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_sxtb16_a1_a_combo_0_70_068f0070() {
    // Encoding: 0x068F0070
    // Test aarch32_SXTB16_A1_A field combination: cond=0, Rd=0, rotate=0, Rm=0
    // ISET: A32
    // Fields: Rm=0, rotate=0, cond=0, Rd=0
    let encoding: u32 = 0x068F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_sxtb16_a1_a_special_cond_0_condition_eq_112_068f0070() {
    // Encoding: 0x068F0070
    // Test aarch32_SXTB16_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rd=0, rotate=0
    let encoding: u32 = 0x068F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_sxtb16_a1_a_special_cond_1_condition_ne_112_168f0070() {
    // Encoding: 0x168F0070
    // Test aarch32_SXTB16_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, rotate=0, Rm=0, Rd=0
    let encoding: u32 = 0x168F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_sxtb16_a1_a_special_cond_2_condition_cs_hs_112_268f0070() {
    // Encoding: 0x268F0070
    // Test aarch32_SXTB16_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, rotate=0, Rd=0, Rm=0
    let encoding: u32 = 0x268F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_sxtb16_a1_a_special_cond_3_condition_cc_lo_112_368f0070() {
    // Encoding: 0x368F0070
    // Test aarch32_SXTB16_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rd=0, Rm=0, rotate=0, cond=3
    let encoding: u32 = 0x368F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_sxtb16_a1_a_special_cond_4_condition_mi_112_468f0070() {
    // Encoding: 0x468F0070
    // Test aarch32_SXTB16_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=4, rotate=0
    let encoding: u32 = 0x468F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_sxtb16_a1_a_special_cond_5_condition_pl_112_568f0070() {
    // Encoding: 0x568F0070
    // Test aarch32_SXTB16_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rd=0, rotate=0, cond=5, Rm=0
    let encoding: u32 = 0x568F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_sxtb16_a1_a_special_cond_6_condition_vs_112_668f0070() {
    // Encoding: 0x668F0070
    // Test aarch32_SXTB16_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=6, rotate=0
    let encoding: u32 = 0x668F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_sxtb16_a1_a_special_cond_7_condition_vc_112_768f0070() {
    // Encoding: 0x768F0070
    // Test aarch32_SXTB16_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: rotate=0, cond=7, Rm=0, Rd=0
    let encoding: u32 = 0x768F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_sxtb16_a1_a_special_cond_8_condition_hi_112_868f0070() {
    // Encoding: 0x868F0070
    // Test aarch32_SXTB16_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, rotate=0, Rm=0, Rd=0
    let encoding: u32 = 0x868F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_sxtb16_a1_a_special_cond_9_condition_ls_112_968f0070() {
    // Encoding: 0x968F0070
    // Test aarch32_SXTB16_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: rotate=0, Rd=0, cond=9, Rm=0
    let encoding: u32 = 0x968F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_sxtb16_a1_a_special_cond_10_condition_ge_112_a68f0070() {
    // Encoding: 0xA68F0070
    // Test aarch32_SXTB16_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rm=0, cond=10, rotate=0, Rd=0
    let encoding: u32 = 0xA68F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_sxtb16_a1_a_special_cond_11_condition_lt_112_b68f0070() {
    // Encoding: 0xB68F0070
    // Test aarch32_SXTB16_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rd=0, rotate=0, Rm=0, cond=11
    let encoding: u32 = 0xB68F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_sxtb16_a1_a_special_cond_12_condition_gt_112_c68f0070() {
    // Encoding: 0xC68F0070
    // Test aarch32_SXTB16_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: cond=12, Rm=0, rotate=0, Rd=0
    let encoding: u32 = 0xC68F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_sxtb16_a1_a_special_cond_13_condition_le_112_d68f0070() {
    // Encoding: 0xD68F0070
    // Test aarch32_SXTB16_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: cond=13, rotate=0, Rm=0, Rd=0
    let encoding: u32 = 0xD68F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_sxtb16_a1_a_special_cond_14_condition_al_112_e68f0070() {
    // Encoding: 0xE68F0070
    // Test aarch32_SXTB16_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: rotate=0, Rm=0, cond=14, Rd=0
    let encoding: u32 = 0xE68F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_sxtb16_a1_a_special_cond_15_condition_nv_112_f68f0070() {
    // Encoding: 0xF68F0070
    // Test aarch32_SXTB16_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, Rm=0, Rd=0, rotate=0
    let encoding: u32 = 0xF68F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sxtb16_a1_a_invalid_0_70_068f0070() {
    // Encoding: 0x068F0070
    // Test aarch32_SXTB16_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: rotate=0, cond=0, Rd=0, Rm=0
    let encoding: u32 = 0x068F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SXTB16_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sxtb16_a1_a_invalid_1_70_068f0070() {
    // Encoding: 0x068F0070
    // Test aarch32_SXTB16_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: rotate=0, cond=0, Rm=0, Rd=0
    let encoding: u32 = 0x068F0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SXTB16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtb16_t1_a_field_rd_0_min_f080_fa2ff080() {
    // Thumb encoding (32): 0xFA2FF080
    // Test aarch32_SXTB16_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, rotate=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA2FF080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTB16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtb16_t1_a_field_rd_1_poweroftwo_f080_fa2ff180() {
    // Thumb encoding (32): 0xFA2FF180
    // Test aarch32_SXTB16_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: rotate=0, Rm=0, Rd=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA2FF180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTB16_T1_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_sxtb16_t1_a_field_rotate_0_min_f080_fa2ff080() {
    // Thumb encoding (32): 0xFA2FF080
    // Test aarch32_SXTB16_T1_A field rotate = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA2FF080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTB16_T1_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_sxtb16_t1_a_field_rotate_1_poweroftwo_f080_fa2ff090() {
    // Thumb encoding (32): 0xFA2FF090
    // Test aarch32_SXTB16_T1_A field rotate = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: rotate=1, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA2FF090;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTB16_T1_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_sxtb16_t1_a_field_rotate_3_max_f080_fa2ff0b0() {
    // Thumb encoding (32): 0xFA2FF0B0
    // Test aarch32_SXTB16_T1_A field rotate = 3 (Max)
    // ISET: T32
    // Fields: Rd=0, Rm=0, rotate=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA2FF0B0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTB16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtb16_t1_a_field_rm_0_min_f080_fa2ff080() {
    // Thumb encoding (32): 0xFA2FF080
    // Test aarch32_SXTB16_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, rotate=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA2FF080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTB16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtb16_t1_a_field_rm_1_poweroftwo_f080_fa2ff081() {
    // Thumb encoding (32): 0xFA2FF081
    // Test aarch32_SXTB16_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=1, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA2FF081;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTB16_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch32_sxtb16_t1_a_combo_0_f080_fa2ff080() {
    // Thumb encoding (32): 0xFA2FF080
    // Test aarch32_SXTB16_T1_A field combination: Rd=0, rotate=0, Rm=0
    // ISET: T32
    // Fields: Rd=0, Rm=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA2FF080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTB16_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sxtb16_t1_a_invalid_0_f080_fa2ff080() {
    // Thumb encoding (32): 0xFA2FF080
    // Test aarch32_SXTB16_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rd=0, Rm=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA2FF080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_SXTB16_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sxtb16_t1_a_invalid_1_f080_fa2ff080() {
    // Thumb encoding (32): 0xFA2FF080
    // Test aarch32_SXTB16_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rd=0, Rm=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA2FF080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_SXTAB_A Tests
// ============================================================================

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_sxtab_a1_a_field_cond_0_min_70_06a00070() {
    // Encoding: 0x06A00070
    // Test aarch32_SXTAB_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, rotate=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x06A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_sxtab_a1_a_field_cond_1_poweroftwo_70_16a00070() {
    // Encoding: 0x16A00070
    // Test aarch32_SXTAB_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, Rn=0, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x16A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_sxtab_a1_a_field_cond_2_poweroftwo_70_26a00070() {
    // Encoding: 0x26A00070
    // Test aarch32_SXTAB_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=2, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x26A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_sxtab_a1_a_field_cond_3_poweroftwo_70_36a00070() {
    // Encoding: 0x36A00070
    // Test aarch32_SXTAB_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, Rn=0, Rm=0, rotate=0, Rd=0
    let encoding: u32 = 0x36A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_sxtab_a1_a_field_cond_4_poweroftwo_70_46a00070() {
    // Encoding: 0x46A00070
    // Test aarch32_SXTAB_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=4, Rm=0, Rd=0, rotate=0, Rn=0
    let encoding: u32 = 0x46A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_sxtab_a1_a_field_cond_5_poweroftwo_70_56a00070() {
    // Encoding: 0x56A00070
    // Test aarch32_SXTAB_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=5, Rn=0, rotate=0
    let encoding: u32 = 0x56A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_sxtab_a1_a_field_cond_6_poweroftwo_70_66a00070() {
    // Encoding: 0x66A00070
    // Test aarch32_SXTAB_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, rotate=0, cond=6, Rd=0
    let encoding: u32 = 0x66A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_sxtab_a1_a_field_cond_7_poweroftwo_70_76a00070() {
    // Encoding: 0x76A00070
    // Test aarch32_SXTAB_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=7, rotate=0, Rn=0, Rm=0
    let encoding: u32 = 0x76A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_sxtab_a1_a_field_cond_8_poweroftwo_70_86a00070() {
    // Encoding: 0x86A00070
    // Test aarch32_SXTAB_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=8, Rm=0, rotate=0, Rd=0, Rn=0
    let encoding: u32 = 0x86A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_sxtab_a1_a_field_cond_9_poweroftwo_70_96a00070() {
    // Encoding: 0x96A00070
    // Test aarch32_SXTAB_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=9, Rm=0, Rn=0, rotate=0
    let encoding: u32 = 0x96A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_sxtab_a1_a_field_cond_10_poweroftwo_70_a6a00070() {
    // Encoding: 0xA6A00070
    // Test aarch32_SXTAB_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, cond=10, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0xA6A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_sxtab_a1_a_field_cond_11_poweroftwo_70_b6a00070() {
    // Encoding: 0xB6A00070
    // Test aarch32_SXTAB_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rm=0, cond=11, Rd=0, Rn=0
    let encoding: u32 = 0xB6A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_sxtab_a1_a_field_cond_12_poweroftwo_70_c6a00070() {
    // Encoding: 0xC6A00070
    // Test aarch32_SXTAB_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, rotate=0, cond=12, Rd=0
    let encoding: u32 = 0xC6A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_sxtab_a1_a_field_cond_13_poweroftwo_70_d6a00070() {
    // Encoding: 0xD6A00070
    // Test aarch32_SXTAB_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, rotate=0, cond=13
    let encoding: u32 = 0xD6A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_sxtab_a1_a_field_cond_14_poweroftwo_70_e6a00070() {
    // Encoding: 0xE6A00070
    // Test aarch32_SXTAB_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=14, Rm=0, Rn=0, rotate=0
    let encoding: u32 = 0xE6A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_sxtab_a1_a_field_cond_15_max_70_f6a00070() {
    // Encoding: 0xF6A00070
    // Test aarch32_SXTAB_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, rotate=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0xF6A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtab_a1_a_field_rn_0_min_70_06a00070() {
    // Encoding: 0x06A00070
    // Test aarch32_SXTAB_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, rotate=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x06A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtab_a1_a_field_rn_1_poweroftwo_70_06a10070() {
    // Encoding: 0x06A10070
    // Test aarch32_SXTAB_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=0, rotate=0, Rm=0, Rn=1
    let encoding: u32 = 0x06A10070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtab_a1_a_field_rd_0_min_70_06a00070() {
    // Encoding: 0x06A00070
    // Test aarch32_SXTAB_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, rotate=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x06A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtab_a1_a_field_rd_1_poweroftwo_70_06a01070() {
    // Encoding: 0x06A01070
    // Test aarch32_SXTAB_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=1, rotate=0, Rn=0, cond=0
    let encoding: u32 = 0x06A01070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_sxtab_a1_a_field_rotate_0_min_70_06a00070() {
    // Encoding: 0x06A00070
    // Test aarch32_SXTAB_A1_A field rotate = 0 (Min)
    // ISET: A32
    // Fields: rotate=0, Rn=0, Rd=0, Rm=0, cond=0
    let encoding: u32 = 0x06A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_sxtab_a1_a_field_rotate_1_poweroftwo_70_06a00470() {
    // Encoding: 0x06A00470
    // Test aarch32_SXTAB_A1_A field rotate = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, rotate=1, Rm=0, Rd=0
    let encoding: u32 = 0x06A00470;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_sxtab_a1_a_field_rotate_3_max_70_06a00c70() {
    // Encoding: 0x06A00C70
    // Test aarch32_SXTAB_A1_A field rotate = 3 (Max)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rd=0, Rn=0, rotate=3
    let encoding: u32 = 0x06A00C70;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtab_a1_a_field_rm_0_min_70_06a00070() {
    // Encoding: 0x06A00070
    // Test aarch32_SXTAB_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=0, rotate=0
    let encoding: u32 = 0x06A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtab_a1_a_field_rm_1_poweroftwo_70_06a00071() {
    // Encoding: 0x06A00071
    // Test aarch32_SXTAB_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rm=1, Rn=0, rotate=0, Rd=0
    let encoding: u32 = 0x06A00071;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_sxtab_a1_a_combo_0_70_06a00070() {
    // Encoding: 0x06A00070
    // Test aarch32_SXTAB_A1_A field combination: cond=0, Rn=0, Rd=0, rotate=0, Rm=0
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=0, rotate=0, Rm=0
    let encoding: u32 = 0x06A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_sxtab_a1_a_special_cond_0_condition_eq_112_06a00070() {
    // Encoding: 0x06A00070
    // Test aarch32_SXTAB_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rd=0, rotate=0, cond=0, Rn=0, Rm=0
    let encoding: u32 = 0x06A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_sxtab_a1_a_special_cond_1_condition_ne_112_16a00070() {
    // Encoding: 0x16A00070
    // Test aarch32_SXTAB_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, rotate=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x16A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_sxtab_a1_a_special_cond_2_condition_cs_hs_112_26a00070() {
    // Encoding: 0x26A00070
    // Test aarch32_SXTAB_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: rotate=0, cond=2, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x26A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_sxtab_a1_a_special_cond_3_condition_cc_lo_112_36a00070() {
    // Encoding: 0x36A00070
    // Test aarch32_SXTAB_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, Rd=0, Rm=0, rotate=0, Rn=0
    let encoding: u32 = 0x36A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_sxtab_a1_a_special_cond_4_condition_mi_112_46a00070() {
    // Encoding: 0x46A00070
    // Test aarch32_SXTAB_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, Rd=0, Rm=0, Rn=0, rotate=0
    let encoding: u32 = 0x46A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_sxtab_a1_a_special_cond_5_condition_pl_112_56a00070() {
    // Encoding: 0x56A00070
    // Test aarch32_SXTAB_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rd=0, Rm=0, rotate=0, cond=5, Rn=0
    let encoding: u32 = 0x56A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_sxtab_a1_a_special_cond_6_condition_vs_112_66a00070() {
    // Encoding: 0x66A00070
    // Test aarch32_SXTAB_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rn=0, cond=6, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x66A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_sxtab_a1_a_special_cond_7_condition_vc_112_76a00070() {
    // Encoding: 0x76A00070
    // Test aarch32_SXTAB_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, Rm=0, Rd=0, Rn=0, rotate=0
    let encoding: u32 = 0x76A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_sxtab_a1_a_special_cond_8_condition_hi_112_86a00070() {
    // Encoding: 0x86A00070
    // Test aarch32_SXTAB_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rm=0, rotate=0, Rn=0, Rd=0, cond=8
    let encoding: u32 = 0x86A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_sxtab_a1_a_special_cond_9_condition_ls_112_96a00070() {
    // Encoding: 0x96A00070
    // Test aarch32_SXTAB_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=9, Rn=0, rotate=0
    let encoding: u32 = 0x96A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_sxtab_a1_a_special_cond_10_condition_ge_112_a6a00070() {
    // Encoding: 0xA6A00070
    // Test aarch32_SXTAB_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: rotate=0, Rn=0, Rd=0, Rm=0, cond=10
    let encoding: u32 = 0xA6A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_sxtab_a1_a_special_cond_11_condition_lt_112_b6a00070() {
    // Encoding: 0xB6A00070
    // Test aarch32_SXTAB_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=11, Rn=0, rotate=0
    let encoding: u32 = 0xB6A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_sxtab_a1_a_special_cond_12_condition_gt_112_c6a00070() {
    // Encoding: 0xC6A00070
    // Test aarch32_SXTAB_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rd=0, cond=12, Rn=0, rotate=0, Rm=0
    let encoding: u32 = 0xC6A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_sxtab_a1_a_special_cond_13_condition_le_112_d6a00070() {
    // Encoding: 0xD6A00070
    // Test aarch32_SXTAB_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: cond=13, Rn=0, Rd=0, Rm=0, rotate=0
    let encoding: u32 = 0xD6A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_sxtab_a1_a_special_cond_14_condition_al_112_e6a00070() {
    // Encoding: 0xE6A00070
    // Test aarch32_SXTAB_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: rotate=0, Rm=0, Rn=0, Rd=0, cond=14
    let encoding: u32 = 0xE6A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_sxtab_a1_a_special_cond_15_condition_nv_112_f6a00070() {
    // Encoding: 0xF6A00070
    // Test aarch32_SXTAB_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, rotate=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0xF6A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sxtab_a1_a_invalid_0_70_06a00070() {
    // Encoding: 0x06A00070
    // Test aarch32_SXTAB_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rm=0, Rn=0, rotate=0, cond=0, Rd=0
    let encoding: u32 = 0x06A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SXTAB_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sxtab_a1_a_invalid_1_70_06a00070() {
    // Encoding: 0x06A00070
    // Test aarch32_SXTAB_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rn=0, cond=0, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x06A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SXTAB_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtab_t1_a_field_rn_0_min_f080_fa40f080() {
    // Thumb encoding (32): 0xFA40F080
    // Test aarch32_SXTAB_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, rotate=0, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA40F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAB_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtab_t1_a_field_rn_1_poweroftwo_f080_fa41f080() {
    // Thumb encoding (32): 0xFA41F080
    // Test aarch32_SXTAB_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, rotate=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA41F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAB_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtab_t1_a_field_rd_0_min_f080_fa40f080() {
    // Thumb encoding (32): 0xFA40F080
    // Test aarch32_SXTAB_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, rotate=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA40F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAB_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtab_t1_a_field_rd_1_poweroftwo_f080_fa40f180() {
    // Thumb encoding (32): 0xFA40F180
    // Test aarch32_SXTAB_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: rotate=0, Rd=1, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA40F180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAB_T1_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_sxtab_t1_a_field_rotate_0_min_f080_fa40f080() {
    // Thumb encoding (32): 0xFA40F080
    // Test aarch32_SXTAB_T1_A field rotate = 0 (Min)
    // ISET: T32
    // Fields: rotate=0, Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA40F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAB_T1_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_sxtab_t1_a_field_rotate_1_poweroftwo_f080_fa40f090() {
    // Thumb encoding (32): 0xFA40F090
    // Test aarch32_SXTAB_T1_A field rotate = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, rotate=1, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA40F090;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAB_T1_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_sxtab_t1_a_field_rotate_3_max_f080_fa40f0b0() {
    // Thumb encoding (32): 0xFA40F0B0
    // Test aarch32_SXTAB_T1_A field rotate = 3 (Max)
    // ISET: T32
    // Fields: Rn=0, rotate=3, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA40F0B0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAB_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtab_t1_a_field_rm_0_min_f080_fa40f080() {
    // Thumb encoding (32): 0xFA40F080
    // Test aarch32_SXTAB_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA40F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAB_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtab_t1_a_field_rm_1_poweroftwo_f080_fa40f081() {
    // Thumb encoding (32): 0xFA40F081
    // Test aarch32_SXTAB_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=1, rotate=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA40F081;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAB_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_sxtab_t1_a_combo_0_f080_fa40f080() {
    // Thumb encoding (32): 0xFA40F080
    // Test aarch32_SXTAB_T1_A field combination: Rn=0, Rd=0, rotate=0, Rm=0
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA40F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAB_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sxtab_t1_a_invalid_0_f080_fa40f080() {
    // Thumb encoding (32): 0xFA40F080
    // Test aarch32_SXTAB_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA40F080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_SXTAB_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sxtab_t1_a_invalid_1_f080_fa40f080() {
    // Thumb encoding (32): 0xFA40F080
    // Test aarch32_SXTAB_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: rotate=0, Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA40F080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_UXTB_A Tests
// ============================================================================

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_uxtb_a1_a_field_cond_0_min_70_06ef0070() {
    // Encoding: 0x06EF0070
    // Test aarch32_UXTB_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: rotate=0, Rm=0, Rd=0, cond=0
    let encoding: u32 = 0x06EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_uxtb_a1_a_field_cond_1_poweroftwo_70_16ef0070() {
    // Encoding: 0x16EF0070
    // Test aarch32_UXTB_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rm=0, cond=1, Rd=0
    let encoding: u32 = 0x16EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_uxtb_a1_a_field_cond_2_poweroftwo_70_26ef0070() {
    // Encoding: 0x26EF0070
    // Test aarch32_UXTB_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, rotate=0, cond=2, Rm=0
    let encoding: u32 = 0x26EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_uxtb_a1_a_field_cond_3_poweroftwo_70_36ef0070() {
    // Encoding: 0x36EF0070
    // Test aarch32_UXTB_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rd=0, cond=3, Rm=0
    let encoding: u32 = 0x36EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_uxtb_a1_a_field_cond_4_poweroftwo_70_46ef0070() {
    // Encoding: 0x46EF0070
    // Test aarch32_UXTB_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=4, rotate=0
    let encoding: u32 = 0x46EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_uxtb_a1_a_field_cond_5_poweroftwo_70_56ef0070() {
    // Encoding: 0x56EF0070
    // Test aarch32_UXTB_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=5, Rd=0, Rm=0, rotate=0
    let encoding: u32 = 0x56EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_uxtb_a1_a_field_cond_6_poweroftwo_70_66ef0070() {
    // Encoding: 0x66EF0070
    // Test aarch32_UXTB_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, rotate=0, cond=6, Rm=0
    let encoding: u32 = 0x66EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_uxtb_a1_a_field_cond_7_poweroftwo_70_76ef0070() {
    // Encoding: 0x76EF0070
    // Test aarch32_UXTB_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=7, rotate=0, Rm=0, Rd=0
    let encoding: u32 = 0x76EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_uxtb_a1_a_field_cond_8_poweroftwo_70_86ef0070() {
    // Encoding: 0x86EF0070
    // Test aarch32_UXTB_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=8, Rm=0, Rd=0, rotate=0
    let encoding: u32 = 0x86EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_uxtb_a1_a_field_cond_9_poweroftwo_70_96ef0070() {
    // Encoding: 0x96EF0070
    // Test aarch32_UXTB_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, Rm=0, rotate=0, Rd=0
    let encoding: u32 = 0x96EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_uxtb_a1_a_field_cond_10_poweroftwo_70_a6ef0070() {
    // Encoding: 0xA6EF0070
    // Test aarch32_UXTB_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, rotate=0, cond=10, Rm=0
    let encoding: u32 = 0xA6EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_uxtb_a1_a_field_cond_11_poweroftwo_70_b6ef0070() {
    // Encoding: 0xB6EF0070
    // Test aarch32_UXTB_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, rotate=0, cond=11, Rm=0
    let encoding: u32 = 0xB6EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_uxtb_a1_a_field_cond_12_poweroftwo_70_c6ef0070() {
    // Encoding: 0xC6EF0070
    // Test aarch32_UXTB_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, Rm=0, rotate=0, Rd=0
    let encoding: u32 = 0xC6EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_uxtb_a1_a_field_cond_13_poweroftwo_70_d6ef0070() {
    // Encoding: 0xD6EF0070
    // Test aarch32_UXTB_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, rotate=0, Rd=0, cond=13
    let encoding: u32 = 0xD6EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_uxtb_a1_a_field_cond_14_poweroftwo_70_e6ef0070() {
    // Encoding: 0xE6EF0070
    // Test aarch32_UXTB_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, cond=14, Rd=0, Rm=0
    let encoding: u32 = 0xE6EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_uxtb_a1_a_field_cond_15_max_70_f6ef0070() {
    // Encoding: 0xF6EF0070
    // Test aarch32_UXTB_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: rotate=0, cond=15, Rd=0, Rm=0
    let encoding: u32 = 0xF6EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtb_a1_a_field_rd_0_min_70_06ef0070() {
    // Encoding: 0x06EF0070
    // Test aarch32_UXTB_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: rotate=0, Rd=0, Rm=0, cond=0
    let encoding: u32 = 0x06EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtb_a1_a_field_rd_1_poweroftwo_70_06ef1070() {
    // Encoding: 0x06EF1070
    // Test aarch32_UXTB_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=0, rotate=0, Rd=1
    let encoding: u32 = 0x06EF1070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_uxtb_a1_a_field_rotate_0_min_70_06ef0070() {
    // Encoding: 0x06EF0070
    // Test aarch32_UXTB_A1_A field rotate = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rm=0, rotate=0, cond=0
    let encoding: u32 = 0x06EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_uxtb_a1_a_field_rotate_1_poweroftwo_70_06ef0470() {
    // Encoding: 0x06EF0470
    // Test aarch32_UXTB_A1_A field rotate = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, rotate=1, Rd=0, Rm=0
    let encoding: u32 = 0x06EF0470;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_uxtb_a1_a_field_rotate_3_max_70_06ef0c70() {
    // Encoding: 0x06EF0C70
    // Test aarch32_UXTB_A1_A field rotate = 3 (Max)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=0, rotate=3
    let encoding: u32 = 0x06EF0C70;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtb_a1_a_field_rm_0_min_70_06ef0070() {
    // Encoding: 0x06EF0070
    // Test aarch32_UXTB_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rd=0, rotate=0, cond=0
    let encoding: u32 = 0x06EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtb_a1_a_field_rm_1_poweroftwo_70_06ef0071() {
    // Encoding: 0x06EF0071
    // Test aarch32_UXTB_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rm=1, cond=0, Rd=0
    let encoding: u32 = 0x06EF0071;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_uxtb_a1_a_combo_0_70_06ef0070() {
    // Encoding: 0x06EF0070
    // Test aarch32_UXTB_A1_A field combination: cond=0, Rd=0, rotate=0, Rm=0
    // ISET: A32
    // Fields: Rd=0, rotate=0, cond=0, Rm=0
    let encoding: u32 = 0x06EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_uxtb_a1_a_special_cond_0_condition_eq_112_06ef0070() {
    // Encoding: 0x06EF0070
    // Test aarch32_UXTB_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rd=0, rotate=0
    let encoding: u32 = 0x06EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_uxtb_a1_a_special_cond_1_condition_ne_112_16ef0070() {
    // Encoding: 0x16EF0070
    // Test aarch32_UXTB_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rd=0, rotate=0, Rm=0, cond=1
    let encoding: u32 = 0x16EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_uxtb_a1_a_special_cond_2_condition_cs_hs_112_26ef0070() {
    // Encoding: 0x26EF0070
    // Test aarch32_UXTB_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rd=0, rotate=0, Rm=0, cond=2
    let encoding: u32 = 0x26EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_uxtb_a1_a_special_cond_3_condition_cc_lo_112_36ef0070() {
    // Encoding: 0x36EF0070
    // Test aarch32_UXTB_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x36EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_uxtb_a1_a_special_cond_4_condition_mi_112_46ef0070() {
    // Encoding: 0x46EF0070
    // Test aarch32_UXTB_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: rotate=0, cond=4, Rd=0, Rm=0
    let encoding: u32 = 0x46EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_uxtb_a1_a_special_cond_5_condition_pl_112_56ef0070() {
    // Encoding: 0x56EF0070
    // Test aarch32_UXTB_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: rotate=0, Rd=0, Rm=0, cond=5
    let encoding: u32 = 0x56EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_uxtb_a1_a_special_cond_6_condition_vs_112_66ef0070() {
    // Encoding: 0x66EF0070
    // Test aarch32_UXTB_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rd=0, cond=6, Rm=0, rotate=0
    let encoding: u32 = 0x66EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_uxtb_a1_a_special_cond_7_condition_vc_112_76ef0070() {
    // Encoding: 0x76EF0070
    // Test aarch32_UXTB_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x76EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_uxtb_a1_a_special_cond_8_condition_hi_112_86ef0070() {
    // Encoding: 0x86EF0070
    // Test aarch32_UXTB_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, Rd=0, Rm=0, rotate=0
    let encoding: u32 = 0x86EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_uxtb_a1_a_special_cond_9_condition_ls_112_96ef0070() {
    // Encoding: 0x96EF0070
    // Test aarch32_UXTB_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: rotate=0, Rd=0, cond=9, Rm=0
    let encoding: u32 = 0x96EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_uxtb_a1_a_special_cond_10_condition_ge_112_a6ef0070() {
    // Encoding: 0xA6EF0070
    // Test aarch32_UXTB_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0xA6EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_uxtb_a1_a_special_cond_11_condition_lt_112_b6ef0070() {
    // Encoding: 0xB6EF0070
    // Test aarch32_UXTB_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rd=0, cond=11, Rm=0, rotate=0
    let encoding: u32 = 0xB6EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_uxtb_a1_a_special_cond_12_condition_gt_112_c6ef0070() {
    // Encoding: 0xC6EF0070
    // Test aarch32_UXTB_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: rotate=0, Rd=0, Rm=0, cond=12
    let encoding: u32 = 0xC6EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_uxtb_a1_a_special_cond_13_condition_le_112_d6ef0070() {
    // Encoding: 0xD6EF0070
    // Test aarch32_UXTB_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rd=0, cond=13, Rm=0, rotate=0
    let encoding: u32 = 0xD6EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_uxtb_a1_a_special_cond_14_condition_al_112_e6ef0070() {
    // Encoding: 0xE6EF0070
    // Test aarch32_UXTB_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rd=0, Rm=0, rotate=0, cond=14
    let encoding: u32 = 0xE6EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_uxtb_a1_a_special_cond_15_condition_nv_112_f6ef0070() {
    // Encoding: 0xF6EF0070
    // Test aarch32_UXTB_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0xF6EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uxtb_a1_a_invalid_0_70_06ef0070() {
    // Encoding: 0x06EF0070
    // Test aarch32_UXTB_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: cond=0, Rm=0, rotate=0, Rd=0
    let encoding: u32 = 0x06EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UXTB_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uxtb_a1_a_invalid_1_70_06ef0070() {
    // Encoding: 0x06EF0070
    // Test aarch32_UXTB_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: rotate=0, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x06EF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UXTB_T1_A
/// ASL: `field Rm 19 +: 3`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtb_t1_a_field_rm_0_min_0_b2c00000() {
    // Thumb encoding (32): 0xB2C00000
    // Test aarch32_UXTB_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB2C00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTB_T1_A
/// ASL: `field Rm 19 +: 3`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtb_t1_a_field_rm_1_poweroftwo_0_b2c80000() {
    // Thumb encoding (32): 0xB2C80000
    // Test aarch32_UXTB_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB2C80000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTB_T1_A
/// ASL: `field Rd 16 +: 3`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtb_t1_a_field_rd_0_min_0_b2c00000() {
    // Thumb encoding (32): 0xB2C00000
    // Test aarch32_UXTB_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB2C00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTB_T1_A
/// ASL: `field Rd 16 +: 3`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtb_t1_a_field_rd_1_poweroftwo_0_b2c10000() {
    // Thumb encoding (32): 0xB2C10000
    // Test aarch32_UXTB_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=1, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB2C10000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTB_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch32_uxtb_t1_a_combo_0_0_b2c00000() {
    // Thumb encoding (32): 0xB2C00000
    // Test aarch32_UXTB_T1_A field combination: Rm=0, Rd=0
    // ISET: T32
    // Fields: Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB2C00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTB_T2_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtb_t2_a_field_rd_0_min_f080_fa5ff080() {
    // Thumb encoding (32): 0xFA5FF080
    // Test aarch32_UXTB_T2_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, rotate=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA5FF080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTB_T2_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtb_t2_a_field_rd_1_poweroftwo_f080_fa5ff180() {
    // Thumb encoding (32): 0xFA5FF180
    // Test aarch32_UXTB_T2_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: rotate=0, Rm=0, Rd=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA5FF180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTB_T2_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_uxtb_t2_a_field_rotate_0_min_f080_fa5ff080() {
    // Thumb encoding (32): 0xFA5FF080
    // Test aarch32_UXTB_T2_A field rotate = 0 (Min)
    // ISET: T32
    // Fields: rotate=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA5FF080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTB_T2_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_uxtb_t2_a_field_rotate_1_poweroftwo_f080_fa5ff090() {
    // Thumb encoding (32): 0xFA5FF090
    // Test aarch32_UXTB_T2_A field rotate = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: rotate=1, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA5FF090;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTB_T2_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_uxtb_t2_a_field_rotate_3_max_f080_fa5ff0b0() {
    // Thumb encoding (32): 0xFA5FF0B0
    // Test aarch32_UXTB_T2_A field rotate = 3 (Max)
    // ISET: T32
    // Fields: rotate=3, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA5FF0B0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTB_T2_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtb_t2_a_field_rm_0_min_f080_fa5ff080() {
    // Thumb encoding (32): 0xFA5FF080
    // Test aarch32_UXTB_T2_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rd=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA5FF080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTB_T2_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtb_t2_a_field_rm_1_poweroftwo_f080_fa5ff081() {
    // Thumb encoding (32): 0xFA5FF081
    // Test aarch32_UXTB_T2_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1, rotate=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA5FF081;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTB_T2_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch32_uxtb_t2_a_combo_0_f080_fa5ff080() {
    // Thumb encoding (32): 0xFA5FF080
    // Test aarch32_UXTB_T2_A field combination: Rd=0, rotate=0, Rm=0
    // ISET: T32
    // Fields: Rm=0, Rd=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA5FF080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTB_T2_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uxtb_t2_a_invalid_0_f080_fa5ff080() {
    // Thumb encoding (32): 0xFA5FF080
    // Test aarch32_UXTB_T2_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rm=0, rotate=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA5FF080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_UXTB_T2_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uxtb_t2_a_invalid_1_f080_fa5ff080() {
    // Thumb encoding (32): 0xFA5FF080
    // Test aarch32_UXTB_T2_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rm=0, Rd=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA5FF080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_UXTB_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 0 (32)
#[test]
fn test_aarch32_uxtb_t1_a_lslv_oracle_32_0_b2c20020() {
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

/// Provenance: aarch32_UXTB_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 0 (64)
#[test]
fn test_aarch32_uxtb_t1_a_lslv_oracle_64_0_b2c20020() {
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

/// Provenance: aarch32_UXTB_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4 (32)
#[test]
fn test_aarch32_uxtb_t1_a_lslv_oracle_32_1_b2c20020() {
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

/// Provenance: aarch32_UXTB_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 4 (64)
#[test]
fn test_aarch32_uxtb_t1_a_lslv_oracle_64_1_b2c20020() {
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

/// Provenance: aarch32_UXTB_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 8 (32)
#[test]
fn test_aarch32_uxtb_t1_a_lslv_oracle_32_2_b2c20020() {
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

/// Provenance: aarch32_UXTB_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 8 (64)
#[test]
fn test_aarch32_uxtb_t1_a_lslv_oracle_64_2_b2c20020() {
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

/// Provenance: aarch32_UXTB_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1 (32)
#[test]
fn test_aarch32_uxtb_t1_a_lslv_oracle_32_3_b2c20020() {
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

/// Provenance: aarch32_UXTB_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// MSB set, shift 1 (64)
#[test]
fn test_aarch32_uxtb_t1_a_lslv_oracle_64_3_b2c20020() {
    // Test LSLV 64-bit: MSB set, shift 1 (oracle)
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
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch32_UXTB_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSB set, max shift (32)
#[test]
fn test_aarch32_uxtb_t1_a_lslv_oracle_32_4_b2c20020() {
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

/// Provenance: aarch32_UXTB_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSB set, max shift (64)
#[test]
fn test_aarch32_uxtb_t1_a_lslv_oracle_64_4_b2c20020() {
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

/// Provenance: aarch32_UXTB_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// all ones, shift 32 (32)
#[test]
fn test_aarch32_uxtb_t1_a_lslv_oracle_32_5_b2c20020() {
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

/// Provenance: aarch32_UXTB_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all ones, shift 32 (64)
#[test]
fn test_aarch32_uxtb_t1_a_lslv_oracle_64_5_b2c20020() {
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

/// Provenance: aarch32_UXTB_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift
#[test]
fn test_aarch32_uxtb_t1_a_t16_oracle_0_b2d00000() {
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

/// Provenance: aarch32_UXTB_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4
#[test]
fn test_aarch32_uxtb_t1_a_t16_oracle_1_b2d00000() {
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

/// Provenance: aarch32_UXTB_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1
#[test]
fn test_aarch32_uxtb_t1_a_t16_oracle_2_b2d00000() {
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

/// Provenance: aarch32_UXTB_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift to MSB
#[test]
fn test_aarch32_uxtb_t1_a_t16_oracle_3_b2d00000() {
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

// ============================================================================
// aarch32_UXTB16_A Tests
// ============================================================================

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_uxtb16_a1_a_field_cond_0_min_70_06cf0070() {
    // Encoding: 0x06CF0070
    // Test aarch32_UXTB16_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x06CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_uxtb16_a1_a_field_cond_1_poweroftwo_70_16cf0070() {
    // Encoding: 0x16CF0070
    // Test aarch32_UXTB16_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rm=0, Rd=0, cond=1
    let encoding: u32 = 0x16CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_uxtb16_a1_a_field_cond_2_poweroftwo_70_26cf0070() {
    // Encoding: 0x26CF0070
    // Test aarch32_UXTB16_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, rotate=0, cond=2, Rm=0
    let encoding: u32 = 0x26CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_uxtb16_a1_a_field_cond_3_poweroftwo_70_36cf0070() {
    // Encoding: 0x36CF0070
    // Test aarch32_UXTB16_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, Rm=0, Rd=0, rotate=0
    let encoding: u32 = 0x36CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_uxtb16_a1_a_field_cond_4_poweroftwo_70_46cf0070() {
    // Encoding: 0x46CF0070
    // Test aarch32_UXTB16_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=4, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x46CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_uxtb16_a1_a_field_cond_5_poweroftwo_70_56cf0070() {
    // Encoding: 0x56CF0070
    // Test aarch32_UXTB16_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, cond=5, Rm=0, Rd=0
    let encoding: u32 = 0x56CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_uxtb16_a1_a_field_cond_6_poweroftwo_70_66cf0070() {
    // Encoding: 0x66CF0070
    // Test aarch32_UXTB16_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, rotate=0, cond=6
    let encoding: u32 = 0x66CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_uxtb16_a1_a_field_cond_7_poweroftwo_70_76cf0070() {
    // Encoding: 0x76CF0070
    // Test aarch32_UXTB16_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=7, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x76CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_uxtb16_a1_a_field_cond_8_poweroftwo_70_86cf0070() {
    // Encoding: 0x86CF0070
    // Test aarch32_UXTB16_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=8, rotate=0, Rm=0, Rd=0
    let encoding: u32 = 0x86CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_uxtb16_a1_a_field_cond_9_poweroftwo_70_96cf0070() {
    // Encoding: 0x96CF0070
    // Test aarch32_UXTB16_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=9, rotate=0
    let encoding: u32 = 0x96CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_uxtb16_a1_a_field_cond_10_poweroftwo_70_a6cf0070() {
    // Encoding: 0xA6CF0070
    // Test aarch32_UXTB16_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rm=0, cond=10, Rd=0
    let encoding: u32 = 0xA6CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_uxtb16_a1_a_field_cond_11_poweroftwo_70_b6cf0070() {
    // Encoding: 0xB6CF0070
    // Test aarch32_UXTB16_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, cond=11, Rm=0, Rd=0
    let encoding: u32 = 0xB6CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_uxtb16_a1_a_field_cond_12_poweroftwo_70_c6cf0070() {
    // Encoding: 0xC6CF0070
    // Test aarch32_UXTB16_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, Rd=0, Rm=0, rotate=0
    let encoding: u32 = 0xC6CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_uxtb16_a1_a_field_cond_13_poweroftwo_70_d6cf0070() {
    // Encoding: 0xD6CF0070
    // Test aarch32_UXTB16_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=13, Rd=0, Rm=0, rotate=0
    let encoding: u32 = 0xD6CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_uxtb16_a1_a_field_cond_14_poweroftwo_70_e6cf0070() {
    // Encoding: 0xE6CF0070
    // Test aarch32_UXTB16_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, rotate=0, Rm=0, cond=14
    let encoding: u32 = 0xE6CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_uxtb16_a1_a_field_cond_15_max_70_f6cf0070() {
    // Encoding: 0xF6CF0070
    // Test aarch32_UXTB16_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: rotate=0, Rm=0, cond=15, Rd=0
    let encoding: u32 = 0xF6CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtb16_a1_a_field_rd_0_min_70_06cf0070() {
    // Encoding: 0x06CF0070
    // Test aarch32_UXTB16_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rd=0, rotate=0
    let encoding: u32 = 0x06CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtb16_a1_a_field_rd_1_poweroftwo_70_06cf1070() {
    // Encoding: 0x06CF1070
    // Test aarch32_UXTB16_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rd=1, cond=0, Rm=0
    let encoding: u32 = 0x06CF1070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_uxtb16_a1_a_field_rotate_0_min_70_06cf0070() {
    // Encoding: 0x06CF0070
    // Test aarch32_UXTB16_A1_A field rotate = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, cond=0, rotate=0, Rm=0
    let encoding: u32 = 0x06CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_uxtb16_a1_a_field_rotate_1_poweroftwo_70_06cf0470() {
    // Encoding: 0x06CF0470
    // Test aarch32_UXTB16_A1_A field rotate = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, rotate=1, cond=0
    let encoding: u32 = 0x06CF0470;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_uxtb16_a1_a_field_rotate_3_max_70_06cf0c70() {
    // Encoding: 0x06CF0C70
    // Test aarch32_UXTB16_A1_A field rotate = 3 (Max)
    // ISET: A32
    // Fields: cond=0, Rm=0, rotate=3, Rd=0
    let encoding: u32 = 0x06CF0C70;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtb16_a1_a_field_rm_0_min_70_06cf0070() {
    // Encoding: 0x06CF0070
    // Test aarch32_UXTB16_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: cond=0, rotate=0, Rd=0, Rm=0
    let encoding: u32 = 0x06CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtb16_a1_a_field_rm_1_poweroftwo_70_06cf0071() {
    // Encoding: 0x06CF0071
    // Test aarch32_UXTB16_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, rotate=0, Rd=0, Rm=1
    let encoding: u32 = 0x06CF0071;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_uxtb16_a1_a_combo_0_70_06cf0070() {
    // Encoding: 0x06CF0070
    // Test aarch32_UXTB16_A1_A field combination: cond=0, Rd=0, rotate=0, Rm=0
    // ISET: A32
    // Fields: cond=0, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x06CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_uxtb16_a1_a_special_cond_0_condition_eq_112_06cf0070() {
    // Encoding: 0x06CF0070
    // Test aarch32_UXTB16_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=0, rotate=0
    let encoding: u32 = 0x06CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_uxtb16_a1_a_special_cond_1_condition_ne_112_16cf0070() {
    // Encoding: 0x16CF0070
    // Test aarch32_UXTB16_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rm=0, rotate=0, Rd=0, cond=1
    let encoding: u32 = 0x16CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_uxtb16_a1_a_special_cond_2_condition_cs_hs_112_26cf0070() {
    // Encoding: 0x26CF0070
    // Test aarch32_UXTB16_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, rotate=0, Rd=0, Rm=0
    let encoding: u32 = 0x26CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_uxtb16_a1_a_special_cond_3_condition_cc_lo_112_36cf0070() {
    // Encoding: 0x36CF0070
    // Test aarch32_UXTB16_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rm=0, rotate=0, Rd=0, cond=3
    let encoding: u32 = 0x36CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_uxtb16_a1_a_special_cond_4_condition_mi_112_46cf0070() {
    // Encoding: 0x46CF0070
    // Test aarch32_UXTB16_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rd=0, rotate=0, cond=4, Rm=0
    let encoding: u32 = 0x46CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_uxtb16_a1_a_special_cond_5_condition_pl_112_56cf0070() {
    // Encoding: 0x56CF0070
    // Test aarch32_UXTB16_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, Rm=0, rotate=0, Rd=0
    let encoding: u32 = 0x56CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_uxtb16_a1_a_special_cond_6_condition_vs_112_66cf0070() {
    // Encoding: 0x66CF0070
    // Test aarch32_UXTB16_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: rotate=0, Rm=0, cond=6, Rd=0
    let encoding: u32 = 0x66CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_uxtb16_a1_a_special_cond_7_condition_vc_112_76cf0070() {
    // Encoding: 0x76CF0070
    // Test aarch32_UXTB16_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, rotate=0, Rm=0, Rd=0
    let encoding: u32 = 0x76CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_uxtb16_a1_a_special_cond_8_condition_hi_112_86cf0070() {
    // Encoding: 0x86CF0070
    // Test aarch32_UXTB16_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: rotate=0, Rd=0, Rm=0, cond=8
    let encoding: u32 = 0x86CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_uxtb16_a1_a_special_cond_9_condition_ls_112_96cf0070() {
    // Encoding: 0x96CF0070
    // Test aarch32_UXTB16_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rd=0, Rm=0, rotate=0, cond=9
    let encoding: u32 = 0x96CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_uxtb16_a1_a_special_cond_10_condition_ge_112_a6cf0070() {
    // Encoding: 0xA6CF0070
    // Test aarch32_UXTB16_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=10, rotate=0
    let encoding: u32 = 0xA6CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_uxtb16_a1_a_special_cond_11_condition_lt_112_b6cf0070() {
    // Encoding: 0xB6CF0070
    // Test aarch32_UXTB16_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rd=0, rotate=0, cond=11, Rm=0
    let encoding: u32 = 0xB6CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_uxtb16_a1_a_special_cond_12_condition_gt_112_c6cf0070() {
    // Encoding: 0xC6CF0070
    // Test aarch32_UXTB16_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rm=0, rotate=0, Rd=0, cond=12
    let encoding: u32 = 0xC6CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_uxtb16_a1_a_special_cond_13_condition_le_112_d6cf0070() {
    // Encoding: 0xD6CF0070
    // Test aarch32_UXTB16_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=13, rotate=0
    let encoding: u32 = 0xD6CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_uxtb16_a1_a_special_cond_14_condition_al_112_e6cf0070() {
    // Encoding: 0xE6CF0070
    // Test aarch32_UXTB16_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rd=0, Rm=0, rotate=0, cond=14
    let encoding: u32 = 0xE6CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_uxtb16_a1_a_special_cond_15_condition_nv_112_f6cf0070() {
    // Encoding: 0xF6CF0070
    // Test aarch32_UXTB16_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rd=0, Rm=0, rotate=0, cond=15
    let encoding: u32 = 0xF6CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uxtb16_a1_a_invalid_0_70_06cf0070() {
    // Encoding: 0x06CF0070
    // Test aarch32_UXTB16_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=0, rotate=0
    let encoding: u32 = 0x06CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UXTB16_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uxtb16_a1_a_invalid_1_70_06cf0070() {
    // Encoding: 0x06CF0070
    // Test aarch32_UXTB16_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: cond=0, rotate=0, Rm=0, Rd=0
    let encoding: u32 = 0x06CF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UXTB16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtb16_t1_a_field_rd_0_min_f080_fa3ff080() {
    // Thumb encoding (32): 0xFA3FF080
    // Test aarch32_UXTB16_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, rotate=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA3FF080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTB16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtb16_t1_a_field_rd_1_poweroftwo_f080_fa3ff180() {
    // Thumb encoding (32): 0xFA3FF180
    // Test aarch32_UXTB16_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: rotate=0, Rm=0, Rd=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA3FF180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTB16_T1_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_uxtb16_t1_a_field_rotate_0_min_f080_fa3ff080() {
    // Thumb encoding (32): 0xFA3FF080
    // Test aarch32_UXTB16_T1_A field rotate = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rd=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA3FF080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTB16_T1_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_uxtb16_t1_a_field_rotate_1_poweroftwo_f080_fa3ff090() {
    // Thumb encoding (32): 0xFA3FF090
    // Test aarch32_UXTB16_T1_A field rotate = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rd=0, rotate=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA3FF090;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTB16_T1_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_uxtb16_t1_a_field_rotate_3_max_f080_fa3ff0b0() {
    // Thumb encoding (32): 0xFA3FF0B0
    // Test aarch32_UXTB16_T1_A field rotate = 3 (Max)
    // ISET: T32
    // Fields: Rm=0, rotate=3, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA3FF0B0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTB16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtb16_t1_a_field_rm_0_min_f080_fa3ff080() {
    // Thumb encoding (32): 0xFA3FF080
    // Test aarch32_UXTB16_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: rotate=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA3FF080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTB16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtb16_t1_a_field_rm_1_poweroftwo_f080_fa3ff081() {
    // Thumb encoding (32): 0xFA3FF081
    // Test aarch32_UXTB16_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: rotate=0, Rm=1, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA3FF081;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTB16_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch32_uxtb16_t1_a_combo_0_f080_fa3ff080() {
    // Thumb encoding (32): 0xFA3FF080
    // Test aarch32_UXTB16_T1_A field combination: Rd=0, rotate=0, Rm=0
    // ISET: T32
    // Fields: Rd=0, Rm=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA3FF080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTB16_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uxtb16_t1_a_invalid_0_f080_fa3ff080() {
    // Thumb encoding (32): 0xFA3FF080
    // Test aarch32_UXTB16_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rm=0, Rd=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA3FF080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_UXTB16_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uxtb16_t1_a_invalid_1_f080_fa3ff080() {
    // Thumb encoding (32): 0xFA3FF080
    // Test aarch32_UXTB16_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rd=0, rotate=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA3FF080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_UXTH_A Tests
// ============================================================================

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_uxth_a1_a_field_cond_0_min_70_06ff0070() {
    // Encoding: 0x06FF0070
    // Test aarch32_UXTH_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, rotate=0, Rd=0, Rm=0
    let encoding: u32 = 0x06FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_uxth_a1_a_field_cond_1_poweroftwo_70_16ff0070() {
    // Encoding: 0x16FF0070
    // Test aarch32_UXTH_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, cond=1, Rm=0, Rd=0
    let encoding: u32 = 0x16FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_uxth_a1_a_field_cond_2_poweroftwo_70_26ff0070() {
    // Encoding: 0x26FF0070
    // Test aarch32_UXTH_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=2, Rm=0, rotate=0
    let encoding: u32 = 0x26FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_uxth_a1_a_field_cond_3_poweroftwo_70_36ff0070() {
    // Encoding: 0x36FF0070
    // Test aarch32_UXTH_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=3, rotate=0, Rm=0
    let encoding: u32 = 0x36FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_uxth_a1_a_field_cond_4_poweroftwo_70_46ff0070() {
    // Encoding: 0x46FF0070
    // Test aarch32_UXTH_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=4, rotate=0
    let encoding: u32 = 0x46FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_uxth_a1_a_field_cond_5_poweroftwo_70_56ff0070() {
    // Encoding: 0x56FF0070
    // Test aarch32_UXTH_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=5, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x56FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_uxth_a1_a_field_cond_6_poweroftwo_70_66ff0070() {
    // Encoding: 0x66FF0070
    // Test aarch32_UXTH_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, rotate=0, cond=6, Rd=0
    let encoding: u32 = 0x66FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_uxth_a1_a_field_cond_7_poweroftwo_70_76ff0070() {
    // Encoding: 0x76FF0070
    // Test aarch32_UXTH_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=7, Rm=0, Rd=0, rotate=0
    let encoding: u32 = 0x76FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_uxth_a1_a_field_cond_8_poweroftwo_70_86ff0070() {
    // Encoding: 0x86FF0070
    // Test aarch32_UXTH_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=8, Rd=0, rotate=0
    let encoding: u32 = 0x86FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_uxth_a1_a_field_cond_9_poweroftwo_70_96ff0070() {
    // Encoding: 0x96FF0070
    // Test aarch32_UXTH_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, rotate=0, Rd=0, Rm=0
    let encoding: u32 = 0x96FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_uxth_a1_a_field_cond_10_poweroftwo_70_a6ff0070() {
    // Encoding: 0xA6FF0070
    // Test aarch32_UXTH_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, rotate=0, Rd=0, cond=10
    let encoding: u32 = 0xA6FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_uxth_a1_a_field_cond_11_poweroftwo_70_b6ff0070() {
    // Encoding: 0xB6FF0070
    // Test aarch32_UXTH_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=11, rotate=0, Rm=0
    let encoding: u32 = 0xB6FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_uxth_a1_a_field_cond_12_poweroftwo_70_c6ff0070() {
    // Encoding: 0xC6FF0070
    // Test aarch32_UXTH_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rd=0, cond=12, Rm=0
    let encoding: u32 = 0xC6FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_uxth_a1_a_field_cond_13_poweroftwo_70_d6ff0070() {
    // Encoding: 0xD6FF0070
    // Test aarch32_UXTH_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=13, rotate=0, Rd=0
    let encoding: u32 = 0xD6FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_uxth_a1_a_field_cond_14_poweroftwo_70_e6ff0070() {
    // Encoding: 0xE6FF0070
    // Test aarch32_UXTH_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=14, rotate=0
    let encoding: u32 = 0xE6FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_uxth_a1_a_field_cond_15_max_70_f6ff0070() {
    // Encoding: 0xF6FF0070
    // Test aarch32_UXTH_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rm=0, rotate=0, cond=15, Rd=0
    let encoding: u32 = 0xF6FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxth_a1_a_field_rd_0_min_70_06ff0070() {
    // Encoding: 0x06FF0070
    // Test aarch32_UXTH_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: rotate=0, Rm=0, Rd=0, cond=0
    let encoding: u32 = 0x06FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxth_a1_a_field_rd_1_poweroftwo_70_06ff1070() {
    // Encoding: 0x06FF1070
    // Test aarch32_UXTH_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=1, rotate=0, Rm=0, cond=0
    let encoding: u32 = 0x06FF1070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_uxth_a1_a_field_rotate_0_min_70_06ff0070() {
    // Encoding: 0x06FF0070
    // Test aarch32_UXTH_A1_A field rotate = 0 (Min)
    // ISET: A32
    // Fields: rotate=0, Rd=0, Rm=0, cond=0
    let encoding: u32 = 0x06FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_uxth_a1_a_field_rotate_1_poweroftwo_70_06ff0470() {
    // Encoding: 0x06FF0470
    // Test aarch32_UXTH_A1_A field rotate = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, rotate=1, Rd=0, Rm=0
    let encoding: u32 = 0x06FF0470;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_uxth_a1_a_field_rotate_3_max_70_06ff0c70() {
    // Encoding: 0x06FF0C70
    // Test aarch32_UXTH_A1_A field rotate = 3 (Max)
    // ISET: A32
    // Fields: Rm=0, Rd=0, rotate=3, cond=0
    let encoding: u32 = 0x06FF0C70;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxth_a1_a_field_rm_0_min_70_06ff0070() {
    // Encoding: 0x06FF0070
    // Test aarch32_UXTH_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: rotate=0, cond=0, Rm=0, Rd=0
    let encoding: u32 = 0x06FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxth_a1_a_field_rm_1_poweroftwo_70_06ff0071() {
    // Encoding: 0x06FF0071
    // Test aarch32_UXTH_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, rotate=0, Rd=0, Rm=1
    let encoding: u32 = 0x06FF0071;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_uxth_a1_a_combo_0_70_06ff0070() {
    // Encoding: 0x06FF0070
    // Test aarch32_UXTH_A1_A field combination: cond=0, Rd=0, rotate=0, Rm=0
    // ISET: A32
    // Fields: rotate=0, cond=0, Rm=0, Rd=0
    let encoding: u32 = 0x06FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_uxth_a1_a_special_cond_0_condition_eq_112_06ff0070() {
    // Encoding: 0x06FF0070
    // Test aarch32_UXTH_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: cond=0, rotate=0, Rm=0, Rd=0
    let encoding: u32 = 0x06FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_uxth_a1_a_special_cond_1_condition_ne_112_16ff0070() {
    // Encoding: 0x16FF0070
    // Test aarch32_UXTH_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rm=0, cond=1, Rd=0, rotate=0
    let encoding: u32 = 0x16FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_uxth_a1_a_special_cond_2_condition_cs_hs_112_26ff0070() {
    // Encoding: 0x26FF0070
    // Test aarch32_UXTH_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=2, rotate=0
    let encoding: u32 = 0x26FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_uxth_a1_a_special_cond_3_condition_cc_lo_112_36ff0070() {
    // Encoding: 0x36FF0070
    // Test aarch32_UXTH_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: rotate=0, cond=3, Rd=0, Rm=0
    let encoding: u32 = 0x36FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_uxth_a1_a_special_cond_4_condition_mi_112_46ff0070() {
    // Encoding: 0x46FF0070
    // Test aarch32_UXTH_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=4, rotate=0
    let encoding: u32 = 0x46FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_uxth_a1_a_special_cond_5_condition_pl_112_56ff0070() {
    // Encoding: 0x56FF0070
    // Test aarch32_UXTH_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: rotate=0, cond=5, Rd=0, Rm=0
    let encoding: u32 = 0x56FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_uxth_a1_a_special_cond_6_condition_vs_112_66ff0070() {
    // Encoding: 0x66FF0070
    // Test aarch32_UXTH_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, Rm=0, Rd=0, rotate=0
    let encoding: u32 = 0x66FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_uxth_a1_a_special_cond_7_condition_vc_112_76ff0070() {
    // Encoding: 0x76FF0070
    // Test aarch32_UXTH_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, rotate=0, Rm=0, Rd=0
    let encoding: u32 = 0x76FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_uxth_a1_a_special_cond_8_condition_hi_112_86ff0070() {
    // Encoding: 0x86FF0070
    // Test aarch32_UXTH_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, rotate=0, Rm=0, Rd=0
    let encoding: u32 = 0x86FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_uxth_a1_a_special_cond_9_condition_ls_112_96ff0070() {
    // Encoding: 0x96FF0070
    // Test aarch32_UXTH_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: rotate=0, cond=9, Rd=0, Rm=0
    let encoding: u32 = 0x96FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_uxth_a1_a_special_cond_10_condition_ge_112_a6ff0070() {
    // Encoding: 0xA6FF0070
    // Test aarch32_UXTH_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rd=0, Rm=0, rotate=0, cond=10
    let encoding: u32 = 0xA6FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_uxth_a1_a_special_cond_11_condition_lt_112_b6ff0070() {
    // Encoding: 0xB6FF0070
    // Test aarch32_UXTH_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rd=0, cond=11, Rm=0, rotate=0
    let encoding: u32 = 0xB6FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_uxth_a1_a_special_cond_12_condition_gt_112_c6ff0070() {
    // Encoding: 0xC6FF0070
    // Test aarch32_UXTH_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: cond=12, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0xC6FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_uxth_a1_a_special_cond_13_condition_le_112_d6ff0070() {
    // Encoding: 0xD6FF0070
    // Test aarch32_UXTH_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rm=0, rotate=0, cond=13, Rd=0
    let encoding: u32 = 0xD6FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_uxth_a1_a_special_cond_14_condition_al_112_e6ff0070() {
    // Encoding: 0xE6FF0070
    // Test aarch32_UXTH_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rm=0, cond=14, Rd=0, rotate=0
    let encoding: u32 = 0xE6FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_uxth_a1_a_special_cond_15_condition_nv_112_f6ff0070() {
    // Encoding: 0xF6FF0070
    // Test aarch32_UXTH_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, rotate=0, Rd=0, Rm=0
    let encoding: u32 = 0xF6FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uxth_a1_a_invalid_0_70_06ff0070() {
    // Encoding: 0x06FF0070
    // Test aarch32_UXTH_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: rotate=0, Rm=0, Rd=0, cond=0
    let encoding: u32 = 0x06FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UXTH_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uxth_a1_a_invalid_1_70_06ff0070() {
    // Encoding: 0x06FF0070
    // Test aarch32_UXTH_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: cond=0, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x06FF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UXTH_T1_A
/// ASL: `field Rm 19 +: 3`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxth_t1_a_field_rm_0_min_0_b2800000() {
    // Thumb encoding (32): 0xB2800000
    // Test aarch32_UXTH_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB2800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTH_T1_A
/// ASL: `field Rm 19 +: 3`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxth_t1_a_field_rm_1_poweroftwo_0_b2880000() {
    // Thumb encoding (32): 0xB2880000
    // Test aarch32_UXTH_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB2880000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTH_T1_A
/// ASL: `field Rd 16 +: 3`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxth_t1_a_field_rd_0_min_0_b2800000() {
    // Thumb encoding (32): 0xB2800000
    // Test aarch32_UXTH_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB2800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTH_T1_A
/// ASL: `field Rd 16 +: 3`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxth_t1_a_field_rd_1_poweroftwo_0_b2810000() {
    // Thumb encoding (32): 0xB2810000
    // Test aarch32_UXTH_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=1, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB2810000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTH_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch32_uxth_t1_a_combo_0_0_b2800000() {
    // Thumb encoding (32): 0xB2800000
    // Test aarch32_UXTH_T1_A field combination: Rm=0, Rd=0
    // ISET: T32
    // Fields: Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB2800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTH_T2_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxth_t2_a_field_rd_0_min_f080_fa1ff080() {
    // Thumb encoding (32): 0xFA1FF080
    // Test aarch32_UXTH_T2_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: rotate=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA1FF080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTH_T2_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxth_t2_a_field_rd_1_poweroftwo_f080_fa1ff180() {
    // Thumb encoding (32): 0xFA1FF180
    // Test aarch32_UXTH_T2_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: rotate=0, Rd=1, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA1FF180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTH_T2_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_uxth_t2_a_field_rotate_0_min_f080_fa1ff080() {
    // Thumb encoding (32): 0xFA1FF080
    // Test aarch32_UXTH_T2_A field rotate = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, rotate=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA1FF080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTH_T2_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_uxth_t2_a_field_rotate_1_poweroftwo_f080_fa1ff090() {
    // Thumb encoding (32): 0xFA1FF090
    // Test aarch32_UXTH_T2_A field rotate = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, rotate=1, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA1FF090;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTH_T2_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_uxth_t2_a_field_rotate_3_max_f080_fa1ff0b0() {
    // Thumb encoding (32): 0xFA1FF0B0
    // Test aarch32_UXTH_T2_A field rotate = 3 (Max)
    // ISET: T32
    // Fields: Rm=0, Rd=0, rotate=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA1FF0B0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTH_T2_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxth_t2_a_field_rm_0_min_f080_fa1ff080() {
    // Thumb encoding (32): 0xFA1FF080
    // Test aarch32_UXTH_T2_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, rotate=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA1FF080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTH_T2_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxth_t2_a_field_rm_1_poweroftwo_f080_fa1ff081() {
    // Thumb encoding (32): 0xFA1FF081
    // Test aarch32_UXTH_T2_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: rotate=0, Rm=1, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA1FF081;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTH_T2_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch32_uxth_t2_a_combo_0_f080_fa1ff080() {
    // Thumb encoding (32): 0xFA1FF080
    // Test aarch32_UXTH_T2_A field combination: Rd=0, rotate=0, Rm=0
    // ISET: T32
    // Fields: Rm=0, Rd=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA1FF080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTH_T2_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uxth_t2_a_invalid_0_f080_fa1ff080() {
    // Thumb encoding (32): 0xFA1FF080
    // Test aarch32_UXTH_T2_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rd=0, Rm=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA1FF080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_UXTH_T2_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uxth_t2_a_invalid_1_f080_fa1ff080() {
    // Thumb encoding (32): 0xFA1FF080
    // Test aarch32_UXTH_T2_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: rotate=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA1FF080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_UXTH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 0 (32)
#[test]
fn test_aarch32_uxth_t1_a_lslv_oracle_32_0_b2820020() {
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

/// Provenance: aarch32_UXTH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 0 (64)
#[test]
fn test_aarch32_uxth_t1_a_lslv_oracle_64_0_b2820020() {
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

/// Provenance: aarch32_UXTH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4 (32)
#[test]
fn test_aarch32_uxth_t1_a_lslv_oracle_32_1_b2820020() {
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

/// Provenance: aarch32_UXTH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 4 (64)
#[test]
fn test_aarch32_uxth_t1_a_lslv_oracle_64_1_b2820020() {
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

/// Provenance: aarch32_UXTH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 8 (32)
#[test]
fn test_aarch32_uxth_t1_a_lslv_oracle_32_2_b2820020() {
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

/// Provenance: aarch32_UXTH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 8 (64)
#[test]
fn test_aarch32_uxth_t1_a_lslv_oracle_64_2_b2820020() {
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

/// Provenance: aarch32_UXTH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1 (32)
#[test]
fn test_aarch32_uxth_t1_a_lslv_oracle_32_3_b2820020() {
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

/// Provenance: aarch32_UXTH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// MSB set, shift 1 (64)
#[test]
fn test_aarch32_uxth_t1_a_lslv_oracle_64_3_b2820020() {
    // Test LSLV 64-bit: MSB set, shift 1 (oracle)
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
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch32_UXTH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSB set, max shift (32)
#[test]
fn test_aarch32_uxth_t1_a_lslv_oracle_32_4_b2820020() {
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

/// Provenance: aarch32_UXTH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSB set, max shift (64)
#[test]
fn test_aarch32_uxth_t1_a_lslv_oracle_64_4_b2820020() {
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

/// Provenance: aarch32_UXTH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// all ones, shift 32 (32)
#[test]
fn test_aarch32_uxth_t1_a_lslv_oracle_32_5_b2820020() {
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

/// Provenance: aarch32_UXTH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all ones, shift 32 (64)
#[test]
fn test_aarch32_uxth_t1_a_lslv_oracle_64_5_b2820020() {
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

/// Provenance: aarch32_UXTH_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift
#[test]
fn test_aarch32_uxth_t1_a_t16_oracle_0_b2900000() {
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

/// Provenance: aarch32_UXTH_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4
#[test]
fn test_aarch32_uxth_t1_a_t16_oracle_1_b2900000() {
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

/// Provenance: aarch32_UXTH_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1
#[test]
fn test_aarch32_uxth_t1_a_t16_oracle_2_b2900000() {
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

/// Provenance: aarch32_UXTH_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift to MSB
#[test]
fn test_aarch32_uxth_t1_a_t16_oracle_3_b2900000() {
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

// ============================================================================
// aarch32_SXTAH_A Tests
// ============================================================================

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_sxtah_a1_a_field_cond_0_min_70_06b00070() {
    // Encoding: 0x06B00070
    // Test aarch32_SXTAH_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: rotate=0, Rm=0, Rd=0, cond=0, Rn=0
    let encoding: u32 = 0x06B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_sxtah_a1_a_field_cond_1_poweroftwo_70_16b00070() {
    // Encoding: 0x16B00070
    // Test aarch32_SXTAH_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, Rn=0, Rm=0, Rd=0, rotate=0
    let encoding: u32 = 0x16B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_sxtah_a1_a_field_cond_2_poweroftwo_70_26b00070() {
    // Encoding: 0x26B00070
    // Test aarch32_SXTAH_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rm=0, cond=2, Rn=0, Rd=0
    let encoding: u32 = 0x26B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_sxtah_a1_a_field_cond_3_poweroftwo_70_36b00070() {
    // Encoding: 0x36B00070
    // Test aarch32_SXTAH_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, rotate=0, Rm=0, cond=3, Rd=0
    let encoding: u32 = 0x36B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_sxtah_a1_a_field_cond_4_poweroftwo_70_46b00070() {
    // Encoding: 0x46B00070
    // Test aarch32_SXTAH_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rd=0, Rn=0, cond=4, Rm=0
    let encoding: u32 = 0x46B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_sxtah_a1_a_field_cond_5_poweroftwo_70_56b00070() {
    // Encoding: 0x56B00070
    // Test aarch32_SXTAH_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, rotate=0, cond=5, Rn=0
    let encoding: u32 = 0x56B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_sxtah_a1_a_field_cond_6_poweroftwo_70_66b00070() {
    // Encoding: 0x66B00070
    // Test aarch32_SXTAH_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rn=0, Rd=0, cond=6, Rm=0
    let encoding: u32 = 0x66B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_sxtah_a1_a_field_cond_7_poweroftwo_70_76b00070() {
    // Encoding: 0x76B00070
    // Test aarch32_SXTAH_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, cond=7, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x76B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_sxtah_a1_a_field_cond_8_poweroftwo_70_86b00070() {
    // Encoding: 0x86B00070
    // Test aarch32_SXTAH_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rn=0, cond=8, Rm=0, Rd=0
    let encoding: u32 = 0x86B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_sxtah_a1_a_field_cond_9_poweroftwo_70_96b00070() {
    // Encoding: 0x96B00070
    // Test aarch32_SXTAH_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rd=0, Rn=0, cond=9, Rm=0
    let encoding: u32 = 0x96B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_sxtah_a1_a_field_cond_10_poweroftwo_70_a6b00070() {
    // Encoding: 0xA6B00070
    // Test aarch32_SXTAH_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, rotate=0, cond=10
    let encoding: u32 = 0xA6B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_sxtah_a1_a_field_cond_11_poweroftwo_70_b6b00070() {
    // Encoding: 0xB6B00070
    // Test aarch32_SXTAH_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=11, Rn=0, rotate=0, Rm=0
    let encoding: u32 = 0xB6B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_sxtah_a1_a_field_cond_12_poweroftwo_70_c6b00070() {
    // Encoding: 0xC6B00070
    // Test aarch32_SXTAH_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, Rd=0, Rn=0, rotate=0, Rm=0
    let encoding: u32 = 0xC6B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_sxtah_a1_a_field_cond_13_poweroftwo_70_d6b00070() {
    // Encoding: 0xD6B00070
    // Test aarch32_SXTAH_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=13, Rd=0, rotate=0, Rm=0, Rn=0
    let encoding: u32 = 0xD6B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_sxtah_a1_a_field_cond_14_poweroftwo_70_e6b00070() {
    // Encoding: 0xE6B00070
    // Test aarch32_SXTAH_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, Rn=0, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0xE6B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_sxtah_a1_a_field_cond_15_max_70_f6b00070() {
    // Encoding: 0xF6B00070
    // Test aarch32_SXTAH_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rn=0, cond=15, Rd=0, Rm=0, rotate=0
    let encoding: u32 = 0xF6B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtah_a1_a_field_rn_0_min_70_06b00070() {
    // Encoding: 0x06B00070
    // Test aarch32_SXTAH_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=0, Rm=0, rotate=0
    let encoding: u32 = 0x06B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtah_a1_a_field_rn_1_poweroftwo_70_06b10070() {
    // Encoding: 0x06B10070
    // Test aarch32_SXTAH_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=1, Rd=0, rotate=0, cond=0, Rm=0
    let encoding: u32 = 0x06B10070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtah_a1_a_field_rd_0_min_70_06b00070() {
    // Encoding: 0x06B00070
    // Test aarch32_SXTAH_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x06B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtah_a1_a_field_rd_1_poweroftwo_70_06b01070() {
    // Encoding: 0x06B01070
    // Test aarch32_SXTAH_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, rotate=0, Rd=1
    let encoding: u32 = 0x06B01070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_sxtah_a1_a_field_rotate_0_min_70_06b00070() {
    // Encoding: 0x06B00070
    // Test aarch32_SXTAH_A1_A field rotate = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, rotate=0, Rn=0, Rm=0
    let encoding: u32 = 0x06B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_sxtah_a1_a_field_rotate_1_poweroftwo_70_06b00470() {
    // Encoding: 0x06B00470
    // Test aarch32_SXTAH_A1_A field rotate = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, rotate=1, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x06B00470;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_sxtah_a1_a_field_rotate_3_max_70_06b00c70() {
    // Encoding: 0x06B00C70
    // Test aarch32_SXTAH_A1_A field rotate = 3 (Max)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=0, rotate=3, Rm=0
    let encoding: u32 = 0x06B00C70;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtah_a1_a_field_rm_0_min_70_06b00070() {
    // Encoding: 0x06B00070
    // Test aarch32_SXTAH_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=0, Rn=0, rotate=0
    let encoding: u32 = 0x06B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtah_a1_a_field_rm_1_poweroftwo_70_06b00071() {
    // Encoding: 0x06B00071
    // Test aarch32_SXTAH_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=1, Rn=0, cond=0, rotate=0
    let encoding: u32 = 0x06B00071;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_sxtah_a1_a_combo_0_70_06b00070() {
    // Encoding: 0x06B00070
    // Test aarch32_SXTAH_A1_A field combination: cond=0, Rn=0, Rd=0, rotate=0, Rm=0
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=0, Rd=0, rotate=0
    let encoding: u32 = 0x06B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_sxtah_a1_a_special_cond_0_condition_eq_112_06b00070() {
    // Encoding: 0x06B00070
    // Test aarch32_SXTAH_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: rotate=0, cond=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x06B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_sxtah_a1_a_special_cond_1_condition_ne_112_16b00070() {
    // Encoding: 0x16B00070
    // Test aarch32_SXTAH_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=1, rotate=0
    let encoding: u32 = 0x16B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_sxtah_a1_a_special_cond_2_condition_cs_hs_112_26b00070() {
    // Encoding: 0x26B00070
    // Test aarch32_SXTAH_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: rotate=0, cond=2, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x26B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_sxtah_a1_a_special_cond_3_condition_cc_lo_112_36b00070() {
    // Encoding: 0x36B00070
    // Test aarch32_SXTAH_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, Rd=0, Rn=0, Rm=0, rotate=0
    let encoding: u32 = 0x36B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_sxtah_a1_a_special_cond_4_condition_mi_112_46b00070() {
    // Encoding: 0x46B00070
    // Test aarch32_SXTAH_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, rotate=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x46B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_sxtah_a1_a_special_cond_5_condition_pl_112_56b00070() {
    // Encoding: 0x56B00070
    // Test aarch32_SXTAH_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rm=0, cond=5, Rn=0, rotate=0, Rd=0
    let encoding: u32 = 0x56B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_sxtah_a1_a_special_cond_6_condition_vs_112_66b00070() {
    // Encoding: 0x66B00070
    // Test aarch32_SXTAH_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rn=0, Rm=0, rotate=0, Rd=0, cond=6
    let encoding: u32 = 0x66B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_sxtah_a1_a_special_cond_7_condition_vc_112_76b00070() {
    // Encoding: 0x76B00070
    // Test aarch32_SXTAH_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, rotate=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x76B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_sxtah_a1_a_special_cond_8_condition_hi_112_86b00070() {
    // Encoding: 0x86B00070
    // Test aarch32_SXTAH_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rm=0, Rn=0, rotate=0, Rd=0, cond=8
    let encoding: u32 = 0x86B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_sxtah_a1_a_special_cond_9_condition_ls_112_96b00070() {
    // Encoding: 0x96B00070
    // Test aarch32_SXTAH_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rd=0, Rn=0, rotate=0, Rm=0, cond=9
    let encoding: u32 = 0x96B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_sxtah_a1_a_special_cond_10_condition_ge_112_a6b00070() {
    // Encoding: 0xA6B00070
    // Test aarch32_SXTAH_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: rotate=0, Rn=0, Rd=0, Rm=0, cond=10
    let encoding: u32 = 0xA6B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_sxtah_a1_a_special_cond_11_condition_lt_112_b6b00070() {
    // Encoding: 0xB6B00070
    // Test aarch32_SXTAH_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: rotate=0, Rn=0, Rm=0, Rd=0, cond=11
    let encoding: u32 = 0xB6B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_sxtah_a1_a_special_cond_12_condition_gt_112_c6b00070() {
    // Encoding: 0xC6B00070
    // Test aarch32_SXTAH_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: rotate=0, Rd=0, cond=12, Rn=0, Rm=0
    let encoding: u32 = 0xC6B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_sxtah_a1_a_special_cond_13_condition_le_112_d6b00070() {
    // Encoding: 0xD6B00070
    // Test aarch32_SXTAH_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=13, rotate=0
    let encoding: u32 = 0xD6B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_sxtah_a1_a_special_cond_14_condition_al_112_e6b00070() {
    // Encoding: 0xE6B00070
    // Test aarch32_SXTAH_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: rotate=0, Rn=0, cond=14, Rm=0, Rd=0
    let encoding: u32 = 0xE6B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_sxtah_a1_a_special_cond_15_condition_nv_112_f6b00070() {
    // Encoding: 0xF6B00070
    // Test aarch32_SXTAH_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, Rn=0, rotate=0, Rd=0, Rm=0
    let encoding: u32 = 0xF6B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sxtah_a1_a_invalid_0_70_06b00070() {
    // Encoding: 0x06B00070
    // Test aarch32_SXTAH_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: cond=0, Rm=0, Rn=0, rotate=0, Rd=0
    let encoding: u32 = 0x06B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SXTAH_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sxtah_a1_a_invalid_1_70_06b00070() {
    // Encoding: 0x06B00070
    // Test aarch32_SXTAH_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rd=0, cond=0, Rn=0, rotate=0, Rm=0
    let encoding: u32 = 0x06B00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SXTAH_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtah_t1_a_field_rn_0_min_f080_fa00f080() {
    // Thumb encoding (32): 0xFA00F080
    // Test aarch32_SXTAH_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, rotate=0, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA00F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAH_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtah_t1_a_field_rn_1_poweroftwo_f080_fa01f080() {
    // Thumb encoding (32): 0xFA01F080
    // Test aarch32_SXTAH_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: rotate=0, Rn=1, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA01F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAH_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtah_t1_a_field_rd_0_min_f080_fa00f080() {
    // Thumb encoding (32): 0xFA00F080
    // Test aarch32_SXTAH_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, rotate=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA00F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAH_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtah_t1_a_field_rd_1_poweroftwo_f080_fa00f180() {
    // Thumb encoding (32): 0xFA00F180
    // Test aarch32_SXTAH_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, rotate=0, Rd=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA00F180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAH_T1_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_sxtah_t1_a_field_rotate_0_min_f080_fa00f080() {
    // Thumb encoding (32): 0xFA00F080
    // Test aarch32_SXTAH_T1_A field rotate = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, rotate=0, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA00F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAH_T1_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_sxtah_t1_a_field_rotate_1_poweroftwo_f080_fa00f090() {
    // Thumb encoding (32): 0xFA00F090
    // Test aarch32_SXTAH_T1_A field rotate = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: rotate=1, Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA00F090;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAH_T1_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_sxtah_t1_a_field_rotate_3_max_f080_fa00f0b0() {
    // Thumb encoding (32): 0xFA00F0B0
    // Test aarch32_SXTAH_T1_A field rotate = 3 (Max)
    // ISET: T32
    // Fields: Rd=0, Rm=0, rotate=3, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA00F0B0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAH_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtah_t1_a_field_rm_0_min_f080_fa00f080() {
    // Thumb encoding (32): 0xFA00F080
    // Test aarch32_SXTAH_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, rotate=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA00F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAH_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtah_t1_a_field_rm_1_poweroftwo_f080_fa00f081() {
    // Thumb encoding (32): 0xFA00F081
    // Test aarch32_SXTAH_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1, Rd=0, Rn=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA00F081;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAH_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_sxtah_t1_a_combo_0_f080_fa00f080() {
    // Thumb encoding (32): 0xFA00F080
    // Test aarch32_SXTAH_T1_A field combination: Rn=0, Rd=0, rotate=0, Rm=0
    // ISET: T32
    // Fields: Rm=0, rotate=0, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA00F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAH_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sxtah_t1_a_invalid_0_f080_fa00f080() {
    // Thumb encoding (32): 0xFA00F080
    // Test aarch32_SXTAH_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rd=0, Rn=0, rotate=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA00F080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_SXTAH_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sxtah_t1_a_invalid_1_f080_fa00f080() {
    // Thumb encoding (32): 0xFA00F080
    // Test aarch32_SXTAH_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rn=0, Rm=0, rotate=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA00F080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_SXTAB16_A Tests
// ============================================================================

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_sxtab16_a1_a_field_cond_0_min_70_06800070() {
    // Encoding: 0x06800070
    // Test aarch32_SXTAB16_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, rotate=0, Rd=0, cond=0, Rm=0
    let encoding: u32 = 0x06800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_sxtab16_a1_a_field_cond_1_poweroftwo_70_16800070() {
    // Encoding: 0x16800070
    // Test aarch32_SXTAB16_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=1, Rm=0, Rn=0, rotate=0
    let encoding: u32 = 0x16800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_sxtab16_a1_a_field_cond_2_poweroftwo_70_26800070() {
    // Encoding: 0x26800070
    // Test aarch32_SXTAB16_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=2, Rn=0, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x26800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_sxtab16_a1_a_field_cond_3_poweroftwo_70_36800070() {
    // Encoding: 0x36800070
    // Test aarch32_SXTAB16_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rd=0, Rm=0, cond=3, Rn=0
    let encoding: u32 = 0x36800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_sxtab16_a1_a_field_cond_4_poweroftwo_70_46800070() {
    // Encoding: 0x46800070
    // Test aarch32_SXTAB16_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=4, Rm=0, rotate=0, Rn=0
    let encoding: u32 = 0x46800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_sxtab16_a1_a_field_cond_5_poweroftwo_70_56800070() {
    // Encoding: 0x56800070
    // Test aarch32_SXTAB16_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, rotate=0, cond=5, Rm=0, Rn=0
    let encoding: u32 = 0x56800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_sxtab16_a1_a_field_cond_6_poweroftwo_70_66800070() {
    // Encoding: 0x66800070
    // Test aarch32_SXTAB16_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=6, rotate=0, Rm=0
    let encoding: u32 = 0x66800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_sxtab16_a1_a_field_cond_7_poweroftwo_70_76800070() {
    // Encoding: 0x76800070
    // Test aarch32_SXTAB16_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=7, Rn=0, Rd=0, rotate=0
    let encoding: u32 = 0x76800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_sxtab16_a1_a_field_cond_8_poweroftwo_70_86800070() {
    // Encoding: 0x86800070
    // Test aarch32_SXTAB16_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, rotate=0, Rm=0, Rn=0, cond=8
    let encoding: u32 = 0x86800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_sxtab16_a1_a_field_cond_9_poweroftwo_70_96800070() {
    // Encoding: 0x96800070
    // Test aarch32_SXTAB16_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=9, Rn=0, Rd=0, rotate=0
    let encoding: u32 = 0x96800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_sxtab16_a1_a_field_cond_10_poweroftwo_70_a6800070() {
    // Encoding: 0xA6800070
    // Test aarch32_SXTAB16_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, Rn=0, Rm=0, Rd=0, rotate=0
    let encoding: u32 = 0xA6800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_sxtab16_a1_a_field_cond_11_poweroftwo_70_b6800070() {
    // Encoding: 0xB6800070
    // Test aarch32_SXTAB16_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=11, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0xB6800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_sxtab16_a1_a_field_cond_12_poweroftwo_70_c6800070() {
    // Encoding: 0xC6800070
    // Test aarch32_SXTAB16_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=12, Rd=0, rotate=0
    let encoding: u32 = 0xC6800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_sxtab16_a1_a_field_cond_13_poweroftwo_70_d6800070() {
    // Encoding: 0xD6800070
    // Test aarch32_SXTAB16_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=13, Rd=0, rotate=0, Rm=0, Rn=0
    let encoding: u32 = 0xD6800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_sxtab16_a1_a_field_cond_14_poweroftwo_70_e6800070() {
    // Encoding: 0xE6800070
    // Test aarch32_SXTAB16_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rn=0, Rd=0, cond=14, Rm=0
    let encoding: u32 = 0xE6800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_sxtab16_a1_a_field_cond_15_max_70_f6800070() {
    // Encoding: 0xF6800070
    // Test aarch32_SXTAB16_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, Rd=0, Rn=0, rotate=0, Rm=0
    let encoding: u32 = 0xF6800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtab16_a1_a_field_rn_0_min_70_06800070() {
    // Encoding: 0x06800070
    // Test aarch32_SXTAB16_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rn=0, rotate=0, Rd=0, cond=0
    let encoding: u32 = 0x06800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtab16_a1_a_field_rn_1_poweroftwo_70_06810070() {
    // Encoding: 0x06810070
    // Test aarch32_SXTAB16_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, rotate=0, Rm=0, Rd=0, Rn=1
    let encoding: u32 = 0x06810070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtab16_a1_a_field_rd_0_min_70_06800070() {
    // Encoding: 0x06800070
    // Test aarch32_SXTAB16_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=0, rotate=0, Rm=0
    let encoding: u32 = 0x06800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtab16_a1_a_field_rd_1_poweroftwo_70_06801070() {
    // Encoding: 0x06801070
    // Test aarch32_SXTAB16_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=1, cond=0, Rn=0, rotate=0
    let encoding: u32 = 0x06801070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_sxtab16_a1_a_field_rotate_0_min_70_06800070() {
    // Encoding: 0x06800070
    // Test aarch32_SXTAB16_A1_A field rotate = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, cond=0, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x06800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_sxtab16_a1_a_field_rotate_1_poweroftwo_70_06800470() {
    // Encoding: 0x06800470
    // Test aarch32_SXTAB16_A1_A field rotate = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=1, Rn=0, Rd=0, cond=0, Rm=0
    let encoding: u32 = 0x06800470;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_sxtab16_a1_a_field_rotate_3_max_70_06800c70() {
    // Encoding: 0x06800C70
    // Test aarch32_SXTAB16_A1_A field rotate = 3 (Max)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=0, rotate=3, Rm=0
    let encoding: u32 = 0x06800C70;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtab16_a1_a_field_rm_0_min_70_06800070() {
    // Encoding: 0x06800070
    // Test aarch32_SXTAB16_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, rotate=0, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x06800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtab16_a1_a_field_rm_1_poweroftwo_70_06800071() {
    // Encoding: 0x06800071
    // Test aarch32_SXTAB16_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rm=1, rotate=0, Rd=0
    let encoding: u32 = 0x06800071;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_sxtab16_a1_a_combo_0_70_06800070() {
    // Encoding: 0x06800070
    // Test aarch32_SXTAB16_A1_A field combination: cond=0, Rn=0, Rd=0, rotate=0, Rm=0
    // ISET: A32
    // Fields: Rd=0, Rn=0, rotate=0, cond=0, Rm=0
    let encoding: u32 = 0x06800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_sxtab16_a1_a_special_cond_0_condition_eq_112_06800070() {
    // Encoding: 0x06800070
    // Test aarch32_SXTAB16_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, rotate=0, cond=0
    let encoding: u32 = 0x06800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_sxtab16_a1_a_special_cond_1_condition_ne_112_16800070() {
    // Encoding: 0x16800070
    // Test aarch32_SXTAB16_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rd=0, cond=1, Rm=0, rotate=0, Rn=0
    let encoding: u32 = 0x16800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_sxtab16_a1_a_special_cond_2_condition_cs_hs_112_26800070() {
    // Encoding: 0x26800070
    // Test aarch32_SXTAB16_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, rotate=0, cond=2
    let encoding: u32 = 0x26800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_sxtab16_a1_a_special_cond_3_condition_cc_lo_112_36800070() {
    // Encoding: 0x36800070
    // Test aarch32_SXTAB16_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: rotate=0, Rm=0, cond=3, Rn=0, Rd=0
    let encoding: u32 = 0x36800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_sxtab16_a1_a_special_cond_4_condition_mi_112_46800070() {
    // Encoding: 0x46800070
    // Test aarch32_SXTAB16_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rd=0, Rn=0, rotate=0, cond=4, Rm=0
    let encoding: u32 = 0x46800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_sxtab16_a1_a_special_cond_5_condition_pl_112_56800070() {
    // Encoding: 0x56800070
    // Test aarch32_SXTAB16_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=5, rotate=0, Rn=0
    let encoding: u32 = 0x56800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_sxtab16_a1_a_special_cond_6_condition_vs_112_66800070() {
    // Encoding: 0x66800070
    // Test aarch32_SXTAB16_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, Rn=0, rotate=0, Rd=0, Rm=0
    let encoding: u32 = 0x66800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_sxtab16_a1_a_special_cond_7_condition_vc_112_76800070() {
    // Encoding: 0x76800070
    // Test aarch32_SXTAB16_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rd=0, Rn=0, rotate=0, Rm=0, cond=7
    let encoding: u32 = 0x76800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_sxtab16_a1_a_special_cond_8_condition_hi_112_86800070() {
    // Encoding: 0x86800070
    // Test aarch32_SXTAB16_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, Rn=0, Rm=0, rotate=0, Rd=0
    let encoding: u32 = 0x86800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_sxtab16_a1_a_special_cond_9_condition_ls_112_96800070() {
    // Encoding: 0x96800070
    // Test aarch32_SXTAB16_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: rotate=0, Rn=0, Rm=0, Rd=0, cond=9
    let encoding: u32 = 0x96800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_sxtab16_a1_a_special_cond_10_condition_ge_112_a6800070() {
    // Encoding: 0xA6800070
    // Test aarch32_SXTAB16_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rd=0, Rn=0, rotate=0, Rm=0, cond=10
    let encoding: u32 = 0xA6800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_sxtab16_a1_a_special_cond_11_condition_lt_112_b6800070() {
    // Encoding: 0xB6800070
    // Test aarch32_SXTAB16_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, Rm=0, Rn=0, Rd=0, rotate=0
    let encoding: u32 = 0xB6800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_sxtab16_a1_a_special_cond_12_condition_gt_112_c6800070() {
    // Encoding: 0xC6800070
    // Test aarch32_SXTAB16_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=12, Rd=0, rotate=0
    let encoding: u32 = 0xC6800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_sxtab16_a1_a_special_cond_13_condition_le_112_d6800070() {
    // Encoding: 0xD6800070
    // Test aarch32_SXTAB16_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=13, rotate=0, Rn=0
    let encoding: u32 = 0xD6800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_sxtab16_a1_a_special_cond_14_condition_al_112_e6800070() {
    // Encoding: 0xE6800070
    // Test aarch32_SXTAB16_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rn=0, Rd=0, rotate=0, cond=14, Rm=0
    let encoding: u32 = 0xE6800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_sxtab16_a1_a_special_cond_15_condition_nv_112_f6800070() {
    // Encoding: 0xF6800070
    // Test aarch32_SXTAB16_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=15, Rn=0, rotate=0
    let encoding: u32 = 0xF6800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sxtab16_a1_a_invalid_0_70_06800070() {
    // Encoding: 0x06800070
    // Test aarch32_SXTAB16_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: rotate=0, Rm=0, cond=0, Rn=0, Rd=0
    let encoding: u32 = 0x06800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SXTAB16_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sxtab16_a1_a_invalid_1_70_06800070() {
    // Encoding: 0x06800070
    // Test aarch32_SXTAB16_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rd=0, cond=0, Rn=0, rotate=0, Rm=0
    let encoding: u32 = 0x06800070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SXTAB16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtab16_t1_a_field_rn_0_min_f080_fa20f080() {
    // Thumb encoding (32): 0xFA20F080
    // Test aarch32_SXTAB16_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, rotate=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA20F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAB16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtab16_t1_a_field_rn_1_poweroftwo_f080_fa21f080() {
    // Thumb encoding (32): 0xFA21F080
    // Test aarch32_SXTAB16_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, Rm=0, Rd=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA21F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAB16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtab16_t1_a_field_rd_0_min_f080_fa20f080() {
    // Thumb encoding (32): 0xFA20F080
    // Test aarch32_SXTAB16_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: rotate=0, Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA20F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAB16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtab16_t1_a_field_rd_1_poweroftwo_f080_fa20f180() {
    // Thumb encoding (32): 0xFA20F180
    // Test aarch32_SXTAB16_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=1, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA20F180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAB16_T1_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_sxtab16_t1_a_field_rotate_0_min_f080_fa20f080() {
    // Thumb encoding (32): 0xFA20F080
    // Test aarch32_SXTAB16_T1_A field rotate = 0 (Min)
    // ISET: T32
    // Fields: rotate=0, Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA20F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAB16_T1_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_sxtab16_t1_a_field_rotate_1_poweroftwo_f080_fa20f090() {
    // Thumb encoding (32): 0xFA20F090
    // Test aarch32_SXTAB16_T1_A field rotate = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, rotate=1, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA20F090;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAB16_T1_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_sxtab16_t1_a_field_rotate_3_max_f080_fa20f0b0() {
    // Thumb encoding (32): 0xFA20F0B0
    // Test aarch32_SXTAB16_T1_A field rotate = 3 (Max)
    // ISET: T32
    // Fields: Rm=0, Rn=0, rotate=3, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA20F0B0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAB16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxtab16_t1_a_field_rm_0_min_f080_fa20f080() {
    // Thumb encoding (32): 0xFA20F080
    // Test aarch32_SXTAB16_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: rotate=0, Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA20F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAB16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxtab16_t1_a_field_rm_1_poweroftwo_f080_fa20f081() {
    // Thumb encoding (32): 0xFA20F081
    // Test aarch32_SXTAB16_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=1, rotate=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA20F081;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAB16_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_sxtab16_t1_a_combo_0_f080_fa20f080() {
    // Thumb encoding (32): 0xFA20F080
    // Test aarch32_SXTAB16_T1_A field combination: Rn=0, Rd=0, rotate=0, Rm=0
    // ISET: T32
    // Fields: Rd=0, Rn=0, rotate=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA20F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTAB16_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sxtab16_t1_a_invalid_0_f080_fa20f080() {
    // Thumb encoding (32): 0xFA20F080
    // Test aarch32_SXTAB16_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA20F080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_SXTAB16_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sxtab16_t1_a_invalid_1_f080_fa20f080() {
    // Thumb encoding (32): 0xFA20F080
    // Test aarch32_SXTAB16_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rn=0, Rd=0, rotate=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA20F080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_SXTH_A Tests
// ============================================================================

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_sxth_a1_a_field_cond_0_min_70_06bf0070() {
    // Encoding: 0x06BF0070
    // Test aarch32_SXTH_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rd=0, rotate=0
    let encoding: u32 = 0x06BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_sxth_a1_a_field_cond_1_poweroftwo_70_16bf0070() {
    // Encoding: 0x16BF0070
    // Test aarch32_SXTH_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, rotate=0, Rd=0, Rm=0
    let encoding: u32 = 0x16BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_sxth_a1_a_field_cond_2_poweroftwo_70_26bf0070() {
    // Encoding: 0x26BF0070
    // Test aarch32_SXTH_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=2, rotate=0, Rm=0
    let encoding: u32 = 0x26BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_sxth_a1_a_field_cond_3_poweroftwo_70_36bf0070() {
    // Encoding: 0x36BF0070
    // Test aarch32_SXTH_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, rotate=0, cond=3, Rd=0
    let encoding: u32 = 0x36BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_sxth_a1_a_field_cond_4_poweroftwo_70_46bf0070() {
    // Encoding: 0x46BF0070
    // Test aarch32_SXTH_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=4, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x46BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_sxth_a1_a_field_cond_5_poweroftwo_70_56bf0070() {
    // Encoding: 0x56BF0070
    // Test aarch32_SXTH_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=5, rotate=0, Rd=0, Rm=0
    let encoding: u32 = 0x56BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_sxth_a1_a_field_cond_6_poweroftwo_70_66bf0070() {
    // Encoding: 0x66BF0070
    // Test aarch32_SXTH_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, Rd=0, Rm=0, rotate=0
    let encoding: u32 = 0x66BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_sxth_a1_a_field_cond_7_poweroftwo_70_76bf0070() {
    // Encoding: 0x76BF0070
    // Test aarch32_SXTH_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=7, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x76BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_sxth_a1_a_field_cond_8_poweroftwo_70_86bf0070() {
    // Encoding: 0x86BF0070
    // Test aarch32_SXTH_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=8, rotate=0
    let encoding: u32 = 0x86BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_sxth_a1_a_field_cond_9_poweroftwo_70_96bf0070() {
    // Encoding: 0x96BF0070
    // Test aarch32_SXTH_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rm=0, cond=9, Rd=0
    let encoding: u32 = 0x96BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_sxth_a1_a_field_cond_10_poweroftwo_70_a6bf0070() {
    // Encoding: 0xA6BF0070
    // Test aarch32_SXTH_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, rotate=0, cond=10, Rm=0
    let encoding: u32 = 0xA6BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_sxth_a1_a_field_cond_11_poweroftwo_70_b6bf0070() {
    // Encoding: 0xB6BF0070
    // Test aarch32_SXTH_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, rotate=0, Rm=0, cond=11
    let encoding: u32 = 0xB6BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_sxth_a1_a_field_cond_12_poweroftwo_70_c6bf0070() {
    // Encoding: 0xC6BF0070
    // Test aarch32_SXTH_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, rotate=0, Rd=0, cond=12
    let encoding: u32 = 0xC6BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_sxth_a1_a_field_cond_13_poweroftwo_70_d6bf0070() {
    // Encoding: 0xD6BF0070
    // Test aarch32_SXTH_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, rotate=0, cond=13
    let encoding: u32 = 0xD6BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_sxth_a1_a_field_cond_14_poweroftwo_70_e6bf0070() {
    // Encoding: 0xE6BF0070
    // Test aarch32_SXTH_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0xE6BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_sxth_a1_a_field_cond_15_max_70_f6bf0070() {
    // Encoding: 0xF6BF0070
    // Test aarch32_SXTH_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rm=0, rotate=0, cond=15, Rd=0
    let encoding: u32 = 0xF6BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxth_a1_a_field_rd_0_min_70_06bf0070() {
    // Encoding: 0x06BF0070
    // Test aarch32_SXTH_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=0, rotate=0
    let encoding: u32 = 0x06BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxth_a1_a_field_rd_1_poweroftwo_70_06bf1070() {
    // Encoding: 0x06BF1070
    // Test aarch32_SXTH_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=1, Rm=0, cond=0, rotate=0
    let encoding: u32 = 0x06BF1070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_sxth_a1_a_field_rotate_0_min_70_06bf0070() {
    // Encoding: 0x06BF0070
    // Test aarch32_SXTH_A1_A field rotate = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rd=0, rotate=0, cond=0
    let encoding: u32 = 0x06BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_sxth_a1_a_field_rotate_1_poweroftwo_70_06bf0470() {
    // Encoding: 0x06BF0470
    // Test aarch32_SXTH_A1_A field rotate = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, rotate=1, cond=0, Rd=0
    let encoding: u32 = 0x06BF0470;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_sxth_a1_a_field_rotate_3_max_70_06bf0c70() {
    // Encoding: 0x06BF0C70
    // Test aarch32_SXTH_A1_A field rotate = 3 (Max)
    // ISET: A32
    // Fields: rotate=3, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x06BF0C70;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxth_a1_a_field_rm_0_min_70_06bf0070() {
    // Encoding: 0x06BF0070
    // Test aarch32_SXTH_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, rotate=0, cond=0, Rm=0
    let encoding: u32 = 0x06BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxth_a1_a_field_rm_1_poweroftwo_70_06bf0071() {
    // Encoding: 0x06BF0071
    // Test aarch32_SXTH_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rm=1, rotate=0, Rd=0
    let encoding: u32 = 0x06BF0071;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_sxth_a1_a_combo_0_70_06bf0070() {
    // Encoding: 0x06BF0070
    // Test aarch32_SXTH_A1_A field combination: cond=0, Rd=0, rotate=0, Rm=0
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=0, rotate=0
    let encoding: u32 = 0x06BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_sxth_a1_a_special_cond_0_condition_eq_112_06bf0070() {
    // Encoding: 0x06BF0070
    // Test aarch32_SXTH_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rm=0, cond=0, rotate=0, Rd=0
    let encoding: u32 = 0x06BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_sxth_a1_a_special_cond_1_condition_ne_112_16bf0070() {
    // Encoding: 0x16BF0070
    // Test aarch32_SXTH_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rm=0, cond=1, Rd=0, rotate=0
    let encoding: u32 = 0x16BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_sxth_a1_a_special_cond_2_condition_cs_hs_112_26bf0070() {
    // Encoding: 0x26BF0070
    // Test aarch32_SXTH_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rd=0, rotate=0, Rm=0, cond=2
    let encoding: u32 = 0x26BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_sxth_a1_a_special_cond_3_condition_cc_lo_112_36bf0070() {
    // Encoding: 0x36BF0070
    // Test aarch32_SXTH_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rm=0, cond=3, rotate=0, Rd=0
    let encoding: u32 = 0x36BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_sxth_a1_a_special_cond_4_condition_mi_112_46bf0070() {
    // Encoding: 0x46BF0070
    // Test aarch32_SXTH_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: rotate=0, Rm=0, cond=4, Rd=0
    let encoding: u32 = 0x46BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_sxth_a1_a_special_cond_5_condition_pl_112_56bf0070() {
    // Encoding: 0x56BF0070
    // Test aarch32_SXTH_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, rotate=0, Rd=0, Rm=0
    let encoding: u32 = 0x56BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_sxth_a1_a_special_cond_6_condition_vs_112_66bf0070() {
    // Encoding: 0x66BF0070
    // Test aarch32_SXTH_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rd=0, rotate=0, Rm=0, cond=6
    let encoding: u32 = 0x66BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_sxth_a1_a_special_cond_7_condition_vc_112_76bf0070() {
    // Encoding: 0x76BF0070
    // Test aarch32_SXTH_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rd=0, rotate=0, Rm=0, cond=7
    let encoding: u32 = 0x76BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_sxth_a1_a_special_cond_8_condition_hi_112_86bf0070() {
    // Encoding: 0x86BF0070
    // Test aarch32_SXTH_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rm=0, cond=8, rotate=0, Rd=0
    let encoding: u32 = 0x86BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_sxth_a1_a_special_cond_9_condition_ls_112_96bf0070() {
    // Encoding: 0x96BF0070
    // Test aarch32_SXTH_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rm=0, rotate=0, cond=9, Rd=0
    let encoding: u32 = 0x96BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_sxth_a1_a_special_cond_10_condition_ge_112_a6bf0070() {
    // Encoding: 0xA6BF0070
    // Test aarch32_SXTH_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rd=0, Rm=0, rotate=0, cond=10
    let encoding: u32 = 0xA6BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_sxth_a1_a_special_cond_11_condition_lt_112_b6bf0070() {
    // Encoding: 0xB6BF0070
    // Test aarch32_SXTH_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rm=0, cond=11, Rd=0, rotate=0
    let encoding: u32 = 0xB6BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_sxth_a1_a_special_cond_12_condition_gt_112_c6bf0070() {
    // Encoding: 0xC6BF0070
    // Test aarch32_SXTH_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: rotate=0, Rm=0, cond=12, Rd=0
    let encoding: u32 = 0xC6BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_sxth_a1_a_special_cond_13_condition_le_112_d6bf0070() {
    // Encoding: 0xD6BF0070
    // Test aarch32_SXTH_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: rotate=0, cond=13, Rd=0, Rm=0
    let encoding: u32 = 0xD6BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_sxth_a1_a_special_cond_14_condition_al_112_e6bf0070() {
    // Encoding: 0xE6BF0070
    // Test aarch32_SXTH_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rm=0, cond=14, Rd=0, rotate=0
    let encoding: u32 = 0xE6BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_sxth_a1_a_special_cond_15_condition_nv_112_f6bf0070() {
    // Encoding: 0xF6BF0070
    // Test aarch32_SXTH_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, rotate=0, Rd=0, Rm=0
    let encoding: u32 = 0xF6BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sxth_a1_a_invalid_0_70_06bf0070() {
    // Encoding: 0x06BF0070
    // Test aarch32_SXTH_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: cond=0, rotate=0, Rd=0, Rm=0
    let encoding: u32 = 0x06BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SXTH_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sxth_a1_a_invalid_1_70_06bf0070() {
    // Encoding: 0x06BF0070
    // Test aarch32_SXTH_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: rotate=0, cond=0, Rm=0, Rd=0
    let encoding: u32 = 0x06BF0070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SXTH_T1_A
/// ASL: `field Rm 19 +: 3`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxth_t1_a_field_rm_0_min_0_b2000000() {
    // Thumb encoding (32): 0xB2000000
    // Test aarch32_SXTH_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB2000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTH_T1_A
/// ASL: `field Rm 19 +: 3`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxth_t1_a_field_rm_1_poweroftwo_0_b2080000() {
    // Thumb encoding (32): 0xB2080000
    // Test aarch32_SXTH_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB2080000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTH_T1_A
/// ASL: `field Rd 16 +: 3`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxth_t1_a_field_rd_0_min_0_b2000000() {
    // Thumb encoding (32): 0xB2000000
    // Test aarch32_SXTH_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB2000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTH_T1_A
/// ASL: `field Rd 16 +: 3`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxth_t1_a_field_rd_1_poweroftwo_0_b2010000() {
    // Thumb encoding (32): 0xB2010000
    // Test aarch32_SXTH_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rd=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB2010000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTH_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch32_sxth_t1_a_combo_0_0_b2000000() {
    // Thumb encoding (32): 0xB2000000
    // Test aarch32_SXTH_T1_A field combination: Rm=0, Rd=0
    // ISET: T32
    // Fields: Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xB2000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTH_T2_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxth_t2_a_field_rd_0_min_f080_fa0ff080() {
    // Thumb encoding (32): 0xFA0FF080
    // Test aarch32_SXTH_T2_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, rotate=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA0FF080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTH_T2_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxth_t2_a_field_rd_1_poweroftwo_f080_fa0ff180() {
    // Thumb encoding (32): 0xFA0FF180
    // Test aarch32_SXTH_T2_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: rotate=0, Rm=0, Rd=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA0FF180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTH_T2_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_sxth_t2_a_field_rotate_0_min_f080_fa0ff080() {
    // Thumb encoding (32): 0xFA0FF080
    // Test aarch32_SXTH_T2_A field rotate = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, rotate=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA0FF080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTH_T2_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_sxth_t2_a_field_rotate_1_poweroftwo_f080_fa0ff090() {
    // Thumb encoding (32): 0xFA0FF090
    // Test aarch32_SXTH_T2_A field rotate = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, rotate=1, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA0FF090;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTH_T2_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_sxth_t2_a_field_rotate_3_max_f080_fa0ff0b0() {
    // Thumb encoding (32): 0xFA0FF0B0
    // Test aarch32_SXTH_T2_A field rotate = 3 (Max)
    // ISET: T32
    // Fields: rotate=3, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA0FF0B0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTH_T2_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sxth_t2_a_field_rm_0_min_f080_fa0ff080() {
    // Thumb encoding (32): 0xFA0FF080
    // Test aarch32_SXTH_T2_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: rotate=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA0FF080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTH_T2_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sxth_t2_a_field_rm_1_poweroftwo_f080_fa0ff081() {
    // Thumb encoding (32): 0xFA0FF081
    // Test aarch32_SXTH_T2_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1, Rd=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA0FF081;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTH_T2_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch32_sxth_t2_a_combo_0_f080_fa0ff080() {
    // Thumb encoding (32): 0xFA0FF080
    // Test aarch32_SXTH_T2_A field combination: Rd=0, rotate=0, Rm=0
    // ISET: T32
    // Fields: Rd=0, Rm=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA0FF080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SXTH_T2_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sxth_t2_a_invalid_0_f080_fa0ff080() {
    // Thumb encoding (32): 0xFA0FF080
    // Test aarch32_SXTH_T2_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rm=0, rotate=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA0FF080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_SXTH_T2_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sxth_t2_a_invalid_1_f080_fa0ff080() {
    // Thumb encoding (32): 0xFA0FF080
    // Test aarch32_SXTH_T2_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rd=0, rotate=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA0FF080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_SXTH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 0 (32)
#[test]
fn test_aarch32_sxth_t1_a_lslv_oracle_32_0_b2020020() {
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

/// Provenance: aarch32_SXTH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 0 (64)
#[test]
fn test_aarch32_sxth_t1_a_lslv_oracle_64_0_b2020020() {
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

/// Provenance: aarch32_SXTH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4 (32)
#[test]
fn test_aarch32_sxth_t1_a_lslv_oracle_32_1_b2020020() {
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

/// Provenance: aarch32_SXTH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 4 (64)
#[test]
fn test_aarch32_sxth_t1_a_lslv_oracle_64_1_b2020020() {
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

/// Provenance: aarch32_SXTH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 8 (32)
#[test]
fn test_aarch32_sxth_t1_a_lslv_oracle_32_2_b2020020() {
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

/// Provenance: aarch32_SXTH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 8 (64)
#[test]
fn test_aarch32_sxth_t1_a_lslv_oracle_64_2_b2020020() {
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

/// Provenance: aarch32_SXTH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1 (32)
#[test]
fn test_aarch32_sxth_t1_a_lslv_oracle_32_3_b2020020() {
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

/// Provenance: aarch32_SXTH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// MSB set, shift 1 (64)
#[test]
fn test_aarch32_sxth_t1_a_lslv_oracle_64_3_b2020020() {
    // Test LSLV 64-bit: MSB set, shift 1 (oracle)
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
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch32_SXTH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSB set, max shift (32)
#[test]
fn test_aarch32_sxth_t1_a_lslv_oracle_32_4_b2020020() {
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

/// Provenance: aarch32_SXTH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSB set, max shift (64)
#[test]
fn test_aarch32_sxth_t1_a_lslv_oracle_64_4_b2020020() {
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

/// Provenance: aarch32_SXTH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// all ones, shift 32 (32)
#[test]
fn test_aarch32_sxth_t1_a_lslv_oracle_32_5_b2020020() {
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

/// Provenance: aarch32_SXTH_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all ones, shift 32 (64)
#[test]
fn test_aarch32_sxth_t1_a_lslv_oracle_64_5_b2020020() {
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

/// Provenance: aarch32_SXTH_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift
#[test]
fn test_aarch32_sxth_t1_a_t16_oracle_0_b2100000() {
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

/// Provenance: aarch32_SXTH_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4
#[test]
fn test_aarch32_sxth_t1_a_t16_oracle_1_b2100000() {
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

/// Provenance: aarch32_SXTH_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1
#[test]
fn test_aarch32_sxth_t1_a_t16_oracle_2_b2100000() {
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

/// Provenance: aarch32_SXTH_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift to MSB
#[test]
fn test_aarch32_sxth_t1_a_t16_oracle_3_b2100000() {
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

// ============================================================================
// aarch32_UXTAB16_A Tests
// ============================================================================

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_uxtab16_a1_a_field_cond_0_min_70_06c00070() {
    // Encoding: 0x06C00070
    // Test aarch32_UXTAB16_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, rotate=0, Rd=0, Rm=0
    let encoding: u32 = 0x06C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_uxtab16_a1_a_field_cond_1_poweroftwo_70_16c00070() {
    // Encoding: 0x16C00070
    // Test aarch32_UXTAB16_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, rotate=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x16C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_uxtab16_a1_a_field_cond_2_poweroftwo_70_26c00070() {
    // Encoding: 0x26C00070
    // Test aarch32_UXTAB16_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=2, Rn=0, Rd=0, rotate=0
    let encoding: u32 = 0x26C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_uxtab16_a1_a_field_cond_3_poweroftwo_70_36c00070() {
    // Encoding: 0x36C00070
    // Test aarch32_UXTAB16_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rn=0, cond=3, Rd=0, Rm=0
    let encoding: u32 = 0x36C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_uxtab16_a1_a_field_cond_4_poweroftwo_70_46c00070() {
    // Encoding: 0x46C00070
    // Test aarch32_UXTAB16_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rm=0, Rn=0, cond=4, Rd=0
    let encoding: u32 = 0x46C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_uxtab16_a1_a_field_cond_5_poweroftwo_70_56c00070() {
    // Encoding: 0x56C00070
    // Test aarch32_UXTAB16_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=5, Rm=0, Rn=0, rotate=0
    let encoding: u32 = 0x56C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_uxtab16_a1_a_field_cond_6_poweroftwo_70_66c00070() {
    // Encoding: 0x66C00070
    // Test aarch32_UXTAB16_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, rotate=0, Rd=0, cond=6
    let encoding: u32 = 0x66C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_uxtab16_a1_a_field_cond_7_poweroftwo_70_76c00070() {
    // Encoding: 0x76C00070
    // Test aarch32_UXTAB16_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=7, Rm=0, Rn=0, rotate=0
    let encoding: u32 = 0x76C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_uxtab16_a1_a_field_cond_8_poweroftwo_70_86c00070() {
    // Encoding: 0x86C00070
    // Test aarch32_UXTAB16_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rm=0, Rd=0, cond=8, Rn=0
    let encoding: u32 = 0x86C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_uxtab16_a1_a_field_cond_9_poweroftwo_70_96c00070() {
    // Encoding: 0x96C00070
    // Test aarch32_UXTAB16_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, rotate=0, cond=9, Rm=0
    let encoding: u32 = 0x96C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_uxtab16_a1_a_field_cond_10_poweroftwo_70_a6c00070() {
    // Encoding: 0xA6C00070
    // Test aarch32_UXTAB16_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, rotate=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0xA6C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_uxtab16_a1_a_field_cond_11_poweroftwo_70_b6c00070() {
    // Encoding: 0xB6C00070
    // Test aarch32_UXTAB16_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, rotate=0, cond=11, Rn=0, Rm=0
    let encoding: u32 = 0xB6C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_uxtab16_a1_a_field_cond_12_poweroftwo_70_c6c00070() {
    // Encoding: 0xC6C00070
    // Test aarch32_UXTAB16_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, Rm=0, rotate=0, Rd=0, Rn=0
    let encoding: u32 = 0xC6C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_uxtab16_a1_a_field_cond_13_poweroftwo_70_d6c00070() {
    // Encoding: 0xD6C00070
    // Test aarch32_UXTAB16_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rn=0, Rm=0, Rd=0, cond=13
    let encoding: u32 = 0xD6C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_uxtab16_a1_a_field_cond_14_poweroftwo_70_e6c00070() {
    // Encoding: 0xE6C00070
    // Test aarch32_UXTAB16_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rd=0, Rm=0, Rn=0, cond=14
    let encoding: u32 = 0xE6C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_uxtab16_a1_a_field_cond_15_max_70_f6c00070() {
    // Encoding: 0xF6C00070
    // Test aarch32_UXTAB16_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=15, rotate=0, Rm=0
    let encoding: u32 = 0xF6C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtab16_a1_a_field_rn_0_min_70_06c00070() {
    // Encoding: 0x06C00070
    // Test aarch32_UXTAB16_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=0, Rm=0, rotate=0
    let encoding: u32 = 0x06C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtab16_a1_a_field_rn_1_poweroftwo_70_06c10070() {
    // Encoding: 0x06C10070
    // Test aarch32_UXTAB16_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=0, rotate=0, Rn=1, Rm=0
    let encoding: u32 = 0x06C10070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtab16_a1_a_field_rd_0_min_70_06c00070() {
    // Encoding: 0x06C00070
    // Test aarch32_UXTAB16_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, rotate=0, Rm=0, Rd=0
    let encoding: u32 = 0x06C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtab16_a1_a_field_rd_1_poweroftwo_70_06c01070() {
    // Encoding: 0x06C01070
    // Test aarch32_UXTAB16_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=1, rotate=0, Rm=0, Rn=0
    let encoding: u32 = 0x06C01070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_uxtab16_a1_a_field_rotate_0_min_70_06c00070() {
    // Encoding: 0x06C00070
    // Test aarch32_UXTAB16_A1_A field rotate = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=0, rotate=0, Rm=0
    let encoding: u32 = 0x06C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_uxtab16_a1_a_field_rotate_1_poweroftwo_70_06c00470() {
    // Encoding: 0x06C00470
    // Test aarch32_UXTAB16_A1_A field rotate = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=1, cond=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x06C00470;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_uxtab16_a1_a_field_rotate_3_max_70_06c00c70() {
    // Encoding: 0x06C00C70
    // Test aarch32_UXTAB16_A1_A field rotate = 3 (Max)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=0, Rd=0, rotate=3
    let encoding: u32 = 0x06C00C70;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtab16_a1_a_field_rm_0_min_70_06c00070() {
    // Encoding: 0x06C00070
    // Test aarch32_UXTAB16_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, Rm=0, rotate=0
    let encoding: u32 = 0x06C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtab16_a1_a_field_rm_1_poweroftwo_70_06c00071() {
    // Encoding: 0x06C00071
    // Test aarch32_UXTAB16_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, rotate=0, Rn=0, Rm=1, cond=0
    let encoding: u32 = 0x06C00071;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_uxtab16_a1_a_combo_0_70_06c00070() {
    // Encoding: 0x06C00070
    // Test aarch32_UXTAB16_A1_A field combination: cond=0, Rn=0, Rd=0, rotate=0, Rm=0
    // ISET: A32
    // Fields: rotate=0, cond=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x06C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_uxtab16_a1_a_special_cond_0_condition_eq_112_06c00070() {
    // Encoding: 0x06C00070
    // Test aarch32_UXTAB16_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rd=0, cond=0, rotate=0, Rm=0, Rn=0
    let encoding: u32 = 0x06C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_uxtab16_a1_a_special_cond_1_condition_ne_112_16c00070() {
    // Encoding: 0x16C00070
    // Test aarch32_UXTAB16_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, Rd=0, Rn=0, rotate=0, Rm=0
    let encoding: u32 = 0x16C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_uxtab16_a1_a_special_cond_2_condition_cs_hs_112_26c00070() {
    // Encoding: 0x26C00070
    // Test aarch32_UXTAB16_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rm=0, cond=2, rotate=0, Rn=0, Rd=0
    let encoding: u32 = 0x26C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_uxtab16_a1_a_special_cond_3_condition_cc_lo_112_36c00070() {
    // Encoding: 0x36C00070
    // Test aarch32_UXTAB16_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, Rd=0, Rn=0, Rm=0, rotate=0
    let encoding: u32 = 0x36C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_uxtab16_a1_a_special_cond_4_condition_mi_112_46c00070() {
    // Encoding: 0x46C00070
    // Test aarch32_UXTAB16_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=4, Rd=0, rotate=0
    let encoding: u32 = 0x46C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_uxtab16_a1_a_special_cond_5_condition_pl_112_56c00070() {
    // Encoding: 0x56C00070
    // Test aarch32_UXTAB16_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, Rn=0, Rm=0, rotate=0, Rd=0
    let encoding: u32 = 0x56C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_uxtab16_a1_a_special_cond_6_condition_vs_112_66c00070() {
    // Encoding: 0x66C00070
    // Test aarch32_UXTAB16_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, Rm=0, Rn=0, rotate=0, Rd=0
    let encoding: u32 = 0x66C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_uxtab16_a1_a_special_cond_7_condition_vc_112_76c00070() {
    // Encoding: 0x76C00070
    // Test aarch32_UXTAB16_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rd=0, cond=7, rotate=0, Rm=0, Rn=0
    let encoding: u32 = 0x76C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_uxtab16_a1_a_special_cond_8_condition_hi_112_86c00070() {
    // Encoding: 0x86C00070
    // Test aarch32_UXTAB16_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: rotate=0, cond=8, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x86C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_uxtab16_a1_a_special_cond_9_condition_ls_112_96c00070() {
    // Encoding: 0x96C00070
    // Test aarch32_UXTAB16_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=9, rotate=0, Rd=0
    let encoding: u32 = 0x96C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_uxtab16_a1_a_special_cond_10_condition_ge_112_a6c00070() {
    // Encoding: 0xA6C00070
    // Test aarch32_UXTAB16_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, Rn=0, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0xA6C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_uxtab16_a1_a_special_cond_11_condition_lt_112_b6c00070() {
    // Encoding: 0xB6C00070
    // Test aarch32_UXTAB16_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: rotate=0, Rn=0, Rd=0, cond=11, Rm=0
    let encoding: u32 = 0xB6C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_uxtab16_a1_a_special_cond_12_condition_gt_112_c6c00070() {
    // Encoding: 0xC6C00070
    // Test aarch32_UXTAB16_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rd=0, cond=12, Rn=0, rotate=0, Rm=0
    let encoding: u32 = 0xC6C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_uxtab16_a1_a_special_cond_13_condition_le_112_d6c00070() {
    // Encoding: 0xD6C00070
    // Test aarch32_UXTAB16_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rm=0, cond=13, Rd=0, Rn=0, rotate=0
    let encoding: u32 = 0xD6C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_uxtab16_a1_a_special_cond_14_condition_al_112_e6c00070() {
    // Encoding: 0xE6C00070
    // Test aarch32_UXTAB16_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rn=0, cond=14, rotate=0, Rd=0, Rm=0
    let encoding: u32 = 0xE6C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_uxtab16_a1_a_special_cond_15_condition_nv_112_f6c00070() {
    // Encoding: 0xF6C00070
    // Test aarch32_UXTAB16_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=15, Rn=0, rotate=0
    let encoding: u32 = 0xF6C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uxtab16_a1_a_invalid_0_70_06c00070() {
    // Encoding: 0x06C00070
    // Test aarch32_UXTAB16_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rn=0, cond=0, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x06C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UXTAB16_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uxtab16_a1_a_invalid_1_70_06c00070() {
    // Encoding: 0x06C00070
    // Test aarch32_UXTAB16_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: rotate=0, Rm=0, cond=0, Rn=0, Rd=0
    let encoding: u32 = 0x06C00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UXTAB16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtab16_t1_a_field_rn_0_min_f080_fa30f080() {
    // Thumb encoding (32): 0xFA30F080
    // Test aarch32_UXTAB16_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA30F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAB16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtab16_t1_a_field_rn_1_poweroftwo_f080_fa31f080() {
    // Thumb encoding (32): 0xFA31F080
    // Test aarch32_UXTAB16_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: rotate=0, Rm=0, Rn=1, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA31F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAB16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtab16_t1_a_field_rd_0_min_f080_fa30f080() {
    // Thumb encoding (32): 0xFA30F080
    // Test aarch32_UXTAB16_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, rotate=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA30F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAB16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtab16_t1_a_field_rd_1_poweroftwo_f080_fa30f180() {
    // Thumb encoding (32): 0xFA30F180
    // Test aarch32_UXTAB16_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=1, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA30F180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAB16_T1_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_uxtab16_t1_a_field_rotate_0_min_f080_fa30f080() {
    // Thumb encoding (32): 0xFA30F080
    // Test aarch32_UXTAB16_T1_A field rotate = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rm=0, rotate=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA30F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAB16_T1_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_uxtab16_t1_a_field_rotate_1_poweroftwo_f080_fa30f090() {
    // Thumb encoding (32): 0xFA30F090
    // Test aarch32_UXTAB16_T1_A field rotate = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, rotate=1, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA30F090;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAB16_T1_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_uxtab16_t1_a_field_rotate_3_max_f080_fa30f0b0() {
    // Thumb encoding (32): 0xFA30F0B0
    // Test aarch32_UXTAB16_T1_A field rotate = 3 (Max)
    // ISET: T32
    // Fields: Rd=0, rotate=3, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA30F0B0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAB16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtab16_t1_a_field_rm_0_min_f080_fa30f080() {
    // Thumb encoding (32): 0xFA30F080
    // Test aarch32_UXTAB16_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA30F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAB16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtab16_t1_a_field_rm_1_poweroftwo_f080_fa30f081() {
    // Thumb encoding (32): 0xFA30F081
    // Test aarch32_UXTAB16_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: rotate=0, Rn=0, Rd=0, Rm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA30F081;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAB16_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_uxtab16_t1_a_combo_0_f080_fa30f080() {
    // Thumb encoding (32): 0xFA30F080
    // Test aarch32_UXTAB16_T1_A field combination: Rn=0, Rd=0, rotate=0, Rm=0
    // ISET: T32
    // Fields: Rn=0, Rm=0, rotate=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA30F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAB16_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uxtab16_t1_a_invalid_0_f080_fa30f080() {
    // Thumb encoding (32): 0xFA30F080
    // Test aarch32_UXTAB16_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: rotate=0, Rm=0, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA30F080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_UXTAB16_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uxtab16_t1_a_invalid_1_f080_fa30f080() {
    // Thumb encoding (32): 0xFA30F080
    // Test aarch32_UXTAB16_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA30F080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_UXTAB_A Tests
// ============================================================================

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_uxtab_a1_a_field_cond_0_min_70_06e00070() {
    // Encoding: 0x06E00070
    // Test aarch32_UXTAB_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=0, rotate=0
    let encoding: u32 = 0x06E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_uxtab_a1_a_field_cond_1_poweroftwo_70_16e00070() {
    // Encoding: 0x16E00070
    // Test aarch32_UXTAB_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rm=0, Rn=0, cond=1, Rd=0
    let encoding: u32 = 0x16E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_uxtab_a1_a_field_cond_2_poweroftwo_70_26e00070() {
    // Encoding: 0x26E00070
    // Test aarch32_UXTAB_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, rotate=0, cond=2, Rn=0, Rm=0
    let encoding: u32 = 0x26E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_uxtab_a1_a_field_cond_3_poweroftwo_70_36e00070() {
    // Encoding: 0x36E00070
    // Test aarch32_UXTAB_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, Rn=0, Rm=0, rotate=0, Rd=0
    let encoding: u32 = 0x36E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_uxtab_a1_a_field_cond_4_poweroftwo_70_46e00070() {
    // Encoding: 0x46E00070
    // Test aarch32_UXTAB_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, rotate=0, Rd=0, cond=4, Rm=0
    let encoding: u32 = 0x46E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_uxtab_a1_a_field_cond_5_poweroftwo_70_56e00070() {
    // Encoding: 0x56E00070
    // Test aarch32_UXTAB_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=5, Rd=0, rotate=0, Rn=0, Rm=0
    let encoding: u32 = 0x56E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_uxtab_a1_a_field_cond_6_poweroftwo_70_66e00070() {
    // Encoding: 0x66E00070
    // Test aarch32_UXTAB_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, rotate=0, Rn=0, cond=6
    let encoding: u32 = 0x66E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_uxtab_a1_a_field_cond_7_poweroftwo_70_76e00070() {
    // Encoding: 0x76E00070
    // Test aarch32_UXTAB_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, rotate=0, Rm=0, cond=7, Rd=0
    let encoding: u32 = 0x76E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_uxtab_a1_a_field_cond_8_poweroftwo_70_86e00070() {
    // Encoding: 0x86E00070
    // Test aarch32_UXTAB_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=8, Rm=0, rotate=0, Rd=0
    let encoding: u32 = 0x86E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_uxtab_a1_a_field_cond_9_poweroftwo_70_96e00070() {
    // Encoding: 0x96E00070
    // Test aarch32_UXTAB_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, rotate=0, Rm=0, cond=9
    let encoding: u32 = 0x96E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_uxtab_a1_a_field_cond_10_poweroftwo_70_a6e00070() {
    // Encoding: 0xA6E00070
    // Test aarch32_UXTAB_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=10, Rn=0, rotate=0, Rd=0
    let encoding: u32 = 0xA6E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_uxtab_a1_a_field_cond_11_poweroftwo_70_b6e00070() {
    // Encoding: 0xB6E00070
    // Test aarch32_UXTAB_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, cond=11, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0xB6E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_uxtab_a1_a_field_cond_12_poweroftwo_70_c6e00070() {
    // Encoding: 0xC6E00070
    // Test aarch32_UXTAB_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, Rn=0, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0xC6E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_uxtab_a1_a_field_cond_13_poweroftwo_70_d6e00070() {
    // Encoding: 0xD6E00070
    // Test aarch32_UXTAB_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=13, rotate=0, Rd=0, Rn=0
    let encoding: u32 = 0xD6E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_uxtab_a1_a_field_cond_14_poweroftwo_70_e6e00070() {
    // Encoding: 0xE6E00070
    // Test aarch32_UXTAB_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, Rn=0, Rd=0, Rm=0, rotate=0
    let encoding: u32 = 0xE6E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_uxtab_a1_a_field_cond_15_max_70_f6e00070() {
    // Encoding: 0xF6E00070
    // Test aarch32_UXTAB_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, rotate=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0xF6E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtab_a1_a_field_rn_0_min_70_06e00070() {
    // Encoding: 0x06E00070
    // Test aarch32_UXTAB_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, cond=0, Rd=0, Rm=0, rotate=0
    let encoding: u32 = 0x06E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtab_a1_a_field_rn_1_poweroftwo_70_06e10070() {
    // Encoding: 0x06E10070
    // Test aarch32_UXTAB_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, rotate=0, cond=0, Rd=0, Rn=1
    let encoding: u32 = 0x06E10070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtab_a1_a_field_rd_0_min_70_06e00070() {
    // Encoding: 0x06E00070
    // Test aarch32_UXTAB_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, rotate=0, cond=0, Rn=0, Rd=0
    let encoding: u32 = 0x06E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtab_a1_a_field_rd_1_poweroftwo_70_06e01070() {
    // Encoding: 0x06E01070
    // Test aarch32_UXTAB_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: rotate=0, Rn=0, Rm=0, cond=0, Rd=1
    let encoding: u32 = 0x06E01070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_uxtab_a1_a_field_rotate_0_min_70_06e00070() {
    // Encoding: 0x06E00070
    // Test aarch32_UXTAB_A1_A field rotate = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rd=0, Rn=0, rotate=0
    let encoding: u32 = 0x06E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_uxtab_a1_a_field_rotate_1_poweroftwo_70_06e00470() {
    // Encoding: 0x06E00470
    // Test aarch32_UXTAB_A1_A field rotate = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rm=0, Rd=0, rotate=1
    let encoding: u32 = 0x06E00470;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field rotate 10 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_uxtab_a1_a_field_rotate_3_max_70_06e00c70() {
    // Encoding: 0x06E00C70
    // Test aarch32_UXTAB_A1_A field rotate = 3 (Max)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=0, rotate=3
    let encoding: u32 = 0x06E00C70;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtab_a1_a_field_rm_0_min_70_06e00070() {
    // Encoding: 0x06E00070
    // Test aarch32_UXTAB_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rd=0, rotate=0, Rn=0
    let encoding: u32 = 0x06E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtab_a1_a_field_rm_1_poweroftwo_70_06e00071() {
    // Encoding: 0x06E00071
    // Test aarch32_UXTAB_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=1, cond=0, rotate=0, Rd=0
    let encoding: u32 = 0x06E00071;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_uxtab_a1_a_combo_0_70_06e00070() {
    // Encoding: 0x06E00070
    // Test aarch32_UXTAB_A1_A field combination: cond=0, Rn=0, Rd=0, rotate=0, Rm=0
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=0, rotate=0
    let encoding: u32 = 0x06E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_uxtab_a1_a_special_cond_0_condition_eq_112_06e00070() {
    // Encoding: 0x06E00070
    // Test aarch32_UXTAB_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: rotate=0, cond=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x06E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_uxtab_a1_a_special_cond_1_condition_ne_112_16e00070() {
    // Encoding: 0x16E00070
    // Test aarch32_UXTAB_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=1, rotate=0, Rm=0
    let encoding: u32 = 0x16E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_uxtab_a1_a_special_cond_2_condition_cs_hs_112_26e00070() {
    // Encoding: 0x26E00070
    // Test aarch32_UXTAB_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=2, Rd=0, rotate=0
    let encoding: u32 = 0x26E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_uxtab_a1_a_special_cond_3_condition_cc_lo_112_36e00070() {
    // Encoding: 0x36E00070
    // Test aarch32_UXTAB_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rd=0, cond=3, Rn=0, Rm=0, rotate=0
    let encoding: u32 = 0x36E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_uxtab_a1_a_special_cond_4_condition_mi_112_46e00070() {
    // Encoding: 0x46E00070
    // Test aarch32_UXTAB_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=4, Rn=0, rotate=0
    let encoding: u32 = 0x46E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_uxtab_a1_a_special_cond_5_condition_pl_112_56e00070() {
    // Encoding: 0x56E00070
    // Test aarch32_UXTAB_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, rotate=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x56E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_uxtab_a1_a_special_cond_6_condition_vs_112_66e00070() {
    // Encoding: 0x66E00070
    // Test aarch32_UXTAB_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rd=0, cond=6, Rn=0, rotate=0, Rm=0
    let encoding: u32 = 0x66E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_uxtab_a1_a_special_cond_7_condition_vc_112_76e00070() {
    // Encoding: 0x76E00070
    // Test aarch32_UXTAB_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, Rn=0, rotate=0, Rm=0, Rd=0
    let encoding: u32 = 0x76E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_uxtab_a1_a_special_cond_8_condition_hi_112_86e00070() {
    // Encoding: 0x86E00070
    // Test aarch32_UXTAB_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rn=0, cond=8, Rd=0, rotate=0, Rm=0
    let encoding: u32 = 0x86E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_uxtab_a1_a_special_cond_9_condition_ls_112_96e00070() {
    // Encoding: 0x96E00070
    // Test aarch32_UXTAB_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rn=0, rotate=0, Rm=0, Rd=0, cond=9
    let encoding: u32 = 0x96E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_uxtab_a1_a_special_cond_10_condition_ge_112_a6e00070() {
    // Encoding: 0xA6E00070
    // Test aarch32_UXTAB_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=10, rotate=0
    let encoding: u32 = 0xA6E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_uxtab_a1_a_special_cond_11_condition_lt_112_b6e00070() {
    // Encoding: 0xB6E00070
    // Test aarch32_UXTAB_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rd=0, rotate=0, Rn=0, cond=11, Rm=0
    let encoding: u32 = 0xB6E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_uxtab_a1_a_special_cond_12_condition_gt_112_c6e00070() {
    // Encoding: 0xC6E00070
    // Test aarch32_UXTAB_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: rotate=0, Rd=0, Rn=0, cond=12, Rm=0
    let encoding: u32 = 0xC6E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_uxtab_a1_a_special_cond_13_condition_le_112_d6e00070() {
    // Encoding: 0xD6E00070
    // Test aarch32_UXTAB_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rd=0, cond=13, Rn=0, rotate=0, Rm=0
    let encoding: u32 = 0xD6E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_uxtab_a1_a_special_cond_14_condition_al_112_e6e00070() {
    // Encoding: 0xE6E00070
    // Test aarch32_UXTAB_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, rotate=0, cond=14
    let encoding: u32 = 0xE6E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_uxtab_a1_a_special_cond_15_condition_nv_112_f6e00070() {
    // Encoding: 0xF6E00070
    // Test aarch32_UXTAB_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rd=0, rotate=0, Rm=0, Rn=0, cond=15
    let encoding: u32 = 0xF6E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uxtab_a1_a_invalid_0_70_06e00070() {
    // Encoding: 0x06E00070
    // Test aarch32_UXTAB_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: rotate=0, Rd=0, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x06E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UXTAB_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uxtab_a1_a_invalid_1_70_06e00070() {
    // Encoding: 0x06E00070
    // Test aarch32_UXTAB_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: rotate=0, Rn=0, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x06E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UXTAB_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtab_t1_a_field_rn_0_min_f080_fa50f080() {
    // Thumb encoding (32): 0xFA50F080
    // Test aarch32_UXTAB_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: rotate=0, Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA50F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAB_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtab_t1_a_field_rn_1_poweroftwo_f080_fa51f080() {
    // Thumb encoding (32): 0xFA51F080
    // Test aarch32_UXTAB_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=1, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA51F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAB_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtab_t1_a_field_rd_0_min_f080_fa50f080() {
    // Thumb encoding (32): 0xFA50F080
    // Test aarch32_UXTAB_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA50F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAB_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtab_t1_a_field_rd_1_poweroftwo_f080_fa50f180() {
    // Thumb encoding (32): 0xFA50F180
    // Test aarch32_UXTAB_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rd=1, Rn=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA50F180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAB_T1_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_uxtab_t1_a_field_rotate_0_min_f080_fa50f080() {
    // Thumb encoding (32): 0xFA50F080
    // Test aarch32_UXTAB_T1_A field rotate = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, rotate=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA50F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAB_T1_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_uxtab_t1_a_field_rotate_1_poweroftwo_f080_fa50f090() {
    // Thumb encoding (32): 0xFA50F090
    // Test aarch32_UXTAB_T1_A field rotate = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0, rotate=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA50F090;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAB_T1_A
/// ASL: `field rotate 4 +: 2`
/// Requirement: FieldBoundary { field: "rotate", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_uxtab_t1_a_field_rotate_3_max_f080_fa50f0b0() {
    // Thumb encoding (32): 0xFA50F0B0
    // Test aarch32_UXTAB_T1_A field rotate = 3 (Max)
    // ISET: T32
    // Fields: Rm=0, rotate=3, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA50F0B0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAB_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uxtab_t1_a_field_rm_0_min_f080_fa50f080() {
    // Thumb encoding (32): 0xFA50F080
    // Test aarch32_UXTAB_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0, rotate=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA50F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAB_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uxtab_t1_a_field_rm_1_poweroftwo_f080_fa50f081() {
    // Thumb encoding (32): 0xFA50F081
    // Test aarch32_UXTAB_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: rotate=0, Rn=0, Rm=1, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA50F081;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAB_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_uxtab_t1_a_combo_0_f080_fa50f080() {
    // Thumb encoding (32): 0xFA50F080
    // Test aarch32_UXTAB_T1_A field combination: Rn=0, Rd=0, rotate=0, Rm=0
    // ISET: T32
    // Fields: rotate=0, Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA50F080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UXTAB_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uxtab_t1_a_invalid_0_f080_fa50f080() {
    // Thumb encoding (32): 0xFA50F080
    // Test aarch32_UXTAB_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, rotate=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA50F080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_UXTAB_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uxtab_t1_a_invalid_1_f080_fa50f080() {
    // Thumb encoding (32): 0xFA50F080
    // Test aarch32_UXTAB_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: rotate=0, Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA50F080;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}
