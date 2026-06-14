//! A32 data_processing parallel tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers_32::*;

// ============================================================================
// aarch32_UHSAX_A Tests
// ============================================================================

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_uhsax_a1_a_field_cond_0_min_50_06700050() {
    // Encoding: 0x06700050
    // Test aarch32_UHSAX_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=0, Rm=0
    let encoding: u32 = 0x06700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_uhsax_a1_a_field_cond_1_poweroftwo_50_16700050() {
    // Encoding: 0x16700050
    // Test aarch32_UHSAX_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=1
    let encoding: u32 = 0x16700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_uhsax_a1_a_field_cond_2_poweroftwo_50_26700050() {
    // Encoding: 0x26700050
    // Test aarch32_UHSAX_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=2, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x26700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_uhsax_a1_a_field_cond_3_poweroftwo_50_36700050() {
    // Encoding: 0x36700050
    // Test aarch32_UHSAX_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=3, Rd=0, Rn=0
    let encoding: u32 = 0x36700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_uhsax_a1_a_field_cond_4_poweroftwo_50_46700050() {
    // Encoding: 0x46700050
    // Test aarch32_UHSAX_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=4
    let encoding: u32 = 0x46700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_uhsax_a1_a_field_cond_5_poweroftwo_50_56700050() {
    // Encoding: 0x56700050
    // Test aarch32_UHSAX_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=5, Rd=0, Rm=0
    let encoding: u32 = 0x56700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_uhsax_a1_a_field_cond_6_poweroftwo_50_66700050() {
    // Encoding: 0x66700050
    // Test aarch32_UHSAX_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=6, Rm=0
    let encoding: u32 = 0x66700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_uhsax_a1_a_field_cond_7_poweroftwo_50_76700050() {
    // Encoding: 0x76700050
    // Test aarch32_UHSAX_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=7
    let encoding: u32 = 0x76700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_uhsax_a1_a_field_cond_8_poweroftwo_50_86700050() {
    // Encoding: 0x86700050
    // Test aarch32_UHSAX_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=8
    let encoding: u32 = 0x86700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_uhsax_a1_a_field_cond_9_poweroftwo_50_96700050() {
    // Encoding: 0x96700050
    // Test aarch32_UHSAX_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=9, Rn=0, Rm=0
    let encoding: u32 = 0x96700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_uhsax_a1_a_field_cond_10_poweroftwo_50_a6700050() {
    // Encoding: 0xA6700050
    // Test aarch32_UHSAX_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0xA6700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_uhsax_a1_a_field_cond_11_poweroftwo_50_b6700050() {
    // Encoding: 0xB6700050
    // Test aarch32_UHSAX_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=11, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xB6700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_uhsax_a1_a_field_cond_12_poweroftwo_50_c6700050() {
    // Encoding: 0xC6700050
    // Test aarch32_UHSAX_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xC6700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_uhsax_a1_a_field_cond_13_poweroftwo_50_d6700050() {
    // Encoding: 0xD6700050
    // Test aarch32_UHSAX_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=13, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xD6700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_uhsax_a1_a_field_cond_14_poweroftwo_50_e6700050() {
    // Encoding: 0xE6700050
    // Test aarch32_UHSAX_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=14, Rn=0, Rd=0
    let encoding: u32 = 0xE6700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_uhsax_a1_a_field_cond_15_max_50_f6700050() {
    // Encoding: 0xF6700050
    // Test aarch32_UHSAX_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rd=0, cond=15, Rn=0, Rm=0
    let encoding: u32 = 0xF6700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhsax_a1_a_field_rn_0_min_50_06700050() {
    // Encoding: 0x06700050
    // Test aarch32_UHSAX_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x06700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhsax_a1_a_field_rn_1_poweroftwo_50_06710050() {
    // Encoding: 0x06710050
    // Test aarch32_UHSAX_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=0, Rn=1, Rm=0
    let encoding: u32 = 0x06710050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhsax_a1_a_field_rd_0_min_50_06700050() {
    // Encoding: 0x06700050
    // Test aarch32_UHSAX_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x06700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhsax_a1_a_field_rd_1_poweroftwo_50_06701050() {
    // Encoding: 0x06701050
    // Test aarch32_UHSAX_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=0, Rd=1
    let encoding: u32 = 0x06701050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhsax_a1_a_field_rm_0_min_50_06700050() {
    // Encoding: 0x06700050
    // Test aarch32_UHSAX_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=0, Rd=0
    let encoding: u32 = 0x06700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhsax_a1_a_field_rm_1_poweroftwo_50_06700051() {
    // Encoding: 0x06700051
    // Test aarch32_UHSAX_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, Rm=1
    let encoding: u32 = 0x06700051;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_uhsax_a1_a_combo_0_50_06700050() {
    // Encoding: 0x06700050
    // Test aarch32_UHSAX_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rn=0, cond=0, Rd=0, Rm=0
    let encoding: u32 = 0x06700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_uhsax_a1_a_special_cond_0_condition_eq_80_06700050() {
    // Encoding: 0x06700050
    // Test aarch32_UHSAX_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=0, Rd=0
    let encoding: u32 = 0x06700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_uhsax_a1_a_special_cond_1_condition_ne_80_16700050() {
    // Encoding: 0x16700050
    // Test aarch32_UHSAX_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=1, Rd=0
    let encoding: u32 = 0x16700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_uhsax_a1_a_special_cond_2_condition_cs_hs_80_26700050() {
    // Encoding: 0x26700050
    // Test aarch32_UHSAX_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=2
    let encoding: u32 = 0x26700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_uhsax_a1_a_special_cond_3_condition_cc_lo_80_36700050() {
    // Encoding: 0x36700050
    // Test aarch32_UHSAX_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=3
    let encoding: u32 = 0x36700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_uhsax_a1_a_special_cond_4_condition_mi_80_46700050() {
    // Encoding: 0x46700050
    // Test aarch32_UHSAX_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=4, Rd=0
    let encoding: u32 = 0x46700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_uhsax_a1_a_special_cond_5_condition_pl_80_56700050() {
    // Encoding: 0x56700050
    // Test aarch32_UHSAX_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=5, Rm=0
    let encoding: u32 = 0x56700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_uhsax_a1_a_special_cond_6_condition_vs_80_66700050() {
    // Encoding: 0x66700050
    // Test aarch32_UHSAX_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rm=0, cond=6, Rd=0, Rn=0
    let encoding: u32 = 0x66700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_uhsax_a1_a_special_cond_7_condition_vc_80_76700050() {
    // Encoding: 0x76700050
    // Test aarch32_UHSAX_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rn=0, cond=7, Rd=0, Rm=0
    let encoding: u32 = 0x76700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_uhsax_a1_a_special_cond_8_condition_hi_80_86700050() {
    // Encoding: 0x86700050
    // Test aarch32_UHSAX_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rm=0, cond=8, Rn=0, Rd=0
    let encoding: u32 = 0x86700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_uhsax_a1_a_special_cond_9_condition_ls_80_96700050() {
    // Encoding: 0x96700050
    // Test aarch32_UHSAX_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=9
    let encoding: u32 = 0x96700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_uhsax_a1_a_special_cond_10_condition_ge_80_a6700050() {
    // Encoding: 0xA6700050
    // Test aarch32_UHSAX_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xA6700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_uhsax_a1_a_special_cond_11_condition_lt_80_b6700050() {
    // Encoding: 0xB6700050
    // Test aarch32_UHSAX_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=11, Rm=0
    let encoding: u32 = 0xB6700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_uhsax_a1_a_special_cond_12_condition_gt_80_c6700050() {
    // Encoding: 0xC6700050
    // Test aarch32_UHSAX_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: cond=12, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0xC6700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_uhsax_a1_a_special_cond_13_condition_le_80_d6700050() {
    // Encoding: 0xD6700050
    // Test aarch32_UHSAX_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rm=0, cond=13, Rn=0, Rd=0
    let encoding: u32 = 0xD6700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_uhsax_a1_a_special_cond_14_condition_al_80_e6700050() {
    // Encoding: 0xE6700050
    // Test aarch32_UHSAX_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: cond=14, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0xE6700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_uhsax_a1_a_special_cond_15_condition_nv_80_f6700050() {
    // Encoding: 0xF6700050
    // Test aarch32_UHSAX_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rm=0, cond=15, Rn=0, Rd=0
    let encoding: u32 = 0xF6700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uhsax_a1_a_invalid_0_50_06700050() {
    // Encoding: 0x06700050
    // Test aarch32_UHSAX_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=0, Rm=0
    let encoding: u32 = 0x06700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UHSAX_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uhsax_a1_a_invalid_1_50_06700050() {
    // Encoding: 0x06700050
    // Test aarch32_UHSAX_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=0, Rd=0
    let encoding: u32 = 0x06700050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UHSAX_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhsax_t1_a_field_rn_0_min_f060_fae0f060() {
    // Thumb encoding (32): 0xFAE0F060
    // Test aarch32_UHSAX_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHSAX_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhsax_t1_a_field_rn_1_poweroftwo_f060_fae1f060() {
    // Thumb encoding (32): 0xFAE1F060
    // Test aarch32_UHSAX_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE1F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHSAX_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhsax_t1_a_field_rd_0_min_f060_fae0f060() {
    // Thumb encoding (32): 0xFAE0F060
    // Test aarch32_UHSAX_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHSAX_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhsax_t1_a_field_rd_1_poweroftwo_f060_fae0f160() {
    // Thumb encoding (32): 0xFAE0F160
    // Test aarch32_UHSAX_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=1, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F160;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHSAX_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhsax_t1_a_field_rm_0_min_f060_fae0f060() {
    // Thumb encoding (32): 0xFAE0F060
    // Test aarch32_UHSAX_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHSAX_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhsax_t1_a_field_rm_1_poweroftwo_f060_fae0f061() {
    // Thumb encoding (32): 0xFAE0F061
    // Test aarch32_UHSAX_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F061;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHSAX_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_uhsax_t1_a_combo_0_f060_fae0f060() {
    // Thumb encoding (32): 0xFAE0F060
    // Test aarch32_UHSAX_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHSAX_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uhsax_t1_a_invalid_0_f060_fae0f060() {
    // Thumb encoding (32): 0xFAE0F060
    // Test aarch32_UHSAX_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F060;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_UHSAX_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uhsax_t1_a_invalid_1_f060_fae0f060() {
    // Thumb encoding (32): 0xFAE0F060
    // Test aarch32_UHSAX_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F060;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_SADD8_A Tests
// ============================================================================

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_sadd8_a1_a_field_cond_0_min_90_06100090() {
    // Encoding: 0x06100090
    // Test aarch32_SADD8_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x06100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_sadd8_a1_a_field_cond_1_poweroftwo_90_16100090() {
    // Encoding: 0x16100090
    // Test aarch32_SADD8_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=1
    let encoding: u32 = 0x16100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_sadd8_a1_a_field_cond_2_poweroftwo_90_26100090() {
    // Encoding: 0x26100090
    // Test aarch32_SADD8_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=2, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x26100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_sadd8_a1_a_field_cond_3_poweroftwo_90_36100090() {
    // Encoding: 0x36100090
    // Test aarch32_SADD8_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=3
    let encoding: u32 = 0x36100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_sadd8_a1_a_field_cond_4_poweroftwo_90_46100090() {
    // Encoding: 0x46100090
    // Test aarch32_SADD8_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=4, Rd=0, Rn=0
    let encoding: u32 = 0x46100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_sadd8_a1_a_field_cond_5_poweroftwo_90_56100090() {
    // Encoding: 0x56100090
    // Test aarch32_SADD8_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=5, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x56100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_sadd8_a1_a_field_cond_6_poweroftwo_90_66100090() {
    // Encoding: 0x66100090
    // Test aarch32_SADD8_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=6, Rm=0
    let encoding: u32 = 0x66100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_sadd8_a1_a_field_cond_7_poweroftwo_90_76100090() {
    // Encoding: 0x76100090
    // Test aarch32_SADD8_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=7
    let encoding: u32 = 0x76100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_sadd8_a1_a_field_cond_8_poweroftwo_90_86100090() {
    // Encoding: 0x86100090
    // Test aarch32_SADD8_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=8, Rn=0, Rd=0
    let encoding: u32 = 0x86100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_sadd8_a1_a_field_cond_9_poweroftwo_90_96100090() {
    // Encoding: 0x96100090
    // Test aarch32_SADD8_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x96100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_sadd8_a1_a_field_cond_10_poweroftwo_90_a6100090() {
    // Encoding: 0xA6100090
    // Test aarch32_SADD8_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=10
    let encoding: u32 = 0xA6100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_sadd8_a1_a_field_cond_11_poweroftwo_90_b6100090() {
    // Encoding: 0xB6100090
    // Test aarch32_SADD8_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=11, Rm=0, Rn=0
    let encoding: u32 = 0xB6100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_sadd8_a1_a_field_cond_12_poweroftwo_90_c6100090() {
    // Encoding: 0xC6100090
    // Test aarch32_SADD8_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xC6100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_sadd8_a1_a_field_cond_13_poweroftwo_90_d6100090() {
    // Encoding: 0xD6100090
    // Test aarch32_SADD8_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=13
    let encoding: u32 = 0xD6100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_sadd8_a1_a_field_cond_14_poweroftwo_90_e6100090() {
    // Encoding: 0xE6100090
    // Test aarch32_SADD8_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=14, Rm=0, Rn=0
    let encoding: u32 = 0xE6100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_sadd8_a1_a_field_cond_15_max_90_f6100090() {
    // Encoding: 0xF6100090
    // Test aarch32_SADD8_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=15, Rd=0
    let encoding: u32 = 0xF6100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sadd8_a1_a_field_rn_0_min_90_06100090() {
    // Encoding: 0x06100090
    // Test aarch32_SADD8_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=0
    let encoding: u32 = 0x06100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sadd8_a1_a_field_rn_1_poweroftwo_90_06110090() {
    // Encoding: 0x06110090
    // Test aarch32_SADD8_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=1, Rm=0
    let encoding: u32 = 0x06110090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sadd8_a1_a_field_rd_0_min_90_06100090() {
    // Encoding: 0x06100090
    // Test aarch32_SADD8_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x06100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sadd8_a1_a_field_rd_1_poweroftwo_90_06101090() {
    // Encoding: 0x06101090
    // Test aarch32_SADD8_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rn=0, Rd=1
    let encoding: u32 = 0x06101090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sadd8_a1_a_field_rm_0_min_90_06100090() {
    // Encoding: 0x06100090
    // Test aarch32_SADD8_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x06100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sadd8_a1_a_field_rm_1_poweroftwo_90_06100091() {
    // Encoding: 0x06100091
    // Test aarch32_SADD8_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=0, Rm=1
    let encoding: u32 = 0x06100091;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_sadd8_a1_a_combo_0_90_06100090() {
    // Encoding: 0x06100090
    // Test aarch32_SADD8_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=0, Rm=0
    let encoding: u32 = 0x06100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_sadd8_a1_a_special_cond_0_condition_eq_144_06100090() {
    // Encoding: 0x06100090
    // Test aarch32_SADD8_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x06100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_sadd8_a1_a_special_cond_1_condition_ne_144_16100090() {
    // Encoding: 0x16100090
    // Test aarch32_SADD8_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x16100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_sadd8_a1_a_special_cond_2_condition_cs_hs_144_26100090() {
    // Encoding: 0x26100090
    // Test aarch32_SADD8_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x26100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_sadd8_a1_a_special_cond_3_condition_cc_lo_144_36100090() {
    // Encoding: 0x36100090
    // Test aarch32_SADD8_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rm=0, cond=3, Rd=0, Rn=0
    let encoding: u32 = 0x36100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_sadd8_a1_a_special_cond_4_condition_mi_144_46100090() {
    // Encoding: 0x46100090
    // Test aarch32_SADD8_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=4, Rd=0
    let encoding: u32 = 0x46100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_sadd8_a1_a_special_cond_5_condition_pl_144_56100090() {
    // Encoding: 0x56100090
    // Test aarch32_SADD8_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=5
    let encoding: u32 = 0x56100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_sadd8_a1_a_special_cond_6_condition_vs_144_66100090() {
    // Encoding: 0x66100090
    // Test aarch32_SADD8_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x66100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_sadd8_a1_a_special_cond_7_condition_vc_144_76100090() {
    // Encoding: 0x76100090
    // Test aarch32_SADD8_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x76100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_sadd8_a1_a_special_cond_8_condition_hi_144_86100090() {
    // Encoding: 0x86100090
    // Test aarch32_SADD8_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rd=0, cond=8, Rm=0, Rn=0
    let encoding: u32 = 0x86100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_sadd8_a1_a_special_cond_9_condition_ls_144_96100090() {
    // Encoding: 0x96100090
    // Test aarch32_SADD8_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=9, Rd=0
    let encoding: u32 = 0x96100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_sadd8_a1_a_special_cond_10_condition_ge_144_a6100090() {
    // Encoding: 0xA6100090
    // Test aarch32_SADD8_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0xA6100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_sadd8_a1_a_special_cond_11_condition_lt_144_b6100090() {
    // Encoding: 0xB6100090
    // Test aarch32_SADD8_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=11, Rn=0
    let encoding: u32 = 0xB6100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_sadd8_a1_a_special_cond_12_condition_gt_144_c6100090() {
    // Encoding: 0xC6100090
    // Test aarch32_SADD8_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rn=0, cond=12, Rd=0, Rm=0
    let encoding: u32 = 0xC6100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_sadd8_a1_a_special_cond_13_condition_le_144_d6100090() {
    // Encoding: 0xD6100090
    // Test aarch32_SADD8_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=13, Rm=0
    let encoding: u32 = 0xD6100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_sadd8_a1_a_special_cond_14_condition_al_144_e6100090() {
    // Encoding: 0xE6100090
    // Test aarch32_SADD8_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=14, Rd=0
    let encoding: u32 = 0xE6100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_sadd8_a1_a_special_cond_15_condition_nv_144_f6100090() {
    // Encoding: 0xF6100090
    // Test aarch32_SADD8_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rn=0, cond=15, Rm=0, Rd=0
    let encoding: u32 = 0xF6100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sadd8_a1_a_invalid_0_90_06100090() {
    // Encoding: 0x06100090
    // Test aarch32_SADD8_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x06100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SADD8_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sadd8_a1_a_invalid_1_90_06100090() {
    // Encoding: 0x06100090
    // Test aarch32_SADD8_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=0
    let encoding: u32 = 0x06100090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SADD8_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sadd8_t1_a_field_rn_0_min_f000_fa80f000() {
    // Thumb encoding (32): 0xFA80F000
    // Test aarch32_SADD8_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SADD8_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sadd8_t1_a_field_rn_1_poweroftwo_f000_fa81f000() {
    // Thumb encoding (32): 0xFA81F000
    // Test aarch32_SADD8_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA81F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SADD8_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sadd8_t1_a_field_rd_0_min_f000_fa80f000() {
    // Thumb encoding (32): 0xFA80F000
    // Test aarch32_SADD8_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SADD8_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sadd8_t1_a_field_rd_1_poweroftwo_f000_fa80f100() {
    // Thumb encoding (32): 0xFA80F100
    // Test aarch32_SADD8_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=1, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F100;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SADD8_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sadd8_t1_a_field_rm_0_min_f000_fa80f000() {
    // Thumb encoding (32): 0xFA80F000
    // Test aarch32_SADD8_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SADD8_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sadd8_t1_a_field_rm_1_poweroftwo_f000_fa80f001() {
    // Thumb encoding (32): 0xFA80F001
    // Test aarch32_SADD8_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F001;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SADD8_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_sadd8_t1_a_combo_0_f000_fa80f000() {
    // Thumb encoding (32): 0xFA80F000
    // Test aarch32_SADD8_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SADD8_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sadd8_t1_a_invalid_0_f000_fa80f000() {
    // Thumb encoding (32): 0xFA80F000
    // Test aarch32_SADD8_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_SADD8_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sadd8_t1_a_invalid_1_f000_fa80f000() {
    // Thumb encoding (32): 0xFA80F000
    // Test aarch32_SADD8_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_UHADD8_A Tests
// ============================================================================

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_uhadd8_a1_a_field_cond_0_min_90_06700090() {
    // Encoding: 0x06700090
    // Test aarch32_UHADD8_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x06700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_uhadd8_a1_a_field_cond_1_poweroftwo_90_16700090() {
    // Encoding: 0x16700090
    // Test aarch32_UHADD8_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x16700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_uhadd8_a1_a_field_cond_2_poweroftwo_90_26700090() {
    // Encoding: 0x26700090
    // Test aarch32_UHADD8_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=2, Rn=0, Rd=0
    let encoding: u32 = 0x26700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_uhadd8_a1_a_field_cond_3_poweroftwo_90_36700090() {
    // Encoding: 0x36700090
    // Test aarch32_UHADD8_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=3, Rd=0, Rn=0
    let encoding: u32 = 0x36700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_uhadd8_a1_a_field_cond_4_poweroftwo_90_46700090() {
    // Encoding: 0x46700090
    // Test aarch32_UHADD8_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=4, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x46700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_uhadd8_a1_a_field_cond_5_poweroftwo_90_56700090() {
    // Encoding: 0x56700090
    // Test aarch32_UHADD8_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=5, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x56700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_uhadd8_a1_a_field_cond_6_poweroftwo_90_66700090() {
    // Encoding: 0x66700090
    // Test aarch32_UHADD8_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x66700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_uhadd8_a1_a_field_cond_7_poweroftwo_90_76700090() {
    // Encoding: 0x76700090
    // Test aarch32_UHADD8_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=7
    let encoding: u32 = 0x76700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_uhadd8_a1_a_field_cond_8_poweroftwo_90_86700090() {
    // Encoding: 0x86700090
    // Test aarch32_UHADD8_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=8
    let encoding: u32 = 0x86700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_uhadd8_a1_a_field_cond_9_poweroftwo_90_96700090() {
    // Encoding: 0x96700090
    // Test aarch32_UHADD8_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=9, Rn=0, Rd=0
    let encoding: u32 = 0x96700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_uhadd8_a1_a_field_cond_10_poweroftwo_90_a6700090() {
    // Encoding: 0xA6700090
    // Test aarch32_UHADD8_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0xA6700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_uhadd8_a1_a_field_cond_11_poweroftwo_90_b6700090() {
    // Encoding: 0xB6700090
    // Test aarch32_UHADD8_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=11, Rn=0
    let encoding: u32 = 0xB6700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_uhadd8_a1_a_field_cond_12_poweroftwo_90_c6700090() {
    // Encoding: 0xC6700090
    // Test aarch32_UHADD8_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=12
    let encoding: u32 = 0xC6700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_uhadd8_a1_a_field_cond_13_poweroftwo_90_d6700090() {
    // Encoding: 0xD6700090
    // Test aarch32_UHADD8_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=13, Rn=0, Rd=0
    let encoding: u32 = 0xD6700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_uhadd8_a1_a_field_cond_14_poweroftwo_90_e6700090() {
    // Encoding: 0xE6700090
    // Test aarch32_UHADD8_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0xE6700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_uhadd8_a1_a_field_cond_15_max_90_f6700090() {
    // Encoding: 0xF6700090
    // Test aarch32_UHADD8_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=15, Rm=0
    let encoding: u32 = 0xF6700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhadd8_a1_a_field_rn_0_min_90_06700090() {
    // Encoding: 0x06700090
    // Test aarch32_UHADD8_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, cond=0, Rd=0, Rm=0
    let encoding: u32 = 0x06700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhadd8_a1_a_field_rn_1_poweroftwo_90_06710090() {
    // Encoding: 0x06710090
    // Test aarch32_UHADD8_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=1, Rd=0, cond=0, Rm=0
    let encoding: u32 = 0x06710090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhadd8_a1_a_field_rd_0_min_90_06700090() {
    // Encoding: 0x06700090
    // Test aarch32_UHADD8_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x06700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhadd8_a1_a_field_rd_1_poweroftwo_90_06701090() {
    // Encoding: 0x06701090
    // Test aarch32_UHADD8_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=0, Rd=1, Rm=0
    let encoding: u32 = 0x06701090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhadd8_a1_a_field_rm_0_min_90_06700090() {
    // Encoding: 0x06700090
    // Test aarch32_UHADD8_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=0, Rd=0
    let encoding: u32 = 0x06700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhadd8_a1_a_field_rm_1_poweroftwo_90_06700091() {
    // Encoding: 0x06700091
    // Test aarch32_UHADD8_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=0, Rm=1
    let encoding: u32 = 0x06700091;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_uhadd8_a1_a_combo_0_90_06700090() {
    // Encoding: 0x06700090
    // Test aarch32_UHADD8_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=0, Rm=0
    let encoding: u32 = 0x06700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_uhadd8_a1_a_special_cond_0_condition_eq_144_06700090() {
    // Encoding: 0x06700090
    // Test aarch32_UHADD8_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rd=0, Rn=0
    let encoding: u32 = 0x06700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_uhadd8_a1_a_special_cond_1_condition_ne_144_16700090() {
    // Encoding: 0x16700090
    // Test aarch32_UHADD8_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rn=0, cond=1, Rm=0, Rd=0
    let encoding: u32 = 0x16700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_uhadd8_a1_a_special_cond_2_condition_cs_hs_144_26700090() {
    // Encoding: 0x26700090
    // Test aarch32_UHADD8_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=2, Rm=0
    let encoding: u32 = 0x26700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_uhadd8_a1_a_special_cond_3_condition_cc_lo_144_36700090() {
    // Encoding: 0x36700090
    // Test aarch32_UHADD8_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=3
    let encoding: u32 = 0x36700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_uhadd8_a1_a_special_cond_4_condition_mi_144_46700090() {
    // Encoding: 0x46700090
    // Test aarch32_UHADD8_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=4
    let encoding: u32 = 0x46700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_uhadd8_a1_a_special_cond_5_condition_pl_144_56700090() {
    // Encoding: 0x56700090
    // Test aarch32_UHADD8_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x56700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_uhadd8_a1_a_special_cond_6_condition_vs_144_66700090() {
    // Encoding: 0x66700090
    // Test aarch32_UHADD8_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=6, Rn=0
    let encoding: u32 = 0x66700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_uhadd8_a1_a_special_cond_7_condition_vc_144_76700090() {
    // Encoding: 0x76700090
    // Test aarch32_UHADD8_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=7
    let encoding: u32 = 0x76700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_uhadd8_a1_a_special_cond_8_condition_hi_144_86700090() {
    // Encoding: 0x86700090
    // Test aarch32_UHADD8_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rd=0, cond=8, Rm=0, Rn=0
    let encoding: u32 = 0x86700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_uhadd8_a1_a_special_cond_9_condition_ls_144_96700090() {
    // Encoding: 0x96700090
    // Test aarch32_UHADD8_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rm=0, cond=9, Rn=0, Rd=0
    let encoding: u32 = 0x96700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_uhadd8_a1_a_special_cond_10_condition_ge_144_a6700090() {
    // Encoding: 0xA6700090
    // Test aarch32_UHADD8_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=10, Rd=0
    let encoding: u32 = 0xA6700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_uhadd8_a1_a_special_cond_11_condition_lt_144_b6700090() {
    // Encoding: 0xB6700090
    // Test aarch32_UHADD8_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=11
    let encoding: u32 = 0xB6700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_uhadd8_a1_a_special_cond_12_condition_gt_144_c6700090() {
    // Encoding: 0xC6700090
    // Test aarch32_UHADD8_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=12
    let encoding: u32 = 0xC6700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_uhadd8_a1_a_special_cond_13_condition_le_144_d6700090() {
    // Encoding: 0xD6700090
    // Test aarch32_UHADD8_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=13, Rn=0
    let encoding: u32 = 0xD6700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_uhadd8_a1_a_special_cond_14_condition_al_144_e6700090() {
    // Encoding: 0xE6700090
    // Test aarch32_UHADD8_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: cond=14, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0xE6700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_uhadd8_a1_a_special_cond_15_condition_nv_144_f6700090() {
    // Encoding: 0xF6700090
    // Test aarch32_UHADD8_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=15, Rn=0
    let encoding: u32 = 0xF6700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uhadd8_a1_a_invalid_0_90_06700090() {
    // Encoding: 0x06700090
    // Test aarch32_UHADD8_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: cond=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x06700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UHADD8_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uhadd8_a1_a_invalid_1_90_06700090() {
    // Encoding: 0x06700090
    // Test aarch32_UHADD8_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=0, Rm=0
    let encoding: u32 = 0x06700090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UHADD8_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhadd8_t1_a_field_rn_0_min_f060_fa80f060() {
    // Thumb encoding (32): 0xFA80F060
    // Test aarch32_UHADD8_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHADD8_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhadd8_t1_a_field_rn_1_poweroftwo_f060_fa81f060() {
    // Thumb encoding (32): 0xFA81F060
    // Test aarch32_UHADD8_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA81F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHADD8_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhadd8_t1_a_field_rd_0_min_f060_fa80f060() {
    // Thumb encoding (32): 0xFA80F060
    // Test aarch32_UHADD8_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHADD8_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhadd8_t1_a_field_rd_1_poweroftwo_f060_fa80f160() {
    // Thumb encoding (32): 0xFA80F160
    // Test aarch32_UHADD8_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rd=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F160;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHADD8_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhadd8_t1_a_field_rm_0_min_f060_fa80f060() {
    // Thumb encoding (32): 0xFA80F060
    // Test aarch32_UHADD8_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHADD8_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhadd8_t1_a_field_rm_1_poweroftwo_f060_fa80f061() {
    // Thumb encoding (32): 0xFA80F061
    // Test aarch32_UHADD8_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F061;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHADD8_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_uhadd8_t1_a_combo_0_f060_fa80f060() {
    // Thumb encoding (32): 0xFA80F060
    // Test aarch32_UHADD8_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHADD8_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uhadd8_t1_a_invalid_0_f060_fa80f060() {
    // Thumb encoding (32): 0xFA80F060
    // Test aarch32_UHADD8_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F060;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_UHADD8_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uhadd8_t1_a_invalid_1_f060_fa80f060() {
    // Thumb encoding (32): 0xFA80F060
    // Test aarch32_UHADD8_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F060;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_USAX_A Tests
// ============================================================================

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_usax_a1_a_field_cond_0_min_50_06500050() {
    // Encoding: 0x06500050
    // Test aarch32_USAX_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=0
    let encoding: u32 = 0x06500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_usax_a1_a_field_cond_1_poweroftwo_50_16500050() {
    // Encoding: 0x16500050
    // Test aarch32_USAX_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=1, Rn=0, Rd=0
    let encoding: u32 = 0x16500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_usax_a1_a_field_cond_2_poweroftwo_50_26500050() {
    // Encoding: 0x26500050
    // Test aarch32_USAX_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=2, Rm=0, Rd=0
    let encoding: u32 = 0x26500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_usax_a1_a_field_cond_3_poweroftwo_50_36500050() {
    // Encoding: 0x36500050
    // Test aarch32_USAX_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=3, Rm=0
    let encoding: u32 = 0x36500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_usax_a1_a_field_cond_4_poweroftwo_50_46500050() {
    // Encoding: 0x46500050
    // Test aarch32_USAX_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=4, Rd=0, Rm=0
    let encoding: u32 = 0x46500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_usax_a1_a_field_cond_5_poweroftwo_50_56500050() {
    // Encoding: 0x56500050
    // Test aarch32_USAX_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=5, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x56500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_usax_a1_a_field_cond_6_poweroftwo_50_66500050() {
    // Encoding: 0x66500050
    // Test aarch32_USAX_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=6, Rn=0, Rd=0
    let encoding: u32 = 0x66500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_usax_a1_a_field_cond_7_poweroftwo_50_76500050() {
    // Encoding: 0x76500050
    // Test aarch32_USAX_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=7, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x76500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_usax_a1_a_field_cond_8_poweroftwo_50_86500050() {
    // Encoding: 0x86500050
    // Test aarch32_USAX_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=8, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x86500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_usax_a1_a_field_cond_9_poweroftwo_50_96500050() {
    // Encoding: 0x96500050
    // Test aarch32_USAX_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x96500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_usax_a1_a_field_cond_10_poweroftwo_50_a6500050() {
    // Encoding: 0xA6500050
    // Test aarch32_USAX_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=10
    let encoding: u32 = 0xA6500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_usax_a1_a_field_cond_11_poweroftwo_50_b6500050() {
    // Encoding: 0xB6500050
    // Test aarch32_USAX_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=11, Rm=0, Rn=0
    let encoding: u32 = 0xB6500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_usax_a1_a_field_cond_12_poweroftwo_50_c6500050() {
    // Encoding: 0xC6500050
    // Test aarch32_USAX_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0xC6500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_usax_a1_a_field_cond_13_poweroftwo_50_d6500050() {
    // Encoding: 0xD6500050
    // Test aarch32_USAX_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=13, Rm=0, Rn=0
    let encoding: u32 = 0xD6500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_usax_a1_a_field_cond_14_poweroftwo_50_e6500050() {
    // Encoding: 0xE6500050
    // Test aarch32_USAX_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=14, Rd=0, Rm=0
    let encoding: u32 = 0xE6500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_usax_a1_a_field_cond_15_max_50_f6500050() {
    // Encoding: 0xF6500050
    // Test aarch32_USAX_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=15
    let encoding: u32 = 0xF6500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_usax_a1_a_field_rn_0_min_50_06500050() {
    // Encoding: 0x06500050
    // Test aarch32_USAX_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=0, Rm=0
    let encoding: u32 = 0x06500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_usax_a1_a_field_rn_1_poweroftwo_50_06510050() {
    // Encoding: 0x06510050
    // Test aarch32_USAX_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=0, Rn=1
    let encoding: u32 = 0x06510050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_usax_a1_a_field_rd_0_min_50_06500050() {
    // Encoding: 0x06500050
    // Test aarch32_USAX_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=0, Rn=0
    let encoding: u32 = 0x06500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_usax_a1_a_field_rd_1_poweroftwo_50_06501050() {
    // Encoding: 0x06501050
    // Test aarch32_USAX_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=1, Rn=0, Rm=0
    let encoding: u32 = 0x06501050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_usax_a1_a_field_rm_0_min_50_06500050() {
    // Encoding: 0x06500050
    // Test aarch32_USAX_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rd=0, Rn=0
    let encoding: u32 = 0x06500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_usax_a1_a_field_rm_1_poweroftwo_50_06500051() {
    // Encoding: 0x06500051
    // Test aarch32_USAX_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=0, Rn=0, Rm=1
    let encoding: u32 = 0x06500051;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_usax_a1_a_combo_0_50_06500050() {
    // Encoding: 0x06500050
    // Test aarch32_USAX_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: cond=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x06500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_usax_a1_a_special_cond_0_condition_eq_80_06500050() {
    // Encoding: 0x06500050
    // Test aarch32_USAX_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=0
    let encoding: u32 = 0x06500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_usax_a1_a_special_cond_1_condition_ne_80_16500050() {
    // Encoding: 0x16500050
    // Test aarch32_USAX_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x16500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_usax_a1_a_special_cond_2_condition_cs_hs_80_26500050() {
    // Encoding: 0x26500050
    // Test aarch32_USAX_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rd=0, cond=2, Rn=0, Rm=0
    let encoding: u32 = 0x26500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_usax_a1_a_special_cond_3_condition_cc_lo_80_36500050() {
    // Encoding: 0x36500050
    // Test aarch32_USAX_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x36500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_usax_a1_a_special_cond_4_condition_mi_80_46500050() {
    // Encoding: 0x46500050
    // Test aarch32_USAX_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=4
    let encoding: u32 = 0x46500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_usax_a1_a_special_cond_5_condition_pl_80_56500050() {
    // Encoding: 0x56500050
    // Test aarch32_USAX_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=5
    let encoding: u32 = 0x56500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_usax_a1_a_special_cond_6_condition_vs_80_66500050() {
    // Encoding: 0x66500050
    // Test aarch32_USAX_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rm=0, cond=6, Rn=0, Rd=0
    let encoding: u32 = 0x66500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_usax_a1_a_special_cond_7_condition_vc_80_76500050() {
    // Encoding: 0x76500050
    // Test aarch32_USAX_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=7, Rm=0
    let encoding: u32 = 0x76500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_usax_a1_a_special_cond_8_condition_hi_80_86500050() {
    // Encoding: 0x86500050
    // Test aarch32_USAX_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rd=0, cond=8, Rn=0, Rm=0
    let encoding: u32 = 0x86500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_usax_a1_a_special_cond_9_condition_ls_80_96500050() {
    // Encoding: 0x96500050
    // Test aarch32_USAX_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x96500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_usax_a1_a_special_cond_10_condition_ge_80_a6500050() {
    // Encoding: 0xA6500050
    // Test aarch32_USAX_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rm=0, cond=10, Rd=0, Rn=0
    let encoding: u32 = 0xA6500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_usax_a1_a_special_cond_11_condition_lt_80_b6500050() {
    // Encoding: 0xB6500050
    // Test aarch32_USAX_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rn=0, cond=11, Rd=0, Rm=0
    let encoding: u32 = 0xB6500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_usax_a1_a_special_cond_12_condition_gt_80_c6500050() {
    // Encoding: 0xC6500050
    // Test aarch32_USAX_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rd=0, cond=12, Rm=0, Rn=0
    let encoding: u32 = 0xC6500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_usax_a1_a_special_cond_13_condition_le_80_d6500050() {
    // Encoding: 0xD6500050
    // Test aarch32_USAX_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=13, Rm=0
    let encoding: u32 = 0xD6500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_usax_a1_a_special_cond_14_condition_al_80_e6500050() {
    // Encoding: 0xE6500050
    // Test aarch32_USAX_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rd=0, cond=14, Rm=0, Rn=0
    let encoding: u32 = 0xE6500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_usax_a1_a_special_cond_15_condition_nv_80_f6500050() {
    // Encoding: 0xF6500050
    // Test aarch32_USAX_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rd=0, cond=15, Rn=0, Rm=0
    let encoding: u32 = 0xF6500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_usax_a1_a_invalid_0_50_06500050() {
    // Encoding: 0x06500050
    // Test aarch32_USAX_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x06500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_USAX_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_usax_a1_a_invalid_1_50_06500050() {
    // Encoding: 0x06500050
    // Test aarch32_USAX_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rd=0, cond=0, Rn=0, Rm=0
    let encoding: u32 = 0x06500050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_USAX_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_usax_t1_a_field_rn_0_min_f040_fae0f040() {
    // Thumb encoding (32): 0xFAE0F040
    // Test aarch32_USAX_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_USAX_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_usax_t1_a_field_rn_1_poweroftwo_f040_fae1f040() {
    // Thumb encoding (32): 0xFAE1F040
    // Test aarch32_USAX_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE1F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_USAX_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_usax_t1_a_field_rd_0_min_f040_fae0f040() {
    // Thumb encoding (32): 0xFAE0F040
    // Test aarch32_USAX_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_USAX_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_usax_t1_a_field_rd_1_poweroftwo_f040_fae0f140() {
    // Thumb encoding (32): 0xFAE0F140
    // Test aarch32_USAX_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F140;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_USAX_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_usax_t1_a_field_rm_0_min_f040_fae0f040() {
    // Thumb encoding (32): 0xFAE0F040
    // Test aarch32_USAX_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_USAX_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_usax_t1_a_field_rm_1_poweroftwo_f040_fae0f041() {
    // Thumb encoding (32): 0xFAE0F041
    // Test aarch32_USAX_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F041;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_USAX_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_usax_t1_a_combo_0_f040_fae0f040() {
    // Thumb encoding (32): 0xFAE0F040
    // Test aarch32_USAX_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_USAX_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_usax_t1_a_invalid_0_f040_fae0f040() {
    // Thumb encoding (32): 0xFAE0F040
    // Test aarch32_USAX_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F040;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_USAX_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_usax_t1_a_invalid_1_f040_fae0f040() {
    // Thumb encoding (32): 0xFAE0F040
    // Test aarch32_USAX_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F040;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_SHADD8_A Tests
// ============================================================================

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_shadd8_a1_a_field_cond_0_min_90_06300090() {
    // Encoding: 0x06300090
    // Test aarch32_SHADD8_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x06300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_shadd8_a1_a_field_cond_1_poweroftwo_90_16300090() {
    // Encoding: 0x16300090
    // Test aarch32_SHADD8_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=1
    let encoding: u32 = 0x16300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_shadd8_a1_a_field_cond_2_poweroftwo_90_26300090() {
    // Encoding: 0x26300090
    // Test aarch32_SHADD8_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=2, Rn=0
    let encoding: u32 = 0x26300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_shadd8_a1_a_field_cond_3_poweroftwo_90_36300090() {
    // Encoding: 0x36300090
    // Test aarch32_SHADD8_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=3, Rn=0
    let encoding: u32 = 0x36300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_shadd8_a1_a_field_cond_4_poweroftwo_90_46300090() {
    // Encoding: 0x46300090
    // Test aarch32_SHADD8_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=4, Rm=0, Rd=0
    let encoding: u32 = 0x46300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_shadd8_a1_a_field_cond_5_poweroftwo_90_56300090() {
    // Encoding: 0x56300090
    // Test aarch32_SHADD8_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=5
    let encoding: u32 = 0x56300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_shadd8_a1_a_field_cond_6_poweroftwo_90_66300090() {
    // Encoding: 0x66300090
    // Test aarch32_SHADD8_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=6, Rm=0
    let encoding: u32 = 0x66300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_shadd8_a1_a_field_cond_7_poweroftwo_90_76300090() {
    // Encoding: 0x76300090
    // Test aarch32_SHADD8_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=7, Rn=0
    let encoding: u32 = 0x76300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_shadd8_a1_a_field_cond_8_poweroftwo_90_86300090() {
    // Encoding: 0x86300090
    // Test aarch32_SHADD8_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=8, Rd=0, Rn=0
    let encoding: u32 = 0x86300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_shadd8_a1_a_field_cond_9_poweroftwo_90_96300090() {
    // Encoding: 0x96300090
    // Test aarch32_SHADD8_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=9, Rn=0, Rd=0
    let encoding: u32 = 0x96300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_shadd8_a1_a_field_cond_10_poweroftwo_90_a6300090() {
    // Encoding: 0xA6300090
    // Test aarch32_SHADD8_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0xA6300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_shadd8_a1_a_field_cond_11_poweroftwo_90_b6300090() {
    // Encoding: 0xB6300090
    // Test aarch32_SHADD8_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=11, Rd=0, Rn=0
    let encoding: u32 = 0xB6300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_shadd8_a1_a_field_cond_12_poweroftwo_90_c6300090() {
    // Encoding: 0xC6300090
    // Test aarch32_SHADD8_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=12, Rn=0
    let encoding: u32 = 0xC6300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_shadd8_a1_a_field_cond_13_poweroftwo_90_d6300090() {
    // Encoding: 0xD6300090
    // Test aarch32_SHADD8_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=13, Rn=0, Rm=0
    let encoding: u32 = 0xD6300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_shadd8_a1_a_field_cond_14_poweroftwo_90_e6300090() {
    // Encoding: 0xE6300090
    // Test aarch32_SHADD8_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=14
    let encoding: u32 = 0xE6300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_shadd8_a1_a_field_cond_15_max_90_f6300090() {
    // Encoding: 0xF6300090
    // Test aarch32_SHADD8_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=15, Rn=0
    let encoding: u32 = 0xF6300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shadd8_a1_a_field_rn_0_min_90_06300090() {
    // Encoding: 0x06300090
    // Test aarch32_SHADD8_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=0, Rm=0
    let encoding: u32 = 0x06300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shadd8_a1_a_field_rn_1_poweroftwo_90_06310090() {
    // Encoding: 0x06310090
    // Test aarch32_SHADD8_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=0, Rn=1
    let encoding: u32 = 0x06310090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shadd8_a1_a_field_rd_0_min_90_06300090() {
    // Encoding: 0x06300090
    // Test aarch32_SHADD8_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=0, Rd=0
    let encoding: u32 = 0x06300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shadd8_a1_a_field_rd_1_poweroftwo_90_06301090() {
    // Encoding: 0x06301090
    // Test aarch32_SHADD8_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=1, Rn=0, Rm=0
    let encoding: u32 = 0x06301090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shadd8_a1_a_field_rm_0_min_90_06300090() {
    // Encoding: 0x06300090
    // Test aarch32_SHADD8_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x06300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shadd8_a1_a_field_rm_1_poweroftwo_90_06300091() {
    // Encoding: 0x06300091
    // Test aarch32_SHADD8_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=0, Rm=1
    let encoding: u32 = 0x06300091;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_shadd8_a1_a_combo_0_90_06300090() {
    // Encoding: 0x06300090
    // Test aarch32_SHADD8_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: cond=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x06300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_shadd8_a1_a_special_cond_0_condition_eq_144_06300090() {
    // Encoding: 0x06300090
    // Test aarch32_SHADD8_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rn=0, cond=0, Rm=0, Rd=0
    let encoding: u32 = 0x06300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_shadd8_a1_a_special_cond_1_condition_ne_144_16300090() {
    // Encoding: 0x16300090
    // Test aarch32_SHADD8_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rn=0, cond=1, Rd=0, Rm=0
    let encoding: u32 = 0x16300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_shadd8_a1_a_special_cond_2_condition_cs_hs_144_26300090() {
    // Encoding: 0x26300090
    // Test aarch32_SHADD8_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x26300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_shadd8_a1_a_special_cond_3_condition_cc_lo_144_36300090() {
    // Encoding: 0x36300090
    // Test aarch32_SHADD8_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=3
    let encoding: u32 = 0x36300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_shadd8_a1_a_special_cond_4_condition_mi_144_46300090() {
    // Encoding: 0x46300090
    // Test aarch32_SHADD8_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x46300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_shadd8_a1_a_special_cond_5_condition_pl_144_56300090() {
    // Encoding: 0x56300090
    // Test aarch32_SHADD8_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rn=0, cond=5, Rm=0, Rd=0
    let encoding: u32 = 0x56300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_shadd8_a1_a_special_cond_6_condition_vs_144_66300090() {
    // Encoding: 0x66300090
    // Test aarch32_SHADD8_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rm=0, cond=6, Rd=0, Rn=0
    let encoding: u32 = 0x66300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_shadd8_a1_a_special_cond_7_condition_vc_144_76300090() {
    // Encoding: 0x76300090
    // Test aarch32_SHADD8_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rm=0, cond=7, Rd=0, Rn=0
    let encoding: u32 = 0x76300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_shadd8_a1_a_special_cond_8_condition_hi_144_86300090() {
    // Encoding: 0x86300090
    // Test aarch32_SHADD8_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x86300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_shadd8_a1_a_special_cond_9_condition_ls_144_96300090() {
    // Encoding: 0x96300090
    // Test aarch32_SHADD8_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=9, Rn=0
    let encoding: u32 = 0x96300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_shadd8_a1_a_special_cond_10_condition_ge_144_a6300090() {
    // Encoding: 0xA6300090
    // Test aarch32_SHADD8_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=10, Rn=0
    let encoding: u32 = 0xA6300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_shadd8_a1_a_special_cond_11_condition_lt_144_b6300090() {
    // Encoding: 0xB6300090
    // Test aarch32_SHADD8_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rm=0, cond=11, Rn=0, Rd=0
    let encoding: u32 = 0xB6300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_shadd8_a1_a_special_cond_12_condition_gt_144_c6300090() {
    // Encoding: 0xC6300090
    // Test aarch32_SHADD8_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=12, Rd=0
    let encoding: u32 = 0xC6300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_shadd8_a1_a_special_cond_13_condition_le_144_d6300090() {
    // Encoding: 0xD6300090
    // Test aarch32_SHADD8_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rn=0, cond=13, Rd=0, Rm=0
    let encoding: u32 = 0xD6300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_shadd8_a1_a_special_cond_14_condition_al_144_e6300090() {
    // Encoding: 0xE6300090
    // Test aarch32_SHADD8_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=14, Rd=0
    let encoding: u32 = 0xE6300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_shadd8_a1_a_special_cond_15_condition_nv_144_f6300090() {
    // Encoding: 0xF6300090
    // Test aarch32_SHADD8_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0xF6300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_shadd8_a1_a_invalid_0_90_06300090() {
    // Encoding: 0x06300090
    // Test aarch32_SHADD8_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x06300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SHADD8_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_shadd8_a1_a_invalid_1_90_06300090() {
    // Encoding: 0x06300090
    // Test aarch32_SHADD8_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x06300090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SHADD8_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shadd8_t1_a_field_rn_0_min_f020_fa80f020() {
    // Thumb encoding (32): 0xFA80F020
    // Test aarch32_SHADD8_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHADD8_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shadd8_t1_a_field_rn_1_poweroftwo_f020_fa81f020() {
    // Thumb encoding (32): 0xFA81F020
    // Test aarch32_SHADD8_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA81F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHADD8_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shadd8_t1_a_field_rd_0_min_f020_fa80f020() {
    // Thumb encoding (32): 0xFA80F020
    // Test aarch32_SHADD8_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHADD8_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shadd8_t1_a_field_rd_1_poweroftwo_f020_fa80f120() {
    // Thumb encoding (32): 0xFA80F120
    // Test aarch32_SHADD8_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F120;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHADD8_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shadd8_t1_a_field_rm_0_min_f020_fa80f020() {
    // Thumb encoding (32): 0xFA80F020
    // Test aarch32_SHADD8_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHADD8_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shadd8_t1_a_field_rm_1_poweroftwo_f020_fa80f021() {
    // Thumb encoding (32): 0xFA80F021
    // Test aarch32_SHADD8_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F021;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHADD8_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_shadd8_t1_a_combo_0_f020_fa80f020() {
    // Thumb encoding (32): 0xFA80F020
    // Test aarch32_SHADD8_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHADD8_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_shadd8_t1_a_invalid_0_f020_fa80f020() {
    // Thumb encoding (32): 0xFA80F020
    // Test aarch32_SHADD8_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F020;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_SHADD8_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_shadd8_t1_a_invalid_1_f020_fa80f020() {
    // Thumb encoding (32): 0xFA80F020
    // Test aarch32_SHADD8_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F020;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_SHASX_A Tests
// ============================================================================

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_shasx_a1_a_field_cond_0_min_30_06300030() {
    // Encoding: 0x06300030
    // Test aarch32_SHASX_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x06300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_shasx_a1_a_field_cond_1_poweroftwo_30_16300030() {
    // Encoding: 0x16300030
    // Test aarch32_SHASX_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=1, Rn=0
    let encoding: u32 = 0x16300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_shasx_a1_a_field_cond_2_poweroftwo_30_26300030() {
    // Encoding: 0x26300030
    // Test aarch32_SHASX_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=2
    let encoding: u32 = 0x26300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_shasx_a1_a_field_cond_3_poweroftwo_30_36300030() {
    // Encoding: 0x36300030
    // Test aarch32_SHASX_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=3
    let encoding: u32 = 0x36300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_shasx_a1_a_field_cond_4_poweroftwo_30_46300030() {
    // Encoding: 0x46300030
    // Test aarch32_SHASX_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=4, Rm=0
    let encoding: u32 = 0x46300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_shasx_a1_a_field_cond_5_poweroftwo_30_56300030() {
    // Encoding: 0x56300030
    // Test aarch32_SHASX_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=5
    let encoding: u32 = 0x56300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_shasx_a1_a_field_cond_6_poweroftwo_30_66300030() {
    // Encoding: 0x66300030
    // Test aarch32_SHASX_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x66300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_shasx_a1_a_field_cond_7_poweroftwo_30_76300030() {
    // Encoding: 0x76300030
    // Test aarch32_SHASX_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=7, Rn=0
    let encoding: u32 = 0x76300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_shasx_a1_a_field_cond_8_poweroftwo_30_86300030() {
    // Encoding: 0x86300030
    // Test aarch32_SHASX_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=8, Rm=0
    let encoding: u32 = 0x86300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_shasx_a1_a_field_cond_9_poweroftwo_30_96300030() {
    // Encoding: 0x96300030
    // Test aarch32_SHASX_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=9
    let encoding: u32 = 0x96300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_shasx_a1_a_field_cond_10_poweroftwo_30_a6300030() {
    // Encoding: 0xA6300030
    // Test aarch32_SHASX_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xA6300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_shasx_a1_a_field_cond_11_poweroftwo_30_b6300030() {
    // Encoding: 0xB6300030
    // Test aarch32_SHASX_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=11
    let encoding: u32 = 0xB6300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_shasx_a1_a_field_cond_12_poweroftwo_30_c6300030() {
    // Encoding: 0xC6300030
    // Test aarch32_SHASX_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=12
    let encoding: u32 = 0xC6300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_shasx_a1_a_field_cond_13_poweroftwo_30_d6300030() {
    // Encoding: 0xD6300030
    // Test aarch32_SHASX_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=13, Rm=0
    let encoding: u32 = 0xD6300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_shasx_a1_a_field_cond_14_poweroftwo_30_e6300030() {
    // Encoding: 0xE6300030
    // Test aarch32_SHASX_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=14
    let encoding: u32 = 0xE6300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_shasx_a1_a_field_cond_15_max_30_f6300030() {
    // Encoding: 0xF6300030
    // Test aarch32_SHASX_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=15
    let encoding: u32 = 0xF6300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shasx_a1_a_field_rn_0_min_30_06300030() {
    // Encoding: 0x06300030
    // Test aarch32_SHASX_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=0, Rm=0
    let encoding: u32 = 0x06300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shasx_a1_a_field_rn_1_poweroftwo_30_06310030() {
    // Encoding: 0x06310030
    // Test aarch32_SHASX_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=1, Rd=0, cond=0
    let encoding: u32 = 0x06310030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shasx_a1_a_field_rd_0_min_30_06300030() {
    // Encoding: 0x06300030
    // Test aarch32_SHASX_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x06300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shasx_a1_a_field_rd_1_poweroftwo_30_06301030() {
    // Encoding: 0x06301030
    // Test aarch32_SHASX_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rd=1
    let encoding: u32 = 0x06301030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shasx_a1_a_field_rm_0_min_30_06300030() {
    // Encoding: 0x06300030
    // Test aarch32_SHASX_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, cond=0, Rn=0, Rm=0
    let encoding: u32 = 0x06300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shasx_a1_a_field_rm_1_poweroftwo_30_06300031() {
    // Encoding: 0x06300031
    // Test aarch32_SHASX_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=1, cond=0, Rn=0, Rd=0
    let encoding: u32 = 0x06300031;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_shasx_a1_a_combo_0_30_06300030() {
    // Encoding: 0x06300030
    // Test aarch32_SHASX_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rn=0, cond=0, Rm=0, Rd=0
    let encoding: u32 = 0x06300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_shasx_a1_a_special_cond_0_condition_eq_48_06300030() {
    // Encoding: 0x06300030
    // Test aarch32_SHASX_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x06300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_shasx_a1_a_special_cond_1_condition_ne_48_16300030() {
    // Encoding: 0x16300030
    // Test aarch32_SHASX_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rm=0, cond=1, Rd=0, Rn=0
    let encoding: u32 = 0x16300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_shasx_a1_a_special_cond_2_condition_cs_hs_48_26300030() {
    // Encoding: 0x26300030
    // Test aarch32_SHASX_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=2, Rd=0
    let encoding: u32 = 0x26300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_shasx_a1_a_special_cond_3_condition_cc_lo_48_36300030() {
    // Encoding: 0x36300030
    // Test aarch32_SHASX_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x36300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_shasx_a1_a_special_cond_4_condition_mi_48_46300030() {
    // Encoding: 0x46300030
    // Test aarch32_SHASX_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rd=0, cond=4, Rn=0, Rm=0
    let encoding: u32 = 0x46300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_shasx_a1_a_special_cond_5_condition_pl_48_56300030() {
    // Encoding: 0x56300030
    // Test aarch32_SHASX_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=5, Rn=0
    let encoding: u32 = 0x56300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_shasx_a1_a_special_cond_6_condition_vs_48_66300030() {
    // Encoding: 0x66300030
    // Test aarch32_SHASX_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x66300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_shasx_a1_a_special_cond_7_condition_vc_48_76300030() {
    // Encoding: 0x76300030
    // Test aarch32_SHASX_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=7
    let encoding: u32 = 0x76300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_shasx_a1_a_special_cond_8_condition_hi_48_86300030() {
    // Encoding: 0x86300030
    // Test aarch32_SHASX_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x86300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_shasx_a1_a_special_cond_9_condition_ls_48_96300030() {
    // Encoding: 0x96300030
    // Test aarch32_SHASX_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=9, Rn=0
    let encoding: u32 = 0x96300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_shasx_a1_a_special_cond_10_condition_ge_48_a6300030() {
    // Encoding: 0xA6300030
    // Test aarch32_SHASX_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0xA6300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_shasx_a1_a_special_cond_11_condition_lt_48_b6300030() {
    // Encoding: 0xB6300030
    // Test aarch32_SHASX_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=11
    let encoding: u32 = 0xB6300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_shasx_a1_a_special_cond_12_condition_gt_48_c6300030() {
    // Encoding: 0xC6300030
    // Test aarch32_SHASX_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=12, Rm=0
    let encoding: u32 = 0xC6300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_shasx_a1_a_special_cond_13_condition_le_48_d6300030() {
    // Encoding: 0xD6300030
    // Test aarch32_SHASX_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: cond=13, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0xD6300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_shasx_a1_a_special_cond_14_condition_al_48_e6300030() {
    // Encoding: 0xE6300030
    // Test aarch32_SHASX_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=14
    let encoding: u32 = 0xE6300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_shasx_a1_a_special_cond_15_condition_nv_48_f6300030() {
    // Encoding: 0xF6300030
    // Test aarch32_SHASX_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xF6300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_shasx_a1_a_invalid_0_30_06300030() {
    // Encoding: 0x06300030
    // Test aarch32_SHASX_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x06300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SHASX_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_shasx_a1_a_invalid_1_30_06300030() {
    // Encoding: 0x06300030
    // Test aarch32_SHASX_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: cond=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x06300030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SHASX_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shasx_t1_a_field_rn_0_min_f020_faa0f020() {
    // Thumb encoding (32): 0xFAA0F020
    // Test aarch32_SHASX_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHASX_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shasx_t1_a_field_rn_1_poweroftwo_f020_faa1f020() {
    // Thumb encoding (32): 0xFAA1F020
    // Test aarch32_SHASX_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA1F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHASX_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shasx_t1_a_field_rd_0_min_f020_faa0f020() {
    // Thumb encoding (32): 0xFAA0F020
    // Test aarch32_SHASX_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHASX_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shasx_t1_a_field_rd_1_poweroftwo_f020_faa0f120() {
    // Thumb encoding (32): 0xFAA0F120
    // Test aarch32_SHASX_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F120;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHASX_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shasx_t1_a_field_rm_0_min_f020_faa0f020() {
    // Thumb encoding (32): 0xFAA0F020
    // Test aarch32_SHASX_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHASX_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shasx_t1_a_field_rm_1_poweroftwo_f020_faa0f021() {
    // Thumb encoding (32): 0xFAA0F021
    // Test aarch32_SHASX_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F021;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHASX_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_shasx_t1_a_combo_0_f020_faa0f020() {
    // Thumb encoding (32): 0xFAA0F020
    // Test aarch32_SHASX_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHASX_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_shasx_t1_a_invalid_0_f020_faa0f020() {
    // Thumb encoding (32): 0xFAA0F020
    // Test aarch32_SHASX_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F020;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_SHASX_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_shasx_t1_a_invalid_1_f020_faa0f020() {
    // Thumb encoding (32): 0xFAA0F020
    // Test aarch32_SHASX_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F020;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_SSAX_A Tests
// ============================================================================

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_ssax_a1_a_field_cond_0_min_50_06100050() {
    // Encoding: 0x06100050
    // Test aarch32_SSAX_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=0
    let encoding: u32 = 0x06100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_ssax_a1_a_field_cond_1_poweroftwo_50_16100050() {
    // Encoding: 0x16100050
    // Test aarch32_SSAX_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=1, Rm=0, Rd=0
    let encoding: u32 = 0x16100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_ssax_a1_a_field_cond_2_poweroftwo_50_26100050() {
    // Encoding: 0x26100050
    // Test aarch32_SSAX_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=2, Rn=0
    let encoding: u32 = 0x26100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_ssax_a1_a_field_cond_3_poweroftwo_50_36100050() {
    // Encoding: 0x36100050
    // Test aarch32_SSAX_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x36100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_ssax_a1_a_field_cond_4_poweroftwo_50_46100050() {
    // Encoding: 0x46100050
    // Test aarch32_SSAX_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=4, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x46100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_ssax_a1_a_field_cond_5_poweroftwo_50_56100050() {
    // Encoding: 0x56100050
    // Test aarch32_SSAX_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=5, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x56100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_ssax_a1_a_field_cond_6_poweroftwo_50_66100050() {
    // Encoding: 0x66100050
    // Test aarch32_SSAX_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=6, Rm=0
    let encoding: u32 = 0x66100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_ssax_a1_a_field_cond_7_poweroftwo_50_76100050() {
    // Encoding: 0x76100050
    // Test aarch32_SSAX_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=7, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x76100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_ssax_a1_a_field_cond_8_poweroftwo_50_86100050() {
    // Encoding: 0x86100050
    // Test aarch32_SSAX_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=8
    let encoding: u32 = 0x86100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_ssax_a1_a_field_cond_9_poweroftwo_50_96100050() {
    // Encoding: 0x96100050
    // Test aarch32_SSAX_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x96100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_ssax_a1_a_field_cond_10_poweroftwo_50_a6100050() {
    // Encoding: 0xA6100050
    // Test aarch32_SSAX_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xA6100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_ssax_a1_a_field_cond_11_poweroftwo_50_b6100050() {
    // Encoding: 0xB6100050
    // Test aarch32_SSAX_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=11
    let encoding: u32 = 0xB6100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_ssax_a1_a_field_cond_12_poweroftwo_50_c6100050() {
    // Encoding: 0xC6100050
    // Test aarch32_SSAX_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=12
    let encoding: u32 = 0xC6100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_ssax_a1_a_field_cond_13_poweroftwo_50_d6100050() {
    // Encoding: 0xD6100050
    // Test aarch32_SSAX_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=13, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0xD6100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_ssax_a1_a_field_cond_14_poweroftwo_50_e6100050() {
    // Encoding: 0xE6100050
    // Test aarch32_SSAX_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=14, Rd=0, Rm=0
    let encoding: u32 = 0xE6100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_ssax_a1_a_field_cond_15_max_50_f6100050() {
    // Encoding: 0xF6100050
    // Test aarch32_SSAX_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=15, Rd=0
    let encoding: u32 = 0xF6100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_ssax_a1_a_field_rn_0_min_50_06100050() {
    // Encoding: 0x06100050
    // Test aarch32_SSAX_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, cond=0, Rm=0, Rd=0
    let encoding: u32 = 0x06100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_ssax_a1_a_field_rn_1_poweroftwo_50_06110050() {
    // Encoding: 0x06110050
    // Test aarch32_SSAX_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=0, Rn=1
    let encoding: u32 = 0x06110050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_ssax_a1_a_field_rd_0_min_50_06100050() {
    // Encoding: 0x06100050
    // Test aarch32_SSAX_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x06100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_ssax_a1_a_field_rd_1_poweroftwo_50_06101050() {
    // Encoding: 0x06101050
    // Test aarch32_SSAX_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=0, Rm=0, Rd=1
    let encoding: u32 = 0x06101050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_ssax_a1_a_field_rm_0_min_50_06100050() {
    // Encoding: 0x06100050
    // Test aarch32_SSAX_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x06100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_ssax_a1_a_field_rm_1_poweroftwo_50_06100051() {
    // Encoding: 0x06100051
    // Test aarch32_SSAX_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=0, Rm=1
    let encoding: u32 = 0x06100051;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_ssax_a1_a_combo_0_50_06100050() {
    // Encoding: 0x06100050
    // Test aarch32_SSAX_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x06100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_ssax_a1_a_special_cond_0_condition_eq_80_06100050() {
    // Encoding: 0x06100050
    // Test aarch32_SSAX_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=0
    let encoding: u32 = 0x06100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_ssax_a1_a_special_cond_1_condition_ne_80_16100050() {
    // Encoding: 0x16100050
    // Test aarch32_SSAX_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x16100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_ssax_a1_a_special_cond_2_condition_cs_hs_80_26100050() {
    // Encoding: 0x26100050
    // Test aarch32_SSAX_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x26100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_ssax_a1_a_special_cond_3_condition_cc_lo_80_36100050() {
    // Encoding: 0x36100050
    // Test aarch32_SSAX_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=3
    let encoding: u32 = 0x36100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_ssax_a1_a_special_cond_4_condition_mi_80_46100050() {
    // Encoding: 0x46100050
    // Test aarch32_SSAX_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rm=0, cond=4, Rn=0, Rd=0
    let encoding: u32 = 0x46100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_ssax_a1_a_special_cond_5_condition_pl_80_56100050() {
    // Encoding: 0x56100050
    // Test aarch32_SSAX_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=5, Rn=0
    let encoding: u32 = 0x56100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_ssax_a1_a_special_cond_6_condition_vs_80_66100050() {
    // Encoding: 0x66100050
    // Test aarch32_SSAX_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x66100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_ssax_a1_a_special_cond_7_condition_vc_80_76100050() {
    // Encoding: 0x76100050
    // Test aarch32_SSAX_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=7
    let encoding: u32 = 0x76100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_ssax_a1_a_special_cond_8_condition_hi_80_86100050() {
    // Encoding: 0x86100050
    // Test aarch32_SSAX_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=8
    let encoding: u32 = 0x86100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_ssax_a1_a_special_cond_9_condition_ls_80_96100050() {
    // Encoding: 0x96100050
    // Test aarch32_SSAX_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=9, Rd=0
    let encoding: u32 = 0x96100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_ssax_a1_a_special_cond_10_condition_ge_80_a6100050() {
    // Encoding: 0xA6100050
    // Test aarch32_SSAX_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xA6100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_ssax_a1_a_special_cond_11_condition_lt_80_b6100050() {
    // Encoding: 0xB6100050
    // Test aarch32_SSAX_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=11
    let encoding: u32 = 0xB6100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_ssax_a1_a_special_cond_12_condition_gt_80_c6100050() {
    // Encoding: 0xC6100050
    // Test aarch32_SSAX_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=12, Rd=0
    let encoding: u32 = 0xC6100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_ssax_a1_a_special_cond_13_condition_le_80_d6100050() {
    // Encoding: 0xD6100050
    // Test aarch32_SSAX_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rd=0, cond=13, Rm=0, Rn=0
    let encoding: u32 = 0xD6100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_ssax_a1_a_special_cond_14_condition_al_80_e6100050() {
    // Encoding: 0xE6100050
    // Test aarch32_SSAX_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rd=0, cond=14, Rm=0, Rn=0
    let encoding: u32 = 0xE6100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_ssax_a1_a_special_cond_15_condition_nv_80_f6100050() {
    // Encoding: 0xF6100050
    // Test aarch32_SSAX_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=15, Rd=0
    let encoding: u32 = 0xF6100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_ssax_a1_a_invalid_0_50_06100050() {
    // Encoding: 0x06100050
    // Test aarch32_SSAX_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: cond=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x06100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SSAX_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_ssax_a1_a_invalid_1_50_06100050() {
    // Encoding: 0x06100050
    // Test aarch32_SSAX_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rd=0, cond=0, Rm=0, Rn=0
    let encoding: u32 = 0x06100050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SSAX_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_ssax_t1_a_field_rn_0_min_f000_fae0f000() {
    // Thumb encoding (32): 0xFAE0F000
    // Test aarch32_SSAX_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SSAX_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_ssax_t1_a_field_rn_1_poweroftwo_f000_fae1f000() {
    // Thumb encoding (32): 0xFAE1F000
    // Test aarch32_SSAX_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE1F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SSAX_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_ssax_t1_a_field_rd_0_min_f000_fae0f000() {
    // Thumb encoding (32): 0xFAE0F000
    // Test aarch32_SSAX_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SSAX_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_ssax_t1_a_field_rd_1_poweroftwo_f000_fae0f100() {
    // Thumb encoding (32): 0xFAE0F100
    // Test aarch32_SSAX_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F100;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SSAX_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_ssax_t1_a_field_rm_0_min_f000_fae0f000() {
    // Thumb encoding (32): 0xFAE0F000
    // Test aarch32_SSAX_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SSAX_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_ssax_t1_a_field_rm_1_poweroftwo_f000_fae0f001() {
    // Thumb encoding (32): 0xFAE0F001
    // Test aarch32_SSAX_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F001;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SSAX_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_ssax_t1_a_combo_0_f000_fae0f000() {
    // Thumb encoding (32): 0xFAE0F000
    // Test aarch32_SSAX_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SSAX_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_ssax_t1_a_invalid_0_f000_fae0f000() {
    // Thumb encoding (32): 0xFAE0F000
    // Test aarch32_SSAX_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_SSAX_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_ssax_t1_a_invalid_1_f000_fae0f000() {
    // Thumb encoding (32): 0xFAE0F000
    // Test aarch32_SSAX_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_USUB8_A Tests
// ============================================================================

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_usub8_a1_a_field_cond_0_min_f0_065000f0() {
    // Encoding: 0x065000F0
    // Test aarch32_USUB8_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=0
    let encoding: u32 = 0x065000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_usub8_a1_a_field_cond_1_poweroftwo_f0_165000f0() {
    // Encoding: 0x165000F0
    // Test aarch32_USUB8_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=1, Rn=0
    let encoding: u32 = 0x165000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_usub8_a1_a_field_cond_2_poweroftwo_f0_265000f0() {
    // Encoding: 0x265000F0
    // Test aarch32_USUB8_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=2
    let encoding: u32 = 0x265000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_usub8_a1_a_field_cond_3_poweroftwo_f0_365000f0() {
    // Encoding: 0x365000F0
    // Test aarch32_USUB8_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x365000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_usub8_a1_a_field_cond_4_poweroftwo_f0_465000f0() {
    // Encoding: 0x465000F0
    // Test aarch32_USUB8_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=4, Rn=0, Rm=0
    let encoding: u32 = 0x465000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_usub8_a1_a_field_cond_5_poweroftwo_f0_565000f0() {
    // Encoding: 0x565000F0
    // Test aarch32_USUB8_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=5, Rm=0, Rd=0
    let encoding: u32 = 0x565000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_usub8_a1_a_field_cond_6_poweroftwo_f0_665000f0() {
    // Encoding: 0x665000F0
    // Test aarch32_USUB8_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x665000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_usub8_a1_a_field_cond_7_poweroftwo_f0_765000f0() {
    // Encoding: 0x765000F0
    // Test aarch32_USUB8_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=7, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x765000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_usub8_a1_a_field_cond_8_poweroftwo_f0_865000f0() {
    // Encoding: 0x865000F0
    // Test aarch32_USUB8_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=8, Rd=0, Rn=0
    let encoding: u32 = 0x865000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_usub8_a1_a_field_cond_9_poweroftwo_f0_965000f0() {
    // Encoding: 0x965000F0
    // Test aarch32_USUB8_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=9
    let encoding: u32 = 0x965000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_usub8_a1_a_field_cond_10_poweroftwo_f0_a65000f0() {
    // Encoding: 0xA65000F0
    // Test aarch32_USUB8_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=10
    let encoding: u32 = 0xA65000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_usub8_a1_a_field_cond_11_poweroftwo_f0_b65000f0() {
    // Encoding: 0xB65000F0
    // Test aarch32_USUB8_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=11
    let encoding: u32 = 0xB65000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_usub8_a1_a_field_cond_12_poweroftwo_f0_c65000f0() {
    // Encoding: 0xC65000F0
    // Test aarch32_USUB8_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0xC65000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_usub8_a1_a_field_cond_13_poweroftwo_f0_d65000f0() {
    // Encoding: 0xD65000F0
    // Test aarch32_USUB8_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=13, Rd=0, Rm=0
    let encoding: u32 = 0xD65000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_usub8_a1_a_field_cond_14_poweroftwo_f0_e65000f0() {
    // Encoding: 0xE65000F0
    // Test aarch32_USUB8_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=14, Rd=0
    let encoding: u32 = 0xE65000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_usub8_a1_a_field_cond_15_max_f0_f65000f0() {
    // Encoding: 0xF65000F0
    // Test aarch32_USUB8_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=15, Rn=0
    let encoding: u32 = 0xF65000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_usub8_a1_a_field_rn_0_min_f0_065000f0() {
    // Encoding: 0x065000F0
    // Test aarch32_USUB8_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rd=0, Rn=0
    let encoding: u32 = 0x065000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_usub8_a1_a_field_rn_1_poweroftwo_f0_065100f0() {
    // Encoding: 0x065100F0
    // Test aarch32_USUB8_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=1, Rm=0
    let encoding: u32 = 0x065100F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_usub8_a1_a_field_rd_0_min_f0_065000f0() {
    // Encoding: 0x065000F0
    // Test aarch32_USUB8_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, cond=0, Rm=0, Rd=0
    let encoding: u32 = 0x065000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_usub8_a1_a_field_rd_1_poweroftwo_f0_065010f0() {
    // Encoding: 0x065010F0
    // Test aarch32_USUB8_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=0, Rd=1
    let encoding: u32 = 0x065010F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_usub8_a1_a_field_rm_0_min_f0_065000f0() {
    // Encoding: 0x065000F0
    // Test aarch32_USUB8_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=0, Rd=0
    let encoding: u32 = 0x065000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_usub8_a1_a_field_rm_1_poweroftwo_f0_065000f1() {
    // Encoding: 0x065000F1
    // Test aarch32_USUB8_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=0, Rm=1
    let encoding: u32 = 0x065000F1;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_usub8_a1_a_combo_0_f0_065000f0() {
    // Encoding: 0x065000F0
    // Test aarch32_USUB8_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=0
    let encoding: u32 = 0x065000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_usub8_a1_a_special_cond_0_condition_eq_240_065000f0() {
    // Encoding: 0x065000F0
    // Test aarch32_USUB8_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=0, Rd=0
    let encoding: u32 = 0x065000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_usub8_a1_a_special_cond_1_condition_ne_240_165000f0() {
    // Encoding: 0x165000F0
    // Test aarch32_USUB8_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=1, Rm=0
    let encoding: u32 = 0x165000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_usub8_a1_a_special_cond_2_condition_cs_hs_240_265000f0() {
    // Encoding: 0x265000F0
    // Test aarch32_USUB8_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=2
    let encoding: u32 = 0x265000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_usub8_a1_a_special_cond_3_condition_cc_lo_240_365000f0() {
    // Encoding: 0x365000F0
    // Test aarch32_USUB8_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x365000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_usub8_a1_a_special_cond_4_condition_mi_240_465000f0() {
    // Encoding: 0x465000F0
    // Test aarch32_USUB8_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x465000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_usub8_a1_a_special_cond_5_condition_pl_240_565000f0() {
    // Encoding: 0x565000F0
    // Test aarch32_USUB8_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rd=0, cond=5, Rm=0, Rn=0
    let encoding: u32 = 0x565000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_usub8_a1_a_special_cond_6_condition_vs_240_665000f0() {
    // Encoding: 0x665000F0
    // Test aarch32_USUB8_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=6, Rn=0
    let encoding: u32 = 0x665000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_usub8_a1_a_special_cond_7_condition_vc_240_765000f0() {
    // Encoding: 0x765000F0
    // Test aarch32_USUB8_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=7, Rn=0
    let encoding: u32 = 0x765000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_usub8_a1_a_special_cond_8_condition_hi_240_865000f0() {
    // Encoding: 0x865000F0
    // Test aarch32_USUB8_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rd=0, cond=8, Rm=0, Rn=0
    let encoding: u32 = 0x865000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_usub8_a1_a_special_cond_9_condition_ls_240_965000f0() {
    // Encoding: 0x965000F0
    // Test aarch32_USUB8_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=9, Rd=0
    let encoding: u32 = 0x965000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_usub8_a1_a_special_cond_10_condition_ge_240_a65000f0() {
    // Encoding: 0xA65000F0
    // Test aarch32_USUB8_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0xA65000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_usub8_a1_a_special_cond_11_condition_lt_240_b65000f0() {
    // Encoding: 0xB65000F0
    // Test aarch32_USUB8_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=11
    let encoding: u32 = 0xB65000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_usub8_a1_a_special_cond_12_condition_gt_240_c65000f0() {
    // Encoding: 0xC65000F0
    // Test aarch32_USUB8_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rd=0, cond=12, Rm=0, Rn=0
    let encoding: u32 = 0xC65000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_usub8_a1_a_special_cond_13_condition_le_240_d65000f0() {
    // Encoding: 0xD65000F0
    // Test aarch32_USUB8_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=13
    let encoding: u32 = 0xD65000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_usub8_a1_a_special_cond_14_condition_al_240_e65000f0() {
    // Encoding: 0xE65000F0
    // Test aarch32_USUB8_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: cond=14, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0xE65000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_usub8_a1_a_special_cond_15_condition_nv_240_f65000f0() {
    // Encoding: 0xF65000F0
    // Test aarch32_USUB8_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=15, Rm=0
    let encoding: u32 = 0xF65000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_usub8_a1_a_invalid_0_f0_065000f0() {
    // Encoding: 0x065000F0
    // Test aarch32_USUB8_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=0
    let encoding: u32 = 0x065000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_USUB8_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_usub8_a1_a_invalid_1_f0_065000f0() {
    // Encoding: 0x065000F0
    // Test aarch32_USUB8_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=0, Rn=0
    let encoding: u32 = 0x065000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_USUB8_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_usub8_t1_a_field_rn_0_min_f040_fac0f040() {
    // Thumb encoding (32): 0xFAC0F040
    // Test aarch32_USUB8_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_USUB8_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_usub8_t1_a_field_rn_1_poweroftwo_f040_fac1f040() {
    // Thumb encoding (32): 0xFAC1F040
    // Test aarch32_USUB8_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC1F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_USUB8_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_usub8_t1_a_field_rd_0_min_f040_fac0f040() {
    // Thumb encoding (32): 0xFAC0F040
    // Test aarch32_USUB8_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_USUB8_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_usub8_t1_a_field_rd_1_poweroftwo_f040_fac0f140() {
    // Thumb encoding (32): 0xFAC0F140
    // Test aarch32_USUB8_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F140;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_USUB8_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_usub8_t1_a_field_rm_0_min_f040_fac0f040() {
    // Thumb encoding (32): 0xFAC0F040
    // Test aarch32_USUB8_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_USUB8_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_usub8_t1_a_field_rm_1_poweroftwo_f040_fac0f041() {
    // Thumb encoding (32): 0xFAC0F041
    // Test aarch32_USUB8_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F041;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_USUB8_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_usub8_t1_a_combo_0_f040_fac0f040() {
    // Thumb encoding (32): 0xFAC0F040
    // Test aarch32_USUB8_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_USUB8_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_usub8_t1_a_invalid_0_f040_fac0f040() {
    // Thumb encoding (32): 0xFAC0F040
    // Test aarch32_USUB8_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F040;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_USUB8_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_usub8_t1_a_invalid_1_f040_fac0f040() {
    // Thumb encoding (32): 0xFAC0F040
    // Test aarch32_USUB8_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F040;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_SASX_A Tests
// ============================================================================

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_sasx_a1_a_field_cond_0_min_30_06100030() {
    // Encoding: 0x06100030
    // Test aarch32_SASX_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=0
    let encoding: u32 = 0x06100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_sasx_a1_a_field_cond_1_poweroftwo_30_16100030() {
    // Encoding: 0x16100030
    // Test aarch32_SASX_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=1
    let encoding: u32 = 0x16100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_sasx_a1_a_field_cond_2_poweroftwo_30_26100030() {
    // Encoding: 0x26100030
    // Test aarch32_SASX_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=2, Rn=0
    let encoding: u32 = 0x26100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_sasx_a1_a_field_cond_3_poweroftwo_30_36100030() {
    // Encoding: 0x36100030
    // Test aarch32_SASX_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x36100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_sasx_a1_a_field_cond_4_poweroftwo_30_46100030() {
    // Encoding: 0x46100030
    // Test aarch32_SASX_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=4, Rd=0
    let encoding: u32 = 0x46100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_sasx_a1_a_field_cond_5_poweroftwo_30_56100030() {
    // Encoding: 0x56100030
    // Test aarch32_SASX_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=5, Rd=0
    let encoding: u32 = 0x56100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_sasx_a1_a_field_cond_6_poweroftwo_30_66100030() {
    // Encoding: 0x66100030
    // Test aarch32_SASX_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x66100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_sasx_a1_a_field_cond_7_poweroftwo_30_76100030() {
    // Encoding: 0x76100030
    // Test aarch32_SASX_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=7, Rd=0, Rm=0
    let encoding: u32 = 0x76100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_sasx_a1_a_field_cond_8_poweroftwo_30_86100030() {
    // Encoding: 0x86100030
    // Test aarch32_SASX_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=8, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x86100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_sasx_a1_a_field_cond_9_poweroftwo_30_96100030() {
    // Encoding: 0x96100030
    // Test aarch32_SASX_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x96100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_sasx_a1_a_field_cond_10_poweroftwo_30_a6100030() {
    // Encoding: 0xA6100030
    // Test aarch32_SASX_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=10, Rm=0, Rn=0
    let encoding: u32 = 0xA6100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_sasx_a1_a_field_cond_11_poweroftwo_30_b6100030() {
    // Encoding: 0xB6100030
    // Test aarch32_SASX_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=11, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xB6100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_sasx_a1_a_field_cond_12_poweroftwo_30_c6100030() {
    // Encoding: 0xC6100030
    // Test aarch32_SASX_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0xC6100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_sasx_a1_a_field_cond_13_poweroftwo_30_d6100030() {
    // Encoding: 0xD6100030
    // Test aarch32_SASX_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=13, Rn=0, Rm=0
    let encoding: u32 = 0xD6100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_sasx_a1_a_field_cond_14_poweroftwo_30_e6100030() {
    // Encoding: 0xE6100030
    // Test aarch32_SASX_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=14, Rm=0, Rn=0
    let encoding: u32 = 0xE6100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_sasx_a1_a_field_cond_15_max_30_f6100030() {
    // Encoding: 0xF6100030
    // Test aarch32_SASX_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=15
    let encoding: u32 = 0xF6100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sasx_a1_a_field_rn_0_min_30_06100030() {
    // Encoding: 0x06100030
    // Test aarch32_SASX_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x06100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sasx_a1_a_field_rn_1_poweroftwo_30_06110030() {
    // Encoding: 0x06110030
    // Test aarch32_SASX_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=1, Rd=0, cond=0, Rm=0
    let encoding: u32 = 0x06110030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sasx_a1_a_field_rd_0_min_30_06100030() {
    // Encoding: 0x06100030
    // Test aarch32_SASX_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x06100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sasx_a1_a_field_rd_1_poweroftwo_30_06101030() {
    // Encoding: 0x06101030
    // Test aarch32_SASX_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=1, Rm=0, cond=0
    let encoding: u32 = 0x06101030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sasx_a1_a_field_rm_0_min_30_06100030() {
    // Encoding: 0x06100030
    // Test aarch32_SASX_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x06100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sasx_a1_a_field_rm_1_poweroftwo_30_06100031() {
    // Encoding: 0x06100031
    // Test aarch32_SASX_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=0, Rm=1
    let encoding: u32 = 0x06100031;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_sasx_a1_a_combo_0_30_06100030() {
    // Encoding: 0x06100030
    // Test aarch32_SASX_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: cond=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x06100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_sasx_a1_a_special_cond_0_condition_eq_48_06100030() {
    // Encoding: 0x06100030
    // Test aarch32_SASX_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x06100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_sasx_a1_a_special_cond_1_condition_ne_48_16100030() {
    // Encoding: 0x16100030
    // Test aarch32_SASX_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x16100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_sasx_a1_a_special_cond_2_condition_cs_hs_48_26100030() {
    // Encoding: 0x26100030
    // Test aarch32_SASX_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x26100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_sasx_a1_a_special_cond_3_condition_cc_lo_48_36100030() {
    // Encoding: 0x36100030
    // Test aarch32_SASX_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rn=0, cond=3, Rd=0, Rm=0
    let encoding: u32 = 0x36100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_sasx_a1_a_special_cond_4_condition_mi_48_46100030() {
    // Encoding: 0x46100030
    // Test aarch32_SASX_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x46100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_sasx_a1_a_special_cond_5_condition_pl_48_56100030() {
    // Encoding: 0x56100030
    // Test aarch32_SASX_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rm=0, cond=5, Rn=0, Rd=0
    let encoding: u32 = 0x56100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_sasx_a1_a_special_cond_6_condition_vs_48_66100030() {
    // Encoding: 0x66100030
    // Test aarch32_SASX_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x66100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_sasx_a1_a_special_cond_7_condition_vc_48_76100030() {
    // Encoding: 0x76100030
    // Test aarch32_SASX_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rn=0, cond=7, Rd=0, Rm=0
    let encoding: u32 = 0x76100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_sasx_a1_a_special_cond_8_condition_hi_48_86100030() {
    // Encoding: 0x86100030
    // Test aarch32_SASX_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x86100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_sasx_a1_a_special_cond_9_condition_ls_48_96100030() {
    // Encoding: 0x96100030
    // Test aarch32_SASX_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rm=0, cond=9, Rn=0, Rd=0
    let encoding: u32 = 0x96100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_sasx_a1_a_special_cond_10_condition_ge_48_a6100030() {
    // Encoding: 0xA6100030
    // Test aarch32_SASX_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=10
    let encoding: u32 = 0xA6100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_sasx_a1_a_special_cond_11_condition_lt_48_b6100030() {
    // Encoding: 0xB6100030
    // Test aarch32_SASX_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=11, Rm=0
    let encoding: u32 = 0xB6100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_sasx_a1_a_special_cond_12_condition_gt_48_c6100030() {
    // Encoding: 0xC6100030
    // Test aarch32_SASX_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rn=0, cond=12, Rm=0, Rd=0
    let encoding: u32 = 0xC6100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_sasx_a1_a_special_cond_13_condition_le_48_d6100030() {
    // Encoding: 0xD6100030
    // Test aarch32_SASX_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: cond=13, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0xD6100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_sasx_a1_a_special_cond_14_condition_al_48_e6100030() {
    // Encoding: 0xE6100030
    // Test aarch32_SASX_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: cond=14, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0xE6100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_sasx_a1_a_special_cond_15_condition_nv_48_f6100030() {
    // Encoding: 0xF6100030
    // Test aarch32_SASX_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=15
    let encoding: u32 = 0xF6100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sasx_a1_a_invalid_0_30_06100030() {
    // Encoding: 0x06100030
    // Test aarch32_SASX_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x06100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SASX_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sasx_a1_a_invalid_1_30_06100030() {
    // Encoding: 0x06100030
    // Test aarch32_SASX_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=0, Rd=0
    let encoding: u32 = 0x06100030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SASX_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sasx_t1_a_field_rn_0_min_f000_faa0f000() {
    // Thumb encoding (32): 0xFAA0F000
    // Test aarch32_SASX_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SASX_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sasx_t1_a_field_rn_1_poweroftwo_f000_faa1f000() {
    // Thumb encoding (32): 0xFAA1F000
    // Test aarch32_SASX_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA1F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SASX_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sasx_t1_a_field_rd_0_min_f000_faa0f000() {
    // Thumb encoding (32): 0xFAA0F000
    // Test aarch32_SASX_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SASX_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sasx_t1_a_field_rd_1_poweroftwo_f000_faa0f100() {
    // Thumb encoding (32): 0xFAA0F100
    // Test aarch32_SASX_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rd=1, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F100;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SASX_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sasx_t1_a_field_rm_0_min_f000_faa0f000() {
    // Thumb encoding (32): 0xFAA0F000
    // Test aarch32_SASX_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SASX_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sasx_t1_a_field_rm_1_poweroftwo_f000_faa0f001() {
    // Thumb encoding (32): 0xFAA0F001
    // Test aarch32_SASX_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F001;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SASX_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_sasx_t1_a_combo_0_f000_faa0f000() {
    // Thumb encoding (32): 0xFAA0F000
    // Test aarch32_SASX_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SASX_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sasx_t1_a_invalid_0_f000_faa0f000() {
    // Thumb encoding (32): 0xFAA0F000
    // Test aarch32_SASX_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_SASX_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sasx_t1_a_invalid_1_f000_faa0f000() {
    // Thumb encoding (32): 0xFAA0F000
    // Test aarch32_SASX_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_UHSUB16_A Tests
// ============================================================================

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_uhsub16_a1_a_field_cond_0_min_70_06700070() {
    // Encoding: 0x06700070
    // Test aarch32_UHSUB16_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=0, Rn=0
    let encoding: u32 = 0x06700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_uhsub16_a1_a_field_cond_1_poweroftwo_70_16700070() {
    // Encoding: 0x16700070
    // Test aarch32_UHSUB16_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=1
    let encoding: u32 = 0x16700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_uhsub16_a1_a_field_cond_2_poweroftwo_70_26700070() {
    // Encoding: 0x26700070
    // Test aarch32_UHSUB16_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=2, Rm=0, Rn=0
    let encoding: u32 = 0x26700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_uhsub16_a1_a_field_cond_3_poweroftwo_70_36700070() {
    // Encoding: 0x36700070
    // Test aarch32_UHSUB16_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=3, Rn=0, Rd=0
    let encoding: u32 = 0x36700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_uhsub16_a1_a_field_cond_4_poweroftwo_70_46700070() {
    // Encoding: 0x46700070
    // Test aarch32_UHSUB16_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=4, Rn=0
    let encoding: u32 = 0x46700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_uhsub16_a1_a_field_cond_5_poweroftwo_70_56700070() {
    // Encoding: 0x56700070
    // Test aarch32_UHSUB16_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=5, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x56700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_uhsub16_a1_a_field_cond_6_poweroftwo_70_66700070() {
    // Encoding: 0x66700070
    // Test aarch32_UHSUB16_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=6
    let encoding: u32 = 0x66700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_uhsub16_a1_a_field_cond_7_poweroftwo_70_76700070() {
    // Encoding: 0x76700070
    // Test aarch32_UHSUB16_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=7
    let encoding: u32 = 0x76700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_uhsub16_a1_a_field_cond_8_poweroftwo_70_86700070() {
    // Encoding: 0x86700070
    // Test aarch32_UHSUB16_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=8, Rd=0
    let encoding: u32 = 0x86700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_uhsub16_a1_a_field_cond_9_poweroftwo_70_96700070() {
    // Encoding: 0x96700070
    // Test aarch32_UHSUB16_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=9, Rm=0
    let encoding: u32 = 0x96700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_uhsub16_a1_a_field_cond_10_poweroftwo_70_a6700070() {
    // Encoding: 0xA6700070
    // Test aarch32_UHSUB16_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=10
    let encoding: u32 = 0xA6700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_uhsub16_a1_a_field_cond_11_poweroftwo_70_b6700070() {
    // Encoding: 0xB6700070
    // Test aarch32_UHSUB16_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=11, Rn=0, Rd=0
    let encoding: u32 = 0xB6700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_uhsub16_a1_a_field_cond_12_poweroftwo_70_c6700070() {
    // Encoding: 0xC6700070
    // Test aarch32_UHSUB16_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=12
    let encoding: u32 = 0xC6700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_uhsub16_a1_a_field_cond_13_poweroftwo_70_d6700070() {
    // Encoding: 0xD6700070
    // Test aarch32_UHSUB16_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=13, Rd=0, Rm=0
    let encoding: u32 = 0xD6700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_uhsub16_a1_a_field_cond_14_poweroftwo_70_e6700070() {
    // Encoding: 0xE6700070
    // Test aarch32_UHSUB16_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0xE6700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_uhsub16_a1_a_field_cond_15_max_70_f6700070() {
    // Encoding: 0xF6700070
    // Test aarch32_UHSUB16_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=15
    let encoding: u32 = 0xF6700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhsub16_a1_a_field_rn_0_min_70_06700070() {
    // Encoding: 0x06700070
    // Test aarch32_UHSUB16_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x06700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhsub16_a1_a_field_rn_1_poweroftwo_70_06710070() {
    // Encoding: 0x06710070
    // Test aarch32_UHSUB16_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=1, Rm=0, cond=0
    let encoding: u32 = 0x06710070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhsub16_a1_a_field_rd_0_min_70_06700070() {
    // Encoding: 0x06700070
    // Test aarch32_UHSUB16_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=0, Rd=0
    let encoding: u32 = 0x06700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhsub16_a1_a_field_rd_1_poweroftwo_70_06701070() {
    // Encoding: 0x06701070
    // Test aarch32_UHSUB16_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=0, Rd=1
    let encoding: u32 = 0x06701070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhsub16_a1_a_field_rm_0_min_70_06700070() {
    // Encoding: 0x06700070
    // Test aarch32_UHSUB16_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x06700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhsub16_a1_a_field_rm_1_poweroftwo_70_06700071() {
    // Encoding: 0x06700071
    // Test aarch32_UHSUB16_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=1, Rn=0
    let encoding: u32 = 0x06700071;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_uhsub16_a1_a_combo_0_70_06700070() {
    // Encoding: 0x06700070
    // Test aarch32_UHSUB16_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=0
    let encoding: u32 = 0x06700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_uhsub16_a1_a_special_cond_0_condition_eq_112_06700070() {
    // Encoding: 0x06700070
    // Test aarch32_UHSUB16_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x06700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_uhsub16_a1_a_special_cond_1_condition_ne_112_16700070() {
    // Encoding: 0x16700070
    // Test aarch32_UHSUB16_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rd=0, cond=1, Rn=0, Rm=0
    let encoding: u32 = 0x16700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_uhsub16_a1_a_special_cond_2_condition_cs_hs_112_26700070() {
    // Encoding: 0x26700070
    // Test aarch32_UHSUB16_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=2
    let encoding: u32 = 0x26700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_uhsub16_a1_a_special_cond_3_condition_cc_lo_112_36700070() {
    // Encoding: 0x36700070
    // Test aarch32_UHSUB16_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x36700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_uhsub16_a1_a_special_cond_4_condition_mi_112_46700070() {
    // Encoding: 0x46700070
    // Test aarch32_UHSUB16_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rm=0, cond=4, Rn=0, Rd=0
    let encoding: u32 = 0x46700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_uhsub16_a1_a_special_cond_5_condition_pl_112_56700070() {
    // Encoding: 0x56700070
    // Test aarch32_UHSUB16_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x56700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_uhsub16_a1_a_special_cond_6_condition_vs_112_66700070() {
    // Encoding: 0x66700070
    // Test aarch32_UHSUB16_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rd=0, cond=6, Rn=0, Rm=0
    let encoding: u32 = 0x66700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_uhsub16_a1_a_special_cond_7_condition_vc_112_76700070() {
    // Encoding: 0x76700070
    // Test aarch32_UHSUB16_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=7, Rn=0
    let encoding: u32 = 0x76700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_uhsub16_a1_a_special_cond_8_condition_hi_112_86700070() {
    // Encoding: 0x86700070
    // Test aarch32_UHSUB16_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x86700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_uhsub16_a1_a_special_cond_9_condition_ls_112_96700070() {
    // Encoding: 0x96700070
    // Test aarch32_UHSUB16_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rd=0, cond=9, Rm=0, Rn=0
    let encoding: u32 = 0x96700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_uhsub16_a1_a_special_cond_10_condition_ge_112_a6700070() {
    // Encoding: 0xA6700070
    // Test aarch32_UHSUB16_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=10
    let encoding: u32 = 0xA6700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_uhsub16_a1_a_special_cond_11_condition_lt_112_b6700070() {
    // Encoding: 0xB6700070
    // Test aarch32_UHSUB16_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rn=0, cond=11, Rd=0, Rm=0
    let encoding: u32 = 0xB6700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_uhsub16_a1_a_special_cond_12_condition_gt_112_c6700070() {
    // Encoding: 0xC6700070
    // Test aarch32_UHSUB16_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=12, Rd=0
    let encoding: u32 = 0xC6700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_uhsub16_a1_a_special_cond_13_condition_le_112_d6700070() {
    // Encoding: 0xD6700070
    // Test aarch32_UHSUB16_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=13
    let encoding: u32 = 0xD6700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_uhsub16_a1_a_special_cond_14_condition_al_112_e6700070() {
    // Encoding: 0xE6700070
    // Test aarch32_UHSUB16_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rm=0, cond=14, Rn=0, Rd=0
    let encoding: u32 = 0xE6700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_uhsub16_a1_a_special_cond_15_condition_nv_112_f6700070() {
    // Encoding: 0xF6700070
    // Test aarch32_UHSUB16_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=15, Rn=0
    let encoding: u32 = 0xF6700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uhsub16_a1_a_invalid_0_70_06700070() {
    // Encoding: 0x06700070
    // Test aarch32_UHSUB16_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=0
    let encoding: u32 = 0x06700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UHSUB16_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uhsub16_a1_a_invalid_1_70_06700070() {
    // Encoding: 0x06700070
    // Test aarch32_UHSUB16_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x06700070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UHSUB16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhsub16_t1_a_field_rn_0_min_f060_fad0f060() {
    // Thumb encoding (32): 0xFAD0F060
    // Test aarch32_UHSUB16_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHSUB16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhsub16_t1_a_field_rn_1_poweroftwo_f060_fad1f060() {
    // Thumb encoding (32): 0xFAD1F060
    // Test aarch32_UHSUB16_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD1F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHSUB16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhsub16_t1_a_field_rd_0_min_f060_fad0f060() {
    // Thumb encoding (32): 0xFAD0F060
    // Test aarch32_UHSUB16_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHSUB16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhsub16_t1_a_field_rd_1_poweroftwo_f060_fad0f160() {
    // Thumb encoding (32): 0xFAD0F160
    // Test aarch32_UHSUB16_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F160;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHSUB16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhsub16_t1_a_field_rm_0_min_f060_fad0f060() {
    // Thumb encoding (32): 0xFAD0F060
    // Test aarch32_UHSUB16_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHSUB16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhsub16_t1_a_field_rm_1_poweroftwo_f060_fad0f061() {
    // Thumb encoding (32): 0xFAD0F061
    // Test aarch32_UHSUB16_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F061;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHSUB16_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_uhsub16_t1_a_combo_0_f060_fad0f060() {
    // Thumb encoding (32): 0xFAD0F060
    // Test aarch32_UHSUB16_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHSUB16_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uhsub16_t1_a_invalid_0_f060_fad0f060() {
    // Thumb encoding (32): 0xFAD0F060
    // Test aarch32_UHSUB16_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F060;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_UHSUB16_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uhsub16_t1_a_invalid_1_f060_fad0f060() {
    // Thumb encoding (32): 0xFAD0F060
    // Test aarch32_UHSUB16_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F060;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_SADD16_A Tests
// ============================================================================

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_sadd16_a1_a_field_cond_0_min_10_06100010() {
    // Encoding: 0x06100010
    // Test aarch32_SADD16_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=0
    let encoding: u32 = 0x06100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_sadd16_a1_a_field_cond_1_poweroftwo_10_16100010() {
    // Encoding: 0x16100010
    // Test aarch32_SADD16_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=1
    let encoding: u32 = 0x16100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_sadd16_a1_a_field_cond_2_poweroftwo_10_26100010() {
    // Encoding: 0x26100010
    // Test aarch32_SADD16_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=2, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x26100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_sadd16_a1_a_field_cond_3_poweroftwo_10_36100010() {
    // Encoding: 0x36100010
    // Test aarch32_SADD16_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=3, Rn=0
    let encoding: u32 = 0x36100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_sadd16_a1_a_field_cond_4_poweroftwo_10_46100010() {
    // Encoding: 0x46100010
    // Test aarch32_SADD16_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=4, Rd=0, Rm=0
    let encoding: u32 = 0x46100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_sadd16_a1_a_field_cond_5_poweroftwo_10_56100010() {
    // Encoding: 0x56100010
    // Test aarch32_SADD16_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=5, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x56100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_sadd16_a1_a_field_cond_6_poweroftwo_10_66100010() {
    // Encoding: 0x66100010
    // Test aarch32_SADD16_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=6
    let encoding: u32 = 0x66100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_sadd16_a1_a_field_cond_7_poweroftwo_10_76100010() {
    // Encoding: 0x76100010
    // Test aarch32_SADD16_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=7, Rm=0, Rn=0
    let encoding: u32 = 0x76100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_sadd16_a1_a_field_cond_8_poweroftwo_10_86100010() {
    // Encoding: 0x86100010
    // Test aarch32_SADD16_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=8, Rd=0
    let encoding: u32 = 0x86100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_sadd16_a1_a_field_cond_9_poweroftwo_10_96100010() {
    // Encoding: 0x96100010
    // Test aarch32_SADD16_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=9, Rm=0
    let encoding: u32 = 0x96100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_sadd16_a1_a_field_cond_10_poweroftwo_10_a6100010() {
    // Encoding: 0xA6100010
    // Test aarch32_SADD16_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=10
    let encoding: u32 = 0xA6100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_sadd16_a1_a_field_cond_11_poweroftwo_10_b6100010() {
    // Encoding: 0xB6100010
    // Test aarch32_SADD16_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=11, Rm=0
    let encoding: u32 = 0xB6100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_sadd16_a1_a_field_cond_12_poweroftwo_10_c6100010() {
    // Encoding: 0xC6100010
    // Test aarch32_SADD16_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0xC6100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_sadd16_a1_a_field_cond_13_poweroftwo_10_d6100010() {
    // Encoding: 0xD6100010
    // Test aarch32_SADD16_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=13, Rd=0, Rm=0
    let encoding: u32 = 0xD6100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_sadd16_a1_a_field_cond_14_poweroftwo_10_e6100010() {
    // Encoding: 0xE6100010
    // Test aarch32_SADD16_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0xE6100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_sadd16_a1_a_field_cond_15_max_10_f6100010() {
    // Encoding: 0xF6100010
    // Test aarch32_SADD16_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rd=0, cond=15, Rn=0, Rm=0
    let encoding: u32 = 0xF6100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sadd16_a1_a_field_rn_0_min_10_06100010() {
    // Encoding: 0x06100010
    // Test aarch32_SADD16_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=0
    let encoding: u32 = 0x06100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sadd16_a1_a_field_rn_1_poweroftwo_10_06110010() {
    // Encoding: 0x06110010
    // Test aarch32_SADD16_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=1, Rm=0
    let encoding: u32 = 0x06110010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sadd16_a1_a_field_rd_0_min_10_06100010() {
    // Encoding: 0x06100010
    // Test aarch32_SADD16_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=0, Rd=0
    let encoding: u32 = 0x06100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sadd16_a1_a_field_rd_1_poweroftwo_10_06101010() {
    // Encoding: 0x06101010
    // Test aarch32_SADD16_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=1, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x06101010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sadd16_a1_a_field_rm_0_min_10_06100010() {
    // Encoding: 0x06100010
    // Test aarch32_SADD16_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x06100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sadd16_a1_a_field_rm_1_poweroftwo_10_06100011() {
    // Encoding: 0x06100011
    // Test aarch32_SADD16_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=1, Rd=0, cond=0
    let encoding: u32 = 0x06100011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_sadd16_a1_a_combo_0_10_06100010() {
    // Encoding: 0x06100010
    // Test aarch32_SADD16_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=0, Rd=0
    let encoding: u32 = 0x06100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_sadd16_a1_a_special_cond_0_condition_eq_16_06100010() {
    // Encoding: 0x06100010
    // Test aarch32_SADD16_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=0
    let encoding: u32 = 0x06100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_sadd16_a1_a_special_cond_1_condition_ne_16_16100010() {
    // Encoding: 0x16100010
    // Test aarch32_SADD16_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=1
    let encoding: u32 = 0x16100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_sadd16_a1_a_special_cond_2_condition_cs_hs_16_26100010() {
    // Encoding: 0x26100010
    // Test aarch32_SADD16_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=2, Rn=0
    let encoding: u32 = 0x26100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_sadd16_a1_a_special_cond_3_condition_cc_lo_16_36100010() {
    // Encoding: 0x36100010
    // Test aarch32_SADD16_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rd=0, cond=3, Rm=0, Rn=0
    let encoding: u32 = 0x36100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_sadd16_a1_a_special_cond_4_condition_mi_16_46100010() {
    // Encoding: 0x46100010
    // Test aarch32_SADD16_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=4, Rm=0
    let encoding: u32 = 0x46100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_sadd16_a1_a_special_cond_5_condition_pl_16_56100010() {
    // Encoding: 0x56100010
    // Test aarch32_SADD16_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x56100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_sadd16_a1_a_special_cond_6_condition_vs_16_66100010() {
    // Encoding: 0x66100010
    // Test aarch32_SADD16_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x66100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_sadd16_a1_a_special_cond_7_condition_vc_16_76100010() {
    // Encoding: 0x76100010
    // Test aarch32_SADD16_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=7, Rd=0
    let encoding: u32 = 0x76100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_sadd16_a1_a_special_cond_8_condition_hi_16_86100010() {
    // Encoding: 0x86100010
    // Test aarch32_SADD16_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x86100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_sadd16_a1_a_special_cond_9_condition_ls_16_96100010() {
    // Encoding: 0x96100010
    // Test aarch32_SADD16_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rm=0, cond=9, Rn=0, Rd=0
    let encoding: u32 = 0x96100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_sadd16_a1_a_special_cond_10_condition_ge_16_a6100010() {
    // Encoding: 0xA6100010
    // Test aarch32_SADD16_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=10
    let encoding: u32 = 0xA6100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_sadd16_a1_a_special_cond_11_condition_lt_16_b6100010() {
    // Encoding: 0xB6100010
    // Test aarch32_SADD16_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=11
    let encoding: u32 = 0xB6100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_sadd16_a1_a_special_cond_12_condition_gt_16_c6100010() {
    // Encoding: 0xC6100010
    // Test aarch32_SADD16_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=12, Rd=0
    let encoding: u32 = 0xC6100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_sadd16_a1_a_special_cond_13_condition_le_16_d6100010() {
    // Encoding: 0xD6100010
    // Test aarch32_SADD16_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: cond=13, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0xD6100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_sadd16_a1_a_special_cond_14_condition_al_16_e6100010() {
    // Encoding: 0xE6100010
    // Test aarch32_SADD16_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rd=0, cond=14, Rn=0, Rm=0
    let encoding: u32 = 0xE6100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_sadd16_a1_a_special_cond_15_condition_nv_16_f6100010() {
    // Encoding: 0xF6100010
    // Test aarch32_SADD16_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rd=0, cond=15, Rm=0, Rn=0
    let encoding: u32 = 0xF6100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sadd16_a1_a_invalid_0_10_06100010() {
    // Encoding: 0x06100010
    // Test aarch32_SADD16_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rm=0, cond=0, Rd=0, Rn=0
    let encoding: u32 = 0x06100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SADD16_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sadd16_a1_a_invalid_1_10_06100010() {
    // Encoding: 0x06100010
    // Test aarch32_SADD16_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=0, Rd=0
    let encoding: u32 = 0x06100010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SADD16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sadd16_t1_a_field_rn_0_min_f000_fa90f000() {
    // Thumb encoding (32): 0xFA90F000
    // Test aarch32_SADD16_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SADD16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sadd16_t1_a_field_rn_1_poweroftwo_f000_fa91f000() {
    // Thumb encoding (32): 0xFA91F000
    // Test aarch32_SADD16_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA91F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SADD16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sadd16_t1_a_field_rd_0_min_f000_fa90f000() {
    // Thumb encoding (32): 0xFA90F000
    // Test aarch32_SADD16_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SADD16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sadd16_t1_a_field_rd_1_poweroftwo_f000_fa90f100() {
    // Thumb encoding (32): 0xFA90F100
    // Test aarch32_SADD16_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F100;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SADD16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_sadd16_t1_a_field_rm_0_min_f000_fa90f000() {
    // Thumb encoding (32): 0xFA90F000
    // Test aarch32_SADD16_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SADD16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_sadd16_t1_a_field_rm_1_poweroftwo_f000_fa90f001() {
    // Thumb encoding (32): 0xFA90F001
    // Test aarch32_SADD16_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F001;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SADD16_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_sadd16_t1_a_combo_0_f000_fa90f000() {
    // Thumb encoding (32): 0xFA90F000
    // Test aarch32_SADD16_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SADD16_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sadd16_t1_a_invalid_0_f000_fa90f000() {
    // Thumb encoding (32): 0xFA90F000
    // Test aarch32_SADD16_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_SADD16_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_sadd16_t1_a_invalid_1_f000_fa90f000() {
    // Thumb encoding (32): 0xFA90F000
    // Test aarch32_SADD16_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_SHSUB8_A Tests
// ============================================================================

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_shsub8_a1_a_field_cond_0_min_f0_063000f0() {
    // Encoding: 0x063000F0
    // Test aarch32_SHSUB8_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x063000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_shsub8_a1_a_field_cond_1_poweroftwo_f0_163000f0() {
    // Encoding: 0x163000F0
    // Test aarch32_SHSUB8_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x163000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_shsub8_a1_a_field_cond_2_poweroftwo_f0_263000f0() {
    // Encoding: 0x263000F0
    // Test aarch32_SHSUB8_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=2, Rd=0, Rm=0
    let encoding: u32 = 0x263000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_shsub8_a1_a_field_cond_3_poweroftwo_f0_363000f0() {
    // Encoding: 0x363000F0
    // Test aarch32_SHSUB8_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x363000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_shsub8_a1_a_field_cond_4_poweroftwo_f0_463000f0() {
    // Encoding: 0x463000F0
    // Test aarch32_SHSUB8_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=4, Rd=0, Rm=0
    let encoding: u32 = 0x463000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_shsub8_a1_a_field_cond_5_poweroftwo_f0_563000f0() {
    // Encoding: 0x563000F0
    // Test aarch32_SHSUB8_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=5, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x563000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_shsub8_a1_a_field_cond_6_poweroftwo_f0_663000f0() {
    // Encoding: 0x663000F0
    // Test aarch32_SHSUB8_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=6, Rm=0, Rd=0
    let encoding: u32 = 0x663000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_shsub8_a1_a_field_cond_7_poweroftwo_f0_763000f0() {
    // Encoding: 0x763000F0
    // Test aarch32_SHSUB8_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=7
    let encoding: u32 = 0x763000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_shsub8_a1_a_field_cond_8_poweroftwo_f0_863000f0() {
    // Encoding: 0x863000F0
    // Test aarch32_SHSUB8_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=8, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x863000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_shsub8_a1_a_field_cond_9_poweroftwo_f0_963000f0() {
    // Encoding: 0x963000F0
    // Test aarch32_SHSUB8_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x963000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_shsub8_a1_a_field_cond_10_poweroftwo_f0_a63000f0() {
    // Encoding: 0xA63000F0
    // Test aarch32_SHSUB8_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=10
    let encoding: u32 = 0xA63000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_shsub8_a1_a_field_cond_11_poweroftwo_f0_b63000f0() {
    // Encoding: 0xB63000F0
    // Test aarch32_SHSUB8_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=11, Rd=0, Rn=0
    let encoding: u32 = 0xB63000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_shsub8_a1_a_field_cond_12_poweroftwo_f0_c63000f0() {
    // Encoding: 0xC63000F0
    // Test aarch32_SHSUB8_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=12, Rm=0, Rd=0
    let encoding: u32 = 0xC63000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_shsub8_a1_a_field_cond_13_poweroftwo_f0_d63000f0() {
    // Encoding: 0xD63000F0
    // Test aarch32_SHSUB8_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=13, Rd=0
    let encoding: u32 = 0xD63000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_shsub8_a1_a_field_cond_14_poweroftwo_f0_e63000f0() {
    // Encoding: 0xE63000F0
    // Test aarch32_SHSUB8_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=14, Rn=0
    let encoding: u32 = 0xE63000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_shsub8_a1_a_field_cond_15_max_f0_f63000f0() {
    // Encoding: 0xF63000F0
    // Test aarch32_SHSUB8_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=15, Rm=0
    let encoding: u32 = 0xF63000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shsub8_a1_a_field_rn_0_min_f0_063000f0() {
    // Encoding: 0x063000F0
    // Test aarch32_SHSUB8_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=0
    let encoding: u32 = 0x063000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shsub8_a1_a_field_rn_1_poweroftwo_f0_063100f0() {
    // Encoding: 0x063100F0
    // Test aarch32_SHSUB8_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=1, cond=0, Rd=0, Rm=0
    let encoding: u32 = 0x063100F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shsub8_a1_a_field_rd_0_min_f0_063000f0() {
    // Encoding: 0x063000F0
    // Test aarch32_SHSUB8_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x063000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shsub8_a1_a_field_rd_1_poweroftwo_f0_063010f0() {
    // Encoding: 0x063010F0
    // Test aarch32_SHSUB8_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=1, Rm=0, Rn=0, cond=0
    let encoding: u32 = 0x063010F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shsub8_a1_a_field_rm_0_min_f0_063000f0() {
    // Encoding: 0x063000F0
    // Test aarch32_SHSUB8_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=0
    let encoding: u32 = 0x063000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shsub8_a1_a_field_rm_1_poweroftwo_f0_063000f1() {
    // Encoding: 0x063000F1
    // Test aarch32_SHSUB8_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=0, Rm=1
    let encoding: u32 = 0x063000F1;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_shsub8_a1_a_combo_0_f0_063000f0() {
    // Encoding: 0x063000F0
    // Test aarch32_SHSUB8_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rd=0, cond=0, Rn=0, Rm=0
    let encoding: u32 = 0x063000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_shsub8_a1_a_special_cond_0_condition_eq_240_063000f0() {
    // Encoding: 0x063000F0
    // Test aarch32_SHSUB8_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rd=0, cond=0, Rm=0, Rn=0
    let encoding: u32 = 0x063000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_shsub8_a1_a_special_cond_1_condition_ne_240_163000f0() {
    // Encoding: 0x163000F0
    // Test aarch32_SHSUB8_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=1, Rd=0
    let encoding: u32 = 0x163000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_shsub8_a1_a_special_cond_2_condition_cs_hs_240_263000f0() {
    // Encoding: 0x263000F0
    // Test aarch32_SHSUB8_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=2, Rn=0
    let encoding: u32 = 0x263000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_shsub8_a1_a_special_cond_3_condition_cc_lo_240_363000f0() {
    // Encoding: 0x363000F0
    // Test aarch32_SHSUB8_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=3
    let encoding: u32 = 0x363000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_shsub8_a1_a_special_cond_4_condition_mi_240_463000f0() {
    // Encoding: 0x463000F0
    // Test aarch32_SHSUB8_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=4, Rd=0
    let encoding: u32 = 0x463000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_shsub8_a1_a_special_cond_5_condition_pl_240_563000f0() {
    // Encoding: 0x563000F0
    // Test aarch32_SHSUB8_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=5, Rm=0
    let encoding: u32 = 0x563000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_shsub8_a1_a_special_cond_6_condition_vs_240_663000f0() {
    // Encoding: 0x663000F0
    // Test aarch32_SHSUB8_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=6, Rd=0
    let encoding: u32 = 0x663000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_shsub8_a1_a_special_cond_7_condition_vc_240_763000f0() {
    // Encoding: 0x763000F0
    // Test aarch32_SHSUB8_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=7
    let encoding: u32 = 0x763000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_shsub8_a1_a_special_cond_8_condition_hi_240_863000f0() {
    // Encoding: 0x863000F0
    // Test aarch32_SHSUB8_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=8, Rn=0
    let encoding: u32 = 0x863000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_shsub8_a1_a_special_cond_9_condition_ls_240_963000f0() {
    // Encoding: 0x963000F0
    // Test aarch32_SHSUB8_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=9
    let encoding: u32 = 0x963000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_shsub8_a1_a_special_cond_10_condition_ge_240_a63000f0() {
    // Encoding: 0xA63000F0
    // Test aarch32_SHSUB8_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0xA63000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_shsub8_a1_a_special_cond_11_condition_lt_240_b63000f0() {
    // Encoding: 0xB63000F0
    // Test aarch32_SHSUB8_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=11
    let encoding: u32 = 0xB63000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_shsub8_a1_a_special_cond_12_condition_gt_240_c63000f0() {
    // Encoding: 0xC63000F0
    // Test aarch32_SHSUB8_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=12, Rd=0
    let encoding: u32 = 0xC63000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_shsub8_a1_a_special_cond_13_condition_le_240_d63000f0() {
    // Encoding: 0xD63000F0
    // Test aarch32_SHSUB8_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rd=0, cond=13, Rm=0, Rn=0
    let encoding: u32 = 0xD63000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_shsub8_a1_a_special_cond_14_condition_al_240_e63000f0() {
    // Encoding: 0xE63000F0
    // Test aarch32_SHSUB8_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=14
    let encoding: u32 = 0xE63000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_shsub8_a1_a_special_cond_15_condition_nv_240_f63000f0() {
    // Encoding: 0xF63000F0
    // Test aarch32_SHSUB8_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xF63000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_shsub8_a1_a_invalid_0_f0_063000f0() {
    // Encoding: 0x063000F0
    // Test aarch32_SHSUB8_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: cond=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x063000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SHSUB8_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_shsub8_a1_a_invalid_1_f0_063000f0() {
    // Encoding: 0x063000F0
    // Test aarch32_SHSUB8_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=0
    let encoding: u32 = 0x063000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SHSUB8_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shsub8_t1_a_field_rn_0_min_f020_fac0f020() {
    // Thumb encoding (32): 0xFAC0F020
    // Test aarch32_SHSUB8_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHSUB8_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shsub8_t1_a_field_rn_1_poweroftwo_f020_fac1f020() {
    // Thumb encoding (32): 0xFAC1F020
    // Test aarch32_SHSUB8_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC1F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHSUB8_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shsub8_t1_a_field_rd_0_min_f020_fac0f020() {
    // Thumb encoding (32): 0xFAC0F020
    // Test aarch32_SHSUB8_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHSUB8_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shsub8_t1_a_field_rd_1_poweroftwo_f020_fac0f120() {
    // Thumb encoding (32): 0xFAC0F120
    // Test aarch32_SHSUB8_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=1, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F120;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHSUB8_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shsub8_t1_a_field_rm_0_min_f020_fac0f020() {
    // Thumb encoding (32): 0xFAC0F020
    // Test aarch32_SHSUB8_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHSUB8_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shsub8_t1_a_field_rm_1_poweroftwo_f020_fac0f021() {
    // Thumb encoding (32): 0xFAC0F021
    // Test aarch32_SHSUB8_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rm=1, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F021;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHSUB8_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_shsub8_t1_a_combo_0_f020_fac0f020() {
    // Thumb encoding (32): 0xFAC0F020
    // Test aarch32_SHSUB8_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHSUB8_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_shsub8_t1_a_invalid_0_f020_fac0f020() {
    // Thumb encoding (32): 0xFAC0F020
    // Test aarch32_SHSUB8_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F020;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_SHSUB8_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_shsub8_t1_a_invalid_1_f020_fac0f020() {
    // Thumb encoding (32): 0xFAC0F020
    // Test aarch32_SHSUB8_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F020;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_SHSUB16_A Tests
// ============================================================================

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_shsub16_a1_a_field_cond_0_min_70_06300070() {
    // Encoding: 0x06300070
    // Test aarch32_SHSUB16_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=0, Rm=0
    let encoding: u32 = 0x06300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_shsub16_a1_a_field_cond_1_poweroftwo_70_16300070() {
    // Encoding: 0x16300070
    // Test aarch32_SHSUB16_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=1, Rn=0, Rd=0
    let encoding: u32 = 0x16300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_shsub16_a1_a_field_cond_2_poweroftwo_70_26300070() {
    // Encoding: 0x26300070
    // Test aarch32_SHSUB16_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=2
    let encoding: u32 = 0x26300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_shsub16_a1_a_field_cond_3_poweroftwo_70_36300070() {
    // Encoding: 0x36300070
    // Test aarch32_SHSUB16_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=3
    let encoding: u32 = 0x36300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_shsub16_a1_a_field_cond_4_poweroftwo_70_46300070() {
    // Encoding: 0x46300070
    // Test aarch32_SHSUB16_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=4, Rm=0, Rn=0
    let encoding: u32 = 0x46300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_shsub16_a1_a_field_cond_5_poweroftwo_70_56300070() {
    // Encoding: 0x56300070
    // Test aarch32_SHSUB16_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=5, Rd=0, Rm=0
    let encoding: u32 = 0x56300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_shsub16_a1_a_field_cond_6_poweroftwo_70_66300070() {
    // Encoding: 0x66300070
    // Test aarch32_SHSUB16_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=6, Rn=0
    let encoding: u32 = 0x66300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_shsub16_a1_a_field_cond_7_poweroftwo_70_76300070() {
    // Encoding: 0x76300070
    // Test aarch32_SHSUB16_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=7, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x76300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_shsub16_a1_a_field_cond_8_poweroftwo_70_86300070() {
    // Encoding: 0x86300070
    // Test aarch32_SHSUB16_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=8, Rn=0
    let encoding: u32 = 0x86300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_shsub16_a1_a_field_cond_9_poweroftwo_70_96300070() {
    // Encoding: 0x96300070
    // Test aarch32_SHSUB16_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x96300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_shsub16_a1_a_field_cond_10_poweroftwo_70_a6300070() {
    // Encoding: 0xA6300070
    // Test aarch32_SHSUB16_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xA6300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_shsub16_a1_a_field_cond_11_poweroftwo_70_b6300070() {
    // Encoding: 0xB6300070
    // Test aarch32_SHSUB16_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=11
    let encoding: u32 = 0xB6300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_shsub16_a1_a_field_cond_12_poweroftwo_70_c6300070() {
    // Encoding: 0xC6300070
    // Test aarch32_SHSUB16_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=12, Rd=0
    let encoding: u32 = 0xC6300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_shsub16_a1_a_field_cond_13_poweroftwo_70_d6300070() {
    // Encoding: 0xD6300070
    // Test aarch32_SHSUB16_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=13, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0xD6300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_shsub16_a1_a_field_cond_14_poweroftwo_70_e6300070() {
    // Encoding: 0xE6300070
    // Test aarch32_SHSUB16_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=14, Rn=0
    let encoding: u32 = 0xE6300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_shsub16_a1_a_field_cond_15_max_70_f6300070() {
    // Encoding: 0xF6300070
    // Test aarch32_SHSUB16_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=15
    let encoding: u32 = 0xF6300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shsub16_a1_a_field_rn_0_min_70_06300070() {
    // Encoding: 0x06300070
    // Test aarch32_SHSUB16_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x06300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shsub16_a1_a_field_rn_1_poweroftwo_70_06310070() {
    // Encoding: 0x06310070
    // Test aarch32_SHSUB16_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=1, Rd=0
    let encoding: u32 = 0x06310070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shsub16_a1_a_field_rd_0_min_70_06300070() {
    // Encoding: 0x06300070
    // Test aarch32_SHSUB16_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=0, Rd=0
    let encoding: u32 = 0x06300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shsub16_a1_a_field_rd_1_poweroftwo_70_06301070() {
    // Encoding: 0x06301070
    // Test aarch32_SHSUB16_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=1, Rm=0
    let encoding: u32 = 0x06301070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shsub16_a1_a_field_rm_0_min_70_06300070() {
    // Encoding: 0x06300070
    // Test aarch32_SHSUB16_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x06300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shsub16_a1_a_field_rm_1_poweroftwo_70_06300071() {
    // Encoding: 0x06300071
    // Test aarch32_SHSUB16_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=1, cond=0, Rd=0, Rn=0
    let encoding: u32 = 0x06300071;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_shsub16_a1_a_combo_0_70_06300070() {
    // Encoding: 0x06300070
    // Test aarch32_SHSUB16_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x06300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_shsub16_a1_a_special_cond_0_condition_eq_112_06300070() {
    // Encoding: 0x06300070
    // Test aarch32_SHSUB16_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=0, Rd=0
    let encoding: u32 = 0x06300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_shsub16_a1_a_special_cond_1_condition_ne_112_16300070() {
    // Encoding: 0x16300070
    // Test aarch32_SHSUB16_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rn=0, cond=1, Rd=0, Rm=0
    let encoding: u32 = 0x16300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_shsub16_a1_a_special_cond_2_condition_cs_hs_112_26300070() {
    // Encoding: 0x26300070
    // Test aarch32_SHSUB16_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rn=0, cond=2, Rd=0, Rm=0
    let encoding: u32 = 0x26300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_shsub16_a1_a_special_cond_3_condition_cc_lo_112_36300070() {
    // Encoding: 0x36300070
    // Test aarch32_SHSUB16_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=3, Rd=0
    let encoding: u32 = 0x36300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_shsub16_a1_a_special_cond_4_condition_mi_112_46300070() {
    // Encoding: 0x46300070
    // Test aarch32_SHSUB16_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x46300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_shsub16_a1_a_special_cond_5_condition_pl_112_56300070() {
    // Encoding: 0x56300070
    // Test aarch32_SHSUB16_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=5
    let encoding: u32 = 0x56300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_shsub16_a1_a_special_cond_6_condition_vs_112_66300070() {
    // Encoding: 0x66300070
    // Test aarch32_SHSUB16_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rd=0, cond=6, Rn=0, Rm=0
    let encoding: u32 = 0x66300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_shsub16_a1_a_special_cond_7_condition_vc_112_76300070() {
    // Encoding: 0x76300070
    // Test aarch32_SHSUB16_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=7, Rd=0
    let encoding: u32 = 0x76300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_shsub16_a1_a_special_cond_8_condition_hi_112_86300070() {
    // Encoding: 0x86300070
    // Test aarch32_SHSUB16_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=8
    let encoding: u32 = 0x86300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_shsub16_a1_a_special_cond_9_condition_ls_112_96300070() {
    // Encoding: 0x96300070
    // Test aarch32_SHSUB16_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rd=0, cond=9, Rn=0, Rm=0
    let encoding: u32 = 0x96300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_shsub16_a1_a_special_cond_10_condition_ge_112_a6300070() {
    // Encoding: 0xA6300070
    // Test aarch32_SHSUB16_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0xA6300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_shsub16_a1_a_special_cond_11_condition_lt_112_b6300070() {
    // Encoding: 0xB6300070
    // Test aarch32_SHSUB16_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0xB6300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_shsub16_a1_a_special_cond_12_condition_gt_112_c6300070() {
    // Encoding: 0xC6300070
    // Test aarch32_SHSUB16_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: cond=12, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xC6300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_shsub16_a1_a_special_cond_13_condition_le_112_d6300070() {
    // Encoding: 0xD6300070
    // Test aarch32_SHSUB16_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rm=0, cond=13, Rn=0, Rd=0
    let encoding: u32 = 0xD6300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_shsub16_a1_a_special_cond_14_condition_al_112_e6300070() {
    // Encoding: 0xE6300070
    // Test aarch32_SHSUB16_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rd=0, cond=14, Rm=0, Rn=0
    let encoding: u32 = 0xE6300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_shsub16_a1_a_special_cond_15_condition_nv_112_f6300070() {
    // Encoding: 0xF6300070
    // Test aarch32_SHSUB16_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=15, Rd=0
    let encoding: u32 = 0xF6300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_shsub16_a1_a_invalid_0_70_06300070() {
    // Encoding: 0x06300070
    // Test aarch32_SHSUB16_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=0
    let encoding: u32 = 0x06300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SHSUB16_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_shsub16_a1_a_invalid_1_70_06300070() {
    // Encoding: 0x06300070
    // Test aarch32_SHSUB16_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x06300070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SHSUB16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shsub16_t1_a_field_rn_0_min_f020_fad0f020() {
    // Thumb encoding (32): 0xFAD0F020
    // Test aarch32_SHSUB16_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHSUB16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shsub16_t1_a_field_rn_1_poweroftwo_f020_fad1f020() {
    // Thumb encoding (32): 0xFAD1F020
    // Test aarch32_SHSUB16_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD1F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHSUB16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shsub16_t1_a_field_rd_0_min_f020_fad0f020() {
    // Thumb encoding (32): 0xFAD0F020
    // Test aarch32_SHSUB16_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHSUB16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shsub16_t1_a_field_rd_1_poweroftwo_f020_fad0f120() {
    // Thumb encoding (32): 0xFAD0F120
    // Test aarch32_SHSUB16_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=1, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F120;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHSUB16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shsub16_t1_a_field_rm_0_min_f020_fad0f020() {
    // Thumb encoding (32): 0xFAD0F020
    // Test aarch32_SHSUB16_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHSUB16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shsub16_t1_a_field_rm_1_poweroftwo_f020_fad0f021() {
    // Thumb encoding (32): 0xFAD0F021
    // Test aarch32_SHSUB16_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F021;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHSUB16_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_shsub16_t1_a_combo_0_f020_fad0f020() {
    // Thumb encoding (32): 0xFAD0F020
    // Test aarch32_SHSUB16_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHSUB16_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_shsub16_t1_a_invalid_0_f020_fad0f020() {
    // Thumb encoding (32): 0xFAD0F020
    // Test aarch32_SHSUB16_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F020;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_SHSUB16_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_shsub16_t1_a_invalid_1_f020_fad0f020() {
    // Thumb encoding (32): 0xFAD0F020
    // Test aarch32_SHSUB16_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F020;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_UADD8_A Tests
// ============================================================================

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_uadd8_a1_a_field_cond_0_min_90_06500090() {
    // Encoding: 0x06500090
    // Test aarch32_UADD8_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=0
    let encoding: u32 = 0x06500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_uadd8_a1_a_field_cond_1_poweroftwo_90_16500090() {
    // Encoding: 0x16500090
    // Test aarch32_UADD8_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x16500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_uadd8_a1_a_field_cond_2_poweroftwo_90_26500090() {
    // Encoding: 0x26500090
    // Test aarch32_UADD8_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=2, Rn=0
    let encoding: u32 = 0x26500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_uadd8_a1_a_field_cond_3_poweroftwo_90_36500090() {
    // Encoding: 0x36500090
    // Test aarch32_UADD8_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=3, Rn=0, Rd=0
    let encoding: u32 = 0x36500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_uadd8_a1_a_field_cond_4_poweroftwo_90_46500090() {
    // Encoding: 0x46500090
    // Test aarch32_UADD8_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=4, Rd=0
    let encoding: u32 = 0x46500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_uadd8_a1_a_field_cond_5_poweroftwo_90_56500090() {
    // Encoding: 0x56500090
    // Test aarch32_UADD8_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=5
    let encoding: u32 = 0x56500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_uadd8_a1_a_field_cond_6_poweroftwo_90_66500090() {
    // Encoding: 0x66500090
    // Test aarch32_UADD8_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x66500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_uadd8_a1_a_field_cond_7_poweroftwo_90_76500090() {
    // Encoding: 0x76500090
    // Test aarch32_UADD8_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=7
    let encoding: u32 = 0x76500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_uadd8_a1_a_field_cond_8_poweroftwo_90_86500090() {
    // Encoding: 0x86500090
    // Test aarch32_UADD8_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=8
    let encoding: u32 = 0x86500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_uadd8_a1_a_field_cond_9_poweroftwo_90_96500090() {
    // Encoding: 0x96500090
    // Test aarch32_UADD8_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=9, Rn=0, Rm=0
    let encoding: u32 = 0x96500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_uadd8_a1_a_field_cond_10_poweroftwo_90_a6500090() {
    // Encoding: 0xA6500090
    // Test aarch32_UADD8_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0xA6500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_uadd8_a1_a_field_cond_11_poweroftwo_90_b6500090() {
    // Encoding: 0xB6500090
    // Test aarch32_UADD8_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=11, Rd=0
    let encoding: u32 = 0xB6500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_uadd8_a1_a_field_cond_12_poweroftwo_90_c6500090() {
    // Encoding: 0xC6500090
    // Test aarch32_UADD8_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=12, Rn=0, Rm=0
    let encoding: u32 = 0xC6500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_uadd8_a1_a_field_cond_13_poweroftwo_90_d6500090() {
    // Encoding: 0xD6500090
    // Test aarch32_UADD8_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=13
    let encoding: u32 = 0xD6500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_uadd8_a1_a_field_cond_14_poweroftwo_90_e6500090() {
    // Encoding: 0xE6500090
    // Test aarch32_UADD8_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xE6500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_uadd8_a1_a_field_cond_15_max_90_f6500090() {
    // Encoding: 0xF6500090
    // Test aarch32_UADD8_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=15
    let encoding: u32 = 0xF6500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uadd8_a1_a_field_rn_0_min_90_06500090() {
    // Encoding: 0x06500090
    // Test aarch32_UADD8_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x06500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uadd8_a1_a_field_rn_1_poweroftwo_90_06510090() {
    // Encoding: 0x06510090
    // Test aarch32_UADD8_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=1, cond=0
    let encoding: u32 = 0x06510090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uadd8_a1_a_field_rd_0_min_90_06500090() {
    // Encoding: 0x06500090
    // Test aarch32_UADD8_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x06500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uadd8_a1_a_field_rd_1_poweroftwo_90_06501090() {
    // Encoding: 0x06501090
    // Test aarch32_UADD8_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=0, Rd=1, Rm=0
    let encoding: u32 = 0x06501090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uadd8_a1_a_field_rm_0_min_90_06500090() {
    // Encoding: 0x06500090
    // Test aarch32_UADD8_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=0, Rm=0
    let encoding: u32 = 0x06500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uadd8_a1_a_field_rm_1_poweroftwo_90_06500091() {
    // Encoding: 0x06500091
    // Test aarch32_UADD8_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rm=1, Rd=0, Rn=0
    let encoding: u32 = 0x06500091;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_uadd8_a1_a_combo_0_90_06500090() {
    // Encoding: 0x06500090
    // Test aarch32_UADD8_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=0
    let encoding: u32 = 0x06500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_uadd8_a1_a_special_cond_0_condition_eq_144_06500090() {
    // Encoding: 0x06500090
    // Test aarch32_UADD8_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x06500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_uadd8_a1_a_special_cond_1_condition_ne_144_16500090() {
    // Encoding: 0x16500090
    // Test aarch32_UADD8_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=1, Rd=0
    let encoding: u32 = 0x16500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_uadd8_a1_a_special_cond_2_condition_cs_hs_144_26500090() {
    // Encoding: 0x26500090
    // Test aarch32_UADD8_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x26500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_uadd8_a1_a_special_cond_3_condition_cc_lo_144_36500090() {
    // Encoding: 0x36500090
    // Test aarch32_UADD8_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rm=0, cond=3, Rn=0, Rd=0
    let encoding: u32 = 0x36500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_uadd8_a1_a_special_cond_4_condition_mi_144_46500090() {
    // Encoding: 0x46500090
    // Test aarch32_UADD8_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=4, Rd=0
    let encoding: u32 = 0x46500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_uadd8_a1_a_special_cond_5_condition_pl_144_56500090() {
    // Encoding: 0x56500090
    // Test aarch32_UADD8_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rd=0, cond=5, Rn=0, Rm=0
    let encoding: u32 = 0x56500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_uadd8_a1_a_special_cond_6_condition_vs_144_66500090() {
    // Encoding: 0x66500090
    // Test aarch32_UADD8_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=6
    let encoding: u32 = 0x66500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_uadd8_a1_a_special_cond_7_condition_vc_144_76500090() {
    // Encoding: 0x76500090
    // Test aarch32_UADD8_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x76500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_uadd8_a1_a_special_cond_8_condition_hi_144_86500090() {
    // Encoding: 0x86500090
    // Test aarch32_UADD8_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=8, Rm=0
    let encoding: u32 = 0x86500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_uadd8_a1_a_special_cond_9_condition_ls_144_96500090() {
    // Encoding: 0x96500090
    // Test aarch32_UADD8_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x96500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_uadd8_a1_a_special_cond_10_condition_ge_144_a6500090() {
    // Encoding: 0xA6500090
    // Test aarch32_UADD8_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=10
    let encoding: u32 = 0xA6500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_uadd8_a1_a_special_cond_11_condition_lt_144_b6500090() {
    // Encoding: 0xB6500090
    // Test aarch32_UADD8_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xB6500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_uadd8_a1_a_special_cond_12_condition_gt_144_c6500090() {
    // Encoding: 0xC6500090
    // Test aarch32_UADD8_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=12, Rm=0
    let encoding: u32 = 0xC6500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_uadd8_a1_a_special_cond_13_condition_le_144_d6500090() {
    // Encoding: 0xD6500090
    // Test aarch32_UADD8_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rn=0, cond=13, Rd=0, Rm=0
    let encoding: u32 = 0xD6500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_uadd8_a1_a_special_cond_14_condition_al_144_e6500090() {
    // Encoding: 0xE6500090
    // Test aarch32_UADD8_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rm=0, cond=14, Rd=0, Rn=0
    let encoding: u32 = 0xE6500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_uadd8_a1_a_special_cond_15_condition_nv_144_f6500090() {
    // Encoding: 0xF6500090
    // Test aarch32_UADD8_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=15, Rn=0
    let encoding: u32 = 0xF6500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uadd8_a1_a_invalid_0_90_06500090() {
    // Encoding: 0x06500090
    // Test aarch32_UADD8_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rn=0, cond=0, Rd=0, Rm=0
    let encoding: u32 = 0x06500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UADD8_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uadd8_a1_a_invalid_1_90_06500090() {
    // Encoding: 0x06500090
    // Test aarch32_UADD8_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=0, Rd=0
    let encoding: u32 = 0x06500090;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UADD8_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uadd8_t1_a_field_rn_0_min_f040_fa80f040() {
    // Thumb encoding (32): 0xFA80F040
    // Test aarch32_UADD8_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UADD8_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uadd8_t1_a_field_rn_1_poweroftwo_f040_fa81f040() {
    // Thumb encoding (32): 0xFA81F040
    // Test aarch32_UADD8_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA81F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UADD8_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uadd8_t1_a_field_rd_0_min_f040_fa80f040() {
    // Thumb encoding (32): 0xFA80F040
    // Test aarch32_UADD8_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UADD8_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uadd8_t1_a_field_rd_1_poweroftwo_f040_fa80f140() {
    // Thumb encoding (32): 0xFA80F140
    // Test aarch32_UADD8_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rd=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F140;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UADD8_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uadd8_t1_a_field_rm_0_min_f040_fa80f040() {
    // Thumb encoding (32): 0xFA80F040
    // Test aarch32_UADD8_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UADD8_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uadd8_t1_a_field_rm_1_poweroftwo_f040_fa80f041() {
    // Thumb encoding (32): 0xFA80F041
    // Test aarch32_UADD8_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rm=1, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F041;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UADD8_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_uadd8_t1_a_combo_0_f040_fa80f040() {
    // Thumb encoding (32): 0xFA80F040
    // Test aarch32_UADD8_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UADD8_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uadd8_t1_a_invalid_0_f040_fa80f040() {
    // Thumb encoding (32): 0xFA80F040
    // Test aarch32_UADD8_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F040;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_UADD8_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uadd8_t1_a_invalid_1_f040_fa80f040() {
    // Thumb encoding (32): 0xFA80F040
    // Test aarch32_UADD8_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA80F040;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_SHSAX_A Tests
// ============================================================================

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_shsax_a1_a_field_cond_0_min_50_06300050() {
    // Encoding: 0x06300050
    // Test aarch32_SHSAX_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x06300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_shsax_a1_a_field_cond_1_poweroftwo_50_16300050() {
    // Encoding: 0x16300050
    // Test aarch32_SHSAX_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=1, Rn=0, Rm=0
    let encoding: u32 = 0x16300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_shsax_a1_a_field_cond_2_poweroftwo_50_26300050() {
    // Encoding: 0x26300050
    // Test aarch32_SHSAX_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=2
    let encoding: u32 = 0x26300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_shsax_a1_a_field_cond_3_poweroftwo_50_36300050() {
    // Encoding: 0x36300050
    // Test aarch32_SHSAX_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=3, Rd=0
    let encoding: u32 = 0x36300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_shsax_a1_a_field_cond_4_poweroftwo_50_46300050() {
    // Encoding: 0x46300050
    // Test aarch32_SHSAX_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=4
    let encoding: u32 = 0x46300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_shsax_a1_a_field_cond_5_poweroftwo_50_56300050() {
    // Encoding: 0x56300050
    // Test aarch32_SHSAX_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=5, Rn=0, Rd=0
    let encoding: u32 = 0x56300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_shsax_a1_a_field_cond_6_poweroftwo_50_66300050() {
    // Encoding: 0x66300050
    // Test aarch32_SHSAX_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x66300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_shsax_a1_a_field_cond_7_poweroftwo_50_76300050() {
    // Encoding: 0x76300050
    // Test aarch32_SHSAX_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=7, Rn=0, Rd=0
    let encoding: u32 = 0x76300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_shsax_a1_a_field_cond_8_poweroftwo_50_86300050() {
    // Encoding: 0x86300050
    // Test aarch32_SHSAX_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=8, Rd=0
    let encoding: u32 = 0x86300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_shsax_a1_a_field_cond_9_poweroftwo_50_96300050() {
    // Encoding: 0x96300050
    // Test aarch32_SHSAX_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x96300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_shsax_a1_a_field_cond_10_poweroftwo_50_a6300050() {
    // Encoding: 0xA6300050
    // Test aarch32_SHSAX_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=10
    let encoding: u32 = 0xA6300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_shsax_a1_a_field_cond_11_poweroftwo_50_b6300050() {
    // Encoding: 0xB6300050
    // Test aarch32_SHSAX_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=11, Rn=0, Rm=0
    let encoding: u32 = 0xB6300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_shsax_a1_a_field_cond_12_poweroftwo_50_c6300050() {
    // Encoding: 0xC6300050
    // Test aarch32_SHSAX_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=12, Rm=0, Rd=0
    let encoding: u32 = 0xC6300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_shsax_a1_a_field_cond_13_poweroftwo_50_d6300050() {
    // Encoding: 0xD6300050
    // Test aarch32_SHSAX_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=13, Rd=0
    let encoding: u32 = 0xD6300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_shsax_a1_a_field_cond_14_poweroftwo_50_e6300050() {
    // Encoding: 0xE6300050
    // Test aarch32_SHSAX_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xE6300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_shsax_a1_a_field_cond_15_max_50_f6300050() {
    // Encoding: 0xF6300050
    // Test aarch32_SHSAX_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=15, Rm=0
    let encoding: u32 = 0xF6300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shsax_a1_a_field_rn_0_min_50_06300050() {
    // Encoding: 0x06300050
    // Test aarch32_SHSAX_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x06300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shsax_a1_a_field_rn_1_poweroftwo_50_06310050() {
    // Encoding: 0x06310050
    // Test aarch32_SHSAX_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=1, Rd=0, Rm=0
    let encoding: u32 = 0x06310050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shsax_a1_a_field_rd_0_min_50_06300050() {
    // Encoding: 0x06300050
    // Test aarch32_SHSAX_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x06300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shsax_a1_a_field_rd_1_poweroftwo_50_06301050() {
    // Encoding: 0x06301050
    // Test aarch32_SHSAX_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rd=1, Rn=0
    let encoding: u32 = 0x06301050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shsax_a1_a_field_rm_0_min_50_06300050() {
    // Encoding: 0x06300050
    // Test aarch32_SHSAX_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, cond=0, Rd=0, Rm=0
    let encoding: u32 = 0x06300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shsax_a1_a_field_rm_1_poweroftwo_50_06300051() {
    // Encoding: 0x06300051
    // Test aarch32_SHSAX_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rm=1, Rd=0
    let encoding: u32 = 0x06300051;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_shsax_a1_a_combo_0_50_06300050() {
    // Encoding: 0x06300050
    // Test aarch32_SHSAX_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=0, Rn=0
    let encoding: u32 = 0x06300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_shsax_a1_a_special_cond_0_condition_eq_80_06300050() {
    // Encoding: 0x06300050
    // Test aarch32_SHSAX_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=0
    let encoding: u32 = 0x06300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_shsax_a1_a_special_cond_1_condition_ne_80_16300050() {
    // Encoding: 0x16300050
    // Test aarch32_SHSAX_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=1
    let encoding: u32 = 0x16300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_shsax_a1_a_special_cond_2_condition_cs_hs_80_26300050() {
    // Encoding: 0x26300050
    // Test aarch32_SHSAX_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x26300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_shsax_a1_a_special_cond_3_condition_cc_lo_80_36300050() {
    // Encoding: 0x36300050
    // Test aarch32_SHSAX_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=3, Rd=0
    let encoding: u32 = 0x36300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_shsax_a1_a_special_cond_4_condition_mi_80_46300050() {
    // Encoding: 0x46300050
    // Test aarch32_SHSAX_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=4
    let encoding: u32 = 0x46300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_shsax_a1_a_special_cond_5_condition_pl_80_56300050() {
    // Encoding: 0x56300050
    // Test aarch32_SHSAX_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rn=0, cond=5, Rm=0, Rd=0
    let encoding: u32 = 0x56300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_shsax_a1_a_special_cond_6_condition_vs_80_66300050() {
    // Encoding: 0x66300050
    // Test aarch32_SHSAX_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=6, Rn=0
    let encoding: u32 = 0x66300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_shsax_a1_a_special_cond_7_condition_vc_80_76300050() {
    // Encoding: 0x76300050
    // Test aarch32_SHSAX_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x76300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_shsax_a1_a_special_cond_8_condition_hi_80_86300050() {
    // Encoding: 0x86300050
    // Test aarch32_SHSAX_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x86300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_shsax_a1_a_special_cond_9_condition_ls_80_96300050() {
    // Encoding: 0x96300050
    // Test aarch32_SHSAX_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x96300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_shsax_a1_a_special_cond_10_condition_ge_80_a6300050() {
    // Encoding: 0xA6300050
    // Test aarch32_SHSAX_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0xA6300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_shsax_a1_a_special_cond_11_condition_lt_80_b6300050() {
    // Encoding: 0xB6300050
    // Test aarch32_SHSAX_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rn=0, cond=11, Rd=0, Rm=0
    let encoding: u32 = 0xB6300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_shsax_a1_a_special_cond_12_condition_gt_80_c6300050() {
    // Encoding: 0xC6300050
    // Test aarch32_SHSAX_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=12, Rd=0
    let encoding: u32 = 0xC6300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_shsax_a1_a_special_cond_13_condition_le_80_d6300050() {
    // Encoding: 0xD6300050
    // Test aarch32_SHSAX_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rm=0, cond=13, Rd=0, Rn=0
    let encoding: u32 = 0xD6300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_shsax_a1_a_special_cond_14_condition_al_80_e6300050() {
    // Encoding: 0xE6300050
    // Test aarch32_SHSAX_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rn=0, cond=14, Rd=0, Rm=0
    let encoding: u32 = 0xE6300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_shsax_a1_a_special_cond_15_condition_nv_80_f6300050() {
    // Encoding: 0xF6300050
    // Test aarch32_SHSAX_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rn=0, cond=15, Rd=0, Rm=0
    let encoding: u32 = 0xF6300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_shsax_a1_a_invalid_0_50_06300050() {
    // Encoding: 0x06300050
    // Test aarch32_SHSAX_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rm=0, cond=0, Rd=0, Rn=0
    let encoding: u32 = 0x06300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SHSAX_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_shsax_a1_a_invalid_1_50_06300050() {
    // Encoding: 0x06300050
    // Test aarch32_SHSAX_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rd=0, cond=0, Rn=0, Rm=0
    let encoding: u32 = 0x06300050;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SHSAX_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shsax_t1_a_field_rn_0_min_f020_fae0f020() {
    // Thumb encoding (32): 0xFAE0F020
    // Test aarch32_SHSAX_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHSAX_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shsax_t1_a_field_rn_1_poweroftwo_f020_fae1f020() {
    // Thumb encoding (32): 0xFAE1F020
    // Test aarch32_SHSAX_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE1F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHSAX_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shsax_t1_a_field_rd_0_min_f020_fae0f020() {
    // Thumb encoding (32): 0xFAE0F020
    // Test aarch32_SHSAX_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHSAX_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shsax_t1_a_field_rd_1_poweroftwo_f020_fae0f120() {
    // Thumb encoding (32): 0xFAE0F120
    // Test aarch32_SHSAX_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rd=1, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F120;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHSAX_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shsax_t1_a_field_rm_0_min_f020_fae0f020() {
    // Thumb encoding (32): 0xFAE0F020
    // Test aarch32_SHSAX_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHSAX_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shsax_t1_a_field_rm_1_poweroftwo_f020_fae0f021() {
    // Thumb encoding (32): 0xFAE0F021
    // Test aarch32_SHSAX_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F021;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHSAX_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_shsax_t1_a_combo_0_f020_fae0f020() {
    // Thumb encoding (32): 0xFAE0F020
    // Test aarch32_SHSAX_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHSAX_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_shsax_t1_a_invalid_0_f020_fae0f020() {
    // Thumb encoding (32): 0xFAE0F020
    // Test aarch32_SHSAX_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F020;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_SHSAX_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_shsax_t1_a_invalid_1_f020_fae0f020() {
    // Thumb encoding (32): 0xFAE0F020
    // Test aarch32_SHSAX_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAE0F020;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_USUB16_A Tests
// ============================================================================

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_usub16_a1_a_field_cond_0_min_70_06500070() {
    // Encoding: 0x06500070
    // Test aarch32_USUB16_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=0
    let encoding: u32 = 0x06500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_usub16_a1_a_field_cond_1_poweroftwo_70_16500070() {
    // Encoding: 0x16500070
    // Test aarch32_USUB16_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=1
    let encoding: u32 = 0x16500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_usub16_a1_a_field_cond_2_poweroftwo_70_26500070() {
    // Encoding: 0x26500070
    // Test aarch32_USUB16_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=2, Rm=0
    let encoding: u32 = 0x26500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_usub16_a1_a_field_cond_3_poweroftwo_70_36500070() {
    // Encoding: 0x36500070
    // Test aarch32_USUB16_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=3
    let encoding: u32 = 0x36500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_usub16_a1_a_field_cond_4_poweroftwo_70_46500070() {
    // Encoding: 0x46500070
    // Test aarch32_USUB16_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=4, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x46500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_usub16_a1_a_field_cond_5_poweroftwo_70_56500070() {
    // Encoding: 0x56500070
    // Test aarch32_USUB16_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=5, Rd=0, Rm=0
    let encoding: u32 = 0x56500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_usub16_a1_a_field_cond_6_poweroftwo_70_66500070() {
    // Encoding: 0x66500070
    // Test aarch32_USUB16_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x66500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_usub16_a1_a_field_cond_7_poweroftwo_70_76500070() {
    // Encoding: 0x76500070
    // Test aarch32_USUB16_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=7, Rm=0, Rn=0
    let encoding: u32 = 0x76500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_usub16_a1_a_field_cond_8_poweroftwo_70_86500070() {
    // Encoding: 0x86500070
    // Test aarch32_USUB16_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=8
    let encoding: u32 = 0x86500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_usub16_a1_a_field_cond_9_poweroftwo_70_96500070() {
    // Encoding: 0x96500070
    // Test aarch32_USUB16_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=9, Rm=0, Rd=0
    let encoding: u32 = 0x96500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_usub16_a1_a_field_cond_10_poweroftwo_70_a6500070() {
    // Encoding: 0xA6500070
    // Test aarch32_USUB16_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0xA6500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_usub16_a1_a_field_cond_11_poweroftwo_70_b6500070() {
    // Encoding: 0xB6500070
    // Test aarch32_USUB16_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=11
    let encoding: u32 = 0xB6500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_usub16_a1_a_field_cond_12_poweroftwo_70_c6500070() {
    // Encoding: 0xC6500070
    // Test aarch32_USUB16_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=12, Rd=0, Rn=0
    let encoding: u32 = 0xC6500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_usub16_a1_a_field_cond_13_poweroftwo_70_d6500070() {
    // Encoding: 0xD6500070
    // Test aarch32_USUB16_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=13, Rm=0
    let encoding: u32 = 0xD6500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_usub16_a1_a_field_cond_14_poweroftwo_70_e6500070() {
    // Encoding: 0xE6500070
    // Test aarch32_USUB16_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=14, Rd=0, Rn=0
    let encoding: u32 = 0xE6500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_usub16_a1_a_field_cond_15_max_70_f6500070() {
    // Encoding: 0xF6500070
    // Test aarch32_USUB16_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rn=0, cond=15, Rm=0, Rd=0
    let encoding: u32 = 0xF6500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_usub16_a1_a_field_rn_0_min_70_06500070() {
    // Encoding: 0x06500070
    // Test aarch32_USUB16_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x06500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_usub16_a1_a_field_rn_1_poweroftwo_70_06510070() {
    // Encoding: 0x06510070
    // Test aarch32_USUB16_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=1, Rd=0, cond=0
    let encoding: u32 = 0x06510070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_usub16_a1_a_field_rd_0_min_70_06500070() {
    // Encoding: 0x06500070
    // Test aarch32_USUB16_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x06500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_usub16_a1_a_field_rd_1_poweroftwo_70_06501070() {
    // Encoding: 0x06501070
    // Test aarch32_USUB16_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=1, cond=0, Rn=0
    let encoding: u32 = 0x06501070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_usub16_a1_a_field_rm_0_min_70_06500070() {
    // Encoding: 0x06500070
    // Test aarch32_USUB16_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x06500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_usub16_a1_a_field_rm_1_poweroftwo_70_06500071() {
    // Encoding: 0x06500071
    // Test aarch32_USUB16_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=1, cond=0, Rn=0, Rd=0
    let encoding: u32 = 0x06500071;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_usub16_a1_a_combo_0_70_06500070() {
    // Encoding: 0x06500070
    // Test aarch32_USUB16_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rd=0, cond=0, Rm=0, Rn=0
    let encoding: u32 = 0x06500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_usub16_a1_a_special_cond_0_condition_eq_112_06500070() {
    // Encoding: 0x06500070
    // Test aarch32_USUB16_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=0
    let encoding: u32 = 0x06500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_usub16_a1_a_special_cond_1_condition_ne_112_16500070() {
    // Encoding: 0x16500070
    // Test aarch32_USUB16_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rd=0, cond=1, Rm=0, Rn=0
    let encoding: u32 = 0x16500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_usub16_a1_a_special_cond_2_condition_cs_hs_112_26500070() {
    // Encoding: 0x26500070
    // Test aarch32_USUB16_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=2, Rm=0
    let encoding: u32 = 0x26500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_usub16_a1_a_special_cond_3_condition_cc_lo_112_36500070() {
    // Encoding: 0x36500070
    // Test aarch32_USUB16_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x36500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_usub16_a1_a_special_cond_4_condition_mi_112_46500070() {
    // Encoding: 0x46500070
    // Test aarch32_USUB16_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=4
    let encoding: u32 = 0x46500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_usub16_a1_a_special_cond_5_condition_pl_112_56500070() {
    // Encoding: 0x56500070
    // Test aarch32_USUB16_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x56500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_usub16_a1_a_special_cond_6_condition_vs_112_66500070() {
    // Encoding: 0x66500070
    // Test aarch32_USUB16_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=6, Rn=0
    let encoding: u32 = 0x66500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_usub16_a1_a_special_cond_7_condition_vc_112_76500070() {
    // Encoding: 0x76500070
    // Test aarch32_USUB16_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=7, Rm=0
    let encoding: u32 = 0x76500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_usub16_a1_a_special_cond_8_condition_hi_112_86500070() {
    // Encoding: 0x86500070
    // Test aarch32_USUB16_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x86500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_usub16_a1_a_special_cond_9_condition_ls_112_96500070() {
    // Encoding: 0x96500070
    // Test aarch32_USUB16_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=9, Rm=0
    let encoding: u32 = 0x96500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_usub16_a1_a_special_cond_10_condition_ge_112_a6500070() {
    // Encoding: 0xA6500070
    // Test aarch32_USUB16_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=10
    let encoding: u32 = 0xA6500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_usub16_a1_a_special_cond_11_condition_lt_112_b6500070() {
    // Encoding: 0xB6500070
    // Test aarch32_USUB16_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0xB6500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_usub16_a1_a_special_cond_12_condition_gt_112_c6500070() {
    // Encoding: 0xC6500070
    // Test aarch32_USUB16_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=12, Rd=0
    let encoding: u32 = 0xC6500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_usub16_a1_a_special_cond_13_condition_le_112_d6500070() {
    // Encoding: 0xD6500070
    // Test aarch32_USUB16_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=13
    let encoding: u32 = 0xD6500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_usub16_a1_a_special_cond_14_condition_al_112_e6500070() {
    // Encoding: 0xE6500070
    // Test aarch32_USUB16_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rd=0, cond=14, Rm=0, Rn=0
    let encoding: u32 = 0xE6500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_usub16_a1_a_special_cond_15_condition_nv_112_f6500070() {
    // Encoding: 0xF6500070
    // Test aarch32_USUB16_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0xF6500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_usub16_a1_a_invalid_0_70_06500070() {
    // Encoding: 0x06500070
    // Test aarch32_USUB16_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rd=0, cond=0, Rm=0, Rn=0
    let encoding: u32 = 0x06500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_USUB16_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_usub16_a1_a_invalid_1_70_06500070() {
    // Encoding: 0x06500070
    // Test aarch32_USUB16_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: cond=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x06500070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_USUB16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_usub16_t1_a_field_rn_0_min_f040_fad0f040() {
    // Thumb encoding (32): 0xFAD0F040
    // Test aarch32_USUB16_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_USUB16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_usub16_t1_a_field_rn_1_poweroftwo_f040_fad1f040() {
    // Thumb encoding (32): 0xFAD1F040
    // Test aarch32_USUB16_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD1F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_USUB16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_usub16_t1_a_field_rd_0_min_f040_fad0f040() {
    // Thumb encoding (32): 0xFAD0F040
    // Test aarch32_USUB16_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_USUB16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_usub16_t1_a_field_rd_1_poweroftwo_f040_fad0f140() {
    // Thumb encoding (32): 0xFAD0F140
    // Test aarch32_USUB16_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=1, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F140;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_USUB16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_usub16_t1_a_field_rm_0_min_f040_fad0f040() {
    // Thumb encoding (32): 0xFAD0F040
    // Test aarch32_USUB16_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_USUB16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_usub16_t1_a_field_rm_1_poweroftwo_f040_fad0f041() {
    // Thumb encoding (32): 0xFAD0F041
    // Test aarch32_USUB16_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F041;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_USUB16_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_usub16_t1_a_combo_0_f040_fad0f040() {
    // Thumb encoding (32): 0xFAD0F040
    // Test aarch32_USUB16_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_USUB16_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_usub16_t1_a_invalid_0_f040_fad0f040() {
    // Thumb encoding (32): 0xFAD0F040
    // Test aarch32_USUB16_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F040;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_USUB16_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_usub16_t1_a_invalid_1_f040_fad0f040() {
    // Thumb encoding (32): 0xFAD0F040
    // Test aarch32_USUB16_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F040;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_UHADD16_A Tests
// ============================================================================

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_uhadd16_a1_a_field_cond_0_min_10_06700010() {
    // Encoding: 0x06700010
    // Test aarch32_UHADD16_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x06700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_uhadd16_a1_a_field_cond_1_poweroftwo_10_16700010() {
    // Encoding: 0x16700010
    // Test aarch32_UHADD16_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x16700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_uhadd16_a1_a_field_cond_2_poweroftwo_10_26700010() {
    // Encoding: 0x26700010
    // Test aarch32_UHADD16_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=2, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x26700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_uhadd16_a1_a_field_cond_3_poweroftwo_10_36700010() {
    // Encoding: 0x36700010
    // Test aarch32_UHADD16_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x36700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_uhadd16_a1_a_field_cond_4_poweroftwo_10_46700010() {
    // Encoding: 0x46700010
    // Test aarch32_UHADD16_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=4
    let encoding: u32 = 0x46700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_uhadd16_a1_a_field_cond_5_poweroftwo_10_56700010() {
    // Encoding: 0x56700010
    // Test aarch32_UHADD16_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=5, Rn=0, Rm=0
    let encoding: u32 = 0x56700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_uhadd16_a1_a_field_cond_6_poweroftwo_10_66700010() {
    // Encoding: 0x66700010
    // Test aarch32_UHADD16_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=6, Rm=0
    let encoding: u32 = 0x66700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_uhadd16_a1_a_field_cond_7_poweroftwo_10_76700010() {
    // Encoding: 0x76700010
    // Test aarch32_UHADD16_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=7, Rn=0, Rm=0
    let encoding: u32 = 0x76700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_uhadd16_a1_a_field_cond_8_poweroftwo_10_86700010() {
    // Encoding: 0x86700010
    // Test aarch32_UHADD16_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=8
    let encoding: u32 = 0x86700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_uhadd16_a1_a_field_cond_9_poweroftwo_10_96700010() {
    // Encoding: 0x96700010
    // Test aarch32_UHADD16_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=9, Rm=0
    let encoding: u32 = 0x96700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_uhadd16_a1_a_field_cond_10_poweroftwo_10_a6700010() {
    // Encoding: 0xA6700010
    // Test aarch32_UHADD16_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0xA6700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_uhadd16_a1_a_field_cond_11_poweroftwo_10_b6700010() {
    // Encoding: 0xB6700010
    // Test aarch32_UHADD16_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=11, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0xB6700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_uhadd16_a1_a_field_cond_12_poweroftwo_10_c6700010() {
    // Encoding: 0xC6700010
    // Test aarch32_UHADD16_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0xC6700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_uhadd16_a1_a_field_cond_13_poweroftwo_10_d6700010() {
    // Encoding: 0xD6700010
    // Test aarch32_UHADD16_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=13, Rm=0
    let encoding: u32 = 0xD6700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_uhadd16_a1_a_field_cond_14_poweroftwo_10_e6700010() {
    // Encoding: 0xE6700010
    // Test aarch32_UHADD16_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0xE6700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_uhadd16_a1_a_field_cond_15_max_10_f6700010() {
    // Encoding: 0xF6700010
    // Test aarch32_UHADD16_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rn=0, cond=15, Rm=0, Rd=0
    let encoding: u32 = 0xF6700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhadd16_a1_a_field_rn_0_min_10_06700010() {
    // Encoding: 0x06700010
    // Test aarch32_UHADD16_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=0, Rm=0
    let encoding: u32 = 0x06700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhadd16_a1_a_field_rn_1_poweroftwo_10_06710010() {
    // Encoding: 0x06710010
    // Test aarch32_UHADD16_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=1, Rd=0, cond=0, Rm=0
    let encoding: u32 = 0x06710010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhadd16_a1_a_field_rd_0_min_10_06700010() {
    // Encoding: 0x06700010
    // Test aarch32_UHADD16_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x06700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhadd16_a1_a_field_rd_1_poweroftwo_10_06701010() {
    // Encoding: 0x06701010
    // Test aarch32_UHADD16_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=0, Rd=1
    let encoding: u32 = 0x06701010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhadd16_a1_a_field_rm_0_min_10_06700010() {
    // Encoding: 0x06700010
    // Test aarch32_UHADD16_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, cond=0, Rm=0, Rn=0
    let encoding: u32 = 0x06700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhadd16_a1_a_field_rm_1_poweroftwo_10_06700011() {
    // Encoding: 0x06700011
    // Test aarch32_UHADD16_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=0, Rm=1
    let encoding: u32 = 0x06700011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_uhadd16_a1_a_combo_0_10_06700010() {
    // Encoding: 0x06700010
    // Test aarch32_UHADD16_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x06700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_uhadd16_a1_a_special_cond_0_condition_eq_16_06700010() {
    // Encoding: 0x06700010
    // Test aarch32_UHADD16_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=0, Rm=0
    let encoding: u32 = 0x06700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_uhadd16_a1_a_special_cond_1_condition_ne_16_16700010() {
    // Encoding: 0x16700010
    // Test aarch32_UHADD16_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rn=0, cond=1, Rm=0, Rd=0
    let encoding: u32 = 0x16700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_uhadd16_a1_a_special_cond_2_condition_cs_hs_16_26700010() {
    // Encoding: 0x26700010
    // Test aarch32_UHADD16_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x26700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_uhadd16_a1_a_special_cond_3_condition_cc_lo_16_36700010() {
    // Encoding: 0x36700010
    // Test aarch32_UHADD16_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rd=0, cond=3, Rn=0, Rm=0
    let encoding: u32 = 0x36700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_uhadd16_a1_a_special_cond_4_condition_mi_16_46700010() {
    // Encoding: 0x46700010
    // Test aarch32_UHADD16_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rm=0, cond=4, Rn=0, Rd=0
    let encoding: u32 = 0x46700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_uhadd16_a1_a_special_cond_5_condition_pl_16_56700010() {
    // Encoding: 0x56700010
    // Test aarch32_UHADD16_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=5
    let encoding: u32 = 0x56700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_uhadd16_a1_a_special_cond_6_condition_vs_16_66700010() {
    // Encoding: 0x66700010
    // Test aarch32_UHADD16_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rm=0, cond=6, Rd=0, Rn=0
    let encoding: u32 = 0x66700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_uhadd16_a1_a_special_cond_7_condition_vc_16_76700010() {
    // Encoding: 0x76700010
    // Test aarch32_UHADD16_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rn=0, cond=7, Rd=0, Rm=0
    let encoding: u32 = 0x76700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_uhadd16_a1_a_special_cond_8_condition_hi_16_86700010() {
    // Encoding: 0x86700010
    // Test aarch32_UHADD16_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rm=0, cond=8, Rd=0, Rn=0
    let encoding: u32 = 0x86700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_uhadd16_a1_a_special_cond_9_condition_ls_16_96700010() {
    // Encoding: 0x96700010
    // Test aarch32_UHADD16_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=9, Rn=0
    let encoding: u32 = 0x96700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_uhadd16_a1_a_special_cond_10_condition_ge_16_a6700010() {
    // Encoding: 0xA6700010
    // Test aarch32_UHADD16_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0xA6700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_uhadd16_a1_a_special_cond_11_condition_lt_16_b6700010() {
    // Encoding: 0xB6700010
    // Test aarch32_UHADD16_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=11, Rm=0
    let encoding: u32 = 0xB6700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_uhadd16_a1_a_special_cond_12_condition_gt_16_c6700010() {
    // Encoding: 0xC6700010
    // Test aarch32_UHADD16_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=12
    let encoding: u32 = 0xC6700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_uhadd16_a1_a_special_cond_13_condition_le_16_d6700010() {
    // Encoding: 0xD6700010
    // Test aarch32_UHADD16_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=13, Rn=0
    let encoding: u32 = 0xD6700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_uhadd16_a1_a_special_cond_14_condition_al_16_e6700010() {
    // Encoding: 0xE6700010
    // Test aarch32_UHADD16_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: cond=14, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0xE6700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_uhadd16_a1_a_special_cond_15_condition_nv_16_f6700010() {
    // Encoding: 0xF6700010
    // Test aarch32_UHADD16_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=15
    let encoding: u32 = 0xF6700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uhadd16_a1_a_invalid_0_10_06700010() {
    // Encoding: 0x06700010
    // Test aarch32_UHADD16_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x06700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UHADD16_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uhadd16_a1_a_invalid_1_10_06700010() {
    // Encoding: 0x06700010
    // Test aarch32_UHADD16_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=0, Rm=0
    let encoding: u32 = 0x06700010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UHADD16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhadd16_t1_a_field_rn_0_min_f060_fa90f060() {
    // Thumb encoding (32): 0xFA90F060
    // Test aarch32_UHADD16_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHADD16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhadd16_t1_a_field_rn_1_poweroftwo_f060_fa91f060() {
    // Thumb encoding (32): 0xFA91F060
    // Test aarch32_UHADD16_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA91F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHADD16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhadd16_t1_a_field_rd_0_min_f060_fa90f060() {
    // Thumb encoding (32): 0xFA90F060
    // Test aarch32_UHADD16_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHADD16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhadd16_t1_a_field_rd_1_poweroftwo_f060_fa90f160() {
    // Thumb encoding (32): 0xFA90F160
    // Test aarch32_UHADD16_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rd=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F160;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHADD16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhadd16_t1_a_field_rm_0_min_f060_fa90f060() {
    // Thumb encoding (32): 0xFA90F060
    // Test aarch32_UHADD16_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHADD16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhadd16_t1_a_field_rm_1_poweroftwo_f060_fa90f061() {
    // Thumb encoding (32): 0xFA90F061
    // Test aarch32_UHADD16_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F061;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHADD16_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_uhadd16_t1_a_combo_0_f060_fa90f060() {
    // Thumb encoding (32): 0xFA90F060
    // Test aarch32_UHADD16_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHADD16_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uhadd16_t1_a_invalid_0_f060_fa90f060() {
    // Thumb encoding (32): 0xFA90F060
    // Test aarch32_UHADD16_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F060;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_UHADD16_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uhadd16_t1_a_invalid_1_f060_fa90f060() {
    // Thumb encoding (32): 0xFA90F060
    // Test aarch32_UHADD16_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F060;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_SSUB16_A Tests
// ============================================================================

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_ssub16_a1_a_field_cond_0_min_70_06100070() {
    // Encoding: 0x06100070
    // Test aarch32_SSUB16_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x06100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_ssub16_a1_a_field_cond_1_poweroftwo_70_16100070() {
    // Encoding: 0x16100070
    // Test aarch32_SSUB16_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=1
    let encoding: u32 = 0x16100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_ssub16_a1_a_field_cond_2_poweroftwo_70_26100070() {
    // Encoding: 0x26100070
    // Test aarch32_SSUB16_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=2, Rm=0, Rn=0
    let encoding: u32 = 0x26100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_ssub16_a1_a_field_cond_3_poweroftwo_70_36100070() {
    // Encoding: 0x36100070
    // Test aarch32_SSUB16_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=3, Rd=0
    let encoding: u32 = 0x36100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_ssub16_a1_a_field_cond_4_poweroftwo_70_46100070() {
    // Encoding: 0x46100070
    // Test aarch32_SSUB16_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=4
    let encoding: u32 = 0x46100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_ssub16_a1_a_field_cond_5_poweroftwo_70_56100070() {
    // Encoding: 0x56100070
    // Test aarch32_SSUB16_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=5, Rn=0, Rd=0
    let encoding: u32 = 0x56100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_ssub16_a1_a_field_cond_6_poweroftwo_70_66100070() {
    // Encoding: 0x66100070
    // Test aarch32_SSUB16_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=6, Rn=0
    let encoding: u32 = 0x66100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_ssub16_a1_a_field_cond_7_poweroftwo_70_76100070() {
    // Encoding: 0x76100070
    // Test aarch32_SSUB16_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=7, Rn=0
    let encoding: u32 = 0x76100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_ssub16_a1_a_field_cond_8_poweroftwo_70_86100070() {
    // Encoding: 0x86100070
    // Test aarch32_SSUB16_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=8
    let encoding: u32 = 0x86100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_ssub16_a1_a_field_cond_9_poweroftwo_70_96100070() {
    // Encoding: 0x96100070
    // Test aarch32_SSUB16_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=9, Rm=0, Rn=0
    let encoding: u32 = 0x96100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_ssub16_a1_a_field_cond_10_poweroftwo_70_a6100070() {
    // Encoding: 0xA6100070
    // Test aarch32_SSUB16_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=10, Rn=0, Rm=0
    let encoding: u32 = 0xA6100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_ssub16_a1_a_field_cond_11_poweroftwo_70_b6100070() {
    // Encoding: 0xB6100070
    // Test aarch32_SSUB16_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=11, Rn=0, Rd=0
    let encoding: u32 = 0xB6100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_ssub16_a1_a_field_cond_12_poweroftwo_70_c6100070() {
    // Encoding: 0xC6100070
    // Test aarch32_SSUB16_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=12
    let encoding: u32 = 0xC6100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_ssub16_a1_a_field_cond_13_poweroftwo_70_d6100070() {
    // Encoding: 0xD6100070
    // Test aarch32_SSUB16_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=13, Rn=0, Rd=0
    let encoding: u32 = 0xD6100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_ssub16_a1_a_field_cond_14_poweroftwo_70_e6100070() {
    // Encoding: 0xE6100070
    // Test aarch32_SSUB16_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=14, Rm=0, Rn=0
    let encoding: u32 = 0xE6100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_ssub16_a1_a_field_cond_15_max_70_f6100070() {
    // Encoding: 0xF6100070
    // Test aarch32_SSUB16_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rn=0, cond=15, Rd=0, Rm=0
    let encoding: u32 = 0xF6100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_ssub16_a1_a_field_rn_0_min_70_06100070() {
    // Encoding: 0x06100070
    // Test aarch32_SSUB16_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x06100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_ssub16_a1_a_field_rn_1_poweroftwo_70_06110070() {
    // Encoding: 0x06110070
    // Test aarch32_SSUB16_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=0, Rn=1, Rm=0
    let encoding: u32 = 0x06110070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_ssub16_a1_a_field_rd_0_min_70_06100070() {
    // Encoding: 0x06100070
    // Test aarch32_SSUB16_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=0
    let encoding: u32 = 0x06100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_ssub16_a1_a_field_rd_1_poweroftwo_70_06101070() {
    // Encoding: 0x06101070
    // Test aarch32_SSUB16_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=0, Rd=1
    let encoding: u32 = 0x06101070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_ssub16_a1_a_field_rm_0_min_70_06100070() {
    // Encoding: 0x06100070
    // Test aarch32_SSUB16_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=0, Rd=0
    let encoding: u32 = 0x06100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_ssub16_a1_a_field_rm_1_poweroftwo_70_06100071() {
    // Encoding: 0x06100071
    // Test aarch32_SSUB16_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=1, Rn=0, Rd=0, cond=0
    let encoding: u32 = 0x06100071;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_ssub16_a1_a_combo_0_70_06100070() {
    // Encoding: 0x06100070
    // Test aarch32_SSUB16_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=0
    let encoding: u32 = 0x06100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_ssub16_a1_a_special_cond_0_condition_eq_112_06100070() {
    // Encoding: 0x06100070
    // Test aarch32_SSUB16_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=0, Rm=0
    let encoding: u32 = 0x06100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_ssub16_a1_a_special_cond_1_condition_ne_112_16100070() {
    // Encoding: 0x16100070
    // Test aarch32_SSUB16_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x16100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_ssub16_a1_a_special_cond_2_condition_cs_hs_112_26100070() {
    // Encoding: 0x26100070
    // Test aarch32_SSUB16_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=2, Rd=0
    let encoding: u32 = 0x26100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_ssub16_a1_a_special_cond_3_condition_cc_lo_112_36100070() {
    // Encoding: 0x36100070
    // Test aarch32_SSUB16_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x36100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_ssub16_a1_a_special_cond_4_condition_mi_112_46100070() {
    // Encoding: 0x46100070
    // Test aarch32_SSUB16_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=4, Rd=0
    let encoding: u32 = 0x46100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_ssub16_a1_a_special_cond_5_condition_pl_112_56100070() {
    // Encoding: 0x56100070
    // Test aarch32_SSUB16_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=5, Rn=0
    let encoding: u32 = 0x56100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_ssub16_a1_a_special_cond_6_condition_vs_112_66100070() {
    // Encoding: 0x66100070
    // Test aarch32_SSUB16_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x66100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_ssub16_a1_a_special_cond_7_condition_vc_112_76100070() {
    // Encoding: 0x76100070
    // Test aarch32_SSUB16_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=7, Rd=0
    let encoding: u32 = 0x76100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_ssub16_a1_a_special_cond_8_condition_hi_112_86100070() {
    // Encoding: 0x86100070
    // Test aarch32_SSUB16_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rm=0, cond=8, Rd=0, Rn=0
    let encoding: u32 = 0x86100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_ssub16_a1_a_special_cond_9_condition_ls_112_96100070() {
    // Encoding: 0x96100070
    // Test aarch32_SSUB16_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=9, Rd=0
    let encoding: u32 = 0x96100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_ssub16_a1_a_special_cond_10_condition_ge_112_a6100070() {
    // Encoding: 0xA6100070
    // Test aarch32_SSUB16_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=10
    let encoding: u32 = 0xA6100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_ssub16_a1_a_special_cond_11_condition_lt_112_b6100070() {
    // Encoding: 0xB6100070
    // Test aarch32_SSUB16_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0xB6100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_ssub16_a1_a_special_cond_12_condition_gt_112_c6100070() {
    // Encoding: 0xC6100070
    // Test aarch32_SSUB16_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=12
    let encoding: u32 = 0xC6100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_ssub16_a1_a_special_cond_13_condition_le_112_d6100070() {
    // Encoding: 0xD6100070
    // Test aarch32_SSUB16_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=13, Rm=0
    let encoding: u32 = 0xD6100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_ssub16_a1_a_special_cond_14_condition_al_112_e6100070() {
    // Encoding: 0xE6100070
    // Test aarch32_SSUB16_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=14, Rd=0
    let encoding: u32 = 0xE6100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_ssub16_a1_a_special_cond_15_condition_nv_112_f6100070() {
    // Encoding: 0xF6100070
    // Test aarch32_SSUB16_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=15
    let encoding: u32 = 0xF6100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_ssub16_a1_a_invalid_0_70_06100070() {
    // Encoding: 0x06100070
    // Test aarch32_SSUB16_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=0, Rm=0
    let encoding: u32 = 0x06100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SSUB16_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_ssub16_a1_a_invalid_1_70_06100070() {
    // Encoding: 0x06100070
    // Test aarch32_SSUB16_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x06100070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SSUB16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_ssub16_t1_a_field_rn_0_min_f000_fad0f000() {
    // Thumb encoding (32): 0xFAD0F000
    // Test aarch32_SSUB16_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SSUB16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_ssub16_t1_a_field_rn_1_poweroftwo_f000_fad1f000() {
    // Thumb encoding (32): 0xFAD1F000
    // Test aarch32_SSUB16_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD1F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SSUB16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_ssub16_t1_a_field_rd_0_min_f000_fad0f000() {
    // Thumb encoding (32): 0xFAD0F000
    // Test aarch32_SSUB16_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SSUB16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_ssub16_t1_a_field_rd_1_poweroftwo_f000_fad0f100() {
    // Thumb encoding (32): 0xFAD0F100
    // Test aarch32_SSUB16_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F100;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SSUB16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_ssub16_t1_a_field_rm_0_min_f000_fad0f000() {
    // Thumb encoding (32): 0xFAD0F000
    // Test aarch32_SSUB16_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SSUB16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_ssub16_t1_a_field_rm_1_poweroftwo_f000_fad0f001() {
    // Thumb encoding (32): 0xFAD0F001
    // Test aarch32_SSUB16_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F001;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SSUB16_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_ssub16_t1_a_combo_0_f000_fad0f000() {
    // Thumb encoding (32): 0xFAD0F000
    // Test aarch32_SSUB16_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SSUB16_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_ssub16_t1_a_invalid_0_f000_fad0f000() {
    // Thumb encoding (32): 0xFAD0F000
    // Test aarch32_SSUB16_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_SSUB16_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_ssub16_t1_a_invalid_1_f000_fad0f000() {
    // Thumb encoding (32): 0xFAD0F000
    // Test aarch32_SSUB16_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAD0F000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_SSUB8_A Tests
// ============================================================================

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_ssub8_a1_a_field_cond_0_min_f0_061000f0() {
    // Encoding: 0x061000F0
    // Test aarch32_SSUB8_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=0
    let encoding: u32 = 0x061000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_ssub8_a1_a_field_cond_1_poweroftwo_f0_161000f0() {
    // Encoding: 0x161000F0
    // Test aarch32_SSUB8_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x161000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_ssub8_a1_a_field_cond_2_poweroftwo_f0_261000f0() {
    // Encoding: 0x261000F0
    // Test aarch32_SSUB8_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=2, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x261000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_ssub8_a1_a_field_cond_3_poweroftwo_f0_361000f0() {
    // Encoding: 0x361000F0
    // Test aarch32_SSUB8_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=3, Rn=0, Rd=0
    let encoding: u32 = 0x361000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_ssub8_a1_a_field_cond_4_poweroftwo_f0_461000f0() {
    // Encoding: 0x461000F0
    // Test aarch32_SSUB8_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=4, Rd=0, Rn=0
    let encoding: u32 = 0x461000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_ssub8_a1_a_field_cond_5_poweroftwo_f0_561000f0() {
    // Encoding: 0x561000F0
    // Test aarch32_SSUB8_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=5, Rd=0
    let encoding: u32 = 0x561000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_ssub8_a1_a_field_cond_6_poweroftwo_f0_661000f0() {
    // Encoding: 0x661000F0
    // Test aarch32_SSUB8_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=6, Rm=0, Rn=0
    let encoding: u32 = 0x661000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_ssub8_a1_a_field_cond_7_poweroftwo_f0_761000f0() {
    // Encoding: 0x761000F0
    // Test aarch32_SSUB8_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=7, Rm=0, Rn=0
    let encoding: u32 = 0x761000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_ssub8_a1_a_field_cond_8_poweroftwo_f0_861000f0() {
    // Encoding: 0x861000F0
    // Test aarch32_SSUB8_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=8, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x861000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_ssub8_a1_a_field_cond_9_poweroftwo_f0_961000f0() {
    // Encoding: 0x961000F0
    // Test aarch32_SSUB8_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=9, Rn=0, Rm=0
    let encoding: u32 = 0x961000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_ssub8_a1_a_field_cond_10_poweroftwo_f0_a61000f0() {
    // Encoding: 0xA61000F0
    // Test aarch32_SSUB8_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=10, Rn=0, Rm=0
    let encoding: u32 = 0xA61000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_ssub8_a1_a_field_cond_11_poweroftwo_f0_b61000f0() {
    // Encoding: 0xB61000F0
    // Test aarch32_SSUB8_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=11, Rd=0
    let encoding: u32 = 0xB61000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_ssub8_a1_a_field_cond_12_poweroftwo_f0_c61000f0() {
    // Encoding: 0xC61000F0
    // Test aarch32_SSUB8_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=12, Rd=0
    let encoding: u32 = 0xC61000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_ssub8_a1_a_field_cond_13_poweroftwo_f0_d61000f0() {
    // Encoding: 0xD61000F0
    // Test aarch32_SSUB8_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=13
    let encoding: u32 = 0xD61000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_ssub8_a1_a_field_cond_14_poweroftwo_f0_e61000f0() {
    // Encoding: 0xE61000F0
    // Test aarch32_SSUB8_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0xE61000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_ssub8_a1_a_field_cond_15_max_f0_f61000f0() {
    // Encoding: 0xF61000F0
    // Test aarch32_SSUB8_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xF61000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_ssub8_a1_a_field_rn_0_min_f0_061000f0() {
    // Encoding: 0x061000F0
    // Test aarch32_SSUB8_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=0
    let encoding: u32 = 0x061000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_ssub8_a1_a_field_rn_1_poweroftwo_f0_061100f0() {
    // Encoding: 0x061100F0
    // Test aarch32_SSUB8_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=0, Rn=1
    let encoding: u32 = 0x061100F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_ssub8_a1_a_field_rd_0_min_f0_061000f0() {
    // Encoding: 0x061000F0
    // Test aarch32_SSUB8_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x061000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_ssub8_a1_a_field_rd_1_poweroftwo_f0_061010f0() {
    // Encoding: 0x061010F0
    // Test aarch32_SSUB8_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=0, Rd=1
    let encoding: u32 = 0x061010F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_ssub8_a1_a_field_rm_0_min_f0_061000f0() {
    // Encoding: 0x061000F0
    // Test aarch32_SSUB8_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x061000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_ssub8_a1_a_field_rm_1_poweroftwo_f0_061000f1() {
    // Encoding: 0x061000F1
    // Test aarch32_SSUB8_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=1, cond=0
    let encoding: u32 = 0x061000F1;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_ssub8_a1_a_combo_0_f0_061000f0() {
    // Encoding: 0x061000F0
    // Test aarch32_SSUB8_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rn=0, cond=0, Rm=0, Rd=0
    let encoding: u32 = 0x061000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_ssub8_a1_a_special_cond_0_condition_eq_240_061000f0() {
    // Encoding: 0x061000F0
    // Test aarch32_SSUB8_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rd=0, cond=0, Rn=0, Rm=0
    let encoding: u32 = 0x061000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_ssub8_a1_a_special_cond_1_condition_ne_240_161000f0() {
    // Encoding: 0x161000F0
    // Test aarch32_SSUB8_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x161000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_ssub8_a1_a_special_cond_2_condition_cs_hs_240_261000f0() {
    // Encoding: 0x261000F0
    // Test aarch32_SSUB8_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=2, Rn=0
    let encoding: u32 = 0x261000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_ssub8_a1_a_special_cond_3_condition_cc_lo_240_361000f0() {
    // Encoding: 0x361000F0
    // Test aarch32_SSUB8_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=3, Rm=0
    let encoding: u32 = 0x361000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_ssub8_a1_a_special_cond_4_condition_mi_240_461000f0() {
    // Encoding: 0x461000F0
    // Test aarch32_SSUB8_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x461000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_ssub8_a1_a_special_cond_5_condition_pl_240_561000f0() {
    // Encoding: 0x561000F0
    // Test aarch32_SSUB8_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rd=0, cond=5, Rn=0, Rm=0
    let encoding: u32 = 0x561000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_ssub8_a1_a_special_cond_6_condition_vs_240_661000f0() {
    // Encoding: 0x661000F0
    // Test aarch32_SSUB8_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=6, Rd=0
    let encoding: u32 = 0x661000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_ssub8_a1_a_special_cond_7_condition_vc_240_761000f0() {
    // Encoding: 0x761000F0
    // Test aarch32_SSUB8_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=7, Rm=0
    let encoding: u32 = 0x761000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_ssub8_a1_a_special_cond_8_condition_hi_240_861000f0() {
    // Encoding: 0x861000F0
    // Test aarch32_SSUB8_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x861000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_ssub8_a1_a_special_cond_9_condition_ls_240_961000f0() {
    // Encoding: 0x961000F0
    // Test aarch32_SSUB8_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x961000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_ssub8_a1_a_special_cond_10_condition_ge_240_a61000f0() {
    // Encoding: 0xA61000F0
    // Test aarch32_SSUB8_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=10
    let encoding: u32 = 0xA61000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_ssub8_a1_a_special_cond_11_condition_lt_240_b61000f0() {
    // Encoding: 0xB61000F0
    // Test aarch32_SSUB8_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=11
    let encoding: u32 = 0xB61000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_ssub8_a1_a_special_cond_12_condition_gt_240_c61000f0() {
    // Encoding: 0xC61000F0
    // Test aarch32_SSUB8_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=12, Rn=0
    let encoding: u32 = 0xC61000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_ssub8_a1_a_special_cond_13_condition_le_240_d61000f0() {
    // Encoding: 0xD61000F0
    // Test aarch32_SSUB8_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rn=0, cond=13, Rm=0, Rd=0
    let encoding: u32 = 0xD61000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_ssub8_a1_a_special_cond_14_condition_al_240_e61000f0() {
    // Encoding: 0xE61000F0
    // Test aarch32_SSUB8_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=14
    let encoding: u32 = 0xE61000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_ssub8_a1_a_special_cond_15_condition_nv_240_f61000f0() {
    // Encoding: 0xF61000F0
    // Test aarch32_SSUB8_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rd=0, cond=15, Rn=0, Rm=0
    let encoding: u32 = 0xF61000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_ssub8_a1_a_invalid_0_f0_061000f0() {
    // Encoding: 0x061000F0
    // Test aarch32_SSUB8_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rm=0, cond=0, Rd=0, Rn=0
    let encoding: u32 = 0x061000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SSUB8_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_ssub8_a1_a_invalid_1_f0_061000f0() {
    // Encoding: 0x061000F0
    // Test aarch32_SSUB8_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: cond=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x061000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SSUB8_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_ssub8_t1_a_field_rn_0_min_f000_fac0f000() {
    // Thumb encoding (32): 0xFAC0F000
    // Test aarch32_SSUB8_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SSUB8_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_ssub8_t1_a_field_rn_1_poweroftwo_f000_fac1f000() {
    // Thumb encoding (32): 0xFAC1F000
    // Test aarch32_SSUB8_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC1F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SSUB8_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_ssub8_t1_a_field_rd_0_min_f000_fac0f000() {
    // Thumb encoding (32): 0xFAC0F000
    // Test aarch32_SSUB8_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SSUB8_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_ssub8_t1_a_field_rd_1_poweroftwo_f000_fac0f100() {
    // Thumb encoding (32): 0xFAC0F100
    // Test aarch32_SSUB8_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=1, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F100;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SSUB8_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_ssub8_t1_a_field_rm_0_min_f000_fac0f000() {
    // Thumb encoding (32): 0xFAC0F000
    // Test aarch32_SSUB8_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SSUB8_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_ssub8_t1_a_field_rm_1_poweroftwo_f000_fac0f001() {
    // Thumb encoding (32): 0xFAC0F001
    // Test aarch32_SSUB8_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F001;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SSUB8_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_ssub8_t1_a_combo_0_f000_fac0f000() {
    // Thumb encoding (32): 0xFAC0F000
    // Test aarch32_SSUB8_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SSUB8_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_ssub8_t1_a_invalid_0_f000_fac0f000() {
    // Thumb encoding (32): 0xFAC0F000
    // Test aarch32_SSUB8_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_SSUB8_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_ssub8_t1_a_invalid_1_f000_fac0f000() {
    // Thumb encoding (32): 0xFAC0F000
    // Test aarch32_SSUB8_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_UASX_A Tests
// ============================================================================

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_uasx_a1_a_field_cond_0_min_30_06500030() {
    // Encoding: 0x06500030
    // Test aarch32_UASX_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x06500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_uasx_a1_a_field_cond_1_poweroftwo_30_16500030() {
    // Encoding: 0x16500030
    // Test aarch32_UASX_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x16500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_uasx_a1_a_field_cond_2_poweroftwo_30_26500030() {
    // Encoding: 0x26500030
    // Test aarch32_UASX_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=2, Rn=0, Rd=0
    let encoding: u32 = 0x26500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_uasx_a1_a_field_cond_3_poweroftwo_30_36500030() {
    // Encoding: 0x36500030
    // Test aarch32_UASX_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x36500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_uasx_a1_a_field_cond_4_poweroftwo_30_46500030() {
    // Encoding: 0x46500030
    // Test aarch32_UASX_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=4
    let encoding: u32 = 0x46500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_uasx_a1_a_field_cond_5_poweroftwo_30_56500030() {
    // Encoding: 0x56500030
    // Test aarch32_UASX_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=5, Rm=0
    let encoding: u32 = 0x56500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_uasx_a1_a_field_cond_6_poweroftwo_30_66500030() {
    // Encoding: 0x66500030
    // Test aarch32_UASX_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x66500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_uasx_a1_a_field_cond_7_poweroftwo_30_76500030() {
    // Encoding: 0x76500030
    // Test aarch32_UASX_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=7, Rn=0, Rm=0
    let encoding: u32 = 0x76500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_uasx_a1_a_field_cond_8_poweroftwo_30_86500030() {
    // Encoding: 0x86500030
    // Test aarch32_UASX_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=8, Rm=0, Rd=0
    let encoding: u32 = 0x86500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_uasx_a1_a_field_cond_9_poweroftwo_30_96500030() {
    // Encoding: 0x96500030
    // Test aarch32_UASX_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x96500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_uasx_a1_a_field_cond_10_poweroftwo_30_a6500030() {
    // Encoding: 0xA6500030
    // Test aarch32_UASX_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xA6500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_uasx_a1_a_field_cond_11_poweroftwo_30_b6500030() {
    // Encoding: 0xB6500030
    // Test aarch32_UASX_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=11
    let encoding: u32 = 0xB6500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_uasx_a1_a_field_cond_12_poweroftwo_30_c6500030() {
    // Encoding: 0xC6500030
    // Test aarch32_UASX_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=12, Rd=0
    let encoding: u32 = 0xC6500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_uasx_a1_a_field_cond_13_poweroftwo_30_d6500030() {
    // Encoding: 0xD6500030
    // Test aarch32_UASX_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=13, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xD6500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_uasx_a1_a_field_cond_14_poweroftwo_30_e6500030() {
    // Encoding: 0xE6500030
    // Test aarch32_UASX_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=14
    let encoding: u32 = 0xE6500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_uasx_a1_a_field_cond_15_max_30_f6500030() {
    // Encoding: 0xF6500030
    // Test aarch32_UASX_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=15
    let encoding: u32 = 0xF6500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uasx_a1_a_field_rn_0_min_30_06500030() {
    // Encoding: 0x06500030
    // Test aarch32_UASX_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, cond=0, Rd=0, Rm=0
    let encoding: u32 = 0x06500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uasx_a1_a_field_rn_1_poweroftwo_30_06510030() {
    // Encoding: 0x06510030
    // Test aarch32_UASX_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=1, Rd=0
    let encoding: u32 = 0x06510030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uasx_a1_a_field_rd_0_min_30_06500030() {
    // Encoding: 0x06500030
    // Test aarch32_UASX_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=0
    let encoding: u32 = 0x06500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uasx_a1_a_field_rd_1_poweroftwo_30_06501030() {
    // Encoding: 0x06501030
    // Test aarch32_UASX_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=1, Rn=0, cond=0, Rm=0
    let encoding: u32 = 0x06501030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uasx_a1_a_field_rm_0_min_30_06500030() {
    // Encoding: 0x06500030
    // Test aarch32_UASX_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x06500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uasx_a1_a_field_rm_1_poweroftwo_30_06500031() {
    // Encoding: 0x06500031
    // Test aarch32_UASX_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=1, Rn=0, cond=0, Rd=0
    let encoding: u32 = 0x06500031;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_uasx_a1_a_combo_0_30_06500030() {
    // Encoding: 0x06500030
    // Test aarch32_UASX_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=0, Rd=0
    let encoding: u32 = 0x06500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_uasx_a1_a_special_cond_0_condition_eq_48_06500030() {
    // Encoding: 0x06500030
    // Test aarch32_UASX_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=0
    let encoding: u32 = 0x06500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_uasx_a1_a_special_cond_1_condition_ne_48_16500030() {
    // Encoding: 0x16500030
    // Test aarch32_UASX_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=1, Rd=0
    let encoding: u32 = 0x16500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_uasx_a1_a_special_cond_2_condition_cs_hs_48_26500030() {
    // Encoding: 0x26500030
    // Test aarch32_UASX_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rd=0, cond=2, Rn=0, Rm=0
    let encoding: u32 = 0x26500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_uasx_a1_a_special_cond_3_condition_cc_lo_48_36500030() {
    // Encoding: 0x36500030
    // Test aarch32_UASX_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rn=0, cond=3, Rm=0, Rd=0
    let encoding: u32 = 0x36500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_uasx_a1_a_special_cond_4_condition_mi_48_46500030() {
    // Encoding: 0x46500030
    // Test aarch32_UASX_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x46500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_uasx_a1_a_special_cond_5_condition_pl_48_56500030() {
    // Encoding: 0x56500030
    // Test aarch32_UASX_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=5, Rd=0
    let encoding: u32 = 0x56500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_uasx_a1_a_special_cond_6_condition_vs_48_66500030() {
    // Encoding: 0x66500030
    // Test aarch32_UASX_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x66500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_uasx_a1_a_special_cond_7_condition_vc_48_76500030() {
    // Encoding: 0x76500030
    // Test aarch32_UASX_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=7
    let encoding: u32 = 0x76500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_uasx_a1_a_special_cond_8_condition_hi_48_86500030() {
    // Encoding: 0x86500030
    // Test aarch32_UASX_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=8, Rd=0
    let encoding: u32 = 0x86500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_uasx_a1_a_special_cond_9_condition_ls_48_96500030() {
    // Encoding: 0x96500030
    // Test aarch32_UASX_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x96500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_uasx_a1_a_special_cond_10_condition_ge_48_a6500030() {
    // Encoding: 0xA6500030
    // Test aarch32_UASX_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rm=0, cond=10, Rn=0, Rd=0
    let encoding: u32 = 0xA6500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_uasx_a1_a_special_cond_11_condition_lt_48_b6500030() {
    // Encoding: 0xB6500030
    // Test aarch32_UASX_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=11, Rn=0
    let encoding: u32 = 0xB6500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_uasx_a1_a_special_cond_12_condition_gt_48_c6500030() {
    // Encoding: 0xC6500030
    // Test aarch32_UASX_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rd=0, cond=12, Rn=0, Rm=0
    let encoding: u32 = 0xC6500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_uasx_a1_a_special_cond_13_condition_le_48_d6500030() {
    // Encoding: 0xD6500030
    // Test aarch32_UASX_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: cond=13, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0xD6500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_uasx_a1_a_special_cond_14_condition_al_48_e6500030() {
    // Encoding: 0xE6500030
    // Test aarch32_UASX_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rn=0, cond=14, Rm=0, Rd=0
    let encoding: u32 = 0xE6500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_uasx_a1_a_special_cond_15_condition_nv_48_f6500030() {
    // Encoding: 0xF6500030
    // Test aarch32_UASX_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=15
    let encoding: u32 = 0xF6500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uasx_a1_a_invalid_0_30_06500030() {
    // Encoding: 0x06500030
    // Test aarch32_UASX_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=0
    let encoding: u32 = 0x06500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UASX_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uasx_a1_a_invalid_1_30_06500030() {
    // Encoding: 0x06500030
    // Test aarch32_UASX_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=0, Rm=0
    let encoding: u32 = 0x06500030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UASX_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uasx_t1_a_field_rn_0_min_f040_faa0f040() {
    // Thumb encoding (32): 0xFAA0F040
    // Test aarch32_UASX_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UASX_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uasx_t1_a_field_rn_1_poweroftwo_f040_faa1f040() {
    // Thumb encoding (32): 0xFAA1F040
    // Test aarch32_UASX_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rn=1, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA1F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UASX_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uasx_t1_a_field_rd_0_min_f040_faa0f040() {
    // Thumb encoding (32): 0xFAA0F040
    // Test aarch32_UASX_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UASX_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uasx_t1_a_field_rd_1_poweroftwo_f040_faa0f140() {
    // Thumb encoding (32): 0xFAA0F140
    // Test aarch32_UASX_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rd=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F140;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UASX_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uasx_t1_a_field_rm_0_min_f040_faa0f040() {
    // Thumb encoding (32): 0xFAA0F040
    // Test aarch32_UASX_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UASX_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uasx_t1_a_field_rm_1_poweroftwo_f040_faa0f041() {
    // Thumb encoding (32): 0xFAA0F041
    // Test aarch32_UASX_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rm=1, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F041;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UASX_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_uasx_t1_a_combo_0_f040_faa0f040() {
    // Thumb encoding (32): 0xFAA0F040
    // Test aarch32_UASX_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UASX_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uasx_t1_a_invalid_0_f040_faa0f040() {
    // Thumb encoding (32): 0xFAA0F040
    // Test aarch32_UASX_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F040;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_UASX_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uasx_t1_a_invalid_1_f040_faa0f040() {
    // Thumb encoding (32): 0xFAA0F040
    // Test aarch32_UASX_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F040;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_SHADD16_A Tests
// ============================================================================

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_shadd16_a1_a_field_cond_0_min_10_06300010() {
    // Encoding: 0x06300010
    // Test aarch32_SHADD16_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x06300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_shadd16_a1_a_field_cond_1_poweroftwo_10_16300010() {
    // Encoding: 0x16300010
    // Test aarch32_SHADD16_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=1, Rm=0
    let encoding: u32 = 0x16300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_shadd16_a1_a_field_cond_2_poweroftwo_10_26300010() {
    // Encoding: 0x26300010
    // Test aarch32_SHADD16_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=2
    let encoding: u32 = 0x26300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_shadd16_a1_a_field_cond_3_poweroftwo_10_36300010() {
    // Encoding: 0x36300010
    // Test aarch32_SHADD16_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=3, Rn=0, Rd=0
    let encoding: u32 = 0x36300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_shadd16_a1_a_field_cond_4_poweroftwo_10_46300010() {
    // Encoding: 0x46300010
    // Test aarch32_SHADD16_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=4, Rm=0, Rn=0
    let encoding: u32 = 0x46300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_shadd16_a1_a_field_cond_5_poweroftwo_10_56300010() {
    // Encoding: 0x56300010
    // Test aarch32_SHADD16_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=5, Rd=0
    let encoding: u32 = 0x56300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_shadd16_a1_a_field_cond_6_poweroftwo_10_66300010() {
    // Encoding: 0x66300010
    // Test aarch32_SHADD16_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x66300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_shadd16_a1_a_field_cond_7_poweroftwo_10_76300010() {
    // Encoding: 0x76300010
    // Test aarch32_SHADD16_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=7, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x76300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_shadd16_a1_a_field_cond_8_poweroftwo_10_86300010() {
    // Encoding: 0x86300010
    // Test aarch32_SHADD16_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=8, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x86300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_shadd16_a1_a_field_cond_9_poweroftwo_10_96300010() {
    // Encoding: 0x96300010
    // Test aarch32_SHADD16_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=9, Rd=0
    let encoding: u32 = 0x96300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_shadd16_a1_a_field_cond_10_poweroftwo_10_a6300010() {
    // Encoding: 0xA6300010
    // Test aarch32_SHADD16_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xA6300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_shadd16_a1_a_field_cond_11_poweroftwo_10_b6300010() {
    // Encoding: 0xB6300010
    // Test aarch32_SHADD16_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=11, Rn=0
    let encoding: u32 = 0xB6300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_shadd16_a1_a_field_cond_12_poweroftwo_10_c6300010() {
    // Encoding: 0xC6300010
    // Test aarch32_SHADD16_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0xC6300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_shadd16_a1_a_field_cond_13_poweroftwo_10_d6300010() {
    // Encoding: 0xD6300010
    // Test aarch32_SHADD16_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=13, Rm=0
    let encoding: u32 = 0xD6300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_shadd16_a1_a_field_cond_14_poweroftwo_10_e6300010() {
    // Encoding: 0xE6300010
    // Test aarch32_SHADD16_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=14
    let encoding: u32 = 0xE6300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_shadd16_a1_a_field_cond_15_max_10_f6300010() {
    // Encoding: 0xF6300010
    // Test aarch32_SHADD16_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=15, Rm=0
    let encoding: u32 = 0xF6300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shadd16_a1_a_field_rn_0_min_10_06300010() {
    // Encoding: 0x06300010
    // Test aarch32_SHADD16_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x06300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shadd16_a1_a_field_rn_1_poweroftwo_10_06310010() {
    // Encoding: 0x06310010
    // Test aarch32_SHADD16_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=0, Rn=1
    let encoding: u32 = 0x06310010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shadd16_a1_a_field_rd_0_min_10_06300010() {
    // Encoding: 0x06300010
    // Test aarch32_SHADD16_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=0
    let encoding: u32 = 0x06300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shadd16_a1_a_field_rd_1_poweroftwo_10_06301010() {
    // Encoding: 0x06301010
    // Test aarch32_SHADD16_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rd=1
    let encoding: u32 = 0x06301010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shadd16_a1_a_field_rm_0_min_10_06300010() {
    // Encoding: 0x06300010
    // Test aarch32_SHADD16_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, cond=0, Rd=0, Rm=0
    let encoding: u32 = 0x06300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shadd16_a1_a_field_rm_1_poweroftwo_10_06300011() {
    // Encoding: 0x06300011
    // Test aarch32_SHADD16_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=1, Rn=0
    let encoding: u32 = 0x06300011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_shadd16_a1_a_combo_0_10_06300010() {
    // Encoding: 0x06300010
    // Test aarch32_SHADD16_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x06300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_shadd16_a1_a_special_cond_0_condition_eq_16_06300010() {
    // Encoding: 0x06300010
    // Test aarch32_SHADD16_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rd=0
    let encoding: u32 = 0x06300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_shadd16_a1_a_special_cond_1_condition_ne_16_16300010() {
    // Encoding: 0x16300010
    // Test aarch32_SHADD16_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x16300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_shadd16_a1_a_special_cond_2_condition_cs_hs_16_26300010() {
    // Encoding: 0x26300010
    // Test aarch32_SHADD16_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=2
    let encoding: u32 = 0x26300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_shadd16_a1_a_special_cond_3_condition_cc_lo_16_36300010() {
    // Encoding: 0x36300010
    // Test aarch32_SHADD16_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=3, Rn=0
    let encoding: u32 = 0x36300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_shadd16_a1_a_special_cond_4_condition_mi_16_46300010() {
    // Encoding: 0x46300010
    // Test aarch32_SHADD16_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x46300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_shadd16_a1_a_special_cond_5_condition_pl_16_56300010() {
    // Encoding: 0x56300010
    // Test aarch32_SHADD16_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=5
    let encoding: u32 = 0x56300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_shadd16_a1_a_special_cond_6_condition_vs_16_66300010() {
    // Encoding: 0x66300010
    // Test aarch32_SHADD16_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=6, Rn=0
    let encoding: u32 = 0x66300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_shadd16_a1_a_special_cond_7_condition_vc_16_76300010() {
    // Encoding: 0x76300010
    // Test aarch32_SHADD16_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rd=0, cond=7, Rn=0, Rm=0
    let encoding: u32 = 0x76300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_shadd16_a1_a_special_cond_8_condition_hi_16_86300010() {
    // Encoding: 0x86300010
    // Test aarch32_SHADD16_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=8
    let encoding: u32 = 0x86300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_shadd16_a1_a_special_cond_9_condition_ls_16_96300010() {
    // Encoding: 0x96300010
    // Test aarch32_SHADD16_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x96300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_shadd16_a1_a_special_cond_10_condition_ge_16_a6300010() {
    // Encoding: 0xA6300010
    // Test aarch32_SHADD16_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0xA6300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_shadd16_a1_a_special_cond_11_condition_lt_16_b6300010() {
    // Encoding: 0xB6300010
    // Test aarch32_SHADD16_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=11, Rm=0
    let encoding: u32 = 0xB6300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_shadd16_a1_a_special_cond_12_condition_gt_16_c6300010() {
    // Encoding: 0xC6300010
    // Test aarch32_SHADD16_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: cond=12, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xC6300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_shadd16_a1_a_special_cond_13_condition_le_16_d6300010() {
    // Encoding: 0xD6300010
    // Test aarch32_SHADD16_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: cond=13, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0xD6300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_shadd16_a1_a_special_cond_14_condition_al_16_e6300010() {
    // Encoding: 0xE6300010
    // Test aarch32_SHADD16_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=14
    let encoding: u32 = 0xE6300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_shadd16_a1_a_special_cond_15_condition_nv_16_f6300010() {
    // Encoding: 0xF6300010
    // Test aarch32_SHADD16_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0xF6300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_shadd16_a1_a_invalid_0_10_06300010() {
    // Encoding: 0x06300010
    // Test aarch32_SHADD16_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=0, Rm=0
    let encoding: u32 = 0x06300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SHADD16_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_shadd16_a1_a_invalid_1_10_06300010() {
    // Encoding: 0x06300010
    // Test aarch32_SHADD16_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x06300010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_SHADD16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shadd16_t1_a_field_rn_0_min_f020_fa90f020() {
    // Thumb encoding (32): 0xFA90F020
    // Test aarch32_SHADD16_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHADD16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shadd16_t1_a_field_rn_1_poweroftwo_f020_fa91f020() {
    // Thumb encoding (32): 0xFA91F020
    // Test aarch32_SHADD16_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA91F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHADD16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shadd16_t1_a_field_rd_0_min_f020_fa90f020() {
    // Thumb encoding (32): 0xFA90F020
    // Test aarch32_SHADD16_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHADD16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shadd16_t1_a_field_rd_1_poweroftwo_f020_fa90f120() {
    // Thumb encoding (32): 0xFA90F120
    // Test aarch32_SHADD16_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rd=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F120;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHADD16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_shadd16_t1_a_field_rm_0_min_f020_fa90f020() {
    // Thumb encoding (32): 0xFA90F020
    // Test aarch32_SHADD16_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHADD16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_shadd16_t1_a_field_rm_1_poweroftwo_f020_fa90f021() {
    // Thumb encoding (32): 0xFA90F021
    // Test aarch32_SHADD16_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F021;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHADD16_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_shadd16_t1_a_combo_0_f020_fa90f020() {
    // Thumb encoding (32): 0xFA90F020
    // Test aarch32_SHADD16_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_SHADD16_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_shadd16_t1_a_invalid_0_f020_fa90f020() {
    // Thumb encoding (32): 0xFA90F020
    // Test aarch32_SHADD16_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F020;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_SHADD16_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_shadd16_t1_a_invalid_1_f020_fa90f020() {
    // Thumb encoding (32): 0xFA90F020
    // Test aarch32_SHADD16_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F020;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_UHASX_A Tests
// ============================================================================

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_uhasx_a1_a_field_cond_0_min_30_06700030() {
    // Encoding: 0x06700030
    // Test aarch32_UHASX_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=0, Rd=0
    let encoding: u32 = 0x06700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_uhasx_a1_a_field_cond_1_poweroftwo_30_16700030() {
    // Encoding: 0x16700030
    // Test aarch32_UHASX_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=1, Rd=0, Rm=0
    let encoding: u32 = 0x16700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_uhasx_a1_a_field_cond_2_poweroftwo_30_26700030() {
    // Encoding: 0x26700030
    // Test aarch32_UHASX_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=2, Rn=0, Rm=0
    let encoding: u32 = 0x26700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_uhasx_a1_a_field_cond_3_poweroftwo_30_36700030() {
    // Encoding: 0x36700030
    // Test aarch32_UHASX_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=3, Rd=0
    let encoding: u32 = 0x36700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_uhasx_a1_a_field_cond_4_poweroftwo_30_46700030() {
    // Encoding: 0x46700030
    // Test aarch32_UHASX_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=4
    let encoding: u32 = 0x46700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_uhasx_a1_a_field_cond_5_poweroftwo_30_56700030() {
    // Encoding: 0x56700030
    // Test aarch32_UHASX_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=5, Rn=0
    let encoding: u32 = 0x56700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_uhasx_a1_a_field_cond_6_poweroftwo_30_66700030() {
    // Encoding: 0x66700030
    // Test aarch32_UHASX_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x66700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_uhasx_a1_a_field_cond_7_poweroftwo_30_76700030() {
    // Encoding: 0x76700030
    // Test aarch32_UHASX_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=7, Rn=0
    let encoding: u32 = 0x76700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_uhasx_a1_a_field_cond_8_poweroftwo_30_86700030() {
    // Encoding: 0x86700030
    // Test aarch32_UHASX_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=8, Rm=0
    let encoding: u32 = 0x86700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_uhasx_a1_a_field_cond_9_poweroftwo_30_96700030() {
    // Encoding: 0x96700030
    // Test aarch32_UHASX_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=9
    let encoding: u32 = 0x96700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_uhasx_a1_a_field_cond_10_poweroftwo_30_a6700030() {
    // Encoding: 0xA6700030
    // Test aarch32_UHASX_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0xA6700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_uhasx_a1_a_field_cond_11_poweroftwo_30_b6700030() {
    // Encoding: 0xB6700030
    // Test aarch32_UHASX_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=11, Rd=0
    let encoding: u32 = 0xB6700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_uhasx_a1_a_field_cond_12_poweroftwo_30_c6700030() {
    // Encoding: 0xC6700030
    // Test aarch32_UHASX_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=12, Rn=0
    let encoding: u32 = 0xC6700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_uhasx_a1_a_field_cond_13_poweroftwo_30_d6700030() {
    // Encoding: 0xD6700030
    // Test aarch32_UHASX_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=13, Rd=0, Rm=0
    let encoding: u32 = 0xD6700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_uhasx_a1_a_field_cond_14_poweroftwo_30_e6700030() {
    // Encoding: 0xE6700030
    // Test aarch32_UHASX_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=14
    let encoding: u32 = 0xE6700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_uhasx_a1_a_field_cond_15_max_30_f6700030() {
    // Encoding: 0xF6700030
    // Test aarch32_UHASX_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=15, Rd=0
    let encoding: u32 = 0xF6700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhasx_a1_a_field_rn_0_min_30_06700030() {
    // Encoding: 0x06700030
    // Test aarch32_UHASX_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, cond=0, Rn=0, Rm=0
    let encoding: u32 = 0x06700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhasx_a1_a_field_rn_1_poweroftwo_30_06710030() {
    // Encoding: 0x06710030
    // Test aarch32_UHASX_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rn=1, Rd=0, Rm=0
    let encoding: u32 = 0x06710030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhasx_a1_a_field_rd_0_min_30_06700030() {
    // Encoding: 0x06700030
    // Test aarch32_UHASX_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=0
    let encoding: u32 = 0x06700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhasx_a1_a_field_rd_1_poweroftwo_30_06701030() {
    // Encoding: 0x06701030
    // Test aarch32_UHASX_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=0, Rd=1
    let encoding: u32 = 0x06701030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhasx_a1_a_field_rm_0_min_30_06700030() {
    // Encoding: 0x06700030
    // Test aarch32_UHASX_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, cond=0, Rd=0, Rm=0
    let encoding: u32 = 0x06700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhasx_a1_a_field_rm_1_poweroftwo_30_06700031() {
    // Encoding: 0x06700031
    // Test aarch32_UHASX_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=1, Rn=0
    let encoding: u32 = 0x06700031;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_uhasx_a1_a_combo_0_30_06700030() {
    // Encoding: 0x06700030
    // Test aarch32_UHASX_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=0
    let encoding: u32 = 0x06700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_uhasx_a1_a_special_cond_0_condition_eq_48_06700030() {
    // Encoding: 0x06700030
    // Test aarch32_UHASX_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=0
    let encoding: u32 = 0x06700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_uhasx_a1_a_special_cond_1_condition_ne_48_16700030() {
    // Encoding: 0x16700030
    // Test aarch32_UHASX_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=1, Rd=0
    let encoding: u32 = 0x16700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_uhasx_a1_a_special_cond_2_condition_cs_hs_48_26700030() {
    // Encoding: 0x26700030
    // Test aarch32_UHASX_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=2
    let encoding: u32 = 0x26700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_uhasx_a1_a_special_cond_3_condition_cc_lo_48_36700030() {
    // Encoding: 0x36700030
    // Test aarch32_UHASX_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x36700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_uhasx_a1_a_special_cond_4_condition_mi_48_46700030() {
    // Encoding: 0x46700030
    // Test aarch32_UHASX_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x46700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_uhasx_a1_a_special_cond_5_condition_pl_48_56700030() {
    // Encoding: 0x56700030
    // Test aarch32_UHASX_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x56700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_uhasx_a1_a_special_cond_6_condition_vs_48_66700030() {
    // Encoding: 0x66700030
    // Test aarch32_UHASX_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x66700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_uhasx_a1_a_special_cond_7_condition_vc_48_76700030() {
    // Encoding: 0x76700030
    // Test aarch32_UHASX_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=7, Rd=0
    let encoding: u32 = 0x76700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_uhasx_a1_a_special_cond_8_condition_hi_48_86700030() {
    // Encoding: 0x86700030
    // Test aarch32_UHASX_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=8
    let encoding: u32 = 0x86700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_uhasx_a1_a_special_cond_9_condition_ls_48_96700030() {
    // Encoding: 0x96700030
    // Test aarch32_UHASX_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rm=0, cond=9, Rd=0, Rn=0
    let encoding: u32 = 0x96700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_uhasx_a1_a_special_cond_10_condition_ge_48_a6700030() {
    // Encoding: 0xA6700030
    // Test aarch32_UHASX_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rn=0, cond=10, Rm=0, Rd=0
    let encoding: u32 = 0xA6700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_uhasx_a1_a_special_cond_11_condition_lt_48_b6700030() {
    // Encoding: 0xB6700030
    // Test aarch32_UHASX_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=11
    let encoding: u32 = 0xB6700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_uhasx_a1_a_special_cond_12_condition_gt_48_c6700030() {
    // Encoding: 0xC6700030
    // Test aarch32_UHASX_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=12
    let encoding: u32 = 0xC6700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_uhasx_a1_a_special_cond_13_condition_le_48_d6700030() {
    // Encoding: 0xD6700030
    // Test aarch32_UHASX_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=13
    let encoding: u32 = 0xD6700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_uhasx_a1_a_special_cond_14_condition_al_48_e6700030() {
    // Encoding: 0xE6700030
    // Test aarch32_UHASX_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=14, Rm=0
    let encoding: u32 = 0xE6700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_uhasx_a1_a_special_cond_15_condition_nv_48_f6700030() {
    // Encoding: 0xF6700030
    // Test aarch32_UHASX_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=15
    let encoding: u32 = 0xF6700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uhasx_a1_a_invalid_0_30_06700030() {
    // Encoding: 0x06700030
    // Test aarch32_UHASX_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=0
    let encoding: u32 = 0x06700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UHASX_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uhasx_a1_a_invalid_1_30_06700030() {
    // Encoding: 0x06700030
    // Test aarch32_UHASX_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x06700030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UHASX_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhasx_t1_a_field_rn_0_min_f060_faa0f060() {
    // Thumb encoding (32): 0xFAA0F060
    // Test aarch32_UHASX_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHASX_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhasx_t1_a_field_rn_1_poweroftwo_f060_faa1f060() {
    // Thumb encoding (32): 0xFAA1F060
    // Test aarch32_UHASX_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA1F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHASX_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhasx_t1_a_field_rd_0_min_f060_faa0f060() {
    // Thumb encoding (32): 0xFAA0F060
    // Test aarch32_UHASX_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHASX_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhasx_t1_a_field_rd_1_poweroftwo_f060_faa0f160() {
    // Thumb encoding (32): 0xFAA0F160
    // Test aarch32_UHASX_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F160;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHASX_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhasx_t1_a_field_rm_0_min_f060_faa0f060() {
    // Thumb encoding (32): 0xFAA0F060
    // Test aarch32_UHASX_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHASX_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhasx_t1_a_field_rm_1_poweroftwo_f060_faa0f061() {
    // Thumb encoding (32): 0xFAA0F061
    // Test aarch32_UHASX_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F061;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHASX_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_uhasx_t1_a_combo_0_f060_faa0f060() {
    // Thumb encoding (32): 0xFAA0F060
    // Test aarch32_UHASX_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHASX_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uhasx_t1_a_invalid_0_f060_faa0f060() {
    // Thumb encoding (32): 0xFAA0F060
    // Test aarch32_UHASX_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F060;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_UHASX_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uhasx_t1_a_invalid_1_f060_faa0f060() {
    // Thumb encoding (32): 0xFAA0F060
    // Test aarch32_UHASX_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAA0F060;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_UADD16_A Tests
// ============================================================================

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_uadd16_a1_a_field_cond_0_min_10_06500010() {
    // Encoding: 0x06500010
    // Test aarch32_UADD16_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=0
    let encoding: u32 = 0x06500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_uadd16_a1_a_field_cond_1_poweroftwo_10_16500010() {
    // Encoding: 0x16500010
    // Test aarch32_UADD16_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x16500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_uadd16_a1_a_field_cond_2_poweroftwo_10_26500010() {
    // Encoding: 0x26500010
    // Test aarch32_UADD16_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=2, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x26500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_uadd16_a1_a_field_cond_3_poweroftwo_10_36500010() {
    // Encoding: 0x36500010
    // Test aarch32_UADD16_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x36500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_uadd16_a1_a_field_cond_4_poweroftwo_10_46500010() {
    // Encoding: 0x46500010
    // Test aarch32_UADD16_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=4, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x46500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_uadd16_a1_a_field_cond_5_poweroftwo_10_56500010() {
    // Encoding: 0x56500010
    // Test aarch32_UADD16_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=5, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x56500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_uadd16_a1_a_field_cond_6_poweroftwo_10_66500010() {
    // Encoding: 0x66500010
    // Test aarch32_UADD16_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=6, Rn=0, Rm=0
    let encoding: u32 = 0x66500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_uadd16_a1_a_field_cond_7_poweroftwo_10_76500010() {
    // Encoding: 0x76500010
    // Test aarch32_UADD16_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=7, Rd=0, Rm=0
    let encoding: u32 = 0x76500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_uadd16_a1_a_field_cond_8_poweroftwo_10_86500010() {
    // Encoding: 0x86500010
    // Test aarch32_UADD16_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=8, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x86500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_uadd16_a1_a_field_cond_9_poweroftwo_10_96500010() {
    // Encoding: 0x96500010
    // Test aarch32_UADD16_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=9, Rm=0, Rd=0
    let encoding: u32 = 0x96500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_uadd16_a1_a_field_cond_10_poweroftwo_10_a6500010() {
    // Encoding: 0xA6500010
    // Test aarch32_UADD16_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=10, Rn=0
    let encoding: u32 = 0xA6500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_uadd16_a1_a_field_cond_11_poweroftwo_10_b6500010() {
    // Encoding: 0xB6500010
    // Test aarch32_UADD16_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=11
    let encoding: u32 = 0xB6500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_uadd16_a1_a_field_cond_12_poweroftwo_10_c6500010() {
    // Encoding: 0xC6500010
    // Test aarch32_UADD16_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=12
    let encoding: u32 = 0xC6500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_uadd16_a1_a_field_cond_13_poweroftwo_10_d6500010() {
    // Encoding: 0xD6500010
    // Test aarch32_UADD16_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=13, Rn=0, Rm=0
    let encoding: u32 = 0xD6500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_uadd16_a1_a_field_cond_14_poweroftwo_10_e6500010() {
    // Encoding: 0xE6500010
    // Test aarch32_UADD16_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=14
    let encoding: u32 = 0xE6500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_uadd16_a1_a_field_cond_15_max_10_f6500010() {
    // Encoding: 0xF6500010
    // Test aarch32_UADD16_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0xF6500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uadd16_a1_a_field_rn_0_min_10_06500010() {
    // Encoding: 0x06500010
    // Test aarch32_UADD16_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=0, Rm=0
    let encoding: u32 = 0x06500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uadd16_a1_a_field_rn_1_poweroftwo_10_06510010() {
    // Encoding: 0x06510010
    // Test aarch32_UADD16_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rm=0, Rd=0, Rn=1
    let encoding: u32 = 0x06510010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uadd16_a1_a_field_rd_0_min_10_06500010() {
    // Encoding: 0x06500010
    // Test aarch32_UADD16_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rd=0, Rn=0, cond=0
    let encoding: u32 = 0x06500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uadd16_a1_a_field_rd_1_poweroftwo_10_06501010() {
    // Encoding: 0x06501010
    // Test aarch32_UADD16_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=0, Rd=1
    let encoding: u32 = 0x06501010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uadd16_a1_a_field_rm_0_min_10_06500010() {
    // Encoding: 0x06500010
    // Test aarch32_UADD16_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, cond=0, Rn=0, Rm=0
    let encoding: u32 = 0x06500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uadd16_a1_a_field_rm_1_poweroftwo_10_06500011() {
    // Encoding: 0x06500011
    // Test aarch32_UADD16_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=1, cond=0, Rd=0
    let encoding: u32 = 0x06500011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_uadd16_a1_a_combo_0_10_06500010() {
    // Encoding: 0x06500010
    // Test aarch32_UADD16_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=0
    let encoding: u32 = 0x06500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_uadd16_a1_a_special_cond_0_condition_eq_16_06500010() {
    // Encoding: 0x06500010
    // Test aarch32_UADD16_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=0
    let encoding: u32 = 0x06500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_uadd16_a1_a_special_cond_1_condition_ne_16_16500010() {
    // Encoding: 0x16500010
    // Test aarch32_UADD16_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x16500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_uadd16_a1_a_special_cond_2_condition_cs_hs_16_26500010() {
    // Encoding: 0x26500010
    // Test aarch32_UADD16_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=2, Rm=0
    let encoding: u32 = 0x26500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_uadd16_a1_a_special_cond_3_condition_cc_lo_16_36500010() {
    // Encoding: 0x36500010
    // Test aarch32_UADD16_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x36500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_uadd16_a1_a_special_cond_4_condition_mi_16_46500010() {
    // Encoding: 0x46500010
    // Test aarch32_UADD16_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rd=0, cond=4, Rm=0, Rn=0
    let encoding: u32 = 0x46500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_uadd16_a1_a_special_cond_5_condition_pl_16_56500010() {
    // Encoding: 0x56500010
    // Test aarch32_UADD16_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rn=0, cond=5, Rd=0, Rm=0
    let encoding: u32 = 0x56500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_uadd16_a1_a_special_cond_6_condition_vs_16_66500010() {
    // Encoding: 0x66500010
    // Test aarch32_UADD16_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=6, Rm=0
    let encoding: u32 = 0x66500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_uadd16_a1_a_special_cond_7_condition_vc_16_76500010() {
    // Encoding: 0x76500010
    // Test aarch32_UADD16_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=7
    let encoding: u32 = 0x76500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_uadd16_a1_a_special_cond_8_condition_hi_16_86500010() {
    // Encoding: 0x86500010
    // Test aarch32_UADD16_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rm=0, Rn=0, cond=8, Rd=0
    let encoding: u32 = 0x86500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_uadd16_a1_a_special_cond_9_condition_ls_16_96500010() {
    // Encoding: 0x96500010
    // Test aarch32_UADD16_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=9, Rm=0
    let encoding: u32 = 0x96500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_uadd16_a1_a_special_cond_10_condition_ge_16_a6500010() {
    // Encoding: 0xA6500010
    // Test aarch32_UADD16_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=10, Rm=0
    let encoding: u32 = 0xA6500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_uadd16_a1_a_special_cond_11_condition_lt_16_b6500010() {
    // Encoding: 0xB6500010
    // Test aarch32_UADD16_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=11
    let encoding: u32 = 0xB6500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_uadd16_a1_a_special_cond_12_condition_gt_16_c6500010() {
    // Encoding: 0xC6500010
    // Test aarch32_UADD16_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=12, Rn=0
    let encoding: u32 = 0xC6500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_uadd16_a1_a_special_cond_13_condition_le_16_d6500010() {
    // Encoding: 0xD6500010
    // Test aarch32_UADD16_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rn=0, cond=13, Rm=0, Rd=0
    let encoding: u32 = 0xD6500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_uadd16_a1_a_special_cond_14_condition_al_16_e6500010() {
    // Encoding: 0xE6500010
    // Test aarch32_UADD16_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rn=0, Rm=0, cond=14, Rd=0
    let encoding: u32 = 0xE6500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_uadd16_a1_a_special_cond_15_condition_nv_16_f6500010() {
    // Encoding: 0xF6500010
    // Test aarch32_UADD16_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xF6500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uadd16_a1_a_invalid_0_10_06500010() {
    // Encoding: 0x06500010
    // Test aarch32_UADD16_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=0, Rd=0
    let encoding: u32 = 0x06500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UADD16_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uadd16_a1_a_invalid_1_10_06500010() {
    // Encoding: 0x06500010
    // Test aarch32_UADD16_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x06500010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UADD16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uadd16_t1_a_field_rn_0_min_f040_fa90f040() {
    // Thumb encoding (32): 0xFA90F040
    // Test aarch32_UADD16_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UADD16_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uadd16_t1_a_field_rn_1_poweroftwo_f040_fa91f040() {
    // Thumb encoding (32): 0xFA91F040
    // Test aarch32_UADD16_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA91F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UADD16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uadd16_t1_a_field_rd_0_min_f040_fa90f040() {
    // Thumb encoding (32): 0xFA90F040
    // Test aarch32_UADD16_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UADD16_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uadd16_t1_a_field_rd_1_poweroftwo_f040_fa90f140() {
    // Thumb encoding (32): 0xFA90F140
    // Test aarch32_UADD16_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rd=1, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F140;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UADD16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uadd16_t1_a_field_rm_0_min_f040_fa90f040() {
    // Thumb encoding (32): 0xFA90F040
    // Test aarch32_UADD16_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UADD16_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uadd16_t1_a_field_rm_1_poweroftwo_f040_fa90f041() {
    // Thumb encoding (32): 0xFA90F041
    // Test aarch32_UADD16_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F041;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UADD16_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_uadd16_t1_a_combo_0_f040_fa90f040() {
    // Thumb encoding (32): 0xFA90F040
    // Test aarch32_UADD16_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UADD16_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uadd16_t1_a_invalid_0_f040_fa90f040() {
    // Thumb encoding (32): 0xFA90F040
    // Test aarch32_UADD16_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F040;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_UADD16_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uadd16_t1_a_invalid_1_f040_fa90f040() {
    // Thumb encoding (32): 0xFA90F040
    // Test aarch32_UADD16_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA90F040;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_UHSUB8_A Tests
// ============================================================================

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_uhsub8_a1_a_field_cond_0_min_f0_067000f0() {
    // Encoding: 0x067000F0
    // Test aarch32_UHSUB8_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, Rm=0, Rn=0, cond=0
    let encoding: u32 = 0x067000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_uhsub8_a1_a_field_cond_1_poweroftwo_f0_167000f0() {
    // Encoding: 0x167000F0
    // Test aarch32_UHSUB8_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=1, Rn=0
    let encoding: u32 = 0x167000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_uhsub8_a1_a_field_cond_2_poweroftwo_f0_267000f0() {
    // Encoding: 0x267000F0
    // Test aarch32_UHSUB8_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=2, Rn=0, Rm=0
    let encoding: u32 = 0x267000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_uhsub8_a1_a_field_cond_3_poweroftwo_f0_367000f0() {
    // Encoding: 0x367000F0
    // Test aarch32_UHSUB8_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=3
    let encoding: u32 = 0x367000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_uhsub8_a1_a_field_cond_4_poweroftwo_f0_467000f0() {
    // Encoding: 0x467000F0
    // Test aarch32_UHSUB8_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=4, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x467000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_uhsub8_a1_a_field_cond_5_poweroftwo_f0_567000f0() {
    // Encoding: 0x567000F0
    // Test aarch32_UHSUB8_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=5, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x567000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_uhsub8_a1_a_field_cond_6_poweroftwo_f0_667000f0() {
    // Encoding: 0x667000F0
    // Test aarch32_UHSUB8_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=6, Rd=0, Rm=0
    let encoding: u32 = 0x667000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_uhsub8_a1_a_field_cond_7_poweroftwo_f0_767000f0() {
    // Encoding: 0x767000F0
    // Test aarch32_UHSUB8_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=7, Rn=0, Rd=0
    let encoding: u32 = 0x767000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_uhsub8_a1_a_field_cond_8_poweroftwo_f0_867000f0() {
    // Encoding: 0x867000F0
    // Test aarch32_UHSUB8_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=8, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x867000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_uhsub8_a1_a_field_cond_9_poweroftwo_f0_967000f0() {
    // Encoding: 0x967000F0
    // Test aarch32_UHSUB8_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=9, Rd=0, Rn=0
    let encoding: u32 = 0x967000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_uhsub8_a1_a_field_cond_10_poweroftwo_f0_a67000f0() {
    // Encoding: 0xA67000F0
    // Test aarch32_UHSUB8_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rn=0, Rd=0, cond=10
    let encoding: u32 = 0xA67000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_uhsub8_a1_a_field_cond_11_poweroftwo_f0_b67000f0() {
    // Encoding: 0xB67000F0
    // Test aarch32_UHSUB8_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=11, Rd=0, Rn=0
    let encoding: u32 = 0xB67000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_uhsub8_a1_a_field_cond_12_poweroftwo_f0_c67000f0() {
    // Encoding: 0xC67000F0
    // Test aarch32_UHSUB8_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=12, Rm=0, Rd=0
    let encoding: u32 = 0xC67000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_uhsub8_a1_a_field_cond_13_poweroftwo_f0_d67000f0() {
    // Encoding: 0xD67000F0
    // Test aarch32_UHSUB8_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=13, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0xD67000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_uhsub8_a1_a_field_cond_14_poweroftwo_f0_e67000f0() {
    // Encoding: 0xE67000F0
    // Test aarch32_UHSUB8_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rd=0, cond=14, Rm=0
    let encoding: u32 = 0xE67000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_uhsub8_a1_a_field_cond_15_max_f0_f67000f0() {
    // Encoding: 0xF67000F0
    // Test aarch32_UHSUB8_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0xF67000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhsub8_a1_a_field_rn_0_min_f0_067000f0() {
    // Encoding: 0x067000F0
    // Test aarch32_UHSUB8_A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, Rm=0, Rd=0, cond=0
    let encoding: u32 = 0x067000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhsub8_a1_a_field_rn_1_poweroftwo_f0_067100f0() {
    // Encoding: 0x067100F0
    // Test aarch32_UHSUB8_A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rn=1, Rd=0
    let encoding: u32 = 0x067100F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhsub8_a1_a_field_rd_0_min_f0_067000f0() {
    // Encoding: 0x067000F0
    // Test aarch32_UHSUB8_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x067000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhsub8_a1_a_field_rd_1_poweroftwo_f0_067010f0() {
    // Encoding: 0x067010F0
    // Test aarch32_UHSUB8_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=1, Rn=0, Rm=0, cond=0
    let encoding: u32 = 0x067010F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhsub8_a1_a_field_rm_0_min_f0_067000f0() {
    // Encoding: 0x067000F0
    // Test aarch32_UHSUB8_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=0, Rn=0
    let encoding: u32 = 0x067000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhsub8_a1_a_field_rm_1_poweroftwo_f0_067000f1() {
    // Encoding: 0x067000F1
    // Test aarch32_UHSUB8_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, Rm=1, cond=0, Rd=0
    let encoding: u32 = 0x067000F1;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_uhsub8_a1_a_combo_0_f0_067000f0() {
    // Encoding: 0x067000F0
    // Test aarch32_UHSUB8_A1_A field combination: cond=0, Rn=0, Rd=0, Rm=0
    // ISET: A32
    // Fields: cond=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x067000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_uhsub8_a1_a_special_cond_0_condition_eq_240_067000f0() {
    // Encoding: 0x067000F0
    // Test aarch32_UHSUB8_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: cond=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x067000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_uhsub8_a1_a_special_cond_1_condition_ne_240_167000f0() {
    // Encoding: 0x167000F0
    // Test aarch32_UHSUB8_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x167000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_uhsub8_a1_a_special_cond_2_condition_cs_hs_240_267000f0() {
    // Encoding: 0x267000F0
    // Test aarch32_UHSUB8_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x267000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_uhsub8_a1_a_special_cond_3_condition_cc_lo_240_367000f0() {
    // Encoding: 0x367000F0
    // Test aarch32_UHSUB8_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=3
    let encoding: u32 = 0x367000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_uhsub8_a1_a_special_cond_4_condition_mi_240_467000f0() {
    // Encoding: 0x467000F0
    // Test aarch32_UHSUB8_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rn=0, cond=4, Rm=0, Rd=0
    let encoding: u32 = 0x467000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_uhsub8_a1_a_special_cond_5_condition_pl_240_567000f0() {
    // Encoding: 0x567000F0
    // Test aarch32_UHSUB8_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rm=0, cond=5, Rn=0, Rd=0
    let encoding: u32 = 0x567000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_uhsub8_a1_a_special_cond_6_condition_vs_240_667000f0() {
    // Encoding: 0x667000F0
    // Test aarch32_UHSUB8_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0x667000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_uhsub8_a1_a_special_cond_7_condition_vc_240_767000f0() {
    // Encoding: 0x767000F0
    // Test aarch32_UHSUB8_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=7, Rm=0
    let encoding: u32 = 0x767000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_uhsub8_a1_a_special_cond_8_condition_hi_240_867000f0() {
    // Encoding: 0x867000F0
    // Test aarch32_UHSUB8_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rn=0, Rd=0, Rm=0, cond=8
    let encoding: u32 = 0x867000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_uhsub8_a1_a_special_cond_9_condition_ls_240_967000f0() {
    // Encoding: 0x967000F0
    // Test aarch32_UHSUB8_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x967000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_uhsub8_a1_a_special_cond_10_condition_ge_240_a67000f0() {
    // Encoding: 0xA67000F0
    // Test aarch32_UHSUB8_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0xA67000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_uhsub8_a1_a_special_cond_11_condition_lt_240_b67000f0() {
    // Encoding: 0xB67000F0
    // Test aarch32_UHSUB8_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0xB67000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_uhsub8_a1_a_special_cond_12_condition_gt_240_c67000f0() {
    // Encoding: 0xC67000F0
    // Test aarch32_UHSUB8_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rd=0, Rn=0, Rm=0, cond=12
    let encoding: u32 = 0xC67000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_uhsub8_a1_a_special_cond_13_condition_le_240_d67000f0() {
    // Encoding: 0xD67000F0
    // Test aarch32_UHSUB8_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rm=0, cond=13, Rn=0, Rd=0
    let encoding: u32 = 0xD67000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_uhsub8_a1_a_special_cond_14_condition_al_240_e67000f0() {
    // Encoding: 0xE67000F0
    // Test aarch32_UHSUB8_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rm=0, cond=14, Rd=0, Rn=0
    let encoding: u32 = 0xE67000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_uhsub8_a1_a_special_cond_15_condition_nv_240_f67000f0() {
    // Encoding: 0xF67000F0
    // Test aarch32_UHSUB8_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, Rm=0, Rn=0, Rd=0
    let encoding: u32 = 0xF67000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uhsub8_a1_a_invalid_0_f0_067000f0() {
    // Encoding: 0x067000F0
    // Test aarch32_UHSUB8_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rd=0, Rn=0, cond=0, Rm=0
    let encoding: u32 = 0x067000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UHSUB8_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uhsub8_a1_a_invalid_1_f0_067000f0() {
    // Encoding: 0x067000F0
    // Test aarch32_UHSUB8_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rd=0, cond=0, Rm=0, Rn=0
    let encoding: u32 = 0x067000F0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_UHSUB8_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhsub8_t1_a_field_rn_0_min_f060_fac0f060() {
    // Thumb encoding (32): 0xFAC0F060
    // Test aarch32_UHSUB8_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHSUB8_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhsub8_t1_a_field_rn_1_poweroftwo_f060_fac1f060() {
    // Thumb encoding (32): 0xFAC1F060
    // Test aarch32_UHSUB8_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC1F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHSUB8_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhsub8_t1_a_field_rd_0_min_f060_fac0f060() {
    // Thumb encoding (32): 0xFAC0F060
    // Test aarch32_UHSUB8_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHSUB8_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhsub8_t1_a_field_rd_1_poweroftwo_f060_fac0f160() {
    // Thumb encoding (32): 0xFAC0F160
    // Test aarch32_UHSUB8_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=1, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F160;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHSUB8_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_uhsub8_t1_a_field_rm_0_min_f060_fac0f060() {
    // Thumb encoding (32): 0xFAC0F060
    // Test aarch32_UHSUB8_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHSUB8_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_uhsub8_t1_a_field_rm_1_poweroftwo_f060_fac0f061() {
    // Thumb encoding (32): 0xFAC0F061
    // Test aarch32_UHSUB8_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F061;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHSUB8_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch32_uhsub8_t1_a_combo_0_f060_fac0f060() {
    // Thumb encoding (32): 0xFAC0F060
    // Test aarch32_UHSUB8_T1_A field combination: Rn=0, Rd=0, Rm=0
    // ISET: T32
    // Fields: Rm=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F060;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_UHSUB8_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uhsub8_t1_a_invalid_0_f060_fac0f060() {
    // Thumb encoding (32): 0xFAC0F060
    // Test aarch32_UHSUB8_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rn=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F060;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_UHSUB8_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_uhsub8_t1_a_invalid_1_f060_fac0f060() {
    // Thumb encoding (32): 0xFAC0F060
    // Test aarch32_UHSUB8_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFAC0F060;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}
