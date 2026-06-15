//! A32 data_processing move_ tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers_32::*;

// ============================================================================
// aarch32_MVN_i_A Tests
// ============================================================================

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_mvn_i_a1_a_field_cond_0_min_0_03e00000() {
    // Encoding: 0x03E00000
    // Test aarch32_MVN_i_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: imm12=0, cond=0, S=0, Rd=0
    let encoding: u32 = 0x03E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_mvn_i_a1_a_field_cond_1_poweroftwo_0_13e00000() {
    // Encoding: 0x13E00000
    // Test aarch32_MVN_i_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, Rd=0, imm12=0, S=0
    let encoding: u32 = 0x13E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_mvn_i_a1_a_field_cond_2_poweroftwo_0_23e00000() {
    // Encoding: 0x23E00000
    // Test aarch32_MVN_i_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, Rd=0, imm12=0, cond=2
    let encoding: u32 = 0x23E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_mvn_i_a1_a_field_cond_3_poweroftwo_0_33e00000() {
    // Encoding: 0x33E00000
    // Test aarch32_MVN_i_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, S=0, Rd=0, cond=3
    let encoding: u32 = 0x33E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_mvn_i_a1_a_field_cond_4_poweroftwo_0_43e00000() {
    // Encoding: 0x43E00000
    // Test aarch32_MVN_i_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=4, imm12=0, S=0, Rd=0
    let encoding: u32 = 0x43E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_mvn_i_a1_a_field_cond_5_poweroftwo_0_53e00000() {
    // Encoding: 0x53E00000
    // Test aarch32_MVN_i_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, cond=5, S=0, Rd=0
    let encoding: u32 = 0x53E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_mvn_i_a1_a_field_cond_6_poweroftwo_0_63e00000() {
    // Encoding: 0x63E00000
    // Test aarch32_MVN_i_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, cond=6, imm12=0, Rd=0
    let encoding: u32 = 0x63E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_mvn_i_a1_a_field_cond_7_poweroftwo_0_73e00000() {
    // Encoding: 0x73E00000
    // Test aarch32_MVN_i_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=7, S=0, imm12=0, Rd=0
    let encoding: u32 = 0x73E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_mvn_i_a1_a_field_cond_8_poweroftwo_0_83e00000() {
    // Encoding: 0x83E00000
    // Test aarch32_MVN_i_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, cond=8, S=0, Rd=0
    let encoding: u32 = 0x83E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_mvn_i_a1_a_field_cond_9_poweroftwo_0_93e00000() {
    // Encoding: 0x93E00000
    // Test aarch32_MVN_i_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=9, imm12=0, S=0
    let encoding: u32 = 0x93E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_mvn_i_a1_a_field_cond_10_poweroftwo_0_a3e00000() {
    // Encoding: 0xA3E00000
    // Test aarch32_MVN_i_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, S=0, imm12=0, cond=10
    let encoding: u32 = 0xA3E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_mvn_i_a1_a_field_cond_11_poweroftwo_0_b3e00000() {
    // Encoding: 0xB3E00000
    // Test aarch32_MVN_i_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, cond=11, Rd=0, S=0
    let encoding: u32 = 0xB3E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_mvn_i_a1_a_field_cond_12_poweroftwo_0_c3e00000() {
    // Encoding: 0xC3E00000
    // Test aarch32_MVN_i_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, S=0, imm12=0, cond=12
    let encoding: u32 = 0xC3E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_mvn_i_a1_a_field_cond_13_poweroftwo_0_d3e00000() {
    // Encoding: 0xD3E00000
    // Test aarch32_MVN_i_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, cond=13, imm12=0, Rd=0
    let encoding: u32 = 0xD3E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_mvn_i_a1_a_field_cond_14_poweroftwo_0_e3e00000() {
    // Encoding: 0xE3E00000
    // Test aarch32_MVN_i_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=14, S=0, imm12=0
    let encoding: u32 = 0xE3E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_mvn_i_a1_a_field_cond_15_max_0_f3e00000() {
    // Encoding: 0xF3E00000
    // Test aarch32_MVN_i_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: imm12=0, S=0, cond=15, Rd=0
    let encoding: u32 = 0xF3E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_mvn_i_a1_a_field_s_0_min_0_03e00000() {
    // Encoding: 0x03E00000
    // Test aarch32_MVN_i_A1_A field S = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, cond=0, S=0, imm12=0
    let encoding: u32 = 0x03E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_mvn_i_a1_a_field_s_1_max_0_03f00000() {
    // Encoding: 0x03F00000
    // Test aarch32_MVN_i_A1_A field S = 1 (Max)
    // ISET: A32
    // Fields: Rd=0, S=1, cond=0, imm12=0
    let encoding: u32 = 0x03F00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mvn_i_a1_a_field_rd_0_min_0_03e00000() {
    // Encoding: 0x03E00000
    // Test aarch32_MVN_i_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, S=0, cond=0, imm12=0
    let encoding: u32 = 0x03E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mvn_i_a1_a_field_rd_1_poweroftwo_0_03e01000() {
    // Encoding: 0x03E01000
    // Test aarch32_MVN_i_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, imm12=0, Rd=1, cond=0
    let encoding: u32 = 0x03E01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_mvn_i_a1_a_field_imm12_0_zero_0_03e00000() {
    // Encoding: 0x03E00000
    // Test aarch32_MVN_i_A1_A field imm12 = 0 (Zero)
    // ISET: A32
    // Fields: cond=0, Rd=0, S=0, imm12=0
    let encoding: u32 = 0x03E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_mvn_i_a1_a_field_imm12_1_poweroftwo_0_03e00001() {
    // Encoding: 0x03E00001
    // Test aarch32_MVN_i_A1_A field imm12 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, S=0, Rd=0, imm12=1
    let encoding: u32 = 0x03E00001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_mvn_i_a1_a_field_imm12_3_poweroftwominusone_0_03e00003() {
    // Encoding: 0x03E00003
    // Test aarch32_MVN_i_A1_A field imm12 = 3 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rd=0, imm12=3, S=0, cond=0
    let encoding: u32 = 0x03E00003;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_mvn_i_a1_a_field_imm12_4_poweroftwo_0_03e00004() {
    // Encoding: 0x03E00004
    // Test aarch32_MVN_i_A1_A field imm12 = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, S=0, Rd=0, imm12=4
    let encoding: u32 = 0x03E00004;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_mvn_i_a1_a_field_imm12_7_poweroftwominusone_0_03e00007() {
    // Encoding: 0x03E00007
    // Test aarch32_MVN_i_A1_A field imm12 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rd=0, imm12=7, S=0, cond=0
    let encoding: u32 = 0x03E00007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_mvn_i_a1_a_field_imm12_8_poweroftwo_0_03e00008() {
    // Encoding: 0x03E00008
    // Test aarch32_MVN_i_A1_A field imm12 = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, cond=0, Rd=0, imm12=8
    let encoding: u32 = 0x03E00008;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_mvn_i_a1_a_field_imm12_15_poweroftwominusone_0_03e0000f() {
    // Encoding: 0x03E0000F
    // Test aarch32_MVN_i_A1_A field imm12 = 15 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: S=0, Rd=0, imm12=15, cond=0
    let encoding: u32 = 0x03E0000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_mvn_i_a1_a_field_imm12_16_poweroftwo_0_03e00010() {
    // Encoding: 0x03E00010
    // Test aarch32_MVN_i_A1_A field imm12 = 16 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, S=0, imm12=16, cond=0
    let encoding: u32 = 0x03E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_mvn_i_a1_a_field_imm12_31_poweroftwominusone_0_03e0001f() {
    // Encoding: 0x03E0001F
    // Test aarch32_MVN_i_A1_A field imm12 = 31 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, Rd=0, imm12=31, S=0
    let encoding: u32 = 0x03E0001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_mvn_i_a1_a_field_imm12_32_poweroftwo_0_03e00020() {
    // Encoding: 0x03E00020
    // Test aarch32_MVN_i_A1_A field imm12 = 32 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=0, S=0, imm12=32
    let encoding: u32 = 0x03E00020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_mvn_i_a1_a_field_imm12_63_poweroftwominusone_0_03e0003f() {
    // Encoding: 0x03E0003F
    // Test aarch32_MVN_i_A1_A field imm12 = 63 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, Rd=0, imm12=63, S=0
    let encoding: u32 = 0x03E0003F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_mvn_i_a1_a_field_imm12_64_poweroftwo_0_03e00040() {
    // Encoding: 0x03E00040
    // Test aarch32_MVN_i_A1_A field imm12 = 64 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, imm12=64, cond=0, S=0
    let encoding: u32 = 0x03E00040;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch32_mvn_i_a1_a_field_imm12_127_poweroftwominusone_0_03e0007f() {
    // Encoding: 0x03E0007F
    // Test aarch32_MVN_i_A1_A field imm12 = 127 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: S=0, Rd=0, cond=0, imm12=127
    let encoding: u32 = 0x03E0007F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_mvn_i_a1_a_field_imm12_128_poweroftwo_0_03e00080() {
    // Encoding: 0x03E00080
    // Test aarch32_MVN_i_A1_A field imm12 = 128 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm12=128, S=0, Rd=0
    let encoding: u32 = 0x03E00080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch32_mvn_i_a1_a_field_imm12_255_poweroftwominusone_0_03e000ff() {
    // Encoding: 0x03E000FF
    // Test aarch32_MVN_i_A1_A field imm12 = 255 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rd=0, cond=0, S=0, imm12=255
    let encoding: u32 = 0x03E000FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch32_mvn_i_a1_a_field_imm12_256_poweroftwo_0_03e00100() {
    // Encoding: 0x03E00100
    // Test aarch32_MVN_i_A1_A field imm12 = 256 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, imm12=256, cond=0, Rd=0
    let encoding: u32 = 0x03E00100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch32_mvn_i_a1_a_field_imm12_511_poweroftwominusone_0_03e001ff() {
    // Encoding: 0x03E001FF
    // Test aarch32_MVN_i_A1_A field imm12 = 511 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rd=0, S=0, cond=0, imm12=511
    let encoding: u32 = 0x03E001FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch32_mvn_i_a1_a_field_imm12_512_poweroftwo_0_03e00200() {
    // Encoding: 0x03E00200
    // Test aarch32_MVN_i_A1_A field imm12 = 512 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=512, S=0, cond=0, Rd=0
    let encoding: u32 = 0x03E00200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1023, boundary: PowerOfTwoMinusOne }
/// 2^10 - 1 = 1023
#[test]
fn test_aarch32_mvn_i_a1_a_field_imm12_1023_poweroftwominusone_0_03e003ff() {
    // Encoding: 0x03E003FF
    // Test aarch32_MVN_i_A1_A field imm12 = 1023 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: S=0, Rd=0, imm12=1023, cond=0
    let encoding: u32 = 0x03E003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch32_mvn_i_a1_a_field_imm12_1024_poweroftwo_0_03e00400() {
    // Encoding: 0x03E00400
    // Test aarch32_MVN_i_A1_A field imm12 = 1024 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, S=0, imm12=1024, Rd=0
    let encoding: u32 = 0x03E00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 2047, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (2047)
#[test]
fn test_aarch32_mvn_i_a1_a_field_imm12_2047_poweroftwominusone_0_03e007ff() {
    // Encoding: 0x03E007FF
    // Test aarch32_MVN_i_A1_A field imm12 = 2047 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: S=0, cond=0, Rd=0, imm12=2047
    let encoding: u32 = 0x03E007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 2048, boundary: PowerOfTwo }
/// power of 2 (2^11 = 2048)
#[test]
fn test_aarch32_mvn_i_a1_a_field_imm12_2048_poweroftwo_0_03e00800() {
    // Encoding: 0x03E00800
    // Test aarch32_MVN_i_A1_A field imm12 = 2048 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, cond=0, imm12=2048, Rd=0
    let encoding: u32 = 0x03E00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 4095, boundary: Max }
/// maximum immediate (4095)
#[test]
fn test_aarch32_mvn_i_a1_a_field_imm12_4095_max_0_03e00fff() {
    // Encoding: 0x03E00FFF
    // Test aarch32_MVN_i_A1_A field imm12 = 4095 (Max)
    // ISET: A32
    // Fields: S=0, cond=0, Rd=0, imm12=4095
    let encoding: u32 = 0x03E00FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_mvn_i_a1_a_combo_0_0_03e00000() {
    // Encoding: 0x03E00000
    // Test aarch32_MVN_i_A1_A field combination: cond=0, S=0, Rd=0, imm12=0
    // ISET: A32
    // Fields: Rd=0, S=0, cond=0, imm12=0
    let encoding: u32 = 0x03E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_mvn_i_a1_a_special_cond_0_condition_eq_0_03e00000() {
    // Encoding: 0x03E00000
    // Test aarch32_MVN_i_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rd=0, imm12=0, cond=0, S=0
    let encoding: u32 = 0x03E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_mvn_i_a1_a_special_cond_1_condition_ne_0_13e00000() {
    // Encoding: 0x13E00000
    // Test aarch32_MVN_i_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: S=0, Rd=0, imm12=0, cond=1
    let encoding: u32 = 0x13E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_mvn_i_a1_a_special_cond_2_condition_cs_hs_0_23e00000() {
    // Encoding: 0x23E00000
    // Test aarch32_MVN_i_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: S=0, Rd=0, cond=2, imm12=0
    let encoding: u32 = 0x23E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_mvn_i_a1_a_special_cond_3_condition_cc_lo_0_33e00000() {
    // Encoding: 0x33E00000
    // Test aarch32_MVN_i_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: S=0, cond=3, Rd=0, imm12=0
    let encoding: u32 = 0x33E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_mvn_i_a1_a_special_cond_4_condition_mi_0_43e00000() {
    // Encoding: 0x43E00000
    // Test aarch32_MVN_i_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: S=0, Rd=0, imm12=0, cond=4
    let encoding: u32 = 0x43E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_mvn_i_a1_a_special_cond_5_condition_pl_0_53e00000() {
    // Encoding: 0x53E00000
    // Test aarch32_MVN_i_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rd=0, S=0, cond=5, imm12=0
    let encoding: u32 = 0x53E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_mvn_i_a1_a_special_cond_6_condition_vs_0_63e00000() {
    // Encoding: 0x63E00000
    // Test aarch32_MVN_i_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: S=0, Rd=0, cond=6, imm12=0
    let encoding: u32 = 0x63E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_mvn_i_a1_a_special_cond_7_condition_vc_0_73e00000() {
    // Encoding: 0x73E00000
    // Test aarch32_MVN_i_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rd=0, S=0, imm12=0, cond=7
    let encoding: u32 = 0x73E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_mvn_i_a1_a_special_cond_8_condition_hi_0_83e00000() {
    // Encoding: 0x83E00000
    // Test aarch32_MVN_i_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: imm12=0, Rd=0, cond=8, S=0
    let encoding: u32 = 0x83E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_mvn_i_a1_a_special_cond_9_condition_ls_0_93e00000() {
    // Encoding: 0x93E00000
    // Test aarch32_MVN_i_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: imm12=0, Rd=0, cond=9, S=0
    let encoding: u32 = 0x93E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_mvn_i_a1_a_special_cond_10_condition_ge_0_a3e00000() {
    // Encoding: 0xA3E00000
    // Test aarch32_MVN_i_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rd=0, imm12=0, cond=10, S=0
    let encoding: u32 = 0xA3E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_mvn_i_a1_a_special_cond_11_condition_lt_0_b3e00000() {
    // Encoding: 0xB3E00000
    // Test aarch32_MVN_i_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, Rd=0, S=0, imm12=0
    let encoding: u32 = 0xB3E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_mvn_i_a1_a_special_cond_12_condition_gt_0_c3e00000() {
    // Encoding: 0xC3E00000
    // Test aarch32_MVN_i_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: cond=12, Rd=0, imm12=0, S=0
    let encoding: u32 = 0xC3E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_mvn_i_a1_a_special_cond_13_condition_le_0_d3e00000() {
    // Encoding: 0xD3E00000
    // Test aarch32_MVN_i_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: S=0, imm12=0, cond=13, Rd=0
    let encoding: u32 = 0xD3E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_mvn_i_a1_a_special_cond_14_condition_al_0_e3e00000() {
    // Encoding: 0xE3E00000
    // Test aarch32_MVN_i_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: imm12=0, Rd=0, cond=14, S=0
    let encoding: u32 = 0xE3E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_mvn_i_a1_a_special_cond_15_condition_nv_0_f3e00000() {
    // Encoding: 0xF3E00000
    // Test aarch32_MVN_i_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, imm12=0, Rd=0, S=0
    let encoding: u32 = 0xF3E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_mvn_i_a1_a_special_s_0_size_variant_0_0_03e00000() {
    // Encoding: 0x03E00000
    // Test aarch32_MVN_i_A1_A special value S = 0 (Size variant 0)
    // ISET: A32
    // Fields: imm12=0, cond=0, Rd=0, S=0
    let encoding: u32 = 0x03E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_mvn_i_a1_a_special_s_1_size_variant_1_0_03f00000() {
    // Encoding: 0x03F00000
    // Test aarch32_MVN_i_A1_A special value S = 1 (Size variant 1)
    // ISET: A32
    // Fields: S=1, Rd=0, imm12=0, cond=0
    let encoding: u32 = 0x03F00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field i 26 +: 1`
/// Requirement: FieldBoundary { field: "i", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mvn_i_t1_a_field_i_0_min_0_f06f0000() {
    // Thumb encoding (32): 0xF06F0000
    // Test aarch32_MVN_i_T1_A field i = 0 (Min)
    // ISET: T32
    // Fields: i=0, imm3=0, Rd=0, imm8=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field i 26 +: 1`
/// Requirement: FieldBoundary { field: "i", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_mvn_i_t1_a_field_i_1_max_0_f46f0000() {
    // Thumb encoding (32): 0xF46F0000
    // Test aarch32_MVN_i_T1_A field i = 1 (Max)
    // ISET: T32
    // Fields: Rd=0, imm3=0, i=1, S=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF46F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_mvn_i_t1_a_field_s_0_min_0_f06f0000() {
    // Thumb encoding (32): 0xF06F0000
    // Test aarch32_MVN_i_T1_A field S = 0 (Min)
    // ISET: T32
    // Fields: i=0, S=0, imm8=0, Rd=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_mvn_i_t1_a_field_s_1_max_0_f07f0000() {
    // Thumb encoding (32): 0xF07F0000
    // Test aarch32_MVN_i_T1_A field S = 1 (Max)
    // ISET: T32
    // Fields: S=1, Rd=0, i=0, imm3=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF07F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_mvn_i_t1_a_field_imm3_0_zero_0_f06f0000() {
    // Thumb encoding (32): 0xF06F0000
    // Test aarch32_MVN_i_T1_A field imm3 = 0 (Zero)
    // ISET: T32
    // Fields: i=0, S=0, Rd=0, imm8=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_mvn_i_t1_a_field_imm3_1_poweroftwo_0_f06f1000() {
    // Thumb encoding (32): 0xF06F1000
    // Test aarch32_MVN_i_T1_A field imm3 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=0, imm3=1, Rd=0, S=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F1000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch32_mvn_i_t1_a_field_imm3_3_poweroftwominusone_0_f06f3000() {
    // Thumb encoding (32): 0xF06F3000
    // Test aarch32_MVN_i_T1_A field imm3 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: i=0, imm3=3, S=0, imm8=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F3000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch32_mvn_i_t1_a_field_imm3_7_max_0_f06f7000() {
    // Thumb encoding (32): 0xF06F7000
    // Test aarch32_MVN_i_T1_A field imm3 = 7 (Max)
    // ISET: T32
    // Fields: i=0, S=0, imm3=7, Rd=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F7000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mvn_i_t1_a_field_rd_0_min_0_f06f0000() {
    // Thumb encoding (32): 0xF06F0000
    // Test aarch32_MVN_i_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: S=0, imm3=0, Rd=0, imm8=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mvn_i_t1_a_field_rd_1_poweroftwo_0_f06f0100() {
    // Thumb encoding (32): 0xF06F0100
    // Test aarch32_MVN_i_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=1, imm3=0, imm8=0, S=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F0100;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_mvn_i_t1_a_field_imm8_0_zero_0_f06f0000() {
    // Thumb encoding (32): 0xF06F0000
    // Test aarch32_MVN_i_T1_A field imm8 = 0 (Zero)
    // ISET: T32
    // Fields: imm8=0, imm3=0, Rd=0, i=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_mvn_i_t1_a_field_imm8_1_poweroftwo_0_f06f0001() {
    // Thumb encoding (32): 0xF06F0001
    // Test aarch32_MVN_i_T1_A field imm8 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=1, imm3=0, i=0, S=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F0001;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_mvn_i_t1_a_field_imm8_3_poweroftwominusone_0_f06f0003() {
    // Thumb encoding (32): 0xF06F0003
    // Test aarch32_MVN_i_T1_A field imm8 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: i=0, Rd=0, imm3=0, imm8=3, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F0003;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_mvn_i_t1_a_field_imm8_4_poweroftwo_0_f06f0004() {
    // Thumb encoding (32): 0xF06F0004
    // Test aarch32_MVN_i_T1_A field imm8 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=4, i=0, Rd=0, S=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F0004;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_mvn_i_t1_a_field_imm8_7_poweroftwominusone_0_f06f0007() {
    // Thumb encoding (32): 0xF06F0007
    // Test aarch32_MVN_i_T1_A field imm8 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=7, S=0, i=0, imm3=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F0007;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_mvn_i_t1_a_field_imm8_8_poweroftwo_0_f06f0008() {
    // Thumb encoding (32): 0xF06F0008
    // Test aarch32_MVN_i_T1_A field imm8 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, Rd=0, i=0, imm8=8, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F0008;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_mvn_i_t1_a_field_imm8_15_poweroftwominusone_0_f06f000f() {
    // Thumb encoding (32): 0xF06F000F
    // Test aarch32_MVN_i_T1_A field imm8 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm3=0, Rd=0, S=0, i=0, imm8=15
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F000F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_mvn_i_t1_a_field_imm8_16_poweroftwo_0_f06f0010() {
    // Thumb encoding (32): 0xF06F0010
    // Test aarch32_MVN_i_T1_A field imm8 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=0, imm8=16, S=0, i=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F0010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_mvn_i_t1_a_field_imm8_31_poweroftwominusone_0_f06f001f() {
    // Thumb encoding (32): 0xF06F001F
    // Test aarch32_MVN_i_T1_A field imm8 = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm3=0, S=0, Rd=0, i=0, imm8=31
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_mvn_i_t1_a_field_imm8_32_poweroftwo_0_f06f0020() {
    // Thumb encoding (32): 0xF06F0020
    // Test aarch32_MVN_i_T1_A field imm8 = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: i=0, S=0, imm3=0, Rd=0, imm8=32
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F0020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_mvn_i_t1_a_field_imm8_63_poweroftwominusone_0_f06f003f() {
    // Thumb encoding (32): 0xF06F003F
    // Test aarch32_MVN_i_T1_A field imm8 = 63 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=63, S=0, Rd=0, imm3=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_mvn_i_t1_a_field_imm8_64_poweroftwo_0_f06f0040() {
    // Thumb encoding (32): 0xF06F0040
    // Test aarch32_MVN_i_T1_A field imm8 = 64 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, S=0, imm8=64, imm3=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F0040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 127, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (127)
#[test]
fn test_aarch32_mvn_i_t1_a_field_imm8_127_poweroftwominusone_0_f06f007f() {
    // Thumb encoding (32): 0xF06F007F
    // Test aarch32_MVN_i_T1_A field imm8 = 127 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rd=0, imm8=127, i=0, S=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F007F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_mvn_i_t1_a_field_imm8_128_poweroftwo_0_f06f0080() {
    // Thumb encoding (32): 0xF06F0080
    // Test aarch32_MVN_i_T1_A field imm8 = 128 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, imm3=0, imm8=128, i=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F0080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 255, boundary: Max }
/// maximum immediate (255)
#[test]
fn test_aarch32_mvn_i_t1_a_field_imm8_255_max_0_f06f00ff() {
    // Thumb encoding (32): 0xF06F00FF
    // Test aarch32_MVN_i_T1_A field imm8 = 255 (Max)
    // ISET: T32
    // Fields: imm8=255, imm3=0, Rd=0, S=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F00FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// i=0 (minimum value)
#[test]
fn test_aarch32_mvn_i_t1_a_combo_0_0_f06f0000() {
    // Thumb encoding (32): 0xF06F0000
    // Test aarch32_MVN_i_T1_A field combination: i=0, S=0, imm3=0, Rd=0, imm8=0
    // ISET: T32
    // Fields: imm3=0, Rd=0, S=0, i=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_mvn_i_t1_a_special_s_0_size_variant_0_0_f06f0000() {
    // Thumb encoding (32): 0xF06F0000
    // Test aarch32_MVN_i_T1_A special value S = 0 (Size variant 0)
    // ISET: T32
    // Fields: S=0, imm8=0, i=0, imm3=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_mvn_i_t1_a_special_s_1_size_variant_1_0_f07f0000() {
    // Thumb encoding (32): 0xF07F0000
    // Test aarch32_MVN_i_T1_A special value S = 1 (Size variant 1)
    // ISET: T32
    // Fields: Rd=0, S=1, imm3=0, imm8=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF07F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mvn_i_t1_a_invalid_0_0_f06f0000() {
    // Thumb encoding (32): 0xF06F0000
    // Test aarch32_MVN_i_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: LitInt(15) }
    // ISET: T32
    // Fields: i=0, imm8=0, S=0, Rd=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F0000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mvn_i_t1_a_invalid_1_0_f06f0000() {
    // Thumb encoding (32): 0xF06F0000
    // Test aarch32_MVN_i_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: i=0, imm8=0, imm3=0, S=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06F0000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `MOVN X0, #0x1234, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// lower 16 bits (32)
#[test]
fn test_aarch32_mvn_i_a1_a_movn_oracle_32_0_03e24680() {
    // Test MOVN 32-bit: lower 16 bits (oracle)
    // Encoding: 0x03E24680
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x03E24680;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFEDCB, "W0 should be 0xFFFFEDCB");
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `MOVN X0, #0x1234, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// lower 16 bits (64)
#[test]
fn test_aarch32_mvn_i_a1_a_movn_oracle_64_0_83e24680() {
    // Test MOVN 64-bit: lower 16 bits (oracle)
    // Encoding: 0x83E24680
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x83E24680;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFEDCB,
        "X0 should be 0xFFFFFFFFFFFFEDCB"
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `MOVN X0, #0xABCD, LSL #16`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shifted 16 bits (32)
#[test]
fn test_aarch32_mvn_i_a1_a_movn_oracle_32_1_03f579a0() {
    // Test MOVN 32-bit: shifted 16 bits (oracle)
    // Encoding: 0x03F579A0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x03F579A0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x5432FFFF, "W0 should be 0x5432FFFF");
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `MOVN X0, #0xABCD, LSL #16`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shifted 16 bits (64)
#[test]
fn test_aarch32_mvn_i_a1_a_movn_oracle_64_1_83f579a0() {
    // Test MOVN 64-bit: shifted 16 bits (oracle)
    // Encoding: 0x83F579A0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x83F579A0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x5432FFFF,
        "X0 should be 0xFFFFFFFF5432FFFF"
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `MOVN X0, #0xFFFF, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm16 (32)
#[test]
fn test_aarch32_mvn_i_a1_a_movn_oracle_32_2_03ffffe0() {
    // Test MOVN 32-bit: max imm16 (oracle)
    // Encoding: 0x03FFFFE0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x03FFFFE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFF0000, "W0 should be 0xFFFF0000");
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `MOVN X0, #0xFFFF, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm16 (64)
#[test]
fn test_aarch32_mvn_i_a1_a_movn_oracle_64_2_83ffffe0() {
    // Test MOVN 64-bit: max imm16 (oracle)
    // Encoding: 0x83FFFFE0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x83FFFFE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFF0000,
        "X0 should be 0xFFFFFFFFFFFF0000"
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `MOVN X0, #0x0000, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero imm16 (32)
#[test]
fn test_aarch32_mvn_i_a1_a_movn_oracle_32_3_03e00000() {
    // Test MOVN 32-bit: zero imm16 (oracle)
    // Encoding: 0x03E00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x03E00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "W0 should be 0xFFFFFFFF");
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `MOVN X0, #0x0000, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero imm16 (64)
#[test]
fn test_aarch32_mvn_i_a1_a_movn_oracle_64_3_83e00000() {
    // Test MOVN 64-bit: zero imm16 (oracle)
    // Encoding: 0x83E00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x83E00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `MOVN X0, #0x5678, LSL #32`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shifted 32 bits (64)
#[test]
fn test_aarch32_mvn_i_a1_a_movn_oracle_64_4_83eacf00() {
    // Test MOVN 64-bit: shifted 32 bits (oracle)
    // Encoding: 0x83EACF00
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x83EACF00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFA987FFFFFFFF"
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `MOVN X0, #0xDEAD, LSL #48`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shifted 48 bits (64)
#[test]
fn test_aarch32_mvn_i_a1_a_movn_oracle_64_5_83fbd5a0() {
    // Test MOVN 64-bit: shifted 48 bits (oracle)
    // Encoding: 0x83FBD5A0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x83FBD5A0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0x2152FFFFFFFFFFFF"
    );
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `MVN R0, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate
#[test]
fn test_aarch32_mvn_i_a1_a_a32_mov_imm_0_03e0000a() {
    // Test A32 MVN: small immediate (oracle)
    // Encoding: 0x03E0000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x03E0000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFF5, "R0 should be 0xFFFFFFF5");
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `MVN R0, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8
#[test]
fn test_aarch32_mvn_i_a1_a_a32_mov_imm_1_03e000ff() {
    // Test A32 MVN: max imm8 (oracle)
    // Encoding: 0x03E000FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x03E000FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFF00, "R0 should be 0xFFFFFF00");
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `MVN R0, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2
#[test]
fn test_aarch32_mvn_i_a1_a_a32_mov_imm_2_03e00180() {
    // Test A32 MVN: rotated by 2 (oracle)
    // Encoding: 0x03E00180
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x03E00180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFDF, "R0 should be 0xFFFFFFDF");
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `MVN R0, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8
#[test]
fn test_aarch32_mvn_i_a1_a_a32_mov_imm_3_03e0040f() {
    // Test A32 MVN: rotated by 8 (oracle)
    // Encoding: 0x03E0040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x03E0040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xF0FFFFFF, "R0 should be 0xF0FFFFFF");
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `MVN R0, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate
#[test]
fn test_aarch32_mvn_i_a1_a_a32_mov_imm_4_03e00000() {
    // Test A32 MVN: zero immediate (oracle)
    // Encoding: 0x03E00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x03E00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "R0 should be 0xFFFFFFFF");
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_mvn_i_a1_a_flags_zeroresult_0_03f00000() {
    // Test aarch32_MVN_i_A1_A flag computation: ZeroResult
    // Encoding: 0x03F00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x03F00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_mvn_i_a1_a_flags_zeroresult_1_03f00000() {
    // Test aarch32_MVN_i_A1_A flag computation: ZeroResult
    // Encoding: 0x03F00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x03F00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_mvn_i_a1_a_flags_negativeresult_2_03f00000() {
    // Test aarch32_MVN_i_A1_A flag computation: NegativeResult
    // Encoding: 0x03F00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x03F00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_mvn_i_a1_a_flags_unsignedoverflow_3_03f00000() {
    // Test aarch32_MVN_i_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x03F00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x03F00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_mvn_i_a1_a_flags_unsignedoverflow_4_03f00000() {
    // Test aarch32_MVN_i_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x03F00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x2);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x03F00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_mvn_i_a1_a_flags_signedoverflow_5_03f00000() {
    // Test aarch32_MVN_i_A1_A flag computation: SignedOverflow
    // Encoding: 0x03F00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x03F00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_mvn_i_a1_a_flags_signedoverflow_6_03f00000() {
    // Test aarch32_MVN_i_A1_A flag computation: SignedOverflow
    // Encoding: 0x03F00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x03F00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MVN_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_mvn_i_a1_a_flags_positiveresult_7_03f00000() {
    // Test aarch32_MVN_i_A1_A flag computation: PositiveResult
    // Encoding: 0x03F00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u32 = 0x03F00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `MOVN X0, #0x1234, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// lower 16 bits (32)
#[test]
fn test_aarch32_mvn_i_t1_a_movn_oracle_32_0_f06f4680() {
    // Test MOVN 32-bit: lower 16 bits (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF06F4680;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFEDCB, "W0 should be 0xFFFFEDCB");
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `MOVN X0, #0x1234, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// lower 16 bits (64)
#[test]
fn test_aarch32_mvn_i_t1_a_movn_oracle_64_0_f06f4680() {
    // Test MOVN 64-bit: lower 16 bits (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF06F4680;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFEDCB,
        "X0 should be 0xFFFFFFFFFFFFEDCB"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `MOVN X0, #0xABCD, LSL #16`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shifted 16 bits (32)
#[test]
fn test_aarch32_mvn_i_t1_a_movn_oracle_32_1_f07f79a0() {
    // Test MOVN 32-bit: shifted 16 bits (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF07F79A0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x5432FFFF, "W0 should be 0x5432FFFF");
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `MOVN X0, #0xABCD, LSL #16`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shifted 16 bits (64)
#[test]
fn test_aarch32_mvn_i_t1_a_movn_oracle_64_1_f07f79a0() {
    // Test MOVN 64-bit: shifted 16 bits (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF07F79A0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x5432FFFF,
        "X0 should be 0xFFFFFFFF5432FFFF"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `MOVN X0, #0xFFFF, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm16 (32)
#[test]
fn test_aarch32_mvn_i_t1_a_movn_oracle_32_2_f07fffe0() {
    // Test MOVN 32-bit: max imm16 (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF07FFFE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFF0000, "W0 should be 0xFFFF0000");
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `MOVN X0, #0xFFFF, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm16 (64)
#[test]
fn test_aarch32_mvn_i_t1_a_movn_oracle_64_2_f07fffe0() {
    // Test MOVN 64-bit: max imm16 (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF07FFFE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFF0000,
        "X0 should be 0xFFFFFFFFFFFF0000"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `MOVN X0, #0x0000, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero imm16 (32)
#[test]
fn test_aarch32_mvn_i_t1_a_movn_oracle_32_3_f06f0000() {
    // Test MOVN 32-bit: zero imm16 (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF06F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "W0 should be 0xFFFFFFFF");
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `MOVN X0, #0x0000, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero imm16 (64)
#[test]
fn test_aarch32_mvn_i_t1_a_movn_oracle_64_3_f06f0000() {
    // Test MOVN 64-bit: zero imm16 (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF06F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `MOVN X0, #0x5678, LSL #32`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shifted 32 bits (64)
#[test]
fn test_aarch32_mvn_i_t1_a_movn_oracle_64_4_f06fcf00() {
    // Test MOVN 64-bit: shifted 32 bits (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF06FCF00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFA987FFFFFFFF"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `MOVN X0, #0xDEAD, LSL #48`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shifted 48 bits (64)
#[test]
fn test_aarch32_mvn_i_t1_a_movn_oracle_64_5_f07fd5a0() {
    // Test MOVN 64-bit: shifted 48 bits (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF07FD5A0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0x2152FFFFFFFFFFFF"
    );
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_mvn_i_t1_a_flags_zeroresult_0_f07f0000() {
    // Test aarch32_MVN_i_T1_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF07F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_mvn_i_t1_a_flags_zeroresult_1_f07f0000() {
    // Test aarch32_MVN_i_T1_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0xF07F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_mvn_i_t1_a_flags_negativeresult_2_f07f0000() {
    // Test aarch32_MVN_i_T1_A flag computation: NegativeResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF07F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_mvn_i_t1_a_flags_unsignedoverflow_3_f07f0000() {
    // Test aarch32_MVN_i_T1_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF07F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_mvn_i_t1_a_flags_unsignedoverflow_4_f07f0000() {
    // Test aarch32_MVN_i_T1_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x2);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF07F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_mvn_i_t1_a_flags_signedoverflow_5_f07f0000() {
    // Test aarch32_MVN_i_T1_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xF07F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_mvn_i_t1_a_flags_signedoverflow_6_f07f0000() {
    // Test aarch32_MVN_i_T1_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF07F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MVN_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_mvn_i_t1_a_flags_positiveresult_7_f07f0000() {
    // Test aarch32_MVN_i_T1_A flag computation: PositiveResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x32);
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF07F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

// ============================================================================
// aarch32_MOV_r_A Tests
// ============================================================================

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_mov_r_a1_a_field_cond_0_min_0_01a00000() {
    // Encoding: 0x01A00000
    // Test aarch32_MOV_r_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: S=0, imm5=0, type1=0, cond=0, Rm=0, Rd=0
    let encoding: u32 = 0x01A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_mov_r_a1_a_field_cond_1_poweroftwo_0_11a00000() {
    // Encoding: 0x11A00000
    // Test aarch32_MOV_r_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=1, imm5=0, type1=0, S=0, Rm=0
    let encoding: u32 = 0x11A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_mov_r_a1_a_field_cond_2_poweroftwo_0_21a00000() {
    // Encoding: 0x21A00000
    // Test aarch32_MOV_r_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, type1=0, cond=2, Rm=0, imm5=0, S=0
    let encoding: u32 = 0x21A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_mov_r_a1_a_field_cond_3_poweroftwo_0_31a00000() {
    // Encoding: 0x31A00000
    // Test aarch32_MOV_r_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, Rd=0, cond=3, type1=0, Rm=0, imm5=0
    let encoding: u32 = 0x31A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_mov_r_a1_a_field_cond_4_poweroftwo_0_41a00000() {
    // Encoding: 0x41A00000
    // Test aarch32_MOV_r_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, type1=0, Rm=0, cond=4, imm5=0, Rd=0
    let encoding: u32 = 0x41A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_mov_r_a1_a_field_cond_5_poweroftwo_0_51a00000() {
    // Encoding: 0x51A00000
    // Test aarch32_MOV_r_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, imm5=0, Rm=0, Rd=0, type1=0, cond=5
    let encoding: u32 = 0x51A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_mov_r_a1_a_field_cond_6_poweroftwo_0_61a00000() {
    // Encoding: 0x61A00000
    // Test aarch32_MOV_r_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rm=0, cond=6, Rd=0, S=0, imm5=0
    let encoding: u32 = 0x61A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_mov_r_a1_a_field_cond_7_poweroftwo_0_71a00000() {
    // Encoding: 0x71A00000
    // Test aarch32_MOV_r_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, imm5=0, Rd=0, Rm=0, cond=7, type1=0
    let encoding: u32 = 0x71A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_mov_r_a1_a_field_cond_8_poweroftwo_0_81a00000() {
    // Encoding: 0x81A00000
    // Test aarch32_MOV_r_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, cond=8, type1=0, Rd=0, imm5=0, Rm=0
    let encoding: u32 = 0x81A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_mov_r_a1_a_field_cond_9_poweroftwo_0_91a00000() {
    // Encoding: 0x91A00000
    // Test aarch32_MOV_r_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rm=0, Rd=0, imm5=0, cond=9, S=0
    let encoding: u32 = 0x91A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_mov_r_a1_a_field_cond_10_poweroftwo_0_a1a00000() {
    // Encoding: 0xA1A00000
    // Test aarch32_MOV_r_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, S=0, imm5=0, Rm=0, Rd=0, type1=0
    let encoding: u32 = 0xA1A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_mov_r_a1_a_field_cond_11_poweroftwo_0_b1a00000() {
    // Encoding: 0xB1A00000
    // Test aarch32_MOV_r_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, imm5=0, Rd=0, cond=11, type1=0, Rm=0
    let encoding: u32 = 0xB1A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_mov_r_a1_a_field_cond_12_poweroftwo_0_c1a00000() {
    // Encoding: 0xC1A00000
    // Test aarch32_MOV_r_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=12, S=0, imm5=0, type1=0
    let encoding: u32 = 0xC1A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_mov_r_a1_a_field_cond_13_poweroftwo_0_d1a00000() {
    // Encoding: 0xD1A00000
    // Test aarch32_MOV_r_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=0, Rd=0, S=0, type1=0, Rm=0, cond=13
    let encoding: u32 = 0xD1A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_mov_r_a1_a_field_cond_14_poweroftwo_0_e1a00000() {
    // Encoding: 0xE1A00000
    // Test aarch32_MOV_r_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, type1=0, cond=14, S=0, imm5=0, Rm=0
    let encoding: u32 = 0xE1A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_mov_r_a1_a_field_cond_15_max_0_f1a00000() {
    // Encoding: 0xF1A00000
    // Test aarch32_MOV_r_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, Rd=0, S=0, imm5=0, type1=0, Rm=0
    let encoding: u32 = 0xF1A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_mov_r_a1_a_field_s_0_min_0_01a00000() {
    // Encoding: 0x01A00000
    // Test aarch32_MOV_r_A1_A field S = 0 (Min)
    // ISET: A32
    // Fields: S=0, imm5=0, Rd=0, type1=0, Rm=0, cond=0
    let encoding: u32 = 0x01A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_mov_r_a1_a_field_s_1_max_0_01b00000() {
    // Encoding: 0x01B00000
    // Test aarch32_MOV_r_A1_A field S = 1 (Max)
    // ISET: A32
    // Fields: type1=0, Rm=0, S=1, cond=0, Rd=0, imm5=0
    let encoding: u32 = 0x01B00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mov_r_a1_a_field_rd_0_min_0_01a00000() {
    // Encoding: 0x01A00000
    // Test aarch32_MOV_r_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, S=0, Rd=0, imm5=0, Rm=0, type1=0
    let encoding: u32 = 0x01A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mov_r_a1_a_field_rd_1_poweroftwo_0_01a01000() {
    // Encoding: 0x01A01000
    // Test aarch32_MOV_r_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, S=0, cond=0, type1=0, Rd=1, imm5=0
    let encoding: u32 = 0x01A01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_mov_r_a1_a_field_imm5_0_zero_0_01a00000() {
    // Encoding: 0x01A00000
    // Test aarch32_MOV_r_A1_A field imm5 = 0 (Zero)
    // ISET: A32
    // Fields: Rm=0, imm5=0, S=0, Rd=0, cond=0, type1=0
    let encoding: u32 = 0x01A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_mov_r_a1_a_field_imm5_1_poweroftwo_0_01a00080() {
    // Encoding: 0x01A00080
    // Test aarch32_MOV_r_A1_A field imm5 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, Rd=0, imm5=1, type1=0, cond=0, Rm=0
    let encoding: u32 = 0x01A00080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_mov_r_a1_a_field_imm5_3_poweroftwominusone_0_01a00180() {
    // Encoding: 0x01A00180
    // Test aarch32_MOV_r_A1_A field imm5 = 3 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, S=0, Rd=0, type1=0, imm5=3, Rm=0
    let encoding: u32 = 0x01A00180;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_mov_r_a1_a_field_imm5_4_poweroftwo_0_01a00200() {
    // Encoding: 0x01A00200
    // Test aarch32_MOV_r_A1_A field imm5 = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=4, Rd=0, type1=0, S=0, Rm=0, cond=0
    let encoding: u32 = 0x01A00200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_mov_r_a1_a_field_imm5_7_poweroftwominusone_0_01a00380() {
    // Encoding: 0x01A00380
    // Test aarch32_MOV_r_A1_A field imm5 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm5=7, type1=0, Rm=0, Rd=0, cond=0, S=0
    let encoding: u32 = 0x01A00380;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_mov_r_a1_a_field_imm5_8_poweroftwo_0_01a00400() {
    // Encoding: 0x01A00400
    // Test aarch32_MOV_r_A1_A field imm5 = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=8, Rd=0, Rm=0, type1=0, S=0, cond=0
    let encoding: u32 = 0x01A00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 15, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (15)
#[test]
fn test_aarch32_mov_r_a1_a_field_imm5_15_poweroftwominusone_0_01a00780() {
    // Encoding: 0x01A00780
    // Test aarch32_MOV_r_A1_A field imm5 = 15 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: S=0, imm5=15, cond=0, Rd=0, Rm=0, type1=0
    let encoding: u32 = 0x01A00780;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_mov_r_a1_a_field_imm5_16_poweroftwo_0_01a00800() {
    // Encoding: 0x01A00800
    // Test aarch32_MOV_r_A1_A field imm5 = 16 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=0, Rd=0, type1=0, imm5=16, S=0
    let encoding: u32 = 0x01A00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 31, boundary: Max }
/// maximum immediate (31)
#[test]
fn test_aarch32_mov_r_a1_a_field_imm5_31_max_0_01a00f80() {
    // Encoding: 0x01A00F80
    // Test aarch32_MOV_r_A1_A field imm5 = 31 (Max)
    // ISET: A32
    // Fields: S=0, Rd=0, cond=0, imm5=31, Rm=0, type1=0
    let encoding: u32 = 0x01A00F80;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mov_r_a1_a_field_type1_0_min_0_01a00000() {
    // Encoding: 0x01A00000
    // Test aarch32_MOV_r_A1_A field type1 = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, S=0, imm5=0, Rm=0, cond=0, type1=0
    let encoding: u32 = 0x01A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mov_r_a1_a_field_type1_1_poweroftwo_0_01a00020() {
    // Encoding: 0x01A00020
    // Test aarch32_MOV_r_A1_A field type1 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=1, Rm=0, Rd=0, imm5=0, S=0, cond=0
    let encoding: u32 = 0x01A00020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_mov_r_a1_a_field_type1_3_max_0_01a00060() {
    // Encoding: 0x01A00060
    // Test aarch32_MOV_r_A1_A field type1 = 3 (Max)
    // ISET: A32
    // Fields: cond=0, S=0, imm5=0, type1=3, Rd=0, Rm=0
    let encoding: u32 = 0x01A00060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mov_r_a1_a_field_rm_0_min_0_01a00000() {
    // Encoding: 0x01A00000
    // Test aarch32_MOV_r_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, imm5=0, cond=0, S=0, type1=0, Rm=0
    let encoding: u32 = 0x01A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mov_r_a1_a_field_rm_1_poweroftwo_0_01a00001() {
    // Encoding: 0x01A00001
    // Test aarch32_MOV_r_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, imm5=0, Rm=1, Rd=0, type1=0, cond=0
    let encoding: u32 = 0x01A00001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_mov_r_a1_a_combo_0_0_01a00000() {
    // Encoding: 0x01A00000
    // Test aarch32_MOV_r_A1_A field combination: cond=0, S=0, Rd=0, imm5=0, type1=0, Rm=0
    // ISET: A32
    // Fields: S=0, type1=0, Rm=0, Rd=0, cond=0, imm5=0
    let encoding: u32 = 0x01A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_mov_r_a1_a_special_cond_0_condition_eq_0_01a00000() {
    // Encoding: 0x01A00000
    // Test aarch32_MOV_r_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: cond=0, Rd=0, imm5=0, type1=0, Rm=0, S=0
    let encoding: u32 = 0x01A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_mov_r_a1_a_special_cond_1_condition_ne_0_11a00000() {
    // Encoding: 0x11A00000
    // Test aarch32_MOV_r_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, imm5=0, type1=0, S=0, Rm=0, Rd=0
    let encoding: u32 = 0x11A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_mov_r_a1_a_special_cond_2_condition_cs_hs_0_21a00000() {
    // Encoding: 0x21A00000
    // Test aarch32_MOV_r_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, imm5=0, type1=0, Rm=0, S=0, Rd=0
    let encoding: u32 = 0x21A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_mov_r_a1_a_special_cond_3_condition_cc_lo_0_31a00000() {
    // Encoding: 0x31A00000
    // Test aarch32_MOV_r_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: type1=0, imm5=0, Rm=0, S=0, cond=3, Rd=0
    let encoding: u32 = 0x31A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_mov_r_a1_a_special_cond_4_condition_mi_0_41a00000() {
    // Encoding: 0x41A00000
    // Test aarch32_MOV_r_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: imm5=0, Rm=0, type1=0, Rd=0, cond=4, S=0
    let encoding: u32 = 0x41A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_mov_r_a1_a_special_cond_5_condition_pl_0_51a00000() {
    // Encoding: 0x51A00000
    // Test aarch32_MOV_r_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: imm5=0, Rm=0, cond=5, Rd=0, type1=0, S=0
    let encoding: u32 = 0x51A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_mov_r_a1_a_special_cond_6_condition_vs_0_61a00000() {
    // Encoding: 0x61A00000
    // Test aarch32_MOV_r_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: type1=0, cond=6, Rd=0, imm5=0, Rm=0, S=0
    let encoding: u32 = 0x61A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_mov_r_a1_a_special_cond_7_condition_vc_0_71a00000() {
    // Encoding: 0x71A00000
    // Test aarch32_MOV_r_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rd=0, cond=7, imm5=0, S=0, Rm=0, type1=0
    let encoding: u32 = 0x71A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_mov_r_a1_a_special_cond_8_condition_hi_0_81a00000() {
    // Encoding: 0x81A00000
    // Test aarch32_MOV_r_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: imm5=0, Rm=0, Rd=0, S=0, cond=8, type1=0
    let encoding: u32 = 0x81A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_mov_r_a1_a_special_cond_9_condition_ls_0_91a00000() {
    // Encoding: 0x91A00000
    // Test aarch32_MOV_r_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rm=0, imm5=0, type1=0, Rd=0, S=0, cond=9
    let encoding: u32 = 0x91A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_mov_r_a1_a_special_cond_10_condition_ge_0_a1a00000() {
    // Encoding: 0xA1A00000
    // Test aarch32_MOV_r_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: S=0, imm5=0, type1=0, cond=10, Rm=0, Rd=0
    let encoding: u32 = 0xA1A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_mov_r_a1_a_special_cond_11_condition_lt_0_b1a00000() {
    // Encoding: 0xB1A00000
    // Test aarch32_MOV_r_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: type1=0, cond=11, Rm=0, Rd=0, S=0, imm5=0
    let encoding: u32 = 0xB1A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_mov_r_a1_a_special_cond_12_condition_gt_0_c1a00000() {
    // Encoding: 0xC1A00000
    // Test aarch32_MOV_r_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: cond=12, Rd=0, Rm=0, S=0, imm5=0, type1=0
    let encoding: u32 = 0xC1A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_mov_r_a1_a_special_cond_13_condition_le_0_d1a00000() {
    // Encoding: 0xD1A00000
    // Test aarch32_MOV_r_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rd=0, type1=0, Rm=0, cond=13, imm5=0, S=0
    let encoding: u32 = 0xD1A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_mov_r_a1_a_special_cond_14_condition_al_0_e1a00000() {
    // Encoding: 0xE1A00000
    // Test aarch32_MOV_r_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rd=0, cond=14, imm5=0, type1=0, Rm=0, S=0
    let encoding: u32 = 0xE1A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_mov_r_a1_a_special_cond_15_condition_nv_0_f1a00000() {
    // Encoding: 0xF1A00000
    // Test aarch32_MOV_r_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, imm5=0, type1=0, S=0, Rd=0, Rm=0
    let encoding: u32 = 0xF1A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_mov_r_a1_a_special_s_0_size_variant_0_0_01a00000() {
    // Encoding: 0x01A00000
    // Test aarch32_MOV_r_A1_A special value S = 0 (Size variant 0)
    // ISET: A32
    // Fields: imm5=0, Rm=0, cond=0, Rd=0, S=0, type1=0
    let encoding: u32 = 0x01A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_mov_r_a1_a_special_s_1_size_variant_1_0_01b00000() {
    // Encoding: 0x01B00000
    // Test aarch32_MOV_r_A1_A special value S = 1 (Size variant 1)
    // ISET: A32
    // Fields: imm5=0, S=1, Rd=0, type1=0, Rm=0, cond=0
    let encoding: u32 = 0x01B00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `field D 23 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mov_r_t1_a_field_d_0_min_0_46000000() {
    // Thumb encoding (32): 0x46000000
    // Test aarch32_MOV_r_T1_A field D = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x46000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `field D 23 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_mov_r_t1_a_field_d_1_max_0_46800000() {
    // Thumb encoding (32): 0x46800000
    // Test aarch32_MOV_r_T1_A field D = 1 (Max)
    // ISET: T32
    // Fields: Rd=0, D=1, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x46800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `field Rm 19 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mov_r_t1_a_field_rm_0_min_0_46000000() {
    // Thumb encoding (32): 0x46000000
    // Test aarch32_MOV_r_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: D=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x46000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `field Rm 19 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mov_r_t1_a_field_rm_1_poweroftwo_0_46080000() {
    // Thumb encoding (32): 0x46080000
    // Test aarch32_MOV_r_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1, Rd=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x46080000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `field Rd 16 +: 3`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mov_r_t1_a_field_rd_0_min_0_46000000() {
    // Thumb encoding (32): 0x46000000
    // Test aarch32_MOV_r_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: D=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x46000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `field Rd 16 +: 3`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mov_r_t1_a_field_rd_1_poweroftwo_0_46010000() {
    // Thumb encoding (32): 0x46010000
    // Test aarch32_MOV_r_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: D=0, Rm=0, Rd=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x46010000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_mov_r_t1_a_combo_0_0_46000000() {
    // Thumb encoding (32): 0x46000000
    // Test aarch32_MOV_r_T1_A field combination: D=0, Rm=0, Rd=0
    // ISET: T32
    // Fields: Rd=0, D=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x46000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: And, lhs: Binary { op: And, lhs: LitInt(15), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] } }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "LastInITBlock" }, args: [] } } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: And, lhs: Binary { op: And, lhs: LitInt(15), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] } }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"LastInITBlock\" }, args: [] } } } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mov_r_t1_a_invalid_0_0_46000000() {
    // Thumb encoding (32): 0x46000000
    // Test aarch32_MOV_r_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: And, lhs: Binary { op: And, lhs: LitInt(15), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] } }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "LastInITBlock" }, args: [] } } } }
    // ISET: T32
    // Fields: D=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x46000000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mov_r_t1_a_invalid_1_0_46000000() {
    // Thumb encoding (32): 0x46000000
    // Test aarch32_MOV_r_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rd=0, D=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x46000000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `field op 27 +: 2`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mov_r_t2_a_field_op_0_min_0_00000000() {
    // Thumb encoding (32): 0x00000000
    // Test aarch32_MOV_r_T2_A field op = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, op=0, imm5=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x00000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `field op 27 +: 2`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mov_r_t2_a_field_op_1_poweroftwo_0_08000000() {
    // Thumb encoding (32): 0x08000000
    // Test aarch32_MOV_r_T2_A field op = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=0, op=1, imm5=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x08000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `field op 27 +: 2`
/// Requirement: FieldBoundary { field: "op", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_mov_r_t2_a_field_op_3_max_0_18000000() {
    // Thumb encoding (32): 0x18000000
    // Test aarch32_MOV_r_T2_A field op = 3 (Max)
    // ISET: T32
    // Fields: op=3, imm5=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x18000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `field imm5 22 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_mov_r_t2_a_field_imm5_0_zero_0_00000000() {
    // Thumb encoding (32): 0x00000000
    // Test aarch32_MOV_r_T2_A field imm5 = 0 (Zero)
    // ISET: T32
    // Fields: imm5=0, op=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x00000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `field imm5 22 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_mov_r_t2_a_field_imm5_1_poweroftwo_0_00400000() {
    // Thumb encoding (32): 0x00400000
    // Test aarch32_MOV_r_T2_A field imm5 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: op=0, imm5=1, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x00400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `field imm5 22 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_mov_r_t2_a_field_imm5_3_poweroftwominusone_0_00c00000() {
    // Thumb encoding (32): 0x00C00000
    // Test aarch32_MOV_r_T2_A field imm5 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rm=0, op=0, Rd=0, imm5=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x00C00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `field imm5 22 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_mov_r_t2_a_field_imm5_4_poweroftwo_0_01000000() {
    // Thumb encoding (32): 0x01000000
    // Test aarch32_MOV_r_T2_A field imm5 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: op=0, imm5=4, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x01000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `field imm5 22 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_mov_r_t2_a_field_imm5_7_poweroftwominusone_0_01c00000() {
    // Thumb encoding (32): 0x01C00000
    // Test aarch32_MOV_r_T2_A field imm5 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rd=0, op=0, imm5=7, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x01C00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `field imm5 22 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_mov_r_t2_a_field_imm5_8_poweroftwo_0_02000000() {
    // Thumb encoding (32): 0x02000000
    // Test aarch32_MOV_r_T2_A field imm5 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: imm5=8, Rd=0, Rm=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x02000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `field imm5 22 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 15, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (15)
#[test]
fn test_aarch32_mov_r_t2_a_field_imm5_15_poweroftwominusone_0_03c00000() {
    // Thumb encoding (32): 0x03C00000
    // Test aarch32_MOV_r_T2_A field imm5 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rd=0, op=0, imm5=15, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x03C00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `field imm5 22 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_mov_r_t2_a_field_imm5_16_poweroftwo_0_04000000() {
    // Thumb encoding (32): 0x04000000
    // Test aarch32_MOV_r_T2_A field imm5 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, imm5=16, Rm=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x04000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `field imm5 22 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 31, boundary: Max }
/// maximum immediate (31)
#[test]
fn test_aarch32_mov_r_t2_a_field_imm5_31_max_0_07c00000() {
    // Thumb encoding (32): 0x07C00000
    // Test aarch32_MOV_r_T2_A field imm5 = 31 (Max)
    // ISET: T32
    // Fields: imm5=31, Rd=0, Rm=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x07C00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `field Rm 19 +: 3`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mov_r_t2_a_field_rm_0_min_0_00000000() {
    // Thumb encoding (32): 0x00000000
    // Test aarch32_MOV_r_T2_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, imm5=0, Rm=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x00000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `field Rm 19 +: 3`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mov_r_t2_a_field_rm_1_poweroftwo_0_00080000() {
    // Thumb encoding (32): 0x00080000
    // Test aarch32_MOV_r_T2_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1, op=0, imm5=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x00080000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `field Rd 16 +: 3`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mov_r_t2_a_field_rd_0_min_0_00000000() {
    // Thumb encoding (32): 0x00000000
    // Test aarch32_MOV_r_T2_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: imm5=0, Rm=0, Rd=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x00000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `field Rd 16 +: 3`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mov_r_t2_a_field_rd_1_poweroftwo_0_00010000() {
    // Thumb encoding (32): 0x00010000
    // Test aarch32_MOV_r_T2_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=1, imm5=0, op=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x00010000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch32_mov_r_t2_a_combo_0_0_00000000() {
    // Thumb encoding (32): 0x00000000
    // Test aarch32_MOV_r_T2_A field combination: op=0, imm5=0, Rm=0, Rd=0
    // ISET: T32
    // Fields: op=0, imm5=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x00000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "op" }), rhs: Binary { op: And, lhs: LitBits([false, false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "imm5" }) } }, rhs: Binary { op: And, lhs: LitBits([false, false, false, false, false]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"op\" }), rhs: Binary { op: And, lhs: LitBits([false, false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"imm5\" }) } }, rhs: Binary { op: And, lhs: LitBits([false, false, false, false, false]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] } } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mov_r_t2_a_invalid_0_0_00000000() {
    // Thumb encoding (32): 0x00000000
    // Test aarch32_MOV_r_T2_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "op" }), rhs: Binary { op: And, lhs: LitBits([false, false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "imm5" }) } }, rhs: Binary { op: And, lhs: LitBits([false, false, false, false, false]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] } } }
    // ISET: T32
    // Fields: op=0, imm5=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x00000000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mov_r_t2_a_invalid_1_0_00000000() {
    // Thumb encoding (32): 0x00000000
    // Test aarch32_MOV_r_T2_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: imm5=0, Rm=0, op=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x00000000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_mov_r_t3_a_field_s_0_min_0_ea4f0000() {
    // Thumb encoding (32): 0xEA4F0000
    // Test aarch32_MOV_r_T3_A field S = 0 (Min)
    // ISET: T32
    // Fields: type1=0, S=0, imm3=0, Rd=0, imm2=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA4F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_mov_r_t3_a_field_s_1_max_0_ea5f0000() {
    // Thumb encoding (32): 0xEA5F0000
    // Test aarch32_MOV_r_T3_A field S = 1 (Max)
    // ISET: T32
    // Fields: imm2=0, Rm=0, S=1, type1=0, imm3=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA5F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_mov_r_t3_a_field_imm3_0_zero_0_ea4f0000() {
    // Thumb encoding (32): 0xEA4F0000
    // Test aarch32_MOV_r_T3_A field imm3 = 0 (Zero)
    // ISET: T32
    // Fields: imm2=0, Rd=0, S=0, imm3=0, type1=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA4F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_mov_r_t3_a_field_imm3_1_poweroftwo_0_ea4f1000() {
    // Thumb encoding (32): 0xEA4F1000
    // Test aarch32_MOV_r_T3_A field imm3 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, S=0, type1=0, imm3=1, imm2=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA4F1000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch32_mov_r_t3_a_field_imm3_3_poweroftwominusone_0_ea4f3000() {
    // Thumb encoding (32): 0xEA4F3000
    // Test aarch32_MOV_r_T3_A field imm3 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rm=0, imm3=3, S=0, type1=0, Rd=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA4F3000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch32_mov_r_t3_a_field_imm3_7_max_0_ea4f7000() {
    // Thumb encoding (32): 0xEA4F7000
    // Test aarch32_MOV_r_T3_A field imm3 = 7 (Max)
    // ISET: T32
    // Fields: imm3=7, S=0, imm2=0, Rd=0, type1=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA4F7000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mov_r_t3_a_field_rd_0_min_0_ea4f0000() {
    // Thumb encoding (32): 0xEA4F0000
    // Test aarch32_MOV_r_T3_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: imm3=0, S=0, Rm=0, type1=0, imm2=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA4F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mov_r_t3_a_field_rd_1_poweroftwo_0_ea4f0100() {
    // Thumb encoding (32): 0xEA4F0100
    // Test aarch32_MOV_r_T3_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, imm3=0, type1=0, S=0, Rd=1, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA4F0100;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `field imm2 6 +: 2`
/// Requirement: FieldBoundary { field: "imm2", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_mov_r_t3_a_field_imm2_0_zero_0_ea4f0000() {
    // Thumb encoding (32): 0xEA4F0000
    // Test aarch32_MOV_r_T3_A field imm2 = 0 (Zero)
    // ISET: T32
    // Fields: type1=0, imm2=0, Rm=0, S=0, imm3=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA4F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `field imm2 6 +: 2`
/// Requirement: FieldBoundary { field: "imm2", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_mov_r_t3_a_field_imm2_1_poweroftwo_0_ea4f0040() {
    // Thumb encoding (32): 0xEA4F0040
    // Test aarch32_MOV_r_T3_A field imm2 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rd=0, imm2=1, S=0, imm3=0, type1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA4F0040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `field imm2 6 +: 2`
/// Requirement: FieldBoundary { field: "imm2", value: 3, boundary: Max }
/// maximum immediate (3)
#[test]
fn test_aarch32_mov_r_t3_a_field_imm2_3_max_0_ea4f00c0() {
    // Thumb encoding (32): 0xEA4F00C0
    // Test aarch32_MOV_r_T3_A field imm2 = 3 (Max)
    // ISET: T32
    // Fields: imm2=3, S=0, type1=0, imm3=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA4F00C0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `field type1 4 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mov_r_t3_a_field_type1_0_min_0_ea4f0000() {
    // Thumb encoding (32): 0xEA4F0000
    // Test aarch32_MOV_r_T3_A field type1 = 0 (Min)
    // ISET: T32
    // Fields: type1=0, Rm=0, imm3=0, S=0, Rd=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA4F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `field type1 4 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mov_r_t3_a_field_type1_1_poweroftwo_0_ea4f0010() {
    // Thumb encoding (32): 0xEA4F0010
    // Test aarch32_MOV_r_T3_A field type1 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, S=0, imm2=0, type1=1, imm3=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA4F0010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `field type1 4 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_mov_r_t3_a_field_type1_3_max_0_ea4f0030() {
    // Thumb encoding (32): 0xEA4F0030
    // Test aarch32_MOV_r_T3_A field type1 = 3 (Max)
    // ISET: T32
    // Fields: type1=3, Rm=0, S=0, imm3=0, Rd=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA4F0030;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mov_r_t3_a_field_rm_0_min_0_ea4f0000() {
    // Thumb encoding (32): 0xEA4F0000
    // Test aarch32_MOV_r_T3_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: imm2=0, Rm=0, type1=0, imm3=0, S=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA4F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mov_r_t3_a_field_rm_1_poweroftwo_0_ea4f0001() {
    // Thumb encoding (32): 0xEA4F0001
    // Test aarch32_MOV_r_T3_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm2=0, Rd=0, type1=0, imm3=0, Rm=1, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA4F0001;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=0 (8-bit / byte size)
#[test]
fn test_aarch32_mov_r_t3_a_combo_0_0_ea4f0000() {
    // Thumb encoding (32): 0xEA4F0000
    // Test aarch32_MOV_r_T3_A field combination: S=0, imm3=0, Rd=0, imm2=0, type1=0, Rm=0
    // ISET: T32
    // Fields: imm3=0, type1=0, imm2=0, Rm=0, Rd=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA4F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_mov_r_t3_a_special_s_0_size_variant_0_0_ea4f0000() {
    // Thumb encoding (32): 0xEA4F0000
    // Test aarch32_MOV_r_T3_A special value S = 0 (Size variant 0)
    // ISET: T32
    // Fields: imm2=0, type1=0, Rm=0, imm3=0, S=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA4F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_mov_r_t3_a_special_s_1_size_variant_1_0_ea5f0000() {
    // Thumb encoding (32): 0xEA5F0000
    // Test aarch32_MOV_r_T3_A special value S = 1 (Size variant 1)
    // ISET: T32
    // Fields: type1=0, S=1, Rd=0, imm2=0, Rm=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA5F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mov_r_t3_a_invalid_0_0_ea4f0000() {
    // Thumb encoding (32): 0xEA4F0000
    // Test aarch32_MOV_r_T3_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: S=0, imm3=0, type1=0, Rm=0, Rd=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA4F0000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mov_r_t3_a_invalid_1_0_ea4f0000() {
    // Thumb encoding (32): 0xEA4F0000
    // Test aarch32_MOV_r_T3_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rm=0, S=0, imm3=0, imm2=0, type1=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA4F0000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 0 (32)
#[test]
fn test_aarch32_mov_r_a1_a_lslv_oracle_32_0_01a20020() {
    // Test LSLV 32-bit: shift by 0 (oracle)
    // Encoding: 0x01A20020
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x01A20020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x12345678, "W0 should be 0x12345678");
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 0 (64)
#[test]
fn test_aarch32_mov_r_a1_a_lslv_oracle_64_0_81a20020() {
    // Test LSLV 64-bit: shift by 0 (oracle)
    // Encoding: 0x81A20020
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x81A20020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x12345678,
        "X0 should be 0x0000000012345678"
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4 (32)
#[test]
fn test_aarch32_mov_r_a1_a_lslv_oracle_32_1_01a20020() {
    // Test LSLV 32-bit: shift by 4 (oracle)
    // Encoding: 0x01A20020
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x4);
    let encoding: u32 = 0x01A20020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x23456780, "W0 should be 0x23456780");
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 4 (64)
#[test]
fn test_aarch32_mov_r_a1_a_lslv_oracle_64_1_81a20020() {
    // Test LSLV 64-bit: shift by 4 (oracle)
    // Encoding: 0x81A20020
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x4);
    let encoding: u32 = 0x81A20020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x23456780,
        "X0 should be 0x0000000123456780"
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 8 (32)
#[test]
fn test_aarch32_mov_r_a1_a_lslv_oracle_32_2_01a20020() {
    // Test LSLV 32-bit: shift by 8 (oracle)
    // Encoding: 0x01A20020
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x8);
    let encoding: u32 = 0x01A20020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x34567800, "W0 should be 0x34567800");
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 8 (64)
#[test]
fn test_aarch32_mov_r_a1_a_lslv_oracle_64_2_81a20020() {
    // Test LSLV 64-bit: shift by 8 (oracle)
    // Encoding: 0x81A20020
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x8);
    set_w(&mut cpu, 1, 0x12345678);
    let encoding: u32 = 0x81A20020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x34567800,
        "X0 should be 0x0000001234567800"
    );
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1 (32)
#[test]
fn test_aarch32_mov_r_a1_a_lslv_oracle_32_3_01a20020() {
    // Test LSLV 32-bit: MSB set, shift 1 (oracle)
    // Encoding: 0x01A20020
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x01A20020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x00000000");
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// MSB set, shift 1 (64)
#[test]
fn test_aarch32_mov_r_a1_a_lslv_oracle_64_3_81a20020() {
    // Test LSLV 64-bit: MSB set, shift 1 (oracle)
    // Encoding: 0x81A20020
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x81A20020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSB set, max shift (32)
#[test]
fn test_aarch32_mov_r_a1_a_lslv_oracle_32_4_01a20020() {
    // Test LSLV 32-bit: LSB set, max shift (oracle)
    // Encoding: 0x01A20020
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x3F);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x01A20020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x80000000, "W0 should be 0x80000000");
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSB set, max shift (64)
#[test]
fn test_aarch32_mov_r_a1_a_lslv_oracle_64_4_81a20020() {
    // Test LSLV 64-bit: LSB set, max shift (oracle)
    // Encoding: 0x81A20020
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x3F);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x81A20020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x8000000000000000");
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// all ones, shift 32 (32)
#[test]
fn test_aarch32_mov_r_a1_a_lslv_oracle_32_5_01a20020() {
    // Test LSLV 32-bit: all ones, shift 32 (oracle)
    // Encoding: 0x01A20020
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x20);
    let encoding: u32 = 0x01A20020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "W0 should be 0xFFFFFFFF");
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all ones, shift 32 (64)
#[test]
fn test_aarch32_mov_r_a1_a_lslv_oracle_64_5_81a20020() {
    // Test LSLV 64-bit: all ones, shift 32 (oracle)
    // Encoding: 0x81A20020
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x20);
    let encoding: u32 = 0x81A20020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0xFFFFFFFF00000000");
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_mov_r_a1_a_flags_zeroresult_0_01b00002() {
    // Test aarch32_MOV_r_A1_A flag computation: ZeroResult
    // Encoding: 0x01B00002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x01B00002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_mov_r_a1_a_flags_zeroresult_1_01b00002() {
    // Test aarch32_MOV_r_A1_A flag computation: ZeroResult
    // Encoding: 0x01B00002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x01B00002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_mov_r_a1_a_flags_negativeresult_2_01b00002() {
    // Test aarch32_MOV_r_A1_A flag computation: NegativeResult
    // Encoding: 0x01B00002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x01B00002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_mov_r_a1_a_flags_unsignedoverflow_3_01b00002() {
    // Test aarch32_MOV_r_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x01B00002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x01B00002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_mov_r_a1_a_flags_unsignedoverflow_4_01b00002() {
    // Test aarch32_MOV_r_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x01B00002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x01B00002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_mov_r_a1_a_flags_signedoverflow_5_01b00002() {
    // Test aarch32_MOV_r_A1_A flag computation: SignedOverflow
    // Encoding: 0x01B00002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x01B00002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_mov_r_a1_a_flags_signedoverflow_6_01b00002() {
    // Test aarch32_MOV_r_A1_A flag computation: SignedOverflow
    // Encoding: 0x01B00002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x01B00002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_mov_r_a1_a_flags_positiveresult_7_01b00002() {
    // Test aarch32_MOV_r_A1_A flag computation: PositiveResult
    // Encoding: 0x01B00002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x32);
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x01B00002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 0 (32)
#[test]
fn test_aarch32_mov_r_t1_a_lslv_oracle_32_0_46020020() {
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

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 0 (64)
#[test]
fn test_aarch32_mov_r_t1_a_lslv_oracle_64_0_c6020020() {
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

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4 (32)
#[test]
fn test_aarch32_mov_r_t1_a_lslv_oracle_32_1_46020020() {
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

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 4 (64)
#[test]
fn test_aarch32_mov_r_t1_a_lslv_oracle_64_1_c6020020() {
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

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 8 (32)
#[test]
fn test_aarch32_mov_r_t1_a_lslv_oracle_32_2_46020020() {
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

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 8 (64)
#[test]
fn test_aarch32_mov_r_t1_a_lslv_oracle_64_2_c6020020() {
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

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1 (32)
#[test]
fn test_aarch32_mov_r_t1_a_lslv_oracle_32_3_46020020() {
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

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// MSB set, shift 1 (64)
#[test]
fn test_aarch32_mov_r_t1_a_lslv_oracle_64_3_c6020020() {
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

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSB set, max shift (32)
#[test]
fn test_aarch32_mov_r_t1_a_lslv_oracle_32_4_46020020() {
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

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSB set, max shift (64)
#[test]
fn test_aarch32_mov_r_t1_a_lslv_oracle_64_4_c6020020() {
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

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// all ones, shift 32 (32)
#[test]
fn test_aarch32_mov_r_t1_a_lslv_oracle_32_5_46020020() {
    // Test LSLV 32-bit: all ones, shift 32 (oracle)
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
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "W0 should be 0xFFFFFFFF");
}

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all ones, shift 32 (64)
#[test]
fn test_aarch32_mov_r_t1_a_lslv_oracle_64_5_c6020020() {
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

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift
#[test]
fn test_aarch32_mov_r_t1_a_t16_oracle_0_46100000() {
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

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4
#[test]
fn test_aarch32_mov_r_t1_a_t16_oracle_1_46100000() {
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

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1
#[test]
fn test_aarch32_mov_r_t1_a_t16_oracle_2_46100000() {
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

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift to MSB
#[test]
fn test_aarch32_mov_r_t1_a_t16_oracle_3_46100000() {
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

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_mov_r_t1_a_flags_zeroresult_0_46100000() {
    // Test aarch32_MOV_r_T1_A flag computation: ZeroResult
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
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_mov_r_t1_a_flags_zeroresult_1_46100000() {
    // Test aarch32_MOV_r_T1_A flag computation: ZeroResult
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

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_mov_r_t1_a_flags_negativeresult_2_46100000() {
    // Test aarch32_MOV_r_T1_A flag computation: NegativeResult
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

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_mov_r_t1_a_flags_unsignedoverflow_3_46100000() {
    // Test aarch32_MOV_r_T1_A flag computation: UnsignedOverflow
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

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_mov_r_t1_a_flags_unsignedoverflow_4_46100000() {
    // Test aarch32_MOV_r_T1_A flag computation: UnsignedOverflow
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

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_mov_r_t1_a_flags_signedoverflow_5_46100000() {
    // Test aarch32_MOV_r_T1_A flag computation: SignedOverflow
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

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_mov_r_t1_a_flags_signedoverflow_6_46100000() {
    // Test aarch32_MOV_r_T1_A flag computation: SignedOverflow
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
}

/// Provenance: aarch32_MOV_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_mov_r_t1_a_flags_positiveresult_7_46100000() {
    // Test aarch32_MOV_r_T1_A flag computation: PositiveResult
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
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 0 (32)
#[test]
fn test_aarch32_mov_r_t2_a_lslv_oracle_32_0_00020020() {
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

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 0 (64)
#[test]
fn test_aarch32_mov_r_t2_a_lslv_oracle_64_0_80020020() {
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

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4 (32)
#[test]
fn test_aarch32_mov_r_t2_a_lslv_oracle_32_1_00020020() {
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

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 4 (64)
#[test]
fn test_aarch32_mov_r_t2_a_lslv_oracle_64_1_80020020() {
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

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 8 (32)
#[test]
fn test_aarch32_mov_r_t2_a_lslv_oracle_32_2_00020020() {
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

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 8 (64)
#[test]
fn test_aarch32_mov_r_t2_a_lslv_oracle_64_2_80020020() {
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

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1 (32)
#[test]
fn test_aarch32_mov_r_t2_a_lslv_oracle_32_3_00020020() {
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

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// MSB set, shift 1 (64)
#[test]
fn test_aarch32_mov_r_t2_a_lslv_oracle_64_3_80020020() {
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

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSB set, max shift (32)
#[test]
fn test_aarch32_mov_r_t2_a_lslv_oracle_32_4_00020020() {
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

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSB set, max shift (64)
#[test]
fn test_aarch32_mov_r_t2_a_lslv_oracle_64_4_80020020() {
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

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// all ones, shift 32 (32)
#[test]
fn test_aarch32_mov_r_t2_a_lslv_oracle_32_5_00020020() {
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

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all ones, shift 32 (64)
#[test]
fn test_aarch32_mov_r_t2_a_lslv_oracle_64_5_80020020() {
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

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift
#[test]
fn test_aarch32_mov_r_t2_a_t16_oracle_0_00100000() {
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

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4
#[test]
fn test_aarch32_mov_r_t2_a_t16_oracle_1_00100000() {
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

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1
#[test]
fn test_aarch32_mov_r_t2_a_t16_oracle_2_00100000() {
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

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift to MSB
#[test]
fn test_aarch32_mov_r_t2_a_t16_oracle_3_00100000() {
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

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_mov_r_t2_a_flags_zeroresult_0_00100000() {
    // Test aarch32_MOV_r_T2_A flag computation: ZeroResult
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
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_mov_r_t2_a_flags_zeroresult_1_00100000() {
    // Test aarch32_MOV_r_T2_A flag computation: ZeroResult
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

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_mov_r_t2_a_flags_negativeresult_2_00100000() {
    // Test aarch32_MOV_r_T2_A flag computation: NegativeResult
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

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_mov_r_t2_a_flags_unsignedoverflow_3_00100000() {
    // Test aarch32_MOV_r_T2_A flag computation: UnsignedOverflow
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
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_mov_r_t2_a_flags_unsignedoverflow_4_00100000() {
    // Test aarch32_MOV_r_T2_A flag computation: UnsignedOverflow
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
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_mov_r_t2_a_flags_signedoverflow_5_00100000() {
    // Test aarch32_MOV_r_T2_A flag computation: SignedOverflow
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
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_mov_r_t2_a_flags_signedoverflow_6_00100000() {
    // Test aarch32_MOV_r_T2_A flag computation: SignedOverflow
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
}

/// Provenance: aarch32_MOV_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_mov_r_t2_a_flags_positiveresult_7_00100000() {
    // Test aarch32_MOV_r_T2_A flag computation: PositiveResult
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

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_mov_r_t3_a_flags_zeroresult_0_ea5f0002() {
    // Test aarch32_MOV_r_T3_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xEA5F0002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_mov_r_t3_a_flags_zeroresult_1_ea5f0002() {
    // Test aarch32_MOV_r_T3_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0xEA5F0002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_mov_r_t3_a_flags_negativeresult_2_ea5f0002() {
    // Test aarch32_MOV_r_T3_A flag computation: NegativeResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xEA5F0002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_mov_r_t3_a_flags_unsignedoverflow_3_ea5f0002() {
    // Test aarch32_MOV_r_T3_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xEA5F0002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_mov_r_t3_a_flags_unsignedoverflow_4_ea5f0002() {
    // Test aarch32_MOV_r_T3_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x2);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xEA5F0002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_mov_r_t3_a_flags_signedoverflow_5_ea5f0002() {
    // Test aarch32_MOV_r_T3_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xEA5F0002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_mov_r_t3_a_flags_signedoverflow_6_ea5f0002() {
    // Test aarch32_MOV_r_T3_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xEA5F0002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_r_T3_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_mov_r_t3_a_flags_positiveresult_7_ea5f0002() {
    // Test aarch32_MOV_r_T3_A flag computation: PositiveResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x32);
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xEA5F0002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

// ============================================================================
// aarch32_MOVT_A Tests
// ============================================================================

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_movt_a1_a_field_cond_0_min_0_03400000() {
    // Encoding: 0x03400000
    // Test aarch32_MOVT_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, cond=0, imm4=0, imm12=0
    let encoding: u32 = 0x03400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_movt_a1_a_field_cond_1_poweroftwo_0_13400000() {
    // Encoding: 0x13400000
    // Test aarch32_MOVT_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: imm4=0, Rd=0, imm12=0, cond=1
    let encoding: u32 = 0x13400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_movt_a1_a_field_cond_2_poweroftwo_0_23400000() {
    // Encoding: 0x23400000
    // Test aarch32_MOVT_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=2, imm4=0, imm12=0
    let encoding: u32 = 0x23400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_movt_a1_a_field_cond_3_poweroftwo_0_33400000() {
    // Encoding: 0x33400000
    // Test aarch32_MOVT_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, imm4=0, imm12=0, Rd=0
    let encoding: u32 = 0x33400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_movt_a1_a_field_cond_4_poweroftwo_0_43400000() {
    // Encoding: 0x43400000
    // Test aarch32_MOVT_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, imm12=0, imm4=0, cond=4
    let encoding: u32 = 0x43400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_movt_a1_a_field_cond_5_poweroftwo_0_53400000() {
    // Encoding: 0x53400000
    // Test aarch32_MOVT_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, cond=5, Rd=0, imm4=0
    let encoding: u32 = 0x53400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_movt_a1_a_field_cond_6_poweroftwo_0_63400000() {
    // Encoding: 0x63400000
    // Test aarch32_MOVT_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, Rd=0, imm4=0, imm12=0
    let encoding: u32 = 0x63400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_movt_a1_a_field_cond_7_poweroftwo_0_73400000() {
    // Encoding: 0x73400000
    // Test aarch32_MOVT_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=7, imm4=0, imm12=0
    let encoding: u32 = 0x73400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_movt_a1_a_field_cond_8_poweroftwo_0_83400000() {
    // Encoding: 0x83400000
    // Test aarch32_MOVT_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: imm4=0, Rd=0, imm12=0, cond=8
    let encoding: u32 = 0x83400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_movt_a1_a_field_cond_9_poweroftwo_0_93400000() {
    // Encoding: 0x93400000
    // Test aarch32_MOVT_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: imm4=0, cond=9, Rd=0, imm12=0
    let encoding: u32 = 0x93400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_movt_a1_a_field_cond_10_poweroftwo_0_a3400000() {
    // Encoding: 0xA3400000
    // Test aarch32_MOVT_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, cond=10, imm4=0, Rd=0
    let encoding: u32 = 0xA3400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_movt_a1_a_field_cond_11_poweroftwo_0_b3400000() {
    // Encoding: 0xB3400000
    // Test aarch32_MOVT_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, imm12=0, imm4=0, cond=11
    let encoding: u32 = 0xB3400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_movt_a1_a_field_cond_12_poweroftwo_0_c3400000() {
    // Encoding: 0xC3400000
    // Test aarch32_MOVT_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, imm4=0, imm12=0, Rd=0
    let encoding: u32 = 0xC3400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_movt_a1_a_field_cond_13_poweroftwo_0_d3400000() {
    // Encoding: 0xD3400000
    // Test aarch32_MOVT_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, cond=13, Rd=0, imm4=0
    let encoding: u32 = 0xD3400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_movt_a1_a_field_cond_14_poweroftwo_0_e3400000() {
    // Encoding: 0xE3400000
    // Test aarch32_MOVT_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=14, imm4=0, imm12=0
    let encoding: u32 = 0xE3400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_movt_a1_a_field_cond_15_max_0_f3400000() {
    // Encoding: 0xF3400000
    // Test aarch32_MOVT_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, Rd=0, imm4=0, imm12=0
    let encoding: u32 = 0xF3400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_movt_a1_a_field_imm4_0_zero_0_03400000() {
    // Encoding: 0x03400000
    // Test aarch32_MOVT_A1_A field imm4 = 0 (Zero)
    // ISET: A32
    // Fields: imm4=0, Rd=0, cond=0, imm12=0
    let encoding: u32 = 0x03400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_movt_a1_a_field_imm4_1_poweroftwo_0_03410000() {
    // Encoding: 0x03410000
    // Test aarch32_MOVT_A1_A field imm4 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: imm4=1, imm12=0, Rd=0, cond=0
    let encoding: u32 = 0x03410000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_movt_a1_a_field_imm4_3_poweroftwominusone_0_03430000() {
    // Encoding: 0x03430000
    // Test aarch32_MOVT_A1_A field imm4 = 3 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm4=3, Rd=0, cond=0, imm12=0
    let encoding: u32 = 0x03430000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_movt_a1_a_field_imm4_4_poweroftwo_0_03440000() {
    // Encoding: 0x03440000
    // Test aarch32_MOVT_A1_A field imm4 = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, imm4=4, cond=0, Rd=0
    let encoding: u32 = 0x03440000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch32_movt_a1_a_field_imm4_7_poweroftwominusone_0_03470000() {
    // Encoding: 0x03470000
    // Test aarch32_MOVT_A1_A field imm4 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rd=0, cond=0, imm4=7, imm12=0
    let encoding: u32 = 0x03470000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_movt_a1_a_field_imm4_8_poweroftwo_0_03480000() {
    // Encoding: 0x03480000
    // Test aarch32_MOVT_A1_A field imm4 = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: imm4=8, cond=0, imm12=0, Rd=0
    let encoding: u32 = 0x03480000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch32_movt_a1_a_field_imm4_15_max_0_034f0000() {
    // Encoding: 0x034F0000
    // Test aarch32_MOVT_A1_A field imm4 = 15 (Max)
    // ISET: A32
    // Fields: cond=0, imm12=0, imm4=15, Rd=0
    let encoding: u32 = 0x034F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_movt_a1_a_field_rd_0_min_0_03400000() {
    // Encoding: 0x03400000
    // Test aarch32_MOVT_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: imm12=0, cond=0, Rd=0, imm4=0
    let encoding: u32 = 0x03400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_movt_a1_a_field_rd_1_poweroftwo_0_03401000() {
    // Encoding: 0x03401000
    // Test aarch32_MOVT_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm4=0, imm12=0, Rd=1
    let encoding: u32 = 0x03401000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_movt_a1_a_field_imm12_0_zero_0_03400000() {
    // Encoding: 0x03400000
    // Test aarch32_MOVT_A1_A field imm12 = 0 (Zero)
    // ISET: A32
    // Fields: cond=0, imm12=0, Rd=0, imm4=0
    let encoding: u32 = 0x03400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_movt_a1_a_field_imm12_1_poweroftwo_0_03400001() {
    // Encoding: 0x03400001
    // Test aarch32_MOVT_A1_A field imm12 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: imm4=0, Rd=0, cond=0, imm12=1
    let encoding: u32 = 0x03400001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_movt_a1_a_field_imm12_3_poweroftwominusone_0_03400003() {
    // Encoding: 0x03400003
    // Test aarch32_MOVT_A1_A field imm12 = 3 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm12=3, imm4=0, Rd=0
    let encoding: u32 = 0x03400003;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_movt_a1_a_field_imm12_4_poweroftwo_0_03400004() {
    // Encoding: 0x03400004
    // Test aarch32_MOVT_A1_A field imm12 = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=0, imm12=4, imm4=0
    let encoding: u32 = 0x03400004;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_movt_a1_a_field_imm12_7_poweroftwominusone_0_03400007() {
    // Encoding: 0x03400007
    // Test aarch32_MOVT_A1_A field imm12 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=7, cond=0, Rd=0, imm4=0
    let encoding: u32 = 0x03400007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_movt_a1_a_field_imm12_8_poweroftwo_0_03400008() {
    // Encoding: 0x03400008
    // Test aarch32_MOVT_A1_A field imm12 = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm4=0, imm12=8, Rd=0
    let encoding: u32 = 0x03400008;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_movt_a1_a_field_imm12_15_poweroftwominusone_0_0340000f() {
    // Encoding: 0x0340000F
    // Test aarch32_MOVT_A1_A field imm12 = 15 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=15, imm4=0, Rd=0, cond=0
    let encoding: u32 = 0x0340000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_movt_a1_a_field_imm12_16_poweroftwo_0_03400010() {
    // Encoding: 0x03400010
    // Test aarch32_MOVT_A1_A field imm12 = 16 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=0, imm4=0, imm12=16
    let encoding: u32 = 0x03400010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_movt_a1_a_field_imm12_31_poweroftwominusone_0_0340001f() {
    // Encoding: 0x0340001F
    // Test aarch32_MOVT_A1_A field imm12 = 31 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm4=0, Rd=0, imm12=31
    let encoding: u32 = 0x0340001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_movt_a1_a_field_imm12_32_poweroftwo_0_03400020() {
    // Encoding: 0x03400020
    // Test aarch32_MOVT_A1_A field imm12 = 32 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=32, Rd=0, imm4=0, cond=0
    let encoding: u32 = 0x03400020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_movt_a1_a_field_imm12_63_poweroftwominusone_0_0340003f() {
    // Encoding: 0x0340003F
    // Test aarch32_MOVT_A1_A field imm12 = 63 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm4=0, Rd=0, imm12=63
    let encoding: u32 = 0x0340003F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_movt_a1_a_field_imm12_64_poweroftwo_0_03400040() {
    // Encoding: 0x03400040
    // Test aarch32_MOVT_A1_A field imm12 = 64 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, imm4=0, imm12=64, cond=0
    let encoding: u32 = 0x03400040;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch32_movt_a1_a_field_imm12_127_poweroftwominusone_0_0340007f() {
    // Encoding: 0x0340007F
    // Test aarch32_MOVT_A1_A field imm12 = 127 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm4=0, cond=0, Rd=0, imm12=127
    let encoding: u32 = 0x0340007F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_movt_a1_a_field_imm12_128_poweroftwo_0_03400080() {
    // Encoding: 0x03400080
    // Test aarch32_MOVT_A1_A field imm12 = 128 (PowerOfTwo)
    // ISET: A32
    // Fields: imm4=0, cond=0, Rd=0, imm12=128
    let encoding: u32 = 0x03400080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch32_movt_a1_a_field_imm12_255_poweroftwominusone_0_034000ff() {
    // Encoding: 0x034000FF
    // Test aarch32_MOVT_A1_A field imm12 = 255 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rd=0, imm4=0, imm12=255, cond=0
    let encoding: u32 = 0x034000FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch32_movt_a1_a_field_imm12_256_poweroftwo_0_03400100() {
    // Encoding: 0x03400100
    // Test aarch32_MOVT_A1_A field imm12 = 256 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, imm4=0, imm12=256, cond=0
    let encoding: u32 = 0x03400100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch32_movt_a1_a_field_imm12_511_poweroftwominusone_0_034001ff() {
    // Encoding: 0x034001FF
    // Test aarch32_MOVT_A1_A field imm12 = 511 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, Rd=0, imm12=511, imm4=0
    let encoding: u32 = 0x034001FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch32_movt_a1_a_field_imm12_512_poweroftwo_0_03400200() {
    // Encoding: 0x03400200
    // Test aarch32_MOVT_A1_A field imm12 = 512 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=512, imm4=0, cond=0, Rd=0
    let encoding: u32 = 0x03400200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1023, boundary: PowerOfTwoMinusOne }
/// 2^10 - 1 = 1023
#[test]
fn test_aarch32_movt_a1_a_field_imm12_1023_poweroftwominusone_0_034003ff() {
    // Encoding: 0x034003FF
    // Test aarch32_MOVT_A1_A field imm12 = 1023 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=1023, cond=0, imm4=0, Rd=0
    let encoding: u32 = 0x034003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch32_movt_a1_a_field_imm12_1024_poweroftwo_0_03400400() {
    // Encoding: 0x03400400
    // Test aarch32_MOVT_A1_A field imm12 = 1024 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=1024, cond=0, imm4=0, Rd=0
    let encoding: u32 = 0x03400400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 2047, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (2047)
#[test]
fn test_aarch32_movt_a1_a_field_imm12_2047_poweroftwominusone_0_034007ff() {
    // Encoding: 0x034007FF
    // Test aarch32_MOVT_A1_A field imm12 = 2047 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rd=0, cond=0, imm12=2047, imm4=0
    let encoding: u32 = 0x034007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 2048, boundary: PowerOfTwo }
/// power of 2 (2^11 = 2048)
#[test]
fn test_aarch32_movt_a1_a_field_imm12_2048_poweroftwo_0_03400800() {
    // Encoding: 0x03400800
    // Test aarch32_MOVT_A1_A field imm12 = 2048 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=0, imm12=2048, imm4=0
    let encoding: u32 = 0x03400800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 4095, boundary: Max }
/// maximum immediate (4095)
#[test]
fn test_aarch32_movt_a1_a_field_imm12_4095_max_0_03400fff() {
    // Encoding: 0x03400FFF
    // Test aarch32_MOVT_A1_A field imm12 = 4095 (Max)
    // ISET: A32
    // Fields: imm4=0, Rd=0, cond=0, imm12=4095
    let encoding: u32 = 0x03400FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_movt_a1_a_combo_0_0_03400000() {
    // Encoding: 0x03400000
    // Test aarch32_MOVT_A1_A field combination: cond=0, imm4=0, Rd=0, imm12=0
    // ISET: A32
    // Fields: cond=0, Rd=0, imm4=0, imm12=0
    let encoding: u32 = 0x03400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_movt_a1_a_special_cond_0_condition_eq_0_03400000() {
    // Encoding: 0x03400000
    // Test aarch32_MOVT_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rd=0, imm12=0, imm4=0, cond=0
    let encoding: u32 = 0x03400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_movt_a1_a_special_cond_1_condition_ne_0_13400000() {
    // Encoding: 0x13400000
    // Test aarch32_MOVT_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, imm12=0, Rd=0, imm4=0
    let encoding: u32 = 0x13400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_movt_a1_a_special_cond_2_condition_cs_hs_0_23400000() {
    // Encoding: 0x23400000
    // Test aarch32_MOVT_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, imm12=0, Rd=0, imm4=0
    let encoding: u32 = 0x23400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_movt_a1_a_special_cond_3_condition_cc_lo_0_33400000() {
    // Encoding: 0x33400000
    // Test aarch32_MOVT_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: imm4=0, Rd=0, cond=3, imm12=0
    let encoding: u32 = 0x33400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_movt_a1_a_special_cond_4_condition_mi_0_43400000() {
    // Encoding: 0x43400000
    // Test aarch32_MOVT_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rd=0, imm12=0, imm4=0, cond=4
    let encoding: u32 = 0x43400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_movt_a1_a_special_cond_5_condition_pl_0_53400000() {
    // Encoding: 0x53400000
    // Test aarch32_MOVT_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, Rd=0, imm4=0, imm12=0
    let encoding: u32 = 0x53400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_movt_a1_a_special_cond_6_condition_vs_0_63400000() {
    // Encoding: 0x63400000
    // Test aarch32_MOVT_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, imm12=0, Rd=0, imm4=0
    let encoding: u32 = 0x63400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_movt_a1_a_special_cond_7_condition_vc_0_73400000() {
    // Encoding: 0x73400000
    // Test aarch32_MOVT_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: imm4=0, Rd=0, imm12=0, cond=7
    let encoding: u32 = 0x73400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_movt_a1_a_special_cond_8_condition_hi_0_83400000() {
    // Encoding: 0x83400000
    // Test aarch32_MOVT_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, imm12=0, Rd=0, imm4=0
    let encoding: u32 = 0x83400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_movt_a1_a_special_cond_9_condition_ls_0_93400000() {
    // Encoding: 0x93400000
    // Test aarch32_MOVT_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Rd=0, imm4=0, cond=9, imm12=0
    let encoding: u32 = 0x93400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_movt_a1_a_special_cond_10_condition_ge_0_a3400000() {
    // Encoding: 0xA3400000
    // Test aarch32_MOVT_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, imm12=0, imm4=0, Rd=0
    let encoding: u32 = 0xA3400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_movt_a1_a_special_cond_11_condition_lt_0_b3400000() {
    // Encoding: 0xB3400000
    // Test aarch32_MOVT_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, imm4=0, Rd=0, imm12=0
    let encoding: u32 = 0xB3400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_movt_a1_a_special_cond_12_condition_gt_0_c3400000() {
    // Encoding: 0xC3400000
    // Test aarch32_MOVT_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rd=0, imm12=0, cond=12, imm4=0
    let encoding: u32 = 0xC3400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_movt_a1_a_special_cond_13_condition_le_0_d3400000() {
    // Encoding: 0xD3400000
    // Test aarch32_MOVT_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: imm4=0, Rd=0, cond=13, imm12=0
    let encoding: u32 = 0xD3400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_movt_a1_a_special_cond_14_condition_al_0_e3400000() {
    // Encoding: 0xE3400000
    // Test aarch32_MOVT_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rd=0, imm4=0, cond=14, imm12=0
    let encoding: u32 = 0xE3400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_movt_a1_a_special_cond_15_condition_nv_0_f3400000() {
    // Encoding: 0xF3400000
    // Test aarch32_MOVT_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, imm4=0, Rd=0, imm12=0
    let encoding: u32 = 0xF3400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_movt_a1_a_invalid_0_0_03400000() {
    // Encoding: 0x03400000
    // Test aarch32_MOVT_A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: LitInt(15) }
    // ISET: A32
    // Fields: cond=0, imm4=0, Rd=0, imm12=0
    let encoding: u32 = 0x03400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_movt_a1_a_invalid_1_0_03400000() {
    // Encoding: 0x03400000
    // Test aarch32_MOVT_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rd=0, imm4=0, imm12=0, cond=0
    let encoding: u32 = 0x03400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field i 26 +: 1`
/// Requirement: FieldBoundary { field: "i", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_movt_t1_a_field_i_0_min_0_f2c00000() {
    // Thumb encoding (32): 0xF2C00000
    // Test aarch32_MOVT_T1_A field i = 0 (Min)
    // ISET: T32
    // Fields: i=0, imm4=0, imm3=0, imm8=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field i 26 +: 1`
/// Requirement: FieldBoundary { field: "i", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_movt_t1_a_field_i_1_max_0_f6c00000() {
    // Thumb encoding (32): 0xF6C00000
    // Test aarch32_MOVT_T1_A field i = 1 (Max)
    // ISET: T32
    // Fields: imm3=0, imm8=0, Rd=0, i=1, imm4=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF6C00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_movt_t1_a_field_imm4_0_zero_0_f2c00000() {
    // Thumb encoding (32): 0xF2C00000
    // Test aarch32_MOVT_T1_A field imm4 = 0 (Zero)
    // ISET: T32
    // Fields: imm8=0, i=0, imm3=0, imm4=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_movt_t1_a_field_imm4_1_poweroftwo_0_f2c10000() {
    // Thumb encoding (32): 0xF2C10000
    // Test aarch32_MOVT_T1_A field imm4 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm4=1, imm8=0, Rd=0, i=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C10000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_movt_t1_a_field_imm4_3_poweroftwominusone_0_f2c30000() {
    // Thumb encoding (32): 0xF2C30000
    // Test aarch32_MOVT_T1_A field imm4 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm4=3, imm8=0, i=0, imm3=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C30000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_movt_t1_a_field_imm4_4_poweroftwo_0_f2c40000() {
    // Thumb encoding (32): 0xF2C40000
    // Test aarch32_MOVT_T1_A field imm4 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=0, imm4=4, imm3=0, Rd=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C40000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch32_movt_t1_a_field_imm4_7_poweroftwominusone_0_f2c70000() {
    // Thumb encoding (32): 0xF2C70000
    // Test aarch32_MOVT_T1_A field imm4 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm4=7, imm3=0, Rd=0, imm8=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C70000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_movt_t1_a_field_imm4_8_poweroftwo_0_f2c80000() {
    // Thumb encoding (32): 0xF2C80000
    // Test aarch32_MOVT_T1_A field imm4 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, imm4=8, imm8=0, imm3=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C80000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch32_movt_t1_a_field_imm4_15_max_0_f2cf0000() {
    // Thumb encoding (32): 0xF2CF0000
    // Test aarch32_MOVT_T1_A field imm4 = 15 (Max)
    // ISET: T32
    // Fields: i=0, imm3=0, Rd=0, imm4=15, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2CF0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_movt_t1_a_field_imm3_0_zero_0_f2c00000() {
    // Thumb encoding (32): 0xF2C00000
    // Test aarch32_MOVT_T1_A field imm3 = 0 (Zero)
    // ISET: T32
    // Fields: i=0, imm8=0, Rd=0, imm4=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_movt_t1_a_field_imm3_1_poweroftwo_0_f2c01000() {
    // Thumb encoding (32): 0xF2C01000
    // Test aarch32_MOVT_T1_A field imm3 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=0, Rd=0, i=0, imm3=1, imm4=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C01000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch32_movt_t1_a_field_imm3_3_poweroftwominusone_0_f2c03000() {
    // Thumb encoding (32): 0xF2C03000
    // Test aarch32_MOVT_T1_A field imm3 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm4=0, imm8=0, imm3=3, Rd=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C03000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch32_movt_t1_a_field_imm3_7_max_0_f2c07000() {
    // Thumb encoding (32): 0xF2C07000
    // Test aarch32_MOVT_T1_A field imm3 = 7 (Max)
    // ISET: T32
    // Fields: i=0, imm3=7, imm8=0, Rd=0, imm4=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C07000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_movt_t1_a_field_rd_0_min_0_f2c00000() {
    // Thumb encoding (32): 0xF2C00000
    // Test aarch32_MOVT_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: imm3=0, i=0, imm4=0, Rd=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_movt_t1_a_field_rd_1_poweroftwo_0_f2c00100() {
    // Thumb encoding (32): 0xF2C00100
    // Test aarch32_MOVT_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=0, imm4=0, Rd=1, i=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C00100;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_movt_t1_a_field_imm8_0_zero_0_f2c00000() {
    // Thumb encoding (32): 0xF2C00000
    // Test aarch32_MOVT_T1_A field imm8 = 0 (Zero)
    // ISET: T32
    // Fields: imm4=0, Rd=0, imm8=0, i=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_movt_t1_a_field_imm8_1_poweroftwo_0_f2c00001() {
    // Thumb encoding (32): 0xF2C00001
    // Test aarch32_MOVT_T1_A field imm8 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm4=0, Rd=0, imm8=1, i=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C00001;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_movt_t1_a_field_imm8_3_poweroftwominusone_0_f2c00003() {
    // Thumb encoding (32): 0xF2C00003
    // Test aarch32_MOVT_T1_A field imm8 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: i=0, imm3=0, imm4=0, Rd=0, imm8=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C00003;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_movt_t1_a_field_imm8_4_poweroftwo_0_f2c00004() {
    // Thumb encoding (32): 0xF2C00004
    // Test aarch32_MOVT_T1_A field imm8 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: i=0, imm4=0, imm3=0, Rd=0, imm8=4
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C00004;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_movt_t1_a_field_imm8_7_poweroftwominusone_0_f2c00007() {
    // Thumb encoding (32): 0xF2C00007
    // Test aarch32_MOVT_T1_A field imm8 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=7, i=0, imm4=0, imm3=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C00007;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_movt_t1_a_field_imm8_8_poweroftwo_0_f2c00008() {
    // Thumb encoding (32): 0xF2C00008
    // Test aarch32_MOVT_T1_A field imm8 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, imm8=8, i=0, imm4=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C00008;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_movt_t1_a_field_imm8_15_poweroftwominusone_0_f2c0000f() {
    // Thumb encoding (32): 0xF2C0000F
    // Test aarch32_MOVT_T1_A field imm8 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm3=0, imm4=0, i=0, imm8=15, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C0000F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_movt_t1_a_field_imm8_16_poweroftwo_0_f2c00010() {
    // Thumb encoding (32): 0xF2C00010
    // Test aarch32_MOVT_T1_A field imm8 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=0, imm8=16, i=0, imm4=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C00010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_movt_t1_a_field_imm8_31_poweroftwominusone_0_f2c0001f() {
    // Thumb encoding (32): 0xF2C0001F
    // Test aarch32_MOVT_T1_A field imm8 = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=31, Rd=0, imm4=0, i=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C0001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_movt_t1_a_field_imm8_32_poweroftwo_0_f2c00020() {
    // Thumb encoding (32): 0xF2C00020
    // Test aarch32_MOVT_T1_A field imm8 = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: i=0, imm4=0, imm3=0, imm8=32, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C00020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_movt_t1_a_field_imm8_63_poweroftwominusone_0_f2c0003f() {
    // Thumb encoding (32): 0xF2C0003F
    // Test aarch32_MOVT_T1_A field imm8 = 63 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=63, imm3=0, imm4=0, Rd=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C0003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_movt_t1_a_field_imm8_64_poweroftwo_0_f2c00040() {
    // Thumb encoding (32): 0xF2C00040
    // Test aarch32_MOVT_T1_A field imm8 = 64 (PowerOfTwo)
    // ISET: T32
    // Fields: i=0, imm4=0, Rd=0, imm3=0, imm8=64
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C00040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 127, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (127)
#[test]
fn test_aarch32_movt_t1_a_field_imm8_127_poweroftwominusone_0_f2c0007f() {
    // Thumb encoding (32): 0xF2C0007F
    // Test aarch32_MOVT_T1_A field imm8 = 127 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: i=0, Rd=0, imm8=127, imm3=0, imm4=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C0007F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_movt_t1_a_field_imm8_128_poweroftwo_0_f2c00080() {
    // Thumb encoding (32): 0xF2C00080
    // Test aarch32_MOVT_T1_A field imm8 = 128 (PowerOfTwo)
    // ISET: T32
    // Fields: i=0, imm3=0, imm4=0, imm8=128, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C00080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 255, boundary: Max }
/// maximum immediate (255)
#[test]
fn test_aarch32_movt_t1_a_field_imm8_255_max_0_f2c000ff() {
    // Thumb encoding (32): 0xF2C000FF
    // Test aarch32_MOVT_T1_A field imm8 = 255 (Max)
    // ISET: T32
    // Fields: imm4=0, Rd=0, i=0, imm3=0, imm8=255
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C000FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// i=0 (minimum value)
#[test]
fn test_aarch32_movt_t1_a_combo_0_0_f2c00000() {
    // Thumb encoding (32): 0xF2C00000
    // Test aarch32_MOVT_T1_A field combination: i=0, imm4=0, imm3=0, Rd=0, imm8=0
    // ISET: T32
    // Fields: imm8=0, i=0, imm4=0, imm3=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_movt_t1_a_invalid_0_0_f2c00000() {
    // Thumb encoding (32): 0xF2C00000
    // Test aarch32_MOVT_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: LitInt(15) }
    // ISET: T32
    // Fields: imm3=0, Rd=0, imm4=0, imm8=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C00000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_MOVT_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_movt_t1_a_invalid_1_0_f2c00000() {
    // Thumb encoding (32): 0xF2C00000
    // Test aarch32_MOVT_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: imm4=0, imm3=0, Rd=0, i=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C00000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// zero == zero (32-bit)
#[test]
fn test_aarch32_movt_a1_a_oracle_32_0_6b02003f() {
    // Test CMP 32-bit: zero == zero (oracle)
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

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// zero == zero (64-bit)
#[test]
fn test_aarch32_movt_a1_a_oracle_64_0_eb02003f() {
    // Test CMP 64-bit: zero == zero (oracle)
    // Encoding: 0xEB02003F
    // ISET: A32
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

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// equal values (32-bit)
#[test]
fn test_aarch32_movt_a1_a_oracle_32_1_6b02003f() {
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

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// equal values (64-bit)
#[test]
fn test_aarch32_movt_a1_a_oracle_64_1_eb02003f() {
    // Test CMP 64-bit: equal values (oracle)
    // Encoding: 0xEB02003F
    // ISET: A32
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

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// greater than (32-bit)
#[test]
fn test_aarch32_movt_a1_a_oracle_32_2_6b02003f() {
    // Test CMP 32-bit: greater than (oracle)
    // Encoding: 0x6B02003F
    // ISET: A32
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

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// greater than (64-bit)
#[test]
fn test_aarch32_movt_a1_a_oracle_64_2_eb02003f() {
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

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// less than (32-bit)
#[test]
fn test_aarch32_movt_a1_a_oracle_32_3_6b02003f() {
    // Test CMP 32-bit: less than (oracle)
    // Encoding: 0x6B02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x5);
    set_w(&mut cpu, 2, 0xA);
    let encoding: u32 = 0x6B02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// less than (64-bit)
#[test]
fn test_aarch32_movt_a1_a_oracle_64_3_eb02003f() {
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

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// max value (32-bit)
#[test]
fn test_aarch32_movt_a1_a_oracle_32_4_6b02003f() {
    // Test CMP 32-bit: max value (oracle)
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

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// max value (64-bit)
#[test]
fn test_aarch32_movt_a1_a_oracle_64_4_eb02003f() {
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

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// negative (MSB set) (32-bit)
#[test]
fn test_aarch32_movt_a1_a_oracle_32_5_6b02003f() {
    // Test CMP 32-bit: negative (MSB set) (oracle)
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

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// negative (MSB set) (64-bit)
#[test]
fn test_aarch32_movt_a1_a_oracle_64_5_eb02003f() {
    // Test CMP 64-bit: negative (MSB set) (oracle)
    // Encoding: 0xEB02003F
    // ISET: A32
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

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// overflow boundary (32-bit)
#[test]
fn test_aarch32_movt_a1_a_oracle_32_6_6b02003f() {
    // Test CMP 32-bit: overflow boundary (oracle)
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

/// Provenance: aarch32_MOVT_A1_A
/// ASL: `CMP X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// overflow boundary (64-bit)
#[test]
fn test_aarch32_movt_a1_a_oracle_64_6_eb02003f() {
    // Test CMP 64-bit: overflow boundary (oracle)
    // Encoding: 0xEB02003F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xEB02003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

// ============================================================================
// aarch32_MOV_rr_A Tests
// ============================================================================

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_mov_rr_a1_a_field_cond_0_min_10_01a00010() {
    // Encoding: 0x01A00010
    // Test aarch32_MOV_rr_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: type1=0, Rm=0, cond=0, S=0, Rd=0, Rs=0
    let encoding: u32 = 0x01A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_mov_rr_a1_a_field_cond_1_poweroftwo_10_11a00010() {
    // Encoding: 0x11A00010
    // Test aarch32_MOV_rr_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, S=0, Rm=0, Rd=0, Rs=0, type1=0
    let encoding: u32 = 0x11A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_mov_rr_a1_a_field_cond_2_poweroftwo_10_21a00010() {
    // Encoding: 0x21A00010
    // Test aarch32_MOV_rr_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=2, Rs=0, type1=0, Rm=0, Rd=0, S=0
    let encoding: u32 = 0x21A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_mov_rr_a1_a_field_cond_3_poweroftwo_10_31a00010() {
    // Encoding: 0x31A00010
    // Test aarch32_MOV_rr_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, Rs=0, type1=0, Rm=0, Rd=0, cond=3
    let encoding: u32 = 0x31A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_mov_rr_a1_a_field_cond_4_poweroftwo_10_41a00010() {
    // Encoding: 0x41A00010
    // Test aarch32_MOV_rr_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rs=0, Rm=0, Rd=0, S=0, cond=4, type1=0
    let encoding: u32 = 0x41A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_mov_rr_a1_a_field_cond_5_poweroftwo_10_51a00010() {
    // Encoding: 0x51A00010
    // Test aarch32_MOV_rr_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rs=0, Rm=0, cond=5, S=0, type1=0
    let encoding: u32 = 0x51A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_mov_rr_a1_a_field_cond_6_poweroftwo_10_61a00010() {
    // Encoding: 0x61A00010
    // Test aarch32_MOV_rr_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, type1=0, S=0, Rd=0, Rs=0, Rm=0
    let encoding: u32 = 0x61A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_mov_rr_a1_a_field_cond_7_poweroftwo_10_71a00010() {
    // Encoding: 0x71A00010
    // Test aarch32_MOV_rr_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, S=0, Rs=0, cond=7, type1=0, Rd=0
    let encoding: u32 = 0x71A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_mov_rr_a1_a_field_cond_8_poweroftwo_10_81a00010() {
    // Encoding: 0x81A00010
    // Test aarch32_MOV_rr_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rm=0, cond=8, S=0, Rd=0, Rs=0
    let encoding: u32 = 0x81A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_mov_rr_a1_a_field_cond_9_poweroftwo_10_91a00010() {
    // Encoding: 0x91A00010
    // Test aarch32_MOV_rr_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rs=0, S=0, Rm=0, Rd=0, cond=9
    let encoding: u32 = 0x91A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_mov_rr_a1_a_field_cond_10_poweroftwo_10_a1a00010() {
    // Encoding: 0xA1A00010
    // Test aarch32_MOV_rr_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, type1=0, S=0, cond=10, Rd=0, Rs=0
    let encoding: u32 = 0xA1A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_mov_rr_a1_a_field_cond_11_poweroftwo_10_b1a00010() {
    // Encoding: 0xB1A00010
    // Test aarch32_MOV_rr_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, Rd=0, Rs=0, type1=0, Rm=0, cond=11
    let encoding: u32 = 0xB1A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_mov_rr_a1_a_field_cond_12_poweroftwo_10_c1a00010() {
    // Encoding: 0xC1A00010
    // Test aarch32_MOV_rr_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rs=0, Rd=0, cond=12, Rm=0, S=0, type1=0
    let encoding: u32 = 0xC1A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_mov_rr_a1_a_field_cond_13_poweroftwo_10_d1a00010() {
    // Encoding: 0xD1A00010
    // Test aarch32_MOV_rr_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, Rs=0, Rm=0, cond=13, type1=0, Rd=0
    let encoding: u32 = 0xD1A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_mov_rr_a1_a_field_cond_14_poweroftwo_10_e1a00010() {
    // Encoding: 0xE1A00010
    // Test aarch32_MOV_rr_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, S=0, Rs=0, type1=0, Rd=0, cond=14
    let encoding: u32 = 0xE1A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_mov_rr_a1_a_field_cond_15_max_10_f1a00010() {
    // Encoding: 0xF1A00010
    // Test aarch32_MOV_rr_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rs=0, Rm=0, type1=0, cond=15, Rd=0, S=0
    let encoding: u32 = 0xF1A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_mov_rr_a1_a_field_s_0_min_10_01a00010() {
    // Encoding: 0x01A00010
    // Test aarch32_MOV_rr_A1_A field S = 0 (Min)
    // ISET: A32
    // Fields: cond=0, S=0, Rd=0, Rm=0, Rs=0, type1=0
    let encoding: u32 = 0x01A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_mov_rr_a1_a_field_s_1_max_10_01b00010() {
    // Encoding: 0x01B00010
    // Test aarch32_MOV_rr_A1_A field S = 1 (Max)
    // ISET: A32
    // Fields: type1=0, Rs=0, Rm=0, cond=0, Rd=0, S=1
    let encoding: u32 = 0x01B00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mov_rr_a1_a_field_rd_0_min_10_01a00010() {
    // Encoding: 0x01A00010
    // Test aarch32_MOV_rr_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: type1=0, Rm=0, cond=0, S=0, Rd=0, Rs=0
    let encoding: u32 = 0x01A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mov_rr_a1_a_field_rd_1_poweroftwo_10_01a01010() {
    // Encoding: 0x01A01010
    // Test aarch32_MOV_rr_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, S=0, Rs=0, type1=0, Rm=0, Rd=1
    let encoding: u32 = 0x01A01010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field Rs 8 +: 4`
/// Requirement: FieldBoundary { field: "Rs", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mov_rr_a1_a_field_rs_0_min_10_01a00010() {
    // Encoding: 0x01A00010
    // Test aarch32_MOV_rr_A1_A field Rs = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, cond=0, type1=0, Rs=0, S=0, Rd=0
    let encoding: u32 = 0x01A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field Rs 8 +: 4`
/// Requirement: FieldBoundary { field: "Rs", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mov_rr_a1_a_field_rs_1_poweroftwo_10_01a00110() {
    // Encoding: 0x01A00110
    // Test aarch32_MOV_rr_A1_A field Rs = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, cond=0, Rm=0, Rd=0, S=0, Rs=1
    let encoding: u32 = 0x01A00110;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mov_rr_a1_a_field_type1_0_min_10_01a00010() {
    // Encoding: 0x01A00010
    // Test aarch32_MOV_rr_A1_A field type1 = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, Rs=0, S=0, cond=0, Rd=0, type1=0
    let encoding: u32 = 0x01A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mov_rr_a1_a_field_type1_1_poweroftwo_10_01a00030() {
    // Encoding: 0x01A00030
    // Test aarch32_MOV_rr_A1_A field type1 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=1, Rd=0, Rs=0, cond=0, S=0, Rm=0
    let encoding: u32 = 0x01A00030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_mov_rr_a1_a_field_type1_3_max_10_01a00070() {
    // Encoding: 0x01A00070
    // Test aarch32_MOV_rr_A1_A field type1 = 3 (Max)
    // ISET: A32
    // Fields: type1=3, cond=0, Rm=0, Rs=0, Rd=0, S=0
    let encoding: u32 = 0x01A00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mov_rr_a1_a_field_rm_0_min_10_01a00010() {
    // Encoding: 0x01A00010
    // Test aarch32_MOV_rr_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, S=0, type1=0, Rm=0, cond=0, Rs=0
    let encoding: u32 = 0x01A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mov_rr_a1_a_field_rm_1_poweroftwo_10_01a00011() {
    // Encoding: 0x01A00011
    // Test aarch32_MOV_rr_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=1, S=0, type1=0, Rd=0, Rs=0, cond=0
    let encoding: u32 = 0x01A00011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_mov_rr_a1_a_combo_0_10_01a00010() {
    // Encoding: 0x01A00010
    // Test aarch32_MOV_rr_A1_A field combination: cond=0, S=0, Rd=0, Rs=0, type1=0, Rm=0
    // ISET: A32
    // Fields: type1=0, Rd=0, cond=0, S=0, Rs=0, Rm=0
    let encoding: u32 = 0x01A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_mov_rr_a1_a_special_cond_0_condition_eq_16_01a00010() {
    // Encoding: 0x01A00010
    // Test aarch32_MOV_rr_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rs=0, type1=0, Rd=0, S=0, Rm=0, cond=0
    let encoding: u32 = 0x01A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_mov_rr_a1_a_special_cond_1_condition_ne_16_11a00010() {
    // Encoding: 0x11A00010
    // Test aarch32_MOV_rr_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=1, Rs=0, type1=0, S=0
    let encoding: u32 = 0x11A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_mov_rr_a1_a_special_cond_2_condition_cs_hs_16_21a00010() {
    // Encoding: 0x21A00010
    // Test aarch32_MOV_rr_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, Rd=0, Rm=0, S=0, Rs=0, type1=0
    let encoding: u32 = 0x21A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_mov_rr_a1_a_special_cond_3_condition_cc_lo_16_31a00010() {
    // Encoding: 0x31A00010
    // Test aarch32_MOV_rr_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rd=0, Rs=0, type1=0, Rm=0, cond=3, S=0
    let encoding: u32 = 0x31A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_mov_rr_a1_a_special_cond_4_condition_mi_16_41a00010() {
    // Encoding: 0x41A00010
    // Test aarch32_MOV_rr_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: S=0, Rs=0, Rm=0, type1=0, Rd=0, cond=4
    let encoding: u32 = 0x41A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_mov_rr_a1_a_special_cond_5_condition_pl_16_51a00010() {
    // Encoding: 0x51A00010
    // Test aarch32_MOV_rr_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: S=0, type1=0, cond=5, Rd=0, Rs=0, Rm=0
    let encoding: u32 = 0x51A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_mov_rr_a1_a_special_cond_6_condition_vs_16_61a00010() {
    // Encoding: 0x61A00010
    // Test aarch32_MOV_rr_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rd=0, S=0, Rs=0, type1=0, Rm=0, cond=6
    let encoding: u32 = 0x61A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_mov_rr_a1_a_special_cond_7_condition_vc_16_71a00010() {
    // Encoding: 0x71A00010
    // Test aarch32_MOV_rr_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: S=0, cond=7, type1=0, Rm=0, Rs=0, Rd=0
    let encoding: u32 = 0x71A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_mov_rr_a1_a_special_cond_8_condition_hi_16_81a00010() {
    // Encoding: 0x81A00010
    // Test aarch32_MOV_rr_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, Rm=0, Rd=0, type1=0, Rs=0, S=0
    let encoding: u32 = 0x81A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_mov_rr_a1_a_special_cond_9_condition_ls_16_91a00010() {
    // Encoding: 0x91A00010
    // Test aarch32_MOV_rr_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: S=0, Rd=0, Rs=0, type1=0, cond=9, Rm=0
    let encoding: u32 = 0x91A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_mov_rr_a1_a_special_cond_10_condition_ge_16_a1a00010() {
    // Encoding: 0xA1A00010
    // Test aarch32_MOV_rr_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: S=0, type1=0, Rm=0, cond=10, Rd=0, Rs=0
    let encoding: u32 = 0xA1A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_mov_rr_a1_a_special_cond_11_condition_lt_16_b1a00010() {
    // Encoding: 0xB1A00010
    // Test aarch32_MOV_rr_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rs=0, Rm=0, type1=0, S=0, cond=11, Rd=0
    let encoding: u32 = 0xB1A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_mov_rr_a1_a_special_cond_12_condition_gt_16_c1a00010() {
    // Encoding: 0xC1A00010
    // Test aarch32_MOV_rr_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: type1=0, Rm=0, cond=12, Rs=0, Rd=0, S=0
    let encoding: u32 = 0xC1A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_mov_rr_a1_a_special_cond_13_condition_le_16_d1a00010() {
    // Encoding: 0xD1A00010
    // Test aarch32_MOV_rr_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: type1=0, S=0, Rm=0, Rd=0, cond=13, Rs=0
    let encoding: u32 = 0xD1A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_mov_rr_a1_a_special_cond_14_condition_al_16_e1a00010() {
    // Encoding: 0xE1A00010
    // Test aarch32_MOV_rr_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: S=0, cond=14, Rs=0, type1=0, Rd=0, Rm=0
    let encoding: u32 = 0xE1A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_mov_rr_a1_a_special_cond_15_condition_nv_16_f1a00010() {
    // Encoding: 0xF1A00010
    // Test aarch32_MOV_rr_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: S=0, type1=0, Rd=0, Rs=0, Rm=0, cond=15
    let encoding: u32 = 0xF1A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_mov_rr_a1_a_special_s_0_size_variant_0_16_01a00010() {
    // Encoding: 0x01A00010
    // Test aarch32_MOV_rr_A1_A special value S = 0 (Size variant 0)
    // ISET: A32
    // Fields: S=0, type1=0, Rd=0, Rs=0, Rm=0, cond=0
    let encoding: u32 = 0x01A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_mov_rr_a1_a_special_s_1_size_variant_1_16_01b00010() {
    // Encoding: 0x01B00010
    // Test aarch32_MOV_rr_A1_A special value S = 1 (Size variant 1)
    // ISET: A32
    // Fields: Rs=0, S=1, cond=0, Rd=0, type1=0, Rm=0
    let encoding: u32 = 0x01B00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "s" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"s\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mov_rr_a1_a_invalid_0_10_01a00010() {
    // Encoding: 0x01A00010
    // Test aarch32_MOV_rr_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "s" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: cond=0, type1=0, Rs=0, S=0, Rm=0, Rd=0
    let encoding: u32 = 0x01A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mov_rr_a1_a_invalid_1_10_01a00010() {
    // Encoding: 0x01A00010
    // Test aarch32_MOV_rr_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=0, Rs=0, type1=0, S=0
    let encoding: u32 = 0x01A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `field op 22 +: 4`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mov_rr_t1_a_field_op_0_min_0_40000000() {
    // Thumb encoding (32): 0x40000000
    // Test aarch32_MOV_rr_T1_A field op = 0 (Min)
    // ISET: T32
    // Fields: Rs=0, op=0, Rdm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x40000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `field op 22 +: 4`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mov_rr_t1_a_field_op_1_poweroftwo_0_40400000() {
    // Thumb encoding (32): 0x40400000
    // Test aarch32_MOV_rr_T1_A field op = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: op=1, Rdm=0, Rs=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x40400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `field op 22 +: 4`
/// Requirement: FieldBoundary { field: "op", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch32_mov_rr_t1_a_field_op_7_poweroftwominusone_0_41c00000() {
    // Thumb encoding (32): 0x41C00000
    // Test aarch32_MOV_rr_T1_A field op = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rdm=0, Rs=0, op=7
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x41C00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `field op 22 +: 4`
/// Requirement: FieldBoundary { field: "op", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch32_mov_rr_t1_a_field_op_15_max_0_43c00000() {
    // Thumb encoding (32): 0x43C00000
    // Test aarch32_MOV_rr_T1_A field op = 15 (Max)
    // ISET: T32
    // Fields: Rs=0, Rdm=0, op=15
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x43C00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `field Rs 19 +: 3`
/// Requirement: FieldBoundary { field: "Rs", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mov_rr_t1_a_field_rs_0_min_0_40000000() {
    // Thumb encoding (32): 0x40000000
    // Test aarch32_MOV_rr_T1_A field Rs = 0 (Min)
    // ISET: T32
    // Fields: op=0, Rs=0, Rdm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x40000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `field Rs 19 +: 3`
/// Requirement: FieldBoundary { field: "Rs", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mov_rr_t1_a_field_rs_1_poweroftwo_0_40080000() {
    // Thumb encoding (32): 0x40080000
    // Test aarch32_MOV_rr_T1_A field Rs = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rs=1, Rdm=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x40080000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `field Rdm 16 +: 3`
/// Requirement: FieldBoundary { field: "Rdm", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mov_rr_t1_a_field_rdm_0_min_0_40000000() {
    // Thumb encoding (32): 0x40000000
    // Test aarch32_MOV_rr_T1_A field Rdm = 0 (Min)
    // ISET: T32
    // Fields: Rs=0, Rdm=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x40000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `field Rdm 16 +: 3`
/// Requirement: FieldBoundary { field: "Rdm", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mov_rr_t1_a_field_rdm_1_poweroftwo_0_40010000() {
    // Thumb encoding (32): 0x40010000
    // Test aarch32_MOV_rr_T1_A field Rdm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: op=0, Rs=0, Rdm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x40010000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `field Rdm 16 +: 3`
/// Requirement: FieldBoundary { field: "Rdm", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch32_mov_rr_t1_a_field_rdm_7_max_0_40070000() {
    // Thumb encoding (32): 0x40070000
    // Test aarch32_MOV_rr_T1_A field Rdm = 7 (Max)
    // ISET: T32
    // Fields: Rs=0, op=0, Rdm=7
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x40070000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch32_mov_rr_t1_a_combo_0_0_40000000() {
    // Thumb encoding (32): 0x40000000
    // Test aarch32_MOV_rr_T1_A field combination: op=0, Rs=0, Rdm=0
    // ISET: T32
    // Fields: Rdm=0, Rs=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x40000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `field type1 21 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mov_rr_t2_a_field_type1_0_min_f000_fa00f000() {
    // Thumb encoding (32): 0xFA00F000
    // Test aarch32_MOV_rr_T2_A field type1 = 0 (Min)
    // ISET: T32
    // Fields: Rs=0, S=0, type1=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA00F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `field type1 21 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mov_rr_t2_a_field_type1_1_poweroftwo_f000_fa20f000() {
    // Thumb encoding (32): 0xFA20F000
    // Test aarch32_MOV_rr_T2_A field type1 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rs=0, S=0, type1=1, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA20F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `field type1 21 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_mov_rr_t2_a_field_type1_3_max_f000_fa60f000() {
    // Thumb encoding (32): 0xFA60F000
    // Test aarch32_MOV_rr_T2_A field type1 = 3 (Max)
    // ISET: T32
    // Fields: type1=3, Rs=0, Rd=0, Rm=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA60F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_mov_rr_t2_a_field_s_0_min_f000_fa00f000() {
    // Thumb encoding (32): 0xFA00F000
    // Test aarch32_MOV_rr_T2_A field S = 0 (Min)
    // ISET: T32
    // Fields: type1=0, S=0, Rd=0, Rm=0, Rs=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA00F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_mov_rr_t2_a_field_s_1_max_f000_fa10f000() {
    // Thumb encoding (32): 0xFA10F000
    // Test aarch32_MOV_rr_T2_A field S = 1 (Max)
    // ISET: T32
    // Fields: Rd=0, type1=0, Rs=0, Rm=0, S=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA10F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `field Rm 16 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mov_rr_t2_a_field_rm_0_min_f000_fa00f000() {
    // Thumb encoding (32): 0xFA00F000
    // Test aarch32_MOV_rr_T2_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: S=0, Rm=0, Rs=0, type1=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA00F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `field Rm 16 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mov_rr_t2_a_field_rm_1_poweroftwo_f000_fa01f000() {
    // Thumb encoding (32): 0xFA01F000
    // Test aarch32_MOV_rr_T2_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, type1=0, Rd=0, Rm=1, Rs=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA01F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mov_rr_t2_a_field_rd_0_min_f000_fa00f000() {
    // Thumb encoding (32): 0xFA00F000
    // Test aarch32_MOV_rr_T2_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: S=0, Rs=0, type1=0, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA00F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mov_rr_t2_a_field_rd_1_poweroftwo_f000_fa00f100() {
    // Thumb encoding (32): 0xFA00F100
    // Test aarch32_MOV_rr_T2_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, type1=0, Rd=1, Rm=0, Rs=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA00F100;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `field Rs 0 +: 4`
/// Requirement: FieldBoundary { field: "Rs", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mov_rr_t2_a_field_rs_0_min_f000_fa00f000() {
    // Thumb encoding (32): 0xFA00F000
    // Test aarch32_MOV_rr_T2_A field Rs = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rs=0, S=0, Rm=0, type1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA00F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `field Rs 0 +: 4`
/// Requirement: FieldBoundary { field: "Rs", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mov_rr_t2_a_field_rs_1_poweroftwo_f000_fa00f001() {
    // Thumb encoding (32): 0xFA00F001
    // Test aarch32_MOV_rr_T2_A field Rs = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rs=1, Rm=0, type1=0, S=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA00F001;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// type1=0 (minimum value)
#[test]
fn test_aarch32_mov_rr_t2_a_combo_0_f000_fa00f000() {
    // Thumb encoding (32): 0xFA00F000
    // Test aarch32_MOV_rr_T2_A field combination: type1=0, S=0, Rm=0, Rd=0, Rs=0
    // ISET: T32
    // Fields: type1=0, Rd=0, Rs=0, Rm=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA00F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_mov_rr_t2_a_special_s_0_size_variant_0_61440_fa00f000() {
    // Thumb encoding (32): 0xFA00F000
    // Test aarch32_MOV_rr_T2_A special value S = 0 (Size variant 0)
    // ISET: T32
    // Fields: Rs=0, type1=0, S=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA00F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_mov_rr_t2_a_special_s_1_size_variant_1_61440_fa10f000() {
    // Thumb encoding (32): 0xFA10F000
    // Test aarch32_MOV_rr_T2_A special value S = 1 (Size variant 1)
    // ISET: T32
    // Fields: S=1, Rd=0, type1=0, Rm=0, Rs=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA10F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "s" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"s\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mov_rr_t2_a_invalid_0_f000_fa00f000() {
    // Thumb encoding (32): 0xFA00F000
    // Test aarch32_MOV_rr_T2_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "s" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: S=0, Rd=0, Rm=0, Rs=0, type1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA00F000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mov_rr_t2_a_invalid_1_f000_fa00f000() {
    // Thumb encoding (32): 0xFA00F000
    // Test aarch32_MOV_rr_T2_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: S=0, type1=0, Rs=0, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFA00F000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 0 (32)
#[test]
fn test_aarch32_mov_rr_a1_a_lslv_oracle_32_0_01a20030() {
    // Test LSLV 32-bit: shift by 0 (oracle)
    // Encoding: 0x01A20030
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x12345678);
    let encoding: u32 = 0x01A20030;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x12345678, "W0 should be 0x12345678");
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 0 (64)
#[test]
fn test_aarch32_mov_rr_a1_a_lslv_oracle_64_0_81a20030() {
    // Test LSLV 64-bit: shift by 0 (oracle)
    // Encoding: 0x81A20030
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x81A20030;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x12345678,
        "X0 should be 0x0000000012345678"
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4 (32)
#[test]
fn test_aarch32_mov_rr_a1_a_lslv_oracle_32_1_01a20030() {
    // Test LSLV 32-bit: shift by 4 (oracle)
    // Encoding: 0x01A20030
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x4);
    let encoding: u32 = 0x01A20030;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x23456780, "W0 should be 0x23456780");
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 4 (64)
#[test]
fn test_aarch32_mov_rr_a1_a_lslv_oracle_64_1_81a20030() {
    // Test LSLV 64-bit: shift by 4 (oracle)
    // Encoding: 0x81A20030
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x4);
    set_w(&mut cpu, 1, 0x12345678);
    let encoding: u32 = 0x81A20030;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x23456780,
        "X0 should be 0x0000000123456780"
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 8 (32)
#[test]
fn test_aarch32_mov_rr_a1_a_lslv_oracle_32_2_01a20030() {
    // Test LSLV 32-bit: shift by 8 (oracle)
    // Encoding: 0x01A20030
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x8);
    set_w(&mut cpu, 1, 0x12345678);
    let encoding: u32 = 0x01A20030;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x34567800, "W0 should be 0x34567800");
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 8 (64)
#[test]
fn test_aarch32_mov_rr_a1_a_lslv_oracle_64_2_81a20030() {
    // Test LSLV 64-bit: shift by 8 (oracle)
    // Encoding: 0x81A20030
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x8);
    let encoding: u32 = 0x81A20030;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x34567800,
        "X0 should be 0x0000001234567800"
    );
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1 (32)
#[test]
fn test_aarch32_mov_rr_a1_a_lslv_oracle_32_3_01a20030() {
    // Test LSLV 32-bit: MSB set, shift 1 (oracle)
    // Encoding: 0x01A20030
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x01A20030;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x00000000");
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// MSB set, shift 1 (64)
#[test]
fn test_aarch32_mov_rr_a1_a_lslv_oracle_64_3_81a20030() {
    // Test LSLV 64-bit: MSB set, shift 1 (oracle)
    // Encoding: 0x81A20030
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x81A20030;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSB set, max shift (32)
#[test]
fn test_aarch32_mov_rr_a1_a_lslv_oracle_32_4_01a20030() {
    // Test LSLV 32-bit: LSB set, max shift (oracle)
    // Encoding: 0x01A20030
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0x3F);
    let encoding: u32 = 0x01A20030;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x80000000, "W0 should be 0x80000000");
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSB set, max shift (64)
#[test]
fn test_aarch32_mov_rr_a1_a_lslv_oracle_64_4_81a20030() {
    // Test LSLV 64-bit: LSB set, max shift (oracle)
    // Encoding: 0x81A20030
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0x3F);
    let encoding: u32 = 0x81A20030;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x8000000000000000");
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// all ones, shift 32 (32)
#[test]
fn test_aarch32_mov_rr_a1_a_lslv_oracle_32_5_01a20030() {
    // Test LSLV 32-bit: all ones, shift 32 (oracle)
    // Encoding: 0x01A20030
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x20);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x01A20030;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "W0 should be 0xFFFFFFFF");
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all ones, shift 32 (64)
#[test]
fn test_aarch32_mov_rr_a1_a_lslv_oracle_64_5_81a20030() {
    // Test LSLV 64-bit: all ones, shift 32 (oracle)
    // Encoding: 0x81A20030
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x20);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x81A20030;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0xFFFFFFFF00000000");
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_mov_rr_a1_a_flags_zeroresult_0_01b00012() {
    // Test aarch32_MOV_rr_A1_A flag computation: ZeroResult
    // Encoding: 0x01B00012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x01B00012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_mov_rr_a1_a_flags_zeroresult_1_01b00012() {
    // Test aarch32_MOV_rr_A1_A flag computation: ZeroResult
    // Encoding: 0x01B00012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x01B00012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_mov_rr_a1_a_flags_negativeresult_2_01b00012() {
    // Test aarch32_MOV_rr_A1_A flag computation: NegativeResult
    // Encoding: 0x01B00012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x01B00012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_mov_rr_a1_a_flags_unsignedoverflow_3_01b00012() {
    // Test aarch32_MOV_rr_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x01B00012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x01B00012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_mov_rr_a1_a_flags_unsignedoverflow_4_01b00012() {
    // Test aarch32_MOV_rr_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x01B00012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x2);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x01B00012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_mov_rr_a1_a_flags_signedoverflow_5_01b00012() {
    // Test aarch32_MOV_rr_A1_A flag computation: SignedOverflow
    // Encoding: 0x01B00012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x01B00012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_mov_rr_a1_a_flags_signedoverflow_6_01b00012() {
    // Test aarch32_MOV_rr_A1_A flag computation: SignedOverflow
    // Encoding: 0x01B00012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x01B00012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_mov_rr_a1_a_flags_positiveresult_7_01b00012() {
    // Test aarch32_MOV_rr_A1_A flag computation: PositiveResult
    // Encoding: 0x01B00012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u32 = 0x01B00012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 0 (32)
#[test]
fn test_aarch32_mov_rr_t1_a_lslv_oracle_32_0_40020020() {
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

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 0 (64)
#[test]
fn test_aarch32_mov_rr_t1_a_lslv_oracle_64_0_c0020020() {
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

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4 (32)
#[test]
fn test_aarch32_mov_rr_t1_a_lslv_oracle_32_1_40020020() {
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

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 4 (64)
#[test]
fn test_aarch32_mov_rr_t1_a_lslv_oracle_64_1_c0020020() {
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

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 8 (32)
#[test]
fn test_aarch32_mov_rr_t1_a_lslv_oracle_32_2_40020020() {
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

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 8 (64)
#[test]
fn test_aarch32_mov_rr_t1_a_lslv_oracle_64_2_c0020020() {
    // Test LSLV 64-bit: shift by 8 (oracle)
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
    assert_eq!(
        get_w(&cpu, 0),
        0x34567800,
        "X0 should be 0x0000001234567800"
    );
}

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1 (32)
#[test]
fn test_aarch32_mov_rr_t1_a_lslv_oracle_32_3_40020020() {
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

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// MSB set, shift 1 (64)
#[test]
fn test_aarch32_mov_rr_t1_a_lslv_oracle_64_3_c0020020() {
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

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSB set, max shift (32)
#[test]
fn test_aarch32_mov_rr_t1_a_lslv_oracle_32_4_40020020() {
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

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSB set, max shift (64)
#[test]
fn test_aarch32_mov_rr_t1_a_lslv_oracle_64_4_c0020020() {
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

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// all ones, shift 32 (32)
#[test]
fn test_aarch32_mov_rr_t1_a_lslv_oracle_32_5_40020020() {
    // Test LSLV 32-bit: all ones, shift 32 (oracle)
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
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "W0 should be 0xFFFFFFFF");
}

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all ones, shift 32 (64)
#[test]
fn test_aarch32_mov_rr_t1_a_lslv_oracle_64_5_c0020020() {
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

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift
#[test]
fn test_aarch32_mov_rr_t1_a_t16_oracle_0_40000000() {
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

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4
#[test]
fn test_aarch32_mov_rr_t1_a_t16_oracle_1_40000000() {
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

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1
#[test]
fn test_aarch32_mov_rr_t1_a_t16_oracle_2_40000000() {
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

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift to MSB
#[test]
fn test_aarch32_mov_rr_t1_a_t16_oracle_3_40000000() {
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

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_mov_rr_t1_a_flags_zeroresult_0_40000000() {
    // Test aarch32_MOV_rr_T1_A flag computation: ZeroResult
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
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_mov_rr_t1_a_flags_zeroresult_1_40000000() {
    // Test aarch32_MOV_rr_T1_A flag computation: ZeroResult
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

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_mov_rr_t1_a_flags_negativeresult_2_40000000() {
    // Test aarch32_MOV_rr_T1_A flag computation: NegativeResult
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

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_mov_rr_t1_a_flags_unsignedoverflow_3_40000000() {
    // Test aarch32_MOV_rr_T1_A flag computation: UnsignedOverflow
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

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_mov_rr_t1_a_flags_unsignedoverflow_4_40000000() {
    // Test aarch32_MOV_rr_T1_A flag computation: UnsignedOverflow
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

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_mov_rr_t1_a_flags_signedoverflow_5_40000000() {
    // Test aarch32_MOV_rr_T1_A flag computation: SignedOverflow
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

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_mov_rr_t1_a_flags_signedoverflow_6_40000000() {
    // Test aarch32_MOV_rr_T1_A flag computation: SignedOverflow
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
}

/// Provenance: aarch32_MOV_rr_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_mov_rr_t1_a_flags_positiveresult_7_40000000() {
    // Test aarch32_MOV_rr_T1_A flag computation: PositiveResult
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
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// simple multiply (32)
#[test]
fn test_aarch32_mov_rr_t2_a_mul_oracle_32_0_1b027c20() {
    // Test MUL 32-bit: simple multiply (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x2);
    set_w(&mut cpu, 2, 0x3);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x6, "W0 should be 0x00000006");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// simple multiply (64)
#[test]
fn test_aarch32_mov_rr_t2_a_mul_oracle_64_0_9b027c20() {
    // Test MUL 64-bit: simple multiply (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x2);
    set_w(&mut cpu, 2, 0x3);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x6, "X0 should be 0x0000000000000006");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// multiply by zero (32)
#[test]
fn test_aarch32_mov_rr_t2_a_mul_oracle_32_1_1b027c20() {
    // Test MUL 32-bit: multiply by zero (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x64);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x00000000");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// multiply by zero (64)
#[test]
fn test_aarch32_mov_rr_t2_a_mul_oracle_64_1_9b027c20() {
    // Test MUL 64-bit: multiply by zero (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x64);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// multiply by one (32)
#[test]
fn test_aarch32_mov_rr_t2_a_mul_oracle_32_2_1b027c20() {
    // Test MUL 32-bit: multiply by one (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1, "W0 should be 0x00000001");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// multiply by one (64)
#[test]
fn test_aarch32_mov_rr_t2_a_mul_oracle_64_2_9b027c20() {
    // Test MUL 64-bit: multiply by one (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1, "X0 should be 0x0000000000000001");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// 16-bit max * 16-bit max (32)
#[test]
fn test_aarch32_mov_rr_t2_a_mul_oracle_32_3_1b027c20() {
    // Test MUL 32-bit: 16-bit max * 16-bit max (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFF);
    set_w(&mut cpu, 1, 0xFFFF);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFE0001, "W0 should be 0xFFFE0001");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// 16-bit max * 16-bit max (64)
#[test]
fn test_aarch32_mov_rr_t2_a_mul_oracle_64_3_9b027c20() {
    // Test MUL 64-bit: 16-bit max * 16-bit max (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFF);
    set_w(&mut cpu, 2, 0xFFFF);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFE0001,
        "X0 should be 0x00000000FFFE0001"
    );
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift-like multiply (32)
#[test]
fn test_aarch32_mov_rr_t2_a_mul_oracle_32_4_1b027c20() {
    // Test MUL 32-bit: shift-like multiply (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x2468ACF0, "W0 should be 0x2468ACF0");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift-like multiply (64)
#[test]
fn test_aarch32_mov_rr_t2_a_mul_oracle_64_4_9b027c20() {
    // Test MUL 64-bit: shift-like multiply (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x2);
    set_w(&mut cpu, 1, 0x12345678);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0x2468ACF0,
        "X0 should be 0x000000002468ACF0"
    );
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// larger values (32)
#[test]
fn test_aarch32_mov_rr_t2_a_mul_oracle_32_5_1b027c20() {
    // Test MUL 32-bit: larger values (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0xC8);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x4E20, "W0 should be 0x00004E20");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// larger values (64)
#[test]
fn test_aarch32_mov_rr_t2_a_mul_oracle_64_5_9b027c20() {
    // Test MUL 64-bit: larger values (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0xC8);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x4E20, "X0 should be 0x0000000000004E20");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// 32-bit overflow (32)
#[test]
fn test_aarch32_mov_rr_t2_a_mul_oracle_32_6_1b027c20() {
    // Test MUL 32-bit: 32-bit overflow (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1, "W0 should be 0x00000001");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// 32-bit overflow (64)
#[test]
fn test_aarch32_mov_rr_t2_a_mul_oracle_64_6_9b027c20() {
    // Test MUL 64-bit: 32-bit overflow (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1, "X0 should be 0xFFFFFFFE00000001");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// prime numbers (32)
#[test]
fn test_aarch32_mov_rr_t2_a_mul_oracle_32_7_1b027c20() {
    // Test MUL 32-bit: prime numbers (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7);
    set_w(&mut cpu, 2, 0xB);
    let encoding: u32 = 0x1B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x4D, "W0 should be 0x0000004D");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `MUL X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// prime numbers (64)
#[test]
fn test_aarch32_mov_rr_t2_a_mul_oracle_64_7_9b027c20() {
    // Test MUL 64-bit: prime numbers (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7);
    set_w(&mut cpu, 2, 0xB);
    let encoding: u32 = 0x9B027C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x4D, "X0 should be 0x000000000000004D");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `MUL R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// simple values
#[test]
fn test_aarch32_mov_rr_t2_a_t32_oracle_0_fa02f000() {
    // Test T32 MUL: simple values (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u32 = 0xFA02F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1388, "R0 should be 0x00001388");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `MUL R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero values
#[test]
fn test_aarch32_mov_rr_t2_a_t32_oracle_1_fa02f000() {
    // Test T32 MUL: zero values (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xFA02F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `MUL R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max value
#[test]
fn test_aarch32_mov_rr_t2_a_t32_oracle_2_fa02f000() {
    // Test T32 MUL: max value (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xFA02F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "R0 should be 0xFFFFFFFF");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `MUL R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// mixed pattern
#[test]
fn test_aarch32_mov_rr_t2_a_t32_oracle_3_fa02f000() {
    // Test T32 MUL: mixed pattern (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x12345678);
    set_w(&mut cpu, 2, 0xABCDEF01);
    let encoding: u32 = 0xFA02F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x55065E78, "R0 should be 0x55065E78");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_mov_rr_t2_a_flags_zeroresult_0_fa12f000() {
    // Test aarch32_MOV_rr_T2_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xFA12F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_mov_rr_t2_a_flags_zeroresult_1_fa12f000() {
    // Test aarch32_MOV_rr_T2_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0xFA12F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_mov_rr_t2_a_flags_negativeresult_2_fa12f000() {
    // Test aarch32_MOV_rr_T2_A flag computation: NegativeResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xFA12F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_mov_rr_t2_a_flags_unsignedoverflow_3_fa12f000() {
    // Test aarch32_MOV_rr_T2_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xFA12F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_mov_rr_t2_a_flags_unsignedoverflow_4_fa12f000() {
    // Test aarch32_MOV_rr_T2_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u32 = 0xFA12F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_mov_rr_t2_a_flags_signedoverflow_5_fa12f000() {
    // Test aarch32_MOV_rr_T2_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xFA12F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_mov_rr_t2_a_flags_signedoverflow_6_fa12f000() {
    // Test aarch32_MOV_rr_T2_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xFA12F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_rr_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_mov_rr_t2_a_flags_positiveresult_7_fa12f000() {
    // Test aarch32_MOV_rr_T2_A flag computation: PositiveResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u32 = 0xFA12F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

// ============================================================================
// aarch32_MVN_rr_A Tests
// ============================================================================

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_mvn_rr_a1_a_field_cond_0_min_10_01e00010() {
    // Encoding: 0x01E00010
    // Test aarch32_MVN_rr_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rs=0, cond=0, type1=0, S=0, Rd=0, Rm=0
    let encoding: u32 = 0x01E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_mvn_rr_a1_a_field_cond_1_poweroftwo_10_11e00010() {
    // Encoding: 0x11E00010
    // Test aarch32_MVN_rr_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, type1=0, S=0, Rd=0, Rs=0, cond=1
    let encoding: u32 = 0x11E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_mvn_rr_a1_a_field_cond_2_poweroftwo_10_21e00010() {
    // Encoding: 0x21E00010
    // Test aarch32_MVN_rr_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=2, type1=0, Rd=0, Rs=0, S=0
    let encoding: u32 = 0x21E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_mvn_rr_a1_a_field_cond_3_poweroftwo_10_31e00010() {
    // Encoding: 0x31E00010
    // Test aarch32_MVN_rr_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, Rd=0, type1=0, Rs=0, S=0, Rm=0
    let encoding: u32 = 0x31E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_mvn_rr_a1_a_field_cond_4_poweroftwo_10_41e00010() {
    // Encoding: 0x41E00010
    // Test aarch32_MVN_rr_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, Rs=0, cond=4, type1=0, Rd=0, Rm=0
    let encoding: u32 = 0x41E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_mvn_rr_a1_a_field_cond_5_poweroftwo_10_51e00010() {
    // Encoding: 0x51E00010
    // Test aarch32_MVN_rr_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rd=0, Rs=0, Rm=0, cond=5, S=0
    let encoding: u32 = 0x51E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_mvn_rr_a1_a_field_cond_6_poweroftwo_10_61e00010() {
    // Encoding: 0x61E00010
    // Test aarch32_MVN_rr_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, Rs=0, cond=6, Rd=0, Rm=0, type1=0
    let encoding: u32 = 0x61E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_mvn_rr_a1_a_field_cond_7_poweroftwo_10_71e00010() {
    // Encoding: 0x71E00010
    // Test aarch32_MVN_rr_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, S=0, cond=7, Rs=0, type1=0
    let encoding: u32 = 0x71E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_mvn_rr_a1_a_field_cond_8_poweroftwo_10_81e00010() {
    // Encoding: 0x81E00010
    // Test aarch32_MVN_rr_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rs=0, type1=0, Rd=0, S=0, cond=8, Rm=0
    let encoding: u32 = 0x81E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_mvn_rr_a1_a_field_cond_9_poweroftwo_10_91e00010() {
    // Encoding: 0x91E00010
    // Test aarch32_MVN_rr_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, Rd=0, cond=9, S=0, Rs=0, type1=0
    let encoding: u32 = 0x91E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_mvn_rr_a1_a_field_cond_10_poweroftwo_10_a1e00010() {
    // Encoding: 0xA1E00010
    // Test aarch32_MVN_rr_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, Rm=0, Rs=0, cond=10, Rd=0, S=0
    let encoding: u32 = 0xA1E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_mvn_rr_a1_a_field_cond_11_poweroftwo_10_b1e00010() {
    // Encoding: 0xB1E00010
    // Test aarch32_MVN_rr_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=11, type1=0, Rd=0, Rs=0, S=0, Rm=0
    let encoding: u32 = 0xB1E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_mvn_rr_a1_a_field_cond_12_poweroftwo_10_c1e00010() {
    // Encoding: 0xC1E00010
    // Test aarch32_MVN_rr_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, Rd=0, cond=12, Rs=0, type1=0, Rm=0
    let encoding: u32 = 0xC1E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_mvn_rr_a1_a_field_cond_13_poweroftwo_10_d1e00010() {
    // Encoding: 0xD1E00010
    // Test aarch32_MVN_rr_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=13, Rd=0, type1=0, Rm=0, S=0, Rs=0
    let encoding: u32 = 0xD1E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_mvn_rr_a1_a_field_cond_14_poweroftwo_10_e1e00010() {
    // Encoding: 0xE1E00010
    // Test aarch32_MVN_rr_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=14, S=0, Rs=0, type1=0, Rm=0
    let encoding: u32 = 0xE1E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_mvn_rr_a1_a_field_cond_15_max_10_f1e00010() {
    // Encoding: 0xF1E00010
    // Test aarch32_MVN_rr_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rd=0, Rs=0, S=0, type1=0, cond=15, Rm=0
    let encoding: u32 = 0xF1E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_mvn_rr_a1_a_field_s_0_min_10_01e00010() {
    // Encoding: 0x01E00010
    // Test aarch32_MVN_rr_A1_A field S = 0 (Min)
    // ISET: A32
    // Fields: Rs=0, type1=0, S=0, Rm=0, Rd=0, cond=0
    let encoding: u32 = 0x01E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_mvn_rr_a1_a_field_s_1_max_10_01f00010() {
    // Encoding: 0x01F00010
    // Test aarch32_MVN_rr_A1_A field S = 1 (Max)
    // ISET: A32
    // Fields: Rs=0, S=1, cond=0, type1=0, Rd=0, Rm=0
    let encoding: u32 = 0x01F00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mvn_rr_a1_a_field_rd_0_min_10_01e00010() {
    // Encoding: 0x01E00010
    // Test aarch32_MVN_rr_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: cond=0, S=0, Rm=0, type1=0, Rd=0, Rs=0
    let encoding: u32 = 0x01E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mvn_rr_a1_a_field_rd_1_poweroftwo_10_01e01010() {
    // Encoding: 0x01E01010
    // Test aarch32_MVN_rr_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=0, type1=0, Rd=1, S=0, Rs=0
    let encoding: u32 = 0x01E01010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field Rs 8 +: 4`
/// Requirement: FieldBoundary { field: "Rs", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mvn_rr_a1_a_field_rs_0_min_10_01e00010() {
    // Encoding: 0x01E00010
    // Test aarch32_MVN_rr_A1_A field Rs = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0, type1=0, S=0, Rd=0, Rs=0
    let encoding: u32 = 0x01E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field Rs 8 +: 4`
/// Requirement: FieldBoundary { field: "Rs", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mvn_rr_a1_a_field_rs_1_poweroftwo_10_01e00110() {
    // Encoding: 0x01E00110
    // Test aarch32_MVN_rr_A1_A field Rs = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rs=1, S=0, cond=0, Rm=0, type1=0
    let encoding: u32 = 0x01E00110;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mvn_rr_a1_a_field_type1_0_min_10_01e00010() {
    // Encoding: 0x01E00010
    // Test aarch32_MVN_rr_A1_A field type1 = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=0, Rs=0, S=0, type1=0
    let encoding: u32 = 0x01E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mvn_rr_a1_a_field_type1_1_poweroftwo_10_01e00030() {
    // Encoding: 0x01E00030
    // Test aarch32_MVN_rr_A1_A field type1 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rs=0, type1=1, S=0, cond=0, Rm=0, Rd=0
    let encoding: u32 = 0x01E00030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_mvn_rr_a1_a_field_type1_3_max_10_01e00070() {
    // Encoding: 0x01E00070
    // Test aarch32_MVN_rr_A1_A field type1 = 3 (Max)
    // ISET: A32
    // Fields: Rs=0, Rd=0, Rm=0, type1=3, cond=0, S=0
    let encoding: u32 = 0x01E00070;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mvn_rr_a1_a_field_rm_0_min_10_01e00010() {
    // Encoding: 0x01E00010
    // Test aarch32_MVN_rr_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rs=0, S=0, type1=0, Rm=0
    let encoding: u32 = 0x01E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mvn_rr_a1_a_field_rm_1_poweroftwo_10_01e00011() {
    // Encoding: 0x01E00011
    // Test aarch32_MVN_rr_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rs=0, Rm=1, cond=0, Rd=0, type1=0, S=0
    let encoding: u32 = 0x01E00011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_mvn_rr_a1_a_combo_0_10_01e00010() {
    // Encoding: 0x01E00010
    // Test aarch32_MVN_rr_A1_A field combination: cond=0, S=0, Rd=0, Rs=0, type1=0, Rm=0
    // ISET: A32
    // Fields: S=0, Rm=0, cond=0, Rs=0, Rd=0, type1=0
    let encoding: u32 = 0x01E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_mvn_rr_a1_a_special_cond_0_condition_eq_16_01e00010() {
    // Encoding: 0x01E00010
    // Test aarch32_MVN_rr_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: S=0, Rd=0, Rs=0, type1=0, Rm=0, cond=0
    let encoding: u32 = 0x01E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_mvn_rr_a1_a_special_cond_1_condition_ne_16_11e00010() {
    // Encoding: 0x11E00010
    // Test aarch32_MVN_rr_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, S=0, Rd=0, Rm=0, Rs=0, type1=0
    let encoding: u32 = 0x11E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_mvn_rr_a1_a_special_cond_2_condition_cs_hs_16_21e00010() {
    // Encoding: 0x21E00010
    // Test aarch32_MVN_rr_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rd=0, Rs=0, cond=2, S=0, type1=0, Rm=0
    let encoding: u32 = 0x21E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_mvn_rr_a1_a_special_cond_3_condition_cc_lo_16_31e00010() {
    // Encoding: 0x31E00010
    // Test aarch32_MVN_rr_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, S=0, Rd=0, type1=0, Rs=0, Rm=0
    let encoding: u32 = 0x31E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_mvn_rr_a1_a_special_cond_4_condition_mi_16_41e00010() {
    // Encoding: 0x41E00010
    // Test aarch32_MVN_rr_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rd=0, Rs=0, S=0, Rm=0, type1=0, cond=4
    let encoding: u32 = 0x41E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_mvn_rr_a1_a_special_cond_5_condition_pl_16_51e00010() {
    // Encoding: 0x51E00010
    // Test aarch32_MVN_rr_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, S=0, Rm=0, type1=0, Rd=0, Rs=0
    let encoding: u32 = 0x51E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_mvn_rr_a1_a_special_cond_6_condition_vs_16_61e00010() {
    // Encoding: 0x61E00010
    // Test aarch32_MVN_rr_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rs=0, Rd=0, cond=6, S=0, type1=0, Rm=0
    let encoding: u32 = 0x61E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_mvn_rr_a1_a_special_cond_7_condition_vc_16_71e00010() {
    // Encoding: 0x71E00010
    // Test aarch32_MVN_rr_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rs=0, cond=7, Rm=0, S=0, Rd=0, type1=0
    let encoding: u32 = 0x71E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_mvn_rr_a1_a_special_cond_8_condition_hi_16_81e00010() {
    // Encoding: 0x81E00010
    // Test aarch32_MVN_rr_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, S=0, Rd=0, Rs=0, type1=0, Rm=0
    let encoding: u32 = 0x81E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_mvn_rr_a1_a_special_cond_9_condition_ls_16_91e00010() {
    // Encoding: 0x91E00010
    // Test aarch32_MVN_rr_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, Rs=0, S=0, Rm=0, Rd=0, type1=0
    let encoding: u32 = 0x91E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_mvn_rr_a1_a_special_cond_10_condition_ge_16_a1e00010() {
    // Encoding: 0xA1E00010
    // Test aarch32_MVN_rr_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, type1=0, Rm=0, S=0, Rs=0, Rd=0
    let encoding: u32 = 0xA1E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_mvn_rr_a1_a_special_cond_11_condition_lt_16_b1e00010() {
    // Encoding: 0xB1E00010
    // Test aarch32_MVN_rr_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rs=0, cond=11, S=0, type1=0, Rm=0, Rd=0
    let encoding: u32 = 0xB1E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_mvn_rr_a1_a_special_cond_12_condition_gt_16_c1e00010() {
    // Encoding: 0xC1E00010
    // Test aarch32_MVN_rr_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: S=0, type1=0, Rd=0, Rm=0, Rs=0, cond=12
    let encoding: u32 = 0xC1E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_mvn_rr_a1_a_special_cond_13_condition_le_16_d1e00010() {
    // Encoding: 0xD1E00010
    // Test aarch32_MVN_rr_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: cond=13, Rm=0, Rs=0, Rd=0, type1=0, S=0
    let encoding: u32 = 0xD1E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_mvn_rr_a1_a_special_cond_14_condition_al_16_e1e00010() {
    // Encoding: 0xE1E00010
    // Test aarch32_MVN_rr_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rm=0, type1=0, Rd=0, S=0, Rs=0, cond=14
    let encoding: u32 = 0xE1E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_mvn_rr_a1_a_special_cond_15_condition_nv_16_f1e00010() {
    // Encoding: 0xF1E00010
    // Test aarch32_MVN_rr_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: type1=0, Rm=0, S=0, Rs=0, Rd=0, cond=15
    let encoding: u32 = 0xF1E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_mvn_rr_a1_a_special_s_0_size_variant_0_16_01e00010() {
    // Encoding: 0x01E00010
    // Test aarch32_MVN_rr_A1_A special value S = 0 (Size variant 0)
    // ISET: A32
    // Fields: cond=0, S=0, Rd=0, Rm=0, type1=0, Rs=0
    let encoding: u32 = 0x01E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_mvn_rr_a1_a_special_s_1_size_variant_1_16_01f00010() {
    // Encoding: 0x01F00010
    // Test aarch32_MVN_rr_A1_A special value S = 1 (Size variant 1)
    // ISET: A32
    // Fields: Rs=0, S=1, type1=0, Rm=0, Rd=0, cond=0
    let encoding: u32 = 0x01F00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "s" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"s\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mvn_rr_a1_a_invalid_0_10_01e00010() {
    // Encoding: 0x01E00010
    // Test aarch32_MVN_rr_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "s" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: cond=0, type1=0, Rs=0, Rm=0, Rd=0, S=0
    let encoding: u32 = 0x01E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mvn_rr_a1_a_invalid_1_10_01e00010() {
    // Encoding: 0x01E00010
    // Test aarch32_MVN_rr_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rs=0, cond=0, S=0, type1=0, Rd=0, Rm=0
    let encoding: u32 = 0x01E00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_mvn_rr_a1_a_flags_zeroresult_0_01f00012() {
    // Test aarch32_MVN_rr_A1_A flag computation: ZeroResult
    // Encoding: 0x01F00012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x01F00012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_mvn_rr_a1_a_flags_zeroresult_1_01f00012() {
    // Test aarch32_MVN_rr_A1_A flag computation: ZeroResult
    // Encoding: 0x01F00012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x01F00012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_mvn_rr_a1_a_flags_negativeresult_2_01f00012() {
    // Test aarch32_MVN_rr_A1_A flag computation: NegativeResult
    // Encoding: 0x01F00012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x01F00012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_mvn_rr_a1_a_flags_unsignedoverflow_3_01f00012() {
    // Test aarch32_MVN_rr_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x01F00012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x01F00012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_mvn_rr_a1_a_flags_unsignedoverflow_4_01f00012() {
    // Test aarch32_MVN_rr_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x01F00012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x2);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x01F00012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_mvn_rr_a1_a_flags_signedoverflow_5_01f00012() {
    // Test aarch32_MVN_rr_A1_A flag computation: SignedOverflow
    // Encoding: 0x01F00012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x01F00012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_mvn_rr_a1_a_flags_signedoverflow_6_01f00012() {
    // Test aarch32_MVN_rr_A1_A flag computation: SignedOverflow
    // Encoding: 0x01F00012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x01F00012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MVN_rr_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_mvn_rr_a1_a_flags_positiveresult_7_01f00012() {
    // Test aarch32_MVN_rr_A1_A flag computation: PositiveResult
    // Encoding: 0x01F00012
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u32 = 0x01F00012;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

// ============================================================================
// aarch32_MOV_i_A Tests
// ============================================================================

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_mov_i_a1_a_field_cond_0_min_0_03a00000() {
    // Encoding: 0x03A00000
    // Test aarch32_MOV_i_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, S=0, cond=0, imm12=0
    let encoding: u32 = 0x03A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_mov_i_a1_a_field_cond_1_poweroftwo_0_13a00000() {
    // Encoding: 0x13A00000
    // Test aarch32_MOV_i_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, Rd=0, S=0, cond=1
    let encoding: u32 = 0x13A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_mov_i_a1_a_field_cond_2_poweroftwo_0_23a00000() {
    // Encoding: 0x23A00000
    // Test aarch32_MOV_i_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=2, imm12=0, S=0
    let encoding: u32 = 0x23A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_mov_i_a1_a_field_cond_3_poweroftwo_0_33a00000() {
    // Encoding: 0x33A00000
    // Test aarch32_MOV_i_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, imm12=0, Rd=0, S=0
    let encoding: u32 = 0x33A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_mov_i_a1_a_field_cond_4_poweroftwo_0_43a00000() {
    // Encoding: 0x43A00000
    // Test aarch32_MOV_i_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=4, S=0, Rd=0, imm12=0
    let encoding: u32 = 0x43A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_mov_i_a1_a_field_cond_5_poweroftwo_0_53a00000() {
    // Encoding: 0x53A00000
    // Test aarch32_MOV_i_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=5, S=0, imm12=0, Rd=0
    let encoding: u32 = 0x53A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_mov_i_a1_a_field_cond_6_poweroftwo_0_63a00000() {
    // Encoding: 0x63A00000
    // Test aarch32_MOV_i_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=6, S=0, imm12=0
    let encoding: u32 = 0x63A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_mov_i_a1_a_field_cond_7_poweroftwo_0_73a00000() {
    // Encoding: 0x73A00000
    // Test aarch32_MOV_i_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, imm12=0, cond=7, S=0
    let encoding: u32 = 0x73A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_mov_i_a1_a_field_cond_8_poweroftwo_0_83a00000() {
    // Encoding: 0x83A00000
    // Test aarch32_MOV_i_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=8, imm12=0, Rd=0, S=0
    let encoding: u32 = 0x83A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_mov_i_a1_a_field_cond_9_poweroftwo_0_93a00000() {
    // Encoding: 0x93A00000
    // Test aarch32_MOV_i_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, Rd=0, S=0, cond=9
    let encoding: u32 = 0x93A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_mov_i_a1_a_field_cond_10_poweroftwo_0_a3a00000() {
    // Encoding: 0xA3A00000
    // Test aarch32_MOV_i_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, Rd=0, imm12=0, S=0
    let encoding: u32 = 0xA3A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_mov_i_a1_a_field_cond_11_poweroftwo_0_b3a00000() {
    // Encoding: 0xB3A00000
    // Test aarch32_MOV_i_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=11, imm12=0, S=0
    let encoding: u32 = 0xB3A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_mov_i_a1_a_field_cond_12_poweroftwo_0_c3a00000() {
    // Encoding: 0xC3A00000
    // Test aarch32_MOV_i_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, imm12=0, Rd=0, S=0
    let encoding: u32 = 0xC3A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_mov_i_a1_a_field_cond_13_poweroftwo_0_d3a00000() {
    // Encoding: 0xD3A00000
    // Test aarch32_MOV_i_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, S=0, cond=13, imm12=0
    let encoding: u32 = 0xD3A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_mov_i_a1_a_field_cond_14_poweroftwo_0_e3a00000() {
    // Encoding: 0xE3A00000
    // Test aarch32_MOV_i_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=14, imm12=0, S=0
    let encoding: u32 = 0xE3A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_mov_i_a1_a_field_cond_15_max_0_f3a00000() {
    // Encoding: 0xF3A00000
    // Test aarch32_MOV_i_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, imm12=0, Rd=0, S=0
    let encoding: u32 = 0xF3A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_mov_i_a1_a_field_s_0_min_0_03a00000() {
    // Encoding: 0x03A00000
    // Test aarch32_MOV_i_A1_A field S = 0 (Min)
    // ISET: A32
    // Fields: S=0, imm12=0, Rd=0, cond=0
    let encoding: u32 = 0x03A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_mov_i_a1_a_field_s_1_max_0_03b00000() {
    // Encoding: 0x03B00000
    // Test aarch32_MOV_i_A1_A field S = 1 (Max)
    // ISET: A32
    // Fields: Rd=0, S=1, cond=0, imm12=0
    let encoding: u32 = 0x03B00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mov_i_a1_a_field_rd_0_min_0_03a00000() {
    // Encoding: 0x03A00000
    // Test aarch32_MOV_i_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: S=0, Rd=0, cond=0, imm12=0
    let encoding: u32 = 0x03A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mov_i_a1_a_field_rd_1_poweroftwo_0_03a01000() {
    // Encoding: 0x03A01000
    // Test aarch32_MOV_i_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, imm12=0, cond=0, Rd=1
    let encoding: u32 = 0x03A01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_mov_i_a1_a_field_imm12_0_zero_0_03a00000() {
    // Encoding: 0x03A00000
    // Test aarch32_MOV_i_A1_A field imm12 = 0 (Zero)
    // ISET: A32
    // Fields: S=0, imm12=0, cond=0, Rd=0
    let encoding: u32 = 0x03A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_mov_i_a1_a_field_imm12_1_poweroftwo_0_03a00001() {
    // Encoding: 0x03A00001
    // Test aarch32_MOV_i_A1_A field imm12 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=0, imm12=1, S=0
    let encoding: u32 = 0x03A00001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_mov_i_a1_a_field_imm12_3_poweroftwominusone_0_03a00003() {
    // Encoding: 0x03A00003
    // Test aarch32_MOV_i_A1_A field imm12 = 3 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: S=0, cond=0, Rd=0, imm12=3
    let encoding: u32 = 0x03A00003;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_mov_i_a1_a_field_imm12_4_poweroftwo_0_03a00004() {
    // Encoding: 0x03A00004
    // Test aarch32_MOV_i_A1_A field imm12 = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=0, imm12=4, S=0
    let encoding: u32 = 0x03A00004;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_mov_i_a1_a_field_imm12_7_poweroftwominusone_0_03a00007() {
    // Encoding: 0x03A00007
    // Test aarch32_MOV_i_A1_A field imm12 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rd=0, imm12=7, S=0, cond=0
    let encoding: u32 = 0x03A00007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_mov_i_a1_a_field_imm12_8_poweroftwo_0_03a00008() {
    // Encoding: 0x03A00008
    // Test aarch32_MOV_i_A1_A field imm12 = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, imm12=8, S=0, cond=0
    let encoding: u32 = 0x03A00008;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_mov_i_a1_a_field_imm12_15_poweroftwominusone_0_03a0000f() {
    // Encoding: 0x03A0000F
    // Test aarch32_MOV_i_A1_A field imm12 = 15 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=15, S=0, cond=0, Rd=0
    let encoding: u32 = 0x03A0000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_mov_i_a1_a_field_imm12_16_poweroftwo_0_03a00010() {
    // Encoding: 0x03A00010
    // Test aarch32_MOV_i_A1_A field imm12 = 16 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, Rd=0, imm12=16, cond=0
    let encoding: u32 = 0x03A00010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_mov_i_a1_a_field_imm12_31_poweroftwominusone_0_03a0001f() {
    // Encoding: 0x03A0001F
    // Test aarch32_MOV_i_A1_A field imm12 = 31 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, S=0, Rd=0, imm12=31
    let encoding: u32 = 0x03A0001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_mov_i_a1_a_field_imm12_32_poweroftwo_0_03a00020() {
    // Encoding: 0x03A00020
    // Test aarch32_MOV_i_A1_A field imm12 = 32 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=0, S=0, imm12=32
    let encoding: u32 = 0x03A00020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_mov_i_a1_a_field_imm12_63_poweroftwominusone_0_03a0003f() {
    // Encoding: 0x03A0003F
    // Test aarch32_MOV_i_A1_A field imm12 = 63 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm12=63, Rd=0, S=0
    let encoding: u32 = 0x03A0003F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_mov_i_a1_a_field_imm12_64_poweroftwo_0_03a00040() {
    // Encoding: 0x03A00040
    // Test aarch32_MOV_i_A1_A field imm12 = 64 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, Rd=0, imm12=64, cond=0
    let encoding: u32 = 0x03A00040;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch32_mov_i_a1_a_field_imm12_127_poweroftwominusone_0_03a0007f() {
    // Encoding: 0x03A0007F
    // Test aarch32_MOV_i_A1_A field imm12 = 127 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=127, cond=0, S=0, Rd=0
    let encoding: u32 = 0x03A0007F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_mov_i_a1_a_field_imm12_128_poweroftwo_0_03a00080() {
    // Encoding: 0x03A00080
    // Test aarch32_MOV_i_A1_A field imm12 = 128 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=128, cond=0, Rd=0, S=0
    let encoding: u32 = 0x03A00080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch32_mov_i_a1_a_field_imm12_255_poweroftwominusone_0_03a000ff() {
    // Encoding: 0x03A000FF
    // Test aarch32_MOV_i_A1_A field imm12 = 255 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=255, S=0, cond=0, Rd=0
    let encoding: u32 = 0x03A000FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch32_mov_i_a1_a_field_imm12_256_poweroftwo_0_03a00100() {
    // Encoding: 0x03A00100
    // Test aarch32_MOV_i_A1_A field imm12 = 256 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, imm12=256, cond=0, S=0
    let encoding: u32 = 0x03A00100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch32_mov_i_a1_a_field_imm12_511_poweroftwominusone_0_03a001ff() {
    // Encoding: 0x03A001FF
    // Test aarch32_MOV_i_A1_A field imm12 = 511 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rd=0, imm12=511, cond=0, S=0
    let encoding: u32 = 0x03A001FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch32_mov_i_a1_a_field_imm12_512_poweroftwo_0_03a00200() {
    // Encoding: 0x03A00200
    // Test aarch32_MOV_i_A1_A field imm12 = 512 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=512, S=0, cond=0, Rd=0
    let encoding: u32 = 0x03A00200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1023, boundary: PowerOfTwoMinusOne }
/// 2^10 - 1 = 1023
#[test]
fn test_aarch32_mov_i_a1_a_field_imm12_1023_poweroftwominusone_0_03a003ff() {
    // Encoding: 0x03A003FF
    // Test aarch32_MOV_i_A1_A field imm12 = 1023 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=1023, cond=0, S=0, Rd=0
    let encoding: u32 = 0x03A003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch32_mov_i_a1_a_field_imm12_1024_poweroftwo_0_03a00400() {
    // Encoding: 0x03A00400
    // Test aarch32_MOV_i_A1_A field imm12 = 1024 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, S=0, cond=0, imm12=1024
    let encoding: u32 = 0x03A00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 2047, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (2047)
#[test]
fn test_aarch32_mov_i_a1_a_field_imm12_2047_poweroftwominusone_0_03a007ff() {
    // Encoding: 0x03A007FF
    // Test aarch32_MOV_i_A1_A field imm12 = 2047 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: S=0, Rd=0, imm12=2047, cond=0
    let encoding: u32 = 0x03A007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 2048, boundary: PowerOfTwo }
/// power of 2 (2^11 = 2048)
#[test]
fn test_aarch32_mov_i_a1_a_field_imm12_2048_poweroftwo_0_03a00800() {
    // Encoding: 0x03A00800
    // Test aarch32_MOV_i_A1_A field imm12 = 2048 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, imm12=2048, S=0, cond=0
    let encoding: u32 = 0x03A00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 4095, boundary: Max }
/// maximum immediate (4095)
#[test]
fn test_aarch32_mov_i_a1_a_field_imm12_4095_max_0_03a00fff() {
    // Encoding: 0x03A00FFF
    // Test aarch32_MOV_i_A1_A field imm12 = 4095 (Max)
    // ISET: A32
    // Fields: cond=0, S=0, imm12=4095, Rd=0
    let encoding: u32 = 0x03A00FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_mov_i_a1_a_combo_0_0_03a00000() {
    // Encoding: 0x03A00000
    // Test aarch32_MOV_i_A1_A field combination: cond=0, S=0, Rd=0, imm12=0
    // ISET: A32
    // Fields: imm12=0, Rd=0, cond=0, S=0
    let encoding: u32 = 0x03A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_mov_i_a1_a_special_cond_0_condition_eq_0_03a00000() {
    // Encoding: 0x03A00000
    // Test aarch32_MOV_i_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: cond=0, imm12=0, Rd=0, S=0
    let encoding: u32 = 0x03A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_mov_i_a1_a_special_cond_1_condition_ne_0_13a00000() {
    // Encoding: 0x13A00000
    // Test aarch32_MOV_i_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, Rd=0, S=0, imm12=0
    let encoding: u32 = 0x13A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_mov_i_a1_a_special_cond_2_condition_cs_hs_0_23a00000() {
    // Encoding: 0x23A00000
    // Test aarch32_MOV_i_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, imm12=0, S=0, Rd=0
    let encoding: u32 = 0x23A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_mov_i_a1_a_special_cond_3_condition_cc_lo_0_33a00000() {
    // Encoding: 0x33A00000
    // Test aarch32_MOV_i_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: S=0, imm12=0, Rd=0, cond=3
    let encoding: u32 = 0x33A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_mov_i_a1_a_special_cond_4_condition_mi_0_43a00000() {
    // Encoding: 0x43A00000
    // Test aarch32_MOV_i_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: S=0, cond=4, Rd=0, imm12=0
    let encoding: u32 = 0x43A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_mov_i_a1_a_special_cond_5_condition_pl_0_53a00000() {
    // Encoding: 0x53A00000
    // Test aarch32_MOV_i_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: imm12=0, cond=5, S=0, Rd=0
    let encoding: u32 = 0x53A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_mov_i_a1_a_special_cond_6_condition_vs_0_63a00000() {
    // Encoding: 0x63A00000
    // Test aarch32_MOV_i_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rd=0, imm12=0, S=0, cond=6
    let encoding: u32 = 0x63A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_mov_i_a1_a_special_cond_7_condition_vc_0_73a00000() {
    // Encoding: 0x73A00000
    // Test aarch32_MOV_i_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, S=0, Rd=0, imm12=0
    let encoding: u32 = 0x73A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_mov_i_a1_a_special_cond_8_condition_hi_0_83a00000() {
    // Encoding: 0x83A00000
    // Test aarch32_MOV_i_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: S=0, Rd=0, cond=8, imm12=0
    let encoding: u32 = 0x83A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_mov_i_a1_a_special_cond_9_condition_ls_0_93a00000() {
    // Encoding: 0x93A00000
    // Test aarch32_MOV_i_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: S=0, imm12=0, cond=9, Rd=0
    let encoding: u32 = 0x93A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_mov_i_a1_a_special_cond_10_condition_ge_0_a3a00000() {
    // Encoding: 0xA3A00000
    // Test aarch32_MOV_i_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, S=0, Rd=0, imm12=0
    let encoding: u32 = 0xA3A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_mov_i_a1_a_special_cond_11_condition_lt_0_b3a00000() {
    // Encoding: 0xB3A00000
    // Test aarch32_MOV_i_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: imm12=0, cond=11, S=0, Rd=0
    let encoding: u32 = 0xB3A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_mov_i_a1_a_special_cond_12_condition_gt_0_c3a00000() {
    // Encoding: 0xC3A00000
    // Test aarch32_MOV_i_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: S=0, Rd=0, cond=12, imm12=0
    let encoding: u32 = 0xC3A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_mov_i_a1_a_special_cond_13_condition_le_0_d3a00000() {
    // Encoding: 0xD3A00000
    // Test aarch32_MOV_i_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: imm12=0, Rd=0, S=0, cond=13
    let encoding: u32 = 0xD3A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_mov_i_a1_a_special_cond_14_condition_al_0_e3a00000() {
    // Encoding: 0xE3A00000
    // Test aarch32_MOV_i_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: imm12=0, Rd=0, cond=14, S=0
    let encoding: u32 = 0xE3A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_mov_i_a1_a_special_cond_15_condition_nv_0_f3a00000() {
    // Encoding: 0xF3A00000
    // Test aarch32_MOV_i_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: S=0, cond=15, imm12=0, Rd=0
    let encoding: u32 = 0xF3A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_mov_i_a1_a_special_s_0_size_variant_0_0_03a00000() {
    // Encoding: 0x03A00000
    // Test aarch32_MOV_i_A1_A special value S = 0 (Size variant 0)
    // ISET: A32
    // Fields: S=0, imm12=0, cond=0, Rd=0
    let encoding: u32 = 0x03A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_mov_i_a1_a_special_s_1_size_variant_1_0_03b00000() {
    // Encoding: 0x03B00000
    // Test aarch32_MOV_i_A1_A special value S = 1 (Size variant 1)
    // ISET: A32
    // Fields: Rd=0, S=1, cond=0, imm12=0
    let encoding: u32 = 0x03B00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_mov_i_a2_a_field_cond_0_min_0_03000000() {
    // Encoding: 0x03000000
    // Test aarch32_MOV_i_A2_A field cond = 0 (Min)
    // ISET: A32
    // Fields: imm4=0, cond=0, Rd=0, imm12=0
    let encoding: u32 = 0x03000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_mov_i_a2_a_field_cond_1_poweroftwo_0_13000000() {
    // Encoding: 0x13000000
    // Test aarch32_MOV_i_A2_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, imm4=0, cond=1, imm12=0
    let encoding: u32 = 0x13000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_mov_i_a2_a_field_cond_2_poweroftwo_0_23000000() {
    // Encoding: 0x23000000
    // Test aarch32_MOV_i_A2_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, imm4=0, imm12=0, cond=2
    let encoding: u32 = 0x23000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_mov_i_a2_a_field_cond_3_poweroftwo_0_33000000() {
    // Encoding: 0x33000000
    // Test aarch32_MOV_i_A2_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, imm12=0, cond=3, imm4=0
    let encoding: u32 = 0x33000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_mov_i_a2_a_field_cond_4_poweroftwo_0_43000000() {
    // Encoding: 0x43000000
    // Test aarch32_MOV_i_A2_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: imm4=0, Rd=0, cond=4, imm12=0
    let encoding: u32 = 0x43000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_mov_i_a2_a_field_cond_5_poweroftwo_0_53000000() {
    // Encoding: 0x53000000
    // Test aarch32_MOV_i_A2_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, imm12=0, cond=5, imm4=0
    let encoding: u32 = 0x53000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_mov_i_a2_a_field_cond_6_poweroftwo_0_63000000() {
    // Encoding: 0x63000000
    // Test aarch32_MOV_i_A2_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, imm4=0, cond=6, imm12=0
    let encoding: u32 = 0x63000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_mov_i_a2_a_field_cond_7_poweroftwo_0_73000000() {
    // Encoding: 0x73000000
    // Test aarch32_MOV_i_A2_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=7, imm4=0, Rd=0, imm12=0
    let encoding: u32 = 0x73000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_mov_i_a2_a_field_cond_8_poweroftwo_0_83000000() {
    // Encoding: 0x83000000
    // Test aarch32_MOV_i_A2_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=8, Rd=0, imm12=0, imm4=0
    let encoding: u32 = 0x83000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_mov_i_a2_a_field_cond_9_poweroftwo_0_93000000() {
    // Encoding: 0x93000000
    // Test aarch32_MOV_i_A2_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, imm4=0, Rd=0, imm12=0
    let encoding: u32 = 0x93000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_mov_i_a2_a_field_cond_10_poweroftwo_0_a3000000() {
    // Encoding: 0xA3000000
    // Test aarch32_MOV_i_A2_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, Rd=0, imm4=0, imm12=0
    let encoding: u32 = 0xA3000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_mov_i_a2_a_field_cond_11_poweroftwo_0_b3000000() {
    // Encoding: 0xB3000000
    // Test aarch32_MOV_i_A2_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, Rd=0, cond=11, imm4=0
    let encoding: u32 = 0xB3000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_mov_i_a2_a_field_cond_12_poweroftwo_0_c3000000() {
    // Encoding: 0xC3000000
    // Test aarch32_MOV_i_A2_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, imm12=0, imm4=0, Rd=0
    let encoding: u32 = 0xC3000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_mov_i_a2_a_field_cond_13_poweroftwo_0_d3000000() {
    // Encoding: 0xD3000000
    // Test aarch32_MOV_i_A2_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, cond=13, imm4=0, Rd=0
    let encoding: u32 = 0xD3000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_mov_i_a2_a_field_cond_14_poweroftwo_0_e3000000() {
    // Encoding: 0xE3000000
    // Test aarch32_MOV_i_A2_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, imm4=0, Rd=0, cond=14
    let encoding: u32 = 0xE3000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_mov_i_a2_a_field_cond_15_max_0_f3000000() {
    // Encoding: 0xF3000000
    // Test aarch32_MOV_i_A2_A field cond = 15 (Max)
    // ISET: A32
    // Fields: imm4=0, Rd=0, imm12=0, cond=15
    let encoding: u32 = 0xF3000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_mov_i_a2_a_field_imm4_0_zero_0_03000000() {
    // Encoding: 0x03000000
    // Test aarch32_MOV_i_A2_A field imm4 = 0 (Zero)
    // ISET: A32
    // Fields: imm4=0, imm12=0, Rd=0, cond=0
    let encoding: u32 = 0x03000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_mov_i_a2_a_field_imm4_1_poweroftwo_0_03010000() {
    // Encoding: 0x03010000
    // Test aarch32_MOV_i_A2_A field imm4 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: imm4=1, Rd=0, imm12=0, cond=0
    let encoding: u32 = 0x03010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_mov_i_a2_a_field_imm4_3_poweroftwominusone_0_03030000() {
    // Encoding: 0x03030000
    // Test aarch32_MOV_i_A2_A field imm4 = 3 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=0, cond=0, imm4=3, Rd=0
    let encoding: u32 = 0x03030000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_mov_i_a2_a_field_imm4_4_poweroftwo_0_03040000() {
    // Encoding: 0x03040000
    // Test aarch32_MOV_i_A2_A field imm4 = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=0, cond=0, Rd=0, imm4=4
    let encoding: u32 = 0x03040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch32_mov_i_a2_a_field_imm4_7_poweroftwominusone_0_03070000() {
    // Encoding: 0x03070000
    // Test aarch32_MOV_i_A2_A field imm4 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rd=0, imm12=0, cond=0, imm4=7
    let encoding: u32 = 0x03070000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_mov_i_a2_a_field_imm4_8_poweroftwo_0_03080000() {
    // Encoding: 0x03080000
    // Test aarch32_MOV_i_A2_A field imm4 = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=0, imm4=8, imm12=0
    let encoding: u32 = 0x03080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch32_mov_i_a2_a_field_imm4_15_max_0_030f0000() {
    // Encoding: 0x030F0000
    // Test aarch32_MOV_i_A2_A field imm4 = 15 (Max)
    // ISET: A32
    // Fields: cond=0, imm4=15, Rd=0, imm12=0
    let encoding: u32 = 0x030F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mov_i_a2_a_field_rd_0_min_0_03000000() {
    // Encoding: 0x03000000
    // Test aarch32_MOV_i_A2_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: Rd=0, cond=0, imm4=0, imm12=0
    let encoding: u32 = 0x03000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mov_i_a2_a_field_rd_1_poweroftwo_0_03001000() {
    // Encoding: 0x03001000
    // Test aarch32_MOV_i_A2_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm4=0, Rd=1, imm12=0
    let encoding: u32 = 0x03001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_mov_i_a2_a_field_imm12_0_zero_0_03000000() {
    // Encoding: 0x03000000
    // Test aarch32_MOV_i_A2_A field imm12 = 0 (Zero)
    // ISET: A32
    // Fields: cond=0, imm4=0, Rd=0, imm12=0
    let encoding: u32 = 0x03000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_mov_i_a2_a_field_imm12_1_poweroftwo_0_03000001() {
    // Encoding: 0x03000001
    // Test aarch32_MOV_i_A2_A field imm12 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm12=1, Rd=0, imm4=0
    let encoding: u32 = 0x03000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_mov_i_a2_a_field_imm12_3_poweroftwominusone_0_03000003() {
    // Encoding: 0x03000003
    // Test aarch32_MOV_i_A2_A field imm12 = 3 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm4=0, Rd=0, imm12=3
    let encoding: u32 = 0x03000003;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_mov_i_a2_a_field_imm12_4_poweroftwo_0_03000004() {
    // Encoding: 0x03000004
    // Test aarch32_MOV_i_A2_A field imm12 = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: imm4=0, Rd=0, cond=0, imm12=4
    let encoding: u32 = 0x03000004;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_mov_i_a2_a_field_imm12_7_poweroftwominusone_0_03000007() {
    // Encoding: 0x03000007
    // Test aarch32_MOV_i_A2_A field imm12 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm4=0, cond=0, Rd=0, imm12=7
    let encoding: u32 = 0x03000007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_mov_i_a2_a_field_imm12_8_poweroftwo_0_03000008() {
    // Encoding: 0x03000008
    // Test aarch32_MOV_i_A2_A field imm12 = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm4=0, imm12=8, Rd=0
    let encoding: u32 = 0x03000008;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_mov_i_a2_a_field_imm12_15_poweroftwominusone_0_0300000f() {
    // Encoding: 0x0300000F
    // Test aarch32_MOV_i_A2_A field imm12 = 15 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=15, Rd=0, cond=0, imm4=0
    let encoding: u32 = 0x0300000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_mov_i_a2_a_field_imm12_16_poweroftwo_0_03000010() {
    // Encoding: 0x03000010
    // Test aarch32_MOV_i_A2_A field imm12 = 16 (PowerOfTwo)
    // ISET: A32
    // Fields: imm4=0, imm12=16, cond=0, Rd=0
    let encoding: u32 = 0x03000010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_mov_i_a2_a_field_imm12_31_poweroftwominusone_0_0300001f() {
    // Encoding: 0x0300001F
    // Test aarch32_MOV_i_A2_A field imm12 = 31 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm4=0, Rd=0, cond=0, imm12=31
    let encoding: u32 = 0x0300001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_mov_i_a2_a_field_imm12_32_poweroftwo_0_03000020() {
    // Encoding: 0x03000020
    // Test aarch32_MOV_i_A2_A field imm12 = 32 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=0, imm4=0, imm12=32
    let encoding: u32 = 0x03000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_mov_i_a2_a_field_imm12_63_poweroftwominusone_0_0300003f() {
    // Encoding: 0x0300003F
    // Test aarch32_MOV_i_A2_A field imm12 = 63 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm4=0, imm12=63, Rd=0
    let encoding: u32 = 0x0300003F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_mov_i_a2_a_field_imm12_64_poweroftwo_0_03000040() {
    // Encoding: 0x03000040
    // Test aarch32_MOV_i_A2_A field imm12 = 64 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=64, cond=0, Rd=0, imm4=0
    let encoding: u32 = 0x03000040;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch32_mov_i_a2_a_field_imm12_127_poweroftwominusone_0_0300007f() {
    // Encoding: 0x0300007F
    // Test aarch32_MOV_i_A2_A field imm12 = 127 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rd=0, imm4=0, cond=0, imm12=127
    let encoding: u32 = 0x0300007F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_mov_i_a2_a_field_imm12_128_poweroftwo_0_03000080() {
    // Encoding: 0x03000080
    // Test aarch32_MOV_i_A2_A field imm12 = 128 (PowerOfTwo)
    // ISET: A32
    // Fields: imm4=0, cond=0, imm12=128, Rd=0
    let encoding: u32 = 0x03000080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch32_mov_i_a2_a_field_imm12_255_poweroftwominusone_0_030000ff() {
    // Encoding: 0x030000FF
    // Test aarch32_MOV_i_A2_A field imm12 = 255 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm12=255, imm4=0, Rd=0
    let encoding: u32 = 0x030000FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch32_mov_i_a2_a_field_imm12_256_poweroftwo_0_03000100() {
    // Encoding: 0x03000100
    // Test aarch32_MOV_i_A2_A field imm12 = 256 (PowerOfTwo)
    // ISET: A32
    // Fields: imm12=256, imm4=0, cond=0, Rd=0
    let encoding: u32 = 0x03000100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch32_mov_i_a2_a_field_imm12_511_poweroftwominusone_0_030001ff() {
    // Encoding: 0x030001FF
    // Test aarch32_MOV_i_A2_A field imm12 = 511 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm12=511, cond=0, Rd=0, imm4=0
    let encoding: u32 = 0x030001FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch32_mov_i_a2_a_field_imm12_512_poweroftwo_0_03000200() {
    // Encoding: 0x03000200
    // Test aarch32_MOV_i_A2_A field imm12 = 512 (PowerOfTwo)
    // ISET: A32
    // Fields: imm4=0, cond=0, imm12=512, Rd=0
    let encoding: u32 = 0x03000200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1023, boundary: PowerOfTwoMinusOne }
/// 2^10 - 1 = 1023
#[test]
fn test_aarch32_mov_i_a2_a_field_imm12_1023_poweroftwominusone_0_030003ff() {
    // Encoding: 0x030003FF
    // Test aarch32_MOV_i_A2_A field imm12 = 1023 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rd=0, cond=0, imm12=1023, imm4=0
    let encoding: u32 = 0x030003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch32_mov_i_a2_a_field_imm12_1024_poweroftwo_0_03000400() {
    // Encoding: 0x03000400
    // Test aarch32_MOV_i_A2_A field imm12 = 1024 (PowerOfTwo)
    // ISET: A32
    // Fields: imm4=0, Rd=0, imm12=1024, cond=0
    let encoding: u32 = 0x03000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 2047, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (2047)
#[test]
fn test_aarch32_mov_i_a2_a_field_imm12_2047_poweroftwominusone_0_030007ff() {
    // Encoding: 0x030007FF
    // Test aarch32_MOV_i_A2_A field imm12 = 2047 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm4=0, cond=0, Rd=0, imm12=2047
    let encoding: u32 = 0x030007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 2048, boundary: PowerOfTwo }
/// power of 2 (2^11 = 2048)
#[test]
fn test_aarch32_mov_i_a2_a_field_imm12_2048_poweroftwo_0_03000800() {
    // Encoding: 0x03000800
    // Test aarch32_MOV_i_A2_A field imm12 = 2048 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rd=0, imm12=2048, imm4=0
    let encoding: u32 = 0x03000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field imm12 0 +: 12`
/// Requirement: FieldBoundary { field: "imm12", value: 4095, boundary: Max }
/// maximum immediate (4095)
#[test]
fn test_aarch32_mov_i_a2_a_field_imm12_4095_max_0_03000fff() {
    // Encoding: 0x03000FFF
    // Test aarch32_MOV_i_A2_A field imm12 = 4095 (Max)
    // ISET: A32
    // Fields: imm4=0, cond=0, imm12=4095, Rd=0
    let encoding: u32 = 0x03000FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_mov_i_a2_a_combo_0_0_03000000() {
    // Encoding: 0x03000000
    // Test aarch32_MOV_i_A2_A field combination: cond=0, imm4=0, Rd=0, imm12=0
    // ISET: A32
    // Fields: imm4=0, imm12=0, cond=0, Rd=0
    let encoding: u32 = 0x03000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_mov_i_a2_a_special_cond_0_condition_eq_0_03000000() {
    // Encoding: 0x03000000
    // Test aarch32_MOV_i_A2_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: imm12=0, Rd=0, cond=0, imm4=0
    let encoding: u32 = 0x03000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_mov_i_a2_a_special_cond_1_condition_ne_0_13000000() {
    // Encoding: 0x13000000
    // Test aarch32_MOV_i_A2_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: imm12=0, cond=1, Rd=0, imm4=0
    let encoding: u32 = 0x13000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_mov_i_a2_a_special_cond_2_condition_cs_hs_0_23000000() {
    // Encoding: 0x23000000
    // Test aarch32_MOV_i_A2_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, Rd=0, imm4=0, imm12=0
    let encoding: u32 = 0x23000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_mov_i_a2_a_special_cond_3_condition_cc_lo_0_33000000() {
    // Encoding: 0x33000000
    // Test aarch32_MOV_i_A2_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: imm4=0, cond=3, imm12=0, Rd=0
    let encoding: u32 = 0x33000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_mov_i_a2_a_special_cond_4_condition_mi_0_43000000() {
    // Encoding: 0x43000000
    // Test aarch32_MOV_i_A2_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: imm4=0, Rd=0, imm12=0, cond=4
    let encoding: u32 = 0x43000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_mov_i_a2_a_special_cond_5_condition_pl_0_53000000() {
    // Encoding: 0x53000000
    // Test aarch32_MOV_i_A2_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: imm4=0, Rd=0, imm12=0, cond=5
    let encoding: u32 = 0x53000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_mov_i_a2_a_special_cond_6_condition_vs_0_63000000() {
    // Encoding: 0x63000000
    // Test aarch32_MOV_i_A2_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: imm12=0, imm4=0, cond=6, Rd=0
    let encoding: u32 = 0x63000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_mov_i_a2_a_special_cond_7_condition_vc_0_73000000() {
    // Encoding: 0x73000000
    // Test aarch32_MOV_i_A2_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, Rd=0, imm4=0, imm12=0
    let encoding: u32 = 0x73000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_mov_i_a2_a_special_cond_8_condition_hi_0_83000000() {
    // Encoding: 0x83000000
    // Test aarch32_MOV_i_A2_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, imm12=0, imm4=0, Rd=0
    let encoding: u32 = 0x83000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_mov_i_a2_a_special_cond_9_condition_ls_0_93000000() {
    // Encoding: 0x93000000
    // Test aarch32_MOV_i_A2_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: imm12=0, Rd=0, cond=9, imm4=0
    let encoding: u32 = 0x93000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_mov_i_a2_a_special_cond_10_condition_ge_0_a3000000() {
    // Encoding: 0xA3000000
    // Test aarch32_MOV_i_A2_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rd=0, cond=10, imm4=0, imm12=0
    let encoding: u32 = 0xA3000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_mov_i_a2_a_special_cond_11_condition_lt_0_b3000000() {
    // Encoding: 0xB3000000
    // Test aarch32_MOV_i_A2_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rd=0, cond=11, imm12=0, imm4=0
    let encoding: u32 = 0xB3000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_mov_i_a2_a_special_cond_12_condition_gt_0_c3000000() {
    // Encoding: 0xC3000000
    // Test aarch32_MOV_i_A2_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: cond=12, imm4=0, imm12=0, Rd=0
    let encoding: u32 = 0xC3000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_mov_i_a2_a_special_cond_13_condition_le_0_d3000000() {
    // Encoding: 0xD3000000
    // Test aarch32_MOV_i_A2_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: imm4=0, cond=13, imm12=0, Rd=0
    let encoding: u32 = 0xD3000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_mov_i_a2_a_special_cond_14_condition_al_0_e3000000() {
    // Encoding: 0xE3000000
    // Test aarch32_MOV_i_A2_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rd=0, cond=14, imm12=0, imm4=0
    let encoding: u32 = 0xE3000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_mov_i_a2_a_special_cond_15_condition_nv_0_f3000000() {
    // Encoding: 0xF3000000
    // Test aarch32_MOV_i_A2_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, Rd=0, imm4=0, imm12=0
    let encoding: u32 = 0xF3000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mov_i_a2_a_invalid_0_0_03000000() {
    // Encoding: 0x03000000
    // Test aarch32_MOV_i_A2_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: LitInt(15) }
    // ISET: A32
    // Fields: cond=0, imm4=0, Rd=0, imm12=0
    let encoding: u32 = 0x03000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mov_i_a2_a_invalid_1_0_03000000() {
    // Encoding: 0x03000000
    // Test aarch32_MOV_i_A2_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: cond=0, imm4=0, Rd=0, imm12=0
    let encoding: u32 = 0x03000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `field Rd 24 +: 3`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mov_i_t1_a_field_rd_0_min_0_20000000() {
    // Thumb encoding (32): 0x20000000
    // Test aarch32_MOV_i_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: imm8=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x20000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `field Rd 24 +: 3`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mov_i_t1_a_field_rd_1_poweroftwo_0_21000000() {
    // Thumb encoding (32): 0x21000000
    // Test aarch32_MOV_i_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=1, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x21000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_mov_i_t1_a_field_imm8_0_zero_0_20000000() {
    // Thumb encoding (32): 0x20000000
    // Test aarch32_MOV_i_T1_A field imm8 = 0 (Zero)
    // ISET: T32
    // Fields: imm8=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x20000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_mov_i_t1_a_field_imm8_1_poweroftwo_0_20010000() {
    // Thumb encoding (32): 0x20010000
    // Test aarch32_MOV_i_T1_A field imm8 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=1, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x20010000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_mov_i_t1_a_field_imm8_3_poweroftwominusone_0_20030000() {
    // Thumb encoding (32): 0x20030000
    // Test aarch32_MOV_i_T1_A field imm8 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rd=0, imm8=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x20030000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_mov_i_t1_a_field_imm8_4_poweroftwo_0_20040000() {
    // Thumb encoding (32): 0x20040000
    // Test aarch32_MOV_i_T1_A field imm8 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=4, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x20040000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_mov_i_t1_a_field_imm8_7_poweroftwominusone_0_20070000() {
    // Thumb encoding (32): 0x20070000
    // Test aarch32_MOV_i_T1_A field imm8 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=7, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x20070000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_mov_i_t1_a_field_imm8_8_poweroftwo_0_20080000() {
    // Thumb encoding (32): 0x20080000
    // Test aarch32_MOV_i_T1_A field imm8 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, imm8=8
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x20080000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_mov_i_t1_a_field_imm8_15_poweroftwominusone_0_200f0000() {
    // Thumb encoding (32): 0x200F0000
    // Test aarch32_MOV_i_T1_A field imm8 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rd=0, imm8=15
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x200F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_mov_i_t1_a_field_imm8_16_poweroftwo_0_20100000() {
    // Thumb encoding (32): 0x20100000
    // Test aarch32_MOV_i_T1_A field imm8 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=16, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x20100000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_mov_i_t1_a_field_imm8_31_poweroftwominusone_0_201f0000() {
    // Thumb encoding (32): 0x201F0000
    // Test aarch32_MOV_i_T1_A field imm8 = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=31, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x201F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_mov_i_t1_a_field_imm8_32_poweroftwo_0_20200000() {
    // Thumb encoding (32): 0x20200000
    // Test aarch32_MOV_i_T1_A field imm8 = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, imm8=32
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x20200000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_mov_i_t1_a_field_imm8_63_poweroftwominusone_0_203f0000() {
    // Thumb encoding (32): 0x203F0000
    // Test aarch32_MOV_i_T1_A field imm8 = 63 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=63, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x203F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_mov_i_t1_a_field_imm8_64_poweroftwo_0_20400000() {
    // Thumb encoding (32): 0x20400000
    // Test aarch32_MOV_i_T1_A field imm8 = 64 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, imm8=64
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x20400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 127, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (127)
#[test]
fn test_aarch32_mov_i_t1_a_field_imm8_127_poweroftwominusone_0_207f0000() {
    // Thumb encoding (32): 0x207F0000
    // Test aarch32_MOV_i_T1_A field imm8 = 127 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=127, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x207F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_mov_i_t1_a_field_imm8_128_poweroftwo_0_20800000() {
    // Thumb encoding (32): 0x20800000
    // Test aarch32_MOV_i_T1_A field imm8 = 128 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, imm8=128
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x20800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 255, boundary: Max }
/// maximum immediate (255)
#[test]
fn test_aarch32_mov_i_t1_a_field_imm8_255_max_0_20ff0000() {
    // Thumb encoding (32): 0x20FF0000
    // Test aarch32_MOV_i_T1_A field imm8 = 255 (Max)
    // ISET: T32
    // Fields: Rd=0, imm8=255
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x20FF0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch32_mov_i_t1_a_combo_0_0_20000000() {
    // Thumb encoding (32): 0x20000000
    // Test aarch32_MOV_i_T1_A field combination: Rd=0, imm8=0
    // ISET: T32
    // Fields: Rd=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x20000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field i 26 +: 1`
/// Requirement: FieldBoundary { field: "i", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mov_i_t2_a_field_i_0_min_0_f04f0000() {
    // Thumb encoding (32): 0xF04F0000
    // Test aarch32_MOV_i_T2_A field i = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, imm3=0, i=0, imm8=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field i 26 +: 1`
/// Requirement: FieldBoundary { field: "i", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_mov_i_t2_a_field_i_1_max_0_f44f0000() {
    // Thumb encoding (32): 0xF44F0000
    // Test aarch32_MOV_i_T2_A field i = 1 (Max)
    // ISET: T32
    // Fields: imm8=0, S=0, i=1, Rd=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF44F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_mov_i_t2_a_field_s_0_min_0_f04f0000() {
    // Thumb encoding (32): 0xF04F0000
    // Test aarch32_MOV_i_T2_A field S = 0 (Min)
    // ISET: T32
    // Fields: imm3=0, i=0, S=0, Rd=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_mov_i_t2_a_field_s_1_max_0_f05f0000() {
    // Thumb encoding (32): 0xF05F0000
    // Test aarch32_MOV_i_T2_A field S = 1 (Max)
    // ISET: T32
    // Fields: S=1, imm8=0, imm3=0, i=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF05F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_mov_i_t2_a_field_imm3_0_zero_0_f04f0000() {
    // Thumb encoding (32): 0xF04F0000
    // Test aarch32_MOV_i_T2_A field imm3 = 0 (Zero)
    // ISET: T32
    // Fields: S=0, imm8=0, i=0, imm3=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_mov_i_t2_a_field_imm3_1_poweroftwo_0_f04f1000() {
    // Thumb encoding (32): 0xF04F1000
    // Test aarch32_MOV_i_T2_A field imm3 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=0, S=0, i=0, Rd=0, imm3=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F1000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch32_mov_i_t2_a_field_imm3_3_poweroftwominusone_0_f04f3000() {
    // Thumb encoding (32): 0xF04F3000
    // Test aarch32_MOV_i_T2_A field imm3 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: S=0, i=0, imm3=3, Rd=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F3000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch32_mov_i_t2_a_field_imm3_7_max_0_f04f7000() {
    // Thumb encoding (32): 0xF04F7000
    // Test aarch32_MOV_i_T2_A field imm3 = 7 (Max)
    // ISET: T32
    // Fields: imm3=7, imm8=0, Rd=0, i=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F7000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mov_i_t2_a_field_rd_0_min_0_f04f0000() {
    // Thumb encoding (32): 0xF04F0000
    // Test aarch32_MOV_i_T2_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, imm8=0, S=0, imm3=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mov_i_t2_a_field_rd_1_poweroftwo_0_f04f0100() {
    // Thumb encoding (32): 0xF04F0100
    // Test aarch32_MOV_i_T2_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=0, imm8=0, i=0, S=0, Rd=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F0100;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_mov_i_t2_a_field_imm8_0_zero_0_f04f0000() {
    // Thumb encoding (32): 0xF04F0000
    // Test aarch32_MOV_i_T2_A field imm8 = 0 (Zero)
    // ISET: T32
    // Fields: Rd=0, imm3=0, S=0, imm8=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_mov_i_t2_a_field_imm8_1_poweroftwo_0_f04f0001() {
    // Thumb encoding (32): 0xF04F0001
    // Test aarch32_MOV_i_T2_A field imm8 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, Rd=0, imm3=0, i=0, imm8=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F0001;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_mov_i_t2_a_field_imm8_3_poweroftwominusone_0_f04f0003() {
    // Thumb encoding (32): 0xF04F0003
    // Test aarch32_MOV_i_T2_A field imm8 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: i=0, imm3=0, Rd=0, imm8=3, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F0003;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_mov_i_t2_a_field_imm8_4_poweroftwo_0_f04f0004() {
    // Thumb encoding (32): 0xF04F0004
    // Test aarch32_MOV_i_T2_A field imm8 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=4, imm3=0, i=0, S=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F0004;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_mov_i_t2_a_field_imm8_7_poweroftwominusone_0_f04f0007() {
    // Thumb encoding (32): 0xF04F0007
    // Test aarch32_MOV_i_T2_A field imm8 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: S=0, Rd=0, i=0, imm3=0, imm8=7
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F0007;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_mov_i_t2_a_field_imm8_8_poweroftwo_0_f04f0008() {
    // Thumb encoding (32): 0xF04F0008
    // Test aarch32_MOV_i_T2_A field imm8 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=8, imm3=0, i=0, Rd=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F0008;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_mov_i_t2_a_field_imm8_15_poweroftwominusone_0_f04f000f() {
    // Thumb encoding (32): 0xF04F000F
    // Test aarch32_MOV_i_T2_A field imm8 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rd=0, imm8=15, S=0, i=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F000F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_mov_i_t2_a_field_imm8_16_poweroftwo_0_f04f0010() {
    // Thumb encoding (32): 0xF04F0010
    // Test aarch32_MOV_i_T2_A field imm8 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=0, imm8=16, S=0, Rd=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F0010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_mov_i_t2_a_field_imm8_31_poweroftwominusone_0_f04f001f() {
    // Thumb encoding (32): 0xF04F001F
    // Test aarch32_MOV_i_T2_A field imm8 = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: S=0, imm3=0, Rd=0, i=0, imm8=31
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_mov_i_t2_a_field_imm8_32_poweroftwo_0_f04f0020() {
    // Thumb encoding (32): 0xF04F0020
    // Test aarch32_MOV_i_T2_A field imm8 = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: i=0, S=0, Rd=0, imm3=0, imm8=32
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F0020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_mov_i_t2_a_field_imm8_63_poweroftwominusone_0_f04f003f() {
    // Thumb encoding (32): 0xF04F003F
    // Test aarch32_MOV_i_T2_A field imm8 = 63 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rd=0, S=0, imm8=63, imm3=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_mov_i_t2_a_field_imm8_64_poweroftwo_0_f04f0040() {
    // Thumb encoding (32): 0xF04F0040
    // Test aarch32_MOV_i_T2_A field imm8 = 64 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, imm8=64, S=0, i=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F0040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 127, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (127)
#[test]
fn test_aarch32_mov_i_t2_a_field_imm8_127_poweroftwominusone_0_f04f007f() {
    // Thumb encoding (32): 0xF04F007F
    // Test aarch32_MOV_i_T2_A field imm8 = 127 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: S=0, i=0, imm3=0, Rd=0, imm8=127
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F007F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_mov_i_t2_a_field_imm8_128_poweroftwo_0_f04f0080() {
    // Thumb encoding (32): 0xF04F0080
    // Test aarch32_MOV_i_T2_A field imm8 = 128 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, imm8=128, Rd=0, i=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F0080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 255, boundary: Max }
/// maximum immediate (255)
#[test]
fn test_aarch32_mov_i_t2_a_field_imm8_255_max_0_f04f00ff() {
    // Thumb encoding (32): 0xF04F00FF
    // Test aarch32_MOV_i_T2_A field imm8 = 255 (Max)
    // ISET: T32
    // Fields: i=0, Rd=0, imm8=255, imm3=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F00FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// i=0 (minimum value)
#[test]
fn test_aarch32_mov_i_t2_a_combo_0_0_f04f0000() {
    // Thumb encoding (32): 0xF04F0000
    // Test aarch32_MOV_i_T2_A field combination: i=0, S=0, imm3=0, Rd=0, imm8=0
    // ISET: T32
    // Fields: S=0, i=0, imm8=0, Rd=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_mov_i_t2_a_special_s_0_size_variant_0_0_f04f0000() {
    // Thumb encoding (32): 0xF04F0000
    // Test aarch32_MOV_i_T2_A special value S = 0 (Size variant 0)
    // ISET: T32
    // Fields: i=0, Rd=0, imm8=0, imm3=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_mov_i_t2_a_special_s_1_size_variant_1_0_f05f0000() {
    // Thumb encoding (32): 0xF05F0000
    // Test aarch32_MOV_i_T2_A special value S = 1 (Size variant 1)
    // ISET: T32
    // Fields: S=1, imm3=0, Rd=0, i=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF05F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mov_i_t2_a_invalid_0_0_f04f0000() {
    // Thumb encoding (32): 0xF04F0000
    // Test aarch32_MOV_i_T2_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rd=0, S=0, imm3=0, imm8=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F0000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mov_i_t2_a_invalid_1_0_f04f0000() {
    // Thumb encoding (32): 0xF04F0000
    // Test aarch32_MOV_i_T2_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: S=0, imm3=0, imm8=0, i=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF04F0000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field i 26 +: 1`
/// Requirement: FieldBoundary { field: "i", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mov_i_t3_a_field_i_0_min_0_f2400000() {
    // Thumb encoding (32): 0xF2400000
    // Test aarch32_MOV_i_T3_A field i = 0 (Min)
    // ISET: T32
    // Fields: imm4=0, imm3=0, i=0, Rd=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field i 26 +: 1`
/// Requirement: FieldBoundary { field: "i", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_mov_i_t3_a_field_i_1_max_0_f6400000() {
    // Thumb encoding (32): 0xF6400000
    // Test aarch32_MOV_i_T3_A field i = 1 (Max)
    // ISET: T32
    // Fields: i=1, imm3=0, Rd=0, imm4=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF6400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_mov_i_t3_a_field_imm4_0_zero_0_f2400000() {
    // Thumb encoding (32): 0xF2400000
    // Test aarch32_MOV_i_T3_A field imm4 = 0 (Zero)
    // ISET: T32
    // Fields: imm8=0, imm3=0, imm4=0, Rd=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_mov_i_t3_a_field_imm4_1_poweroftwo_0_f2410000() {
    // Thumb encoding (32): 0xF2410000
    // Test aarch32_MOV_i_T3_A field imm4 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=0, imm4=1, Rd=0, i=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2410000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_mov_i_t3_a_field_imm4_3_poweroftwominusone_0_f2430000() {
    // Thumb encoding (32): 0xF2430000
    // Test aarch32_MOV_i_T3_A field imm4 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm4=3, i=0, imm3=0, imm8=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2430000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_mov_i_t3_a_field_imm4_4_poweroftwo_0_f2440000() {
    // Thumb encoding (32): 0xF2440000
    // Test aarch32_MOV_i_T3_A field imm4 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: i=0, imm3=0, imm8=0, imm4=4, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2440000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch32_mov_i_t3_a_field_imm4_7_poweroftwominusone_0_f2470000() {
    // Thumb encoding (32): 0xF2470000
    // Test aarch32_MOV_i_T3_A field imm4 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=0, i=0, imm3=0, Rd=0, imm4=7
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2470000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_mov_i_t3_a_field_imm4_8_poweroftwo_0_f2480000() {
    // Thumb encoding (32): 0xF2480000
    // Test aarch32_MOV_i_T3_A field imm4 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=0, Rd=0, i=0, imm8=0, imm4=8
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2480000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm4 16 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch32_mov_i_t3_a_field_imm4_15_max_0_f24f0000() {
    // Thumb encoding (32): 0xF24F0000
    // Test aarch32_MOV_i_T3_A field imm4 = 15 (Max)
    // ISET: T32
    // Fields: i=0, imm8=0, imm4=15, imm3=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF24F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_mov_i_t3_a_field_imm3_0_zero_0_f2400000() {
    // Thumb encoding (32): 0xF2400000
    // Test aarch32_MOV_i_T3_A field imm3 = 0 (Zero)
    // ISET: T32
    // Fields: imm4=0, i=0, Rd=0, imm8=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_mov_i_t3_a_field_imm3_1_poweroftwo_0_f2401000() {
    // Thumb encoding (32): 0xF2401000
    // Test aarch32_MOV_i_T3_A field imm3 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=0, imm3=1, i=0, imm4=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2401000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch32_mov_i_t3_a_field_imm3_3_poweroftwominusone_0_f2403000() {
    // Thumb encoding (32): 0xF2403000
    // Test aarch32_MOV_i_T3_A field imm3 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm4=0, i=0, Rd=0, imm8=0, imm3=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2403000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch32_mov_i_t3_a_field_imm3_7_max_0_f2407000() {
    // Thumb encoding (32): 0xF2407000
    // Test aarch32_MOV_i_T3_A field imm3 = 7 (Max)
    // ISET: T32
    // Fields: Rd=0, imm8=0, i=0, imm4=0, imm3=7
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2407000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mov_i_t3_a_field_rd_0_min_0_f2400000() {
    // Thumb encoding (32): 0xF2400000
    // Test aarch32_MOV_i_T3_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: imm3=0, i=0, imm4=0, Rd=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mov_i_t3_a_field_rd_1_poweroftwo_0_f2400100() {
    // Thumb encoding (32): 0xF2400100
    // Test aarch32_MOV_i_T3_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm4=0, i=0, imm8=0, Rd=1, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2400100;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_mov_i_t3_a_field_imm8_0_zero_0_f2400000() {
    // Thumb encoding (32): 0xF2400000
    // Test aarch32_MOV_i_T3_A field imm8 = 0 (Zero)
    // ISET: T32
    // Fields: i=0, imm8=0, Rd=0, imm4=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_mov_i_t3_a_field_imm8_1_poweroftwo_0_f2400001() {
    // Thumb encoding (32): 0xF2400001
    // Test aarch32_MOV_i_T3_A field imm8 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=1, imm4=0, i=0, imm3=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2400001;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_mov_i_t3_a_field_imm8_3_poweroftwominusone_0_f2400003() {
    // Thumb encoding (32): 0xF2400003
    // Test aarch32_MOV_i_T3_A field imm8 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm4=0, imm8=3, imm3=0, Rd=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2400003;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_mov_i_t3_a_field_imm8_4_poweroftwo_0_f2400004() {
    // Thumb encoding (32): 0xF2400004
    // Test aarch32_MOV_i_T3_A field imm8 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: imm4=0, imm3=0, imm8=4, i=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2400004;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_mov_i_t3_a_field_imm8_7_poweroftwominusone_0_f2400007() {
    // Thumb encoding (32): 0xF2400007
    // Test aarch32_MOV_i_T3_A field imm8 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm3=0, imm8=7, imm4=0, i=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2400007;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_mov_i_t3_a_field_imm8_8_poweroftwo_0_f2400008() {
    // Thumb encoding (32): 0xF2400008
    // Test aarch32_MOV_i_T3_A field imm8 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=0, i=0, imm4=0, Rd=0, imm8=8
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2400008;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_mov_i_t3_a_field_imm8_15_poweroftwominusone_0_f240000f() {
    // Thumb encoding (32): 0xF240000F
    // Test aarch32_MOV_i_T3_A field imm8 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rd=0, imm8=15, imm3=0, i=0, imm4=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF240000F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_mov_i_t3_a_field_imm8_16_poweroftwo_0_f2400010() {
    // Thumb encoding (32): 0xF2400010
    // Test aarch32_MOV_i_T3_A field imm8 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=16, imm3=0, imm4=0, i=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2400010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_mov_i_t3_a_field_imm8_31_poweroftwominusone_0_f240001f() {
    // Thumb encoding (32): 0xF240001F
    // Test aarch32_MOV_i_T3_A field imm8 = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm4=0, imm3=0, Rd=0, imm8=31, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF240001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_mov_i_t3_a_field_imm8_32_poweroftwo_0_f2400020() {
    // Thumb encoding (32): 0xF2400020
    // Test aarch32_MOV_i_T3_A field imm8 = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: i=0, Rd=0, imm8=32, imm4=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_mov_i_t3_a_field_imm8_63_poweroftwominusone_0_f240003f() {
    // Thumb encoding (32): 0xF240003F
    // Test aarch32_MOV_i_T3_A field imm8 = 63 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rd=0, imm8=63, imm3=0, i=0, imm4=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF240003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_mov_i_t3_a_field_imm8_64_poweroftwo_0_f2400040() {
    // Thumb encoding (32): 0xF2400040
    // Test aarch32_MOV_i_T3_A field imm8 = 64 (PowerOfTwo)
    // ISET: T32
    // Fields: i=0, Rd=0, imm4=0, imm8=64, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2400040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 127, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (127)
#[test]
fn test_aarch32_mov_i_t3_a_field_imm8_127_poweroftwominusone_0_f240007f() {
    // Thumb encoding (32): 0xF240007F
    // Test aarch32_MOV_i_T3_A field imm8 = 127 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: i=0, imm8=127, imm3=0, imm4=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF240007F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_mov_i_t3_a_field_imm8_128_poweroftwo_0_f2400080() {
    // Thumb encoding (32): 0xF2400080
    // Test aarch32_MOV_i_T3_A field imm8 = 128 (PowerOfTwo)
    // ISET: T32
    // Fields: i=0, imm4=0, Rd=0, imm3=0, imm8=128
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2400080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 255, boundary: Max }
/// maximum immediate (255)
#[test]
fn test_aarch32_mov_i_t3_a_field_imm8_255_max_0_f24000ff() {
    // Thumb encoding (32): 0xF24000FF
    // Test aarch32_MOV_i_T3_A field imm8 = 255 (Max)
    // ISET: T32
    // Fields: imm4=0, imm8=255, Rd=0, i=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF24000FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// i=0 (minimum value)
#[test]
fn test_aarch32_mov_i_t3_a_combo_0_0_f2400000() {
    // Thumb encoding (32): 0xF2400000
    // Test aarch32_MOV_i_T3_A field combination: i=0, imm4=0, imm3=0, Rd=0, imm8=0
    // ISET: T32
    // Fields: imm4=0, imm8=0, imm3=0, Rd=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mov_i_t3_a_invalid_0_0_f2400000() {
    // Thumb encoding (32): 0xF2400000
    // Test aarch32_MOV_i_T3_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: LitInt(15) }
    // ISET: T32
    // Fields: i=0, imm3=0, Rd=0, imm8=0, imm4=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2400000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mov_i_t3_a_invalid_1_0_f2400000() {
    // Thumb encoding (32): 0xF2400000
    // Test aarch32_MOV_i_T3_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: imm3=0, i=0, imm4=0, Rd=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2400000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `MOVZ X0, #0x1234, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// lower 16 bits (32)
#[test]
fn test_aarch32_mov_i_a1_a_movz_oracle_32_0_03a24680() {
    // Test MOVZ 32-bit: lower 16 bits (oracle)
    // Encoding: 0x03A24680
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x03A24680;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1234, "W0 should be 0x00001234");
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `MOVZ X0, #0x1234, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// lower 16 bits (64)
#[test]
fn test_aarch32_mov_i_a1_a_movz_oracle_64_0_83a24680() {
    // Test MOVZ 64-bit: lower 16 bits (oracle)
    // Encoding: 0x83A24680
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x83A24680;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1234, "X0 should be 0x0000000000001234");
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `MOVZ X0, #0xABCD, LSL #16`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shifted 16 bits (32)
#[test]
fn test_aarch32_mov_i_a1_a_movz_oracle_32_1_03b579a0() {
    // Test MOVZ 32-bit: shifted 16 bits (oracle)
    // Encoding: 0x03B579A0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x03B579A0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xABCD0000, "W0 should be 0xABCD0000");
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `MOVZ X0, #0xABCD, LSL #16`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shifted 16 bits (64)
#[test]
fn test_aarch32_mov_i_a1_a_movz_oracle_64_1_83b579a0() {
    // Test MOVZ 64-bit: shifted 16 bits (oracle)
    // Encoding: 0x83B579A0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x83B579A0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xABCD0000,
        "X0 should be 0x00000000ABCD0000"
    );
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `MOVZ X0, #0xFFFF, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm16 (32)
#[test]
fn test_aarch32_mov_i_a1_a_movz_oracle_32_2_03bfffe0() {
    // Test MOVZ 32-bit: max imm16 (oracle)
    // Encoding: 0x03BFFFE0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x03BFFFE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFF, "W0 should be 0x0000FFFF");
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `MOVZ X0, #0xFFFF, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm16 (64)
#[test]
fn test_aarch32_mov_i_a1_a_movz_oracle_64_2_83bfffe0() {
    // Test MOVZ 64-bit: max imm16 (oracle)
    // Encoding: 0x83BFFFE0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x83BFFFE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFF, "X0 should be 0x000000000000FFFF");
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `MOVZ X0, #0x0000, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero imm16 (32)
#[test]
fn test_aarch32_mov_i_a1_a_movz_oracle_32_3_03a00000() {
    // Test MOVZ 32-bit: zero imm16 (oracle)
    // Encoding: 0x03A00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x03A00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x00000000");
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `MOVZ X0, #0x0000, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero imm16 (64)
#[test]
fn test_aarch32_mov_i_a1_a_movz_oracle_64_3_83a00000() {
    // Test MOVZ 64-bit: zero imm16 (oracle)
    // Encoding: 0x83A00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x83A00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `MOVZ X0, #0x5678, LSL #32`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shifted 32 bits (64)
#[test]
fn test_aarch32_mov_i_a1_a_movz_oracle_64_4_83eacf00() {
    // Test MOVZ 64-bit: shifted 32 bits (oracle)
    // Encoding: 0x83EACF00
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x83EACF00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000567800000000");
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `MOVZ X0, #0xDEAD, LSL #48`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shifted 48 bits (64)
#[test]
fn test_aarch32_mov_i_a1_a_movz_oracle_64_5_83fbd5a0() {
    // Test MOVZ 64-bit: shifted 48 bits (oracle)
    // Encoding: 0x83FBD5A0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x83FBD5A0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0xDEAD000000000000");
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `MOV R0, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate
#[test]
fn test_aarch32_mov_i_a1_a_a32_mov_imm_0_03a0000a() {
    // Test A32 MOV: small immediate (oracle)
    // Encoding: 0x03A0000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x03A0000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xA, "R0 should be 0x0000000A");
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `MOV R0, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8
#[test]
fn test_aarch32_mov_i_a1_a_a32_mov_imm_1_03a000ff() {
    // Test A32 MOV: max imm8 (oracle)
    // Encoding: 0x03A000FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x03A000FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF, "R0 should be 0x000000FF");
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `MOV R0, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2
#[test]
fn test_aarch32_mov_i_a1_a_a32_mov_imm_2_03a00180() {
    // Test A32 MOV: rotated by 2 (oracle)
    // Encoding: 0x03A00180
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x03A00180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x20, "R0 should be 0x00000020");
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `MOV R0, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8
#[test]
fn test_aarch32_mov_i_a1_a_a32_mov_imm_3_03a0040f() {
    // Test A32 MOV: rotated by 8 (oracle)
    // Encoding: 0x03A0040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x03A0040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xF000000, "R0 should be 0x0F000000");
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `MOV R0, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate
#[test]
fn test_aarch32_mov_i_a1_a_a32_mov_imm_4_03a00000() {
    // Test A32 MOV: zero immediate (oracle)
    // Encoding: 0x03A00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x03A00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_mov_i_a1_a_flags_zeroresult_0_03b00000() {
    // Test aarch32_MOV_i_A1_A flag computation: ZeroResult
    // Encoding: 0x03B00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x03B00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_mov_i_a1_a_flags_zeroresult_1_03b00000() {
    // Test aarch32_MOV_i_A1_A flag computation: ZeroResult
    // Encoding: 0x03B00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x03B00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_mov_i_a1_a_flags_negativeresult_2_03b00000() {
    // Test aarch32_MOV_i_A1_A flag computation: NegativeResult
    // Encoding: 0x03B00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x03B00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_mov_i_a1_a_flags_unsignedoverflow_3_03b00000() {
    // Test aarch32_MOV_i_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x03B00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x03B00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_mov_i_a1_a_flags_unsignedoverflow_4_03b00000() {
    // Test aarch32_MOV_i_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x03B00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x03B00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_mov_i_a1_a_flags_signedoverflow_5_03b00000() {
    // Test aarch32_MOV_i_A1_A flag computation: SignedOverflow
    // Encoding: 0x03B00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x03B00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_mov_i_a1_a_flags_signedoverflow_6_03b00000() {
    // Test aarch32_MOV_i_A1_A flag computation: SignedOverflow
    // Encoding: 0x03B00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x03B00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_i_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_mov_i_a1_a_flags_positiveresult_7_03b00000() {
    // Test aarch32_MOV_i_A1_A flag computation: PositiveResult
    // Encoding: 0x03B00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u32 = 0x03B00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// zero AND zero (32-bit)
#[test]
fn test_aarch32_mov_i_a2_a_oracle_32_0_6a02003f() {
    // Test TST 32-bit: zero AND zero (oracle)
    // Encoding: 0x6A02003F
    // ISET: A32
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

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// zero AND zero (64-bit)
#[test]
fn test_aarch32_mov_i_a2_a_oracle_64_0_ea02003f() {
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

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// partial overlap (32-bit)
#[test]
fn test_aarch32_mov_i_a2_a_oracle_32_1_6a02003f() {
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

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// partial overlap (64-bit)
#[test]
fn test_aarch32_mov_i_a2_a_oracle_64_1_ea02003f() {
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

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// no overlap (32-bit)
#[test]
fn test_aarch32_mov_i_a2_a_oracle_32_2_6a02003f() {
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

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// no overlap (64-bit)
#[test]
fn test_aarch32_mov_i_a2_a_oracle_64_2_ea02003f() {
    // Test TST 64-bit: no overlap (oracle)
    // Encoding: 0xEA02003F
    // ISET: A32
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

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// MSB set (32-bit)
#[test]
fn test_aarch32_mov_i_a2_a_oracle_32_3_6a02003f() {
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

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// MSB set (64-bit)
#[test]
fn test_aarch32_mov_i_a2_a_oracle_64_3_ea02003f() {
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

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// all ones (32-bit)
#[test]
fn test_aarch32_mov_i_a2_a_oracle_32_4_6a02003f() {
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

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// all ones (64-bit)
#[test]
fn test_aarch32_mov_i_a2_a_oracle_64_4_ea02003f() {
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

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// alternating (no match) (32-bit)
#[test]
fn test_aarch32_mov_i_a2_a_oracle_32_5_6a02003f() {
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

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `TST X1, X2`
/// Requirement: FlagComputation { flag: Z, scenario: ZeroResult }
/// alternating (no match) (64-bit)
#[test]
fn test_aarch32_mov_i_a2_a_oracle_64_5_ea02003f() {
    // Test TST 64-bit: alternating (no match) (oracle)
    // Encoding: 0xEA02003F
    // ISET: A32
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

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_mov_i_a2_a_flags_zeroresult_0_03000000() {
    // Test aarch32_MOV_i_A2_A flag computation: ZeroResult
    // Encoding: 0x03000000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x03000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_mov_i_a2_a_flags_zeroresult_1_03000000() {
    // Test aarch32_MOV_i_A2_A flag computation: ZeroResult
    // Encoding: 0x03000000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x03000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_mov_i_a2_a_flags_negativeresult_2_03000000() {
    // Test aarch32_MOV_i_A2_A flag computation: NegativeResult
    // Encoding: 0x03000000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x03000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_mov_i_a2_a_flags_unsignedoverflow_3_03000000() {
    // Test aarch32_MOV_i_A2_A flag computation: UnsignedOverflow
    // Encoding: 0x03000000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x03000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_mov_i_a2_a_flags_unsignedoverflow_4_03000000() {
    // Test aarch32_MOV_i_A2_A flag computation: UnsignedOverflow
    // Encoding: 0x03000000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x03000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_mov_i_a2_a_flags_signedoverflow_5_03000000() {
    // Test aarch32_MOV_i_A2_A flag computation: SignedOverflow
    // Encoding: 0x03000000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x03000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_mov_i_a2_a_flags_signedoverflow_6_03000000() {
    // Test aarch32_MOV_i_A2_A flag computation: SignedOverflow
    // Encoding: 0x03000000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x03000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_i_A2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_mov_i_a2_a_flags_positiveresult_7_03000000() {
    // Test aarch32_MOV_i_A2_A flag computation: PositiveResult
    // Encoding: 0x03000000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u32 = 0x03000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 0 (32)
#[test]
fn test_aarch32_mov_i_t1_a_lslv_oracle_32_0_20020020() {
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

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 0 (64)
#[test]
fn test_aarch32_mov_i_t1_a_lslv_oracle_64_0_a0020020() {
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

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4 (32)
#[test]
fn test_aarch32_mov_i_t1_a_lslv_oracle_32_1_20020020() {
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

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 4 (64)
#[test]
fn test_aarch32_mov_i_t1_a_lslv_oracle_64_1_a0020020() {
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

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 8 (32)
#[test]
fn test_aarch32_mov_i_t1_a_lslv_oracle_32_2_20020020() {
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

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 8 (64)
#[test]
fn test_aarch32_mov_i_t1_a_lslv_oracle_64_2_a0020020() {
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

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1 (32)
#[test]
fn test_aarch32_mov_i_t1_a_lslv_oracle_32_3_20020020() {
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

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// MSB set, shift 1 (64)
#[test]
fn test_aarch32_mov_i_t1_a_lslv_oracle_64_3_a0020020() {
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

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSB set, max shift (32)
#[test]
fn test_aarch32_mov_i_t1_a_lslv_oracle_32_4_20020020() {
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

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSB set, max shift (64)
#[test]
fn test_aarch32_mov_i_t1_a_lslv_oracle_64_4_a0020020() {
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

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// all ones, shift 32 (32)
#[test]
fn test_aarch32_mov_i_t1_a_lslv_oracle_32_5_20020020() {
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

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all ones, shift 32 (64)
#[test]
fn test_aarch32_mov_i_t1_a_lslv_oracle_64_5_a0020020() {
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

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift
#[test]
fn test_aarch32_mov_i_t1_a_t16_oracle_0_20000000() {
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

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4
#[test]
fn test_aarch32_mov_i_t1_a_t16_oracle_1_20000000() {
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

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1
#[test]
fn test_aarch32_mov_i_t1_a_t16_oracle_2_20000000() {
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

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift to MSB
#[test]
fn test_aarch32_mov_i_t1_a_t16_oracle_3_20000000() {
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

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_mov_i_t1_a_flags_zeroresult_0_20000000() {
    // Test aarch32_MOV_i_T1_A flag computation: ZeroResult
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
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_mov_i_t1_a_flags_zeroresult_1_20000000() {
    // Test aarch32_MOV_i_T1_A flag computation: ZeroResult
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

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_mov_i_t1_a_flags_negativeresult_2_20000000() {
    // Test aarch32_MOV_i_T1_A flag computation: NegativeResult
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

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_mov_i_t1_a_flags_unsignedoverflow_3_20000000() {
    // Test aarch32_MOV_i_T1_A flag computation: UnsignedOverflow
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

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_mov_i_t1_a_flags_unsignedoverflow_4_20000000() {
    // Test aarch32_MOV_i_T1_A flag computation: UnsignedOverflow
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
}

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_mov_i_t1_a_flags_signedoverflow_5_20000000() {
    // Test aarch32_MOV_i_T1_A flag computation: SignedOverflow
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
}

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_mov_i_t1_a_flags_signedoverflow_6_20000000() {
    // Test aarch32_MOV_i_T1_A flag computation: SignedOverflow
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
}

/// Provenance: aarch32_MOV_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_mov_i_t1_a_flags_positiveresult_7_20000000() {
    // Test aarch32_MOV_i_T1_A flag computation: PositiveResult
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
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `MOVZ X0, #0x1234, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// lower 16 bits (32)
#[test]
fn test_aarch32_mov_i_t2_a_movz_oracle_32_0_f04f4680() {
    // Test MOVZ 32-bit: lower 16 bits (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF04F4680;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1234, "W0 should be 0x00001234");
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `MOVZ X0, #0x1234, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// lower 16 bits (64)
#[test]
fn test_aarch32_mov_i_t2_a_movz_oracle_64_0_f04f4680() {
    // Test MOVZ 64-bit: lower 16 bits (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF04F4680;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1234, "X0 should be 0x0000000000001234");
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `MOVZ X0, #0xABCD, LSL #16`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shifted 16 bits (32)
#[test]
fn test_aarch32_mov_i_t2_a_movz_oracle_32_1_f07f79a0() {
    // Test MOVZ 32-bit: shifted 16 bits (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF07F79A0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xABCD0000, "W0 should be 0xABCD0000");
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `MOVZ X0, #0xABCD, LSL #16`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shifted 16 bits (64)
#[test]
fn test_aarch32_mov_i_t2_a_movz_oracle_64_1_f07f79a0() {
    // Test MOVZ 64-bit: shifted 16 bits (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF07F79A0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xABCD0000,
        "X0 should be 0x00000000ABCD0000"
    );
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `MOVZ X0, #0xFFFF, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm16 (32)
#[test]
fn test_aarch32_mov_i_t2_a_movz_oracle_32_2_f05fffe0() {
    // Test MOVZ 32-bit: max imm16 (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF05FFFE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFF, "W0 should be 0x0000FFFF");
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `MOVZ X0, #0xFFFF, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm16 (64)
#[test]
fn test_aarch32_mov_i_t2_a_movz_oracle_64_2_f05fffe0() {
    // Test MOVZ 64-bit: max imm16 (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF05FFFE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFF, "X0 should be 0x000000000000FFFF");
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `MOVZ X0, #0x0000, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero imm16 (32)
#[test]
fn test_aarch32_mov_i_t2_a_movz_oracle_32_3_f04f0000() {
    // Test MOVZ 32-bit: zero imm16 (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF04F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x00000000");
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `MOVZ X0, #0x0000, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero imm16 (64)
#[test]
fn test_aarch32_mov_i_t2_a_movz_oracle_64_3_f04f0000() {
    // Test MOVZ 64-bit: zero imm16 (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF04F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `MOVZ X0, #0x5678, LSL #32`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shifted 32 bits (64)
#[test]
fn test_aarch32_mov_i_t2_a_movz_oracle_64_4_f04fcf00() {
    // Test MOVZ 64-bit: shifted 32 bits (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF04FCF00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000567800000000");
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `MOVZ X0, #0xDEAD, LSL #48`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shifted 48 bits (64)
#[test]
fn test_aarch32_mov_i_t2_a_movz_oracle_64_5_f07fd5a0() {
    // Test MOVZ 64-bit: shifted 48 bits (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF07FD5A0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0xDEAD000000000000");
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_mov_i_t2_a_flags_zeroresult_0_f05f0000() {
    // Test aarch32_MOV_i_T2_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF05F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_mov_i_t2_a_flags_zeroresult_1_f05f0000() {
    // Test aarch32_MOV_i_T2_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF05F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_mov_i_t2_a_flags_negativeresult_2_f05f0000() {
    // Test aarch32_MOV_i_T2_A flag computation: NegativeResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF05F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_mov_i_t2_a_flags_unsignedoverflow_3_f05f0000() {
    // Test aarch32_MOV_i_T2_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF05F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_mov_i_t2_a_flags_unsignedoverflow_4_f05f0000() {
    // Test aarch32_MOV_i_T2_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x2);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF05F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_mov_i_t2_a_flags_signedoverflow_5_f05f0000() {
    // Test aarch32_MOV_i_T2_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xF05F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_mov_i_t2_a_flags_signedoverflow_6_f05f0000() {
    // Test aarch32_MOV_i_T2_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0xF05F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_i_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_mov_i_t2_a_flags_positiveresult_7_f05f0000() {
    // Test aarch32_MOV_i_T2_A flag computation: PositiveResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u32 = 0xF05F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `ORR X0, X1, #0xFF`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// mask lower 8 bits (64)
#[test]
fn test_aarch32_mov_i_t3_a_orr_oracle_64_0_b2401c20() {
    // Test ORR 64-bit: mask lower 8 bits (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xB2401C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `ORR X0, X1, #0xFFFF`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// mask lower 16 bits (64)
#[test]
fn test_aarch32_mov_i_t3_a_orr_oracle_64_1_b2403c20() {
    // Test ORR 64-bit: mask lower 16 bits (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xB2403C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `ORR X0, X1, #0xFFFFFFFF`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// mask lower 32 bits (64)
#[test]
fn test_aarch32_mov_i_t3_a_orr_oracle_64_2_b2407c20() {
    // Test ORR 64-bit: mask lower 32 bits (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xB2407C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `ORR X0, X1, #0x1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// single bit mask (64)
#[test]
fn test_aarch32_mov_i_t3_a_orr_oracle_64_3_b2400020() {
    // Test ORR 64-bit: single bit mask (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xCAFEBABE);
    let encoding: u32 = 0xB2400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xCAFEBABF,
        "X0 should be 0xDEADBEEFCAFEBABF"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `ORR X0, X1, #0x7FFFFFFFFFFFFFFF`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all but MSB (64)
#[test]
fn test_aarch32_mov_i_t3_a_orr_oracle_64_4_b240f820() {
    // Test ORR 64-bit: all but MSB (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0xB240F820;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `ORR W0, W1, #0xFF`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// mask lower 8 bits (32)
#[test]
fn test_aarch32_mov_i_t3_a_orr_oracle_32_0_32001c20() {
    // Test ORR 32-bit: mask lower 8 bits (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x32001C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "W0 should be 0xFFFFFFFF");
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `ORR W0, W1, #0xFFFF`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// mask lower 16 bits (32)
#[test]
fn test_aarch32_mov_i_t3_a_orr_oracle_32_1_32003c20() {
    // Test ORR 32-bit: mask lower 16 bits (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x32003C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "W0 should be 0xFFFFFFFF");
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `ORR W0, W1, #0x1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// single bit mask (32)
#[test]
fn test_aarch32_mov_i_t3_a_orr_oracle_32_2_32000020() {
    // Test ORR 32-bit: single bit mask (oracle)
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xDEADBEEF);
    let encoding: u32 = 0x32000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xDEADBEEF, "W0 should be 0xDEADBEEF");
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_mov_i_t3_a_flags_zeroresult_0_f2400000() {
    // Test aarch32_MOV_i_T3_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_mov_i_t3_a_flags_zeroresult_1_f2400000() {
    // Test aarch32_MOV_i_T3_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_mov_i_t3_a_flags_negativeresult_2_f2400000() {
    // Test aarch32_MOV_i_T3_A flag computation: NegativeResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xF2400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_mov_i_t3_a_flags_unsignedoverflow_3_f2400000() {
    // Test aarch32_MOV_i_T3_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_mov_i_t3_a_flags_unsignedoverflow_4_f2400000() {
    // Test aarch32_MOV_i_T3_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u32 = 0xF2400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_mov_i_t3_a_flags_signedoverflow_5_f2400000() {
    // Test aarch32_MOV_i_T3_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xF2400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_mov_i_t3_a_flags_signedoverflow_6_f2400000() {
    // Test aarch32_MOV_i_T3_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MOV_i_T3_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_mov_i_t3_a_flags_positiveresult_7_f2400000() {
    // Test aarch32_MOV_i_T3_A flag computation: PositiveResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x32);
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

// ============================================================================
// aarch32_MVN_r_A Tests
// ============================================================================

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_mvn_r_a1_a_field_cond_0_min_0_01e00000() {
    // Encoding: 0x01E00000
    // Test aarch32_MVN_r_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, cond=0, imm5=0, type1=0, Rd=0, S=0
    let encoding: u32 = 0x01E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_mvn_r_a1_a_field_cond_1_poweroftwo_0_11e00000() {
    // Encoding: 0x11E00000
    // Test aarch32_MVN_r_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=0, S=0, Rd=0, cond=1, type1=0, Rm=0
    let encoding: u32 = 0x11E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_mvn_r_a1_a_field_cond_2_poweroftwo_0_21e00000() {
    // Encoding: 0x21E00000
    // Test aarch32_MVN_r_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, type1=0, cond=2, S=0, imm5=0, Rd=0
    let encoding: u32 = 0x21E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_mvn_r_a1_a_field_cond_3_poweroftwo_0_31e00000() {
    // Encoding: 0x31E00000
    // Test aarch32_MVN_r_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, imm5=0, type1=0, Rm=0, S=0, Rd=0
    let encoding: u32 = 0x31E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_mvn_r_a1_a_field_cond_4_poweroftwo_0_41e00000() {
    // Encoding: 0x41E00000
    // Test aarch32_MVN_r_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=4, S=0, imm5=0, type1=0
    let encoding: u32 = 0x41E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_mvn_r_a1_a_field_cond_5_poweroftwo_0_51e00000() {
    // Encoding: 0x51E00000
    // Test aarch32_MVN_r_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=0, type1=0, Rm=0, cond=5, Rd=0, S=0
    let encoding: u32 = 0x51E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_mvn_r_a1_a_field_cond_6_poweroftwo_0_61e00000() {
    // Encoding: 0x61E00000
    // Test aarch32_MVN_r_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, Rm=0, cond=6, type1=0, imm5=0, S=0
    let encoding: u32 = 0x61E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_mvn_r_a1_a_field_cond_7_poweroftwo_0_71e00000() {
    // Encoding: 0x71E00000
    // Test aarch32_MVN_r_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, cond=7, Rm=0, S=0, imm5=0, Rd=0
    let encoding: u32 = 0x71E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_mvn_r_a1_a_field_cond_8_poweroftwo_0_81e00000() {
    // Encoding: 0x81E00000
    // Test aarch32_MVN_r_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=0, Rd=0, cond=8, S=0, type1=0, Rm=0
    let encoding: u32 = 0x81E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_mvn_r_a1_a_field_cond_9_poweroftwo_0_91e00000() {
    // Encoding: 0x91E00000
    // Test aarch32_MVN_r_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, cond=9, S=0, imm5=0, type1=0, Rm=0
    let encoding: u32 = 0x91E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_mvn_r_a1_a_field_cond_10_poweroftwo_0_a1e00000() {
    // Encoding: 0xA1E00000
    // Test aarch32_MVN_r_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, S=0, imm5=0, Rd=0, cond=10, type1=0
    let encoding: u32 = 0xA1E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_mvn_r_a1_a_field_cond_11_poweroftwo_0_b1e00000() {
    // Encoding: 0xB1E00000
    // Test aarch32_MVN_r_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, type1=0, Rd=0, Rm=0, cond=11, imm5=0
    let encoding: u32 = 0xB1E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_mvn_r_a1_a_field_cond_12_poweroftwo_0_c1e00000() {
    // Encoding: 0xC1E00000
    // Test aarch32_MVN_r_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, type1=0, imm5=0, cond=12, S=0, Rd=0
    let encoding: u32 = 0xC1E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_mvn_r_a1_a_field_cond_13_poweroftwo_0_d1e00000() {
    // Encoding: 0xD1E00000
    // Test aarch32_MVN_r_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=0, S=0, type1=0, Rd=0, Rm=0, cond=13
    let encoding: u32 = 0xD1E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_mvn_r_a1_a_field_cond_14_poweroftwo_0_e1e00000() {
    // Encoding: 0xE1E00000
    // Test aarch32_MVN_r_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, S=0, imm5=0, Rd=0, type1=0, Rm=0
    let encoding: u32 = 0xE1E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_mvn_r_a1_a_field_cond_15_max_0_f1e00000() {
    // Encoding: 0xF1E00000
    // Test aarch32_MVN_r_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, type1=0, imm5=0, Rm=0, S=0, Rd=0
    let encoding: u32 = 0xF1E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_mvn_r_a1_a_field_s_0_min_0_01e00000() {
    // Encoding: 0x01E00000
    // Test aarch32_MVN_r_A1_A field S = 0 (Min)
    // ISET: A32
    // Fields: cond=0, S=0, Rd=0, imm5=0, type1=0, Rm=0
    let encoding: u32 = 0x01E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_mvn_r_a1_a_field_s_1_max_0_01f00000() {
    // Encoding: 0x01F00000
    // Test aarch32_MVN_r_A1_A field S = 1 (Max)
    // ISET: A32
    // Fields: Rm=0, imm5=0, cond=0, type1=0, S=1, Rd=0
    let encoding: u32 = 0x01F00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mvn_r_a1_a_field_rd_0_min_0_01e00000() {
    // Encoding: 0x01E00000
    // Test aarch32_MVN_r_A1_A field Rd = 0 (Min)
    // ISET: A32
    // Fields: S=0, cond=0, type1=0, Rd=0, imm5=0, Rm=0
    let encoding: u32 = 0x01E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field Rd 12 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mvn_r_a1_a_field_rd_1_poweroftwo_0_01e01000() {
    // Encoding: 0x01E01000
    // Test aarch32_MVN_r_A1_A field Rd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: S=0, type1=0, cond=0, Rm=0, Rd=1, imm5=0
    let encoding: u32 = 0x01E01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_mvn_r_a1_a_field_imm5_0_zero_0_01e00000() {
    // Encoding: 0x01E00000
    // Test aarch32_MVN_r_A1_A field imm5 = 0 (Zero)
    // ISET: A32
    // Fields: imm5=0, S=0, type1=0, Rd=0, Rm=0, cond=0
    let encoding: u32 = 0x01E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_mvn_r_a1_a_field_imm5_1_poweroftwo_0_01e00080() {
    // Encoding: 0x01E00080
    // Test aarch32_MVN_r_A1_A field imm5 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, S=0, imm5=1, Rm=0, type1=0, cond=0
    let encoding: u32 = 0x01E00080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_mvn_r_a1_a_field_imm5_3_poweroftwominusone_0_01e00180() {
    // Encoding: 0x01E00180
    // Test aarch32_MVN_r_A1_A field imm5 = 3 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: type1=0, cond=0, Rd=0, S=0, imm5=3, Rm=0
    let encoding: u32 = 0x01E00180;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_mvn_r_a1_a_field_imm5_4_poweroftwo_0_01e00200() {
    // Encoding: 0x01E00200
    // Test aarch32_MVN_r_A1_A field imm5 = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: type1=0, imm5=4, cond=0, S=0, Rd=0, Rm=0
    let encoding: u32 = 0x01E00200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_mvn_r_a1_a_field_imm5_7_poweroftwominusone_0_01e00380() {
    // Encoding: 0x01E00380
    // Test aarch32_MVN_r_A1_A field imm5 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rm=0, cond=0, type1=0, S=0, imm5=7, Rd=0
    let encoding: u32 = 0x01E00380;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_mvn_r_a1_a_field_imm5_8_poweroftwo_0_01e00400() {
    // Encoding: 0x01E00400
    // Test aarch32_MVN_r_A1_A field imm5 = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: imm5=8, Rd=0, type1=0, cond=0, Rm=0, S=0
    let encoding: u32 = 0x01E00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 15, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (15)
#[test]
fn test_aarch32_mvn_r_a1_a_field_imm5_15_poweroftwominusone_0_01e00780() {
    // Encoding: 0x01E00780
    // Test aarch32_MVN_r_A1_A field imm5 = 15 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, S=0, imm5=15, type1=0, Rd=0, Rm=0
    let encoding: u32 = 0x01E00780;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_mvn_r_a1_a_field_imm5_16_poweroftwo_0_01e00800() {
    // Encoding: 0x01E00800
    // Test aarch32_MVN_r_A1_A field imm5 = 16 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, imm5=16, type1=0, cond=0, S=0, Rm=0
    let encoding: u32 = 0x01E00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field imm5 7 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 31, boundary: Max }
/// maximum immediate (31)
#[test]
fn test_aarch32_mvn_r_a1_a_field_imm5_31_max_0_01e00f80() {
    // Encoding: 0x01E00F80
    // Test aarch32_MVN_r_A1_A field imm5 = 31 (Max)
    // ISET: A32
    // Fields: cond=0, Rd=0, S=0, imm5=31, type1=0, Rm=0
    let encoding: u32 = 0x01E00F80;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mvn_r_a1_a_field_type1_0_min_0_01e00000() {
    // Encoding: 0x01E00000
    // Test aarch32_MVN_r_A1_A field type1 = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rd=0, Rm=0, imm5=0, type1=0, S=0
    let encoding: u32 = 0x01E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mvn_r_a1_a_field_type1_1_poweroftwo_0_01e00020() {
    // Encoding: 0x01E00020
    // Test aarch32_MVN_r_A1_A field type1 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rd=0, type1=1, Rm=0, cond=0, imm5=0, S=0
    let encoding: u32 = 0x01E00020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field type1 5 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_mvn_r_a1_a_field_type1_3_max_0_01e00060() {
    // Encoding: 0x01E00060
    // Test aarch32_MVN_r_A1_A field type1 = 3 (Max)
    // ISET: A32
    // Fields: type1=3, cond=0, Rd=0, imm5=0, Rm=0, S=0
    let encoding: u32 = 0x01E00060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mvn_r_a1_a_field_rm_0_min_0_01e00000() {
    // Encoding: 0x01E00000
    // Test aarch32_MVN_r_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: S=0, imm5=0, cond=0, Rm=0, type1=0, Rd=0
    let encoding: u32 = 0x01E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mvn_r_a1_a_field_rm_1_poweroftwo_0_01e00001() {
    // Encoding: 0x01E00001
    // Test aarch32_MVN_r_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, type1=0, Rd=0, S=0, imm5=0, Rm=1
    let encoding: u32 = 0x01E00001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_mvn_r_a1_a_combo_0_0_01e00000() {
    // Encoding: 0x01E00000
    // Test aarch32_MVN_r_A1_A field combination: cond=0, S=0, Rd=0, imm5=0, type1=0, Rm=0
    // ISET: A32
    // Fields: Rm=0, S=0, Rd=0, cond=0, imm5=0, type1=0
    let encoding: u32 = 0x01E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_mvn_r_a1_a_special_cond_0_condition_eq_0_01e00000() {
    // Encoding: 0x01E00000
    // Test aarch32_MVN_r_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: imm5=0, cond=0, S=0, type1=0, Rm=0, Rd=0
    let encoding: u32 = 0x01E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_mvn_r_a1_a_special_cond_1_condition_ne_0_11e00000() {
    // Encoding: 0x11E00000
    // Test aarch32_MVN_r_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, Rd=0, type1=0, Rm=0, imm5=0, S=0
    let encoding: u32 = 0x11E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_mvn_r_a1_a_special_cond_2_condition_cs_hs_0_21e00000() {
    // Encoding: 0x21E00000
    // Test aarch32_MVN_r_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: S=0, imm5=0, Rm=0, type1=0, Rd=0, cond=2
    let encoding: u32 = 0x21E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_mvn_r_a1_a_special_cond_3_condition_cc_lo_0_31e00000() {
    // Encoding: 0x31E00000
    // Test aarch32_MVN_r_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: imm5=0, S=0, type1=0, Rd=0, Rm=0, cond=3
    let encoding: u32 = 0x31E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_mvn_r_a1_a_special_cond_4_condition_mi_0_41e00000() {
    // Encoding: 0x41E00000
    // Test aarch32_MVN_r_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, type1=0, S=0, Rd=0, imm5=0, Rm=0
    let encoding: u32 = 0x41E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_mvn_r_a1_a_special_cond_5_condition_pl_0_51e00000() {
    // Encoding: 0x51E00000
    // Test aarch32_MVN_r_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: S=0, imm5=0, cond=5, type1=0, Rm=0, Rd=0
    let encoding: u32 = 0x51E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_mvn_r_a1_a_special_cond_6_condition_vs_0_61e00000() {
    // Encoding: 0x61E00000
    // Test aarch32_MVN_r_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rd=0, S=0, type1=0, Rm=0, cond=6, imm5=0
    let encoding: u32 = 0x61E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_mvn_r_a1_a_special_cond_7_condition_vc_0_71e00000() {
    // Encoding: 0x71E00000
    // Test aarch32_MVN_r_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: type1=0, S=0, imm5=0, Rd=0, Rm=0, cond=7
    let encoding: u32 = 0x71E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_mvn_r_a1_a_special_cond_8_condition_hi_0_81e00000() {
    // Encoding: 0x81E00000
    // Test aarch32_MVN_r_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: S=0, type1=0, Rm=0, cond=8, Rd=0, imm5=0
    let encoding: u32 = 0x81E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_mvn_r_a1_a_special_cond_9_condition_ls_0_91e00000() {
    // Encoding: 0x91E00000
    // Test aarch32_MVN_r_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: imm5=0, Rd=0, type1=0, cond=9, S=0, Rm=0
    let encoding: u32 = 0x91E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_mvn_r_a1_a_special_cond_10_condition_ge_0_a1e00000() {
    // Encoding: 0xA1E00000
    // Test aarch32_MVN_r_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rd=0, S=0, cond=10, imm5=0, type1=0, Rm=0
    let encoding: u32 = 0xA1E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_mvn_r_a1_a_special_cond_11_condition_lt_0_b1e00000() {
    // Encoding: 0xB1E00000
    // Test aarch32_MVN_r_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: imm5=0, S=0, Rm=0, cond=11, type1=0, Rd=0
    let encoding: u32 = 0xB1E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_mvn_r_a1_a_special_cond_12_condition_gt_0_c1e00000() {
    // Encoding: 0xC1E00000
    // Test aarch32_MVN_r_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rd=0, cond=12, type1=0, Rm=0, imm5=0, S=0
    let encoding: u32 = 0xC1E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_mvn_r_a1_a_special_cond_13_condition_le_0_d1e00000() {
    // Encoding: 0xD1E00000
    // Test aarch32_MVN_r_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: S=0, Rd=0, imm5=0, Rm=0, cond=13, type1=0
    let encoding: u32 = 0xD1E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_mvn_r_a1_a_special_cond_14_condition_al_0_e1e00000() {
    // Encoding: 0xE1E00000
    // Test aarch32_MVN_r_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rm=0, cond=14, type1=0, S=0, Rd=0, imm5=0
    let encoding: u32 = 0xE1E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_mvn_r_a1_a_special_cond_15_condition_nv_0_f1e00000() {
    // Encoding: 0xF1E00000
    // Test aarch32_MVN_r_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rm=0, Rd=0, type1=0, S=0, cond=15, imm5=0
    let encoding: u32 = 0xF1E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_mvn_r_a1_a_special_s_0_size_variant_0_0_01e00000() {
    // Encoding: 0x01E00000
    // Test aarch32_MVN_r_A1_A special value S = 0 (Size variant 0)
    // ISET: A32
    // Fields: cond=0, Rd=0, S=0, imm5=0, type1=0, Rm=0
    let encoding: u32 = 0x01E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_mvn_r_a1_a_special_s_1_size_variant_1_0_01f00000() {
    // Encoding: 0x01F00000
    // Test aarch32_MVN_r_A1_A special value S = 1 (Size variant 1)
    // ISET: A32
    // Fields: imm5=0, cond=0, S=1, type1=0, Rm=0, Rd=0
    let encoding: u32 = 0x01F00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `field Rm 19 +: 3`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mvn_r_t1_a_field_rm_0_min_0_43c00000() {
    // Thumb encoding (32): 0x43C00000
    // Test aarch32_MVN_r_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x43C00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `field Rm 19 +: 3`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mvn_r_t1_a_field_rm_1_poweroftwo_0_43c80000() {
    // Thumb encoding (32): 0x43C80000
    // Test aarch32_MVN_r_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, Rm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x43C80000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `field Rd 16 +: 3`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mvn_r_t1_a_field_rd_0_min_0_43c00000() {
    // Thumb encoding (32): 0x43C00000
    // Test aarch32_MVN_r_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x43C00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `field Rd 16 +: 3`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mvn_r_t1_a_field_rd_1_poweroftwo_0_43c10000() {
    // Thumb encoding (32): 0x43C10000
    // Test aarch32_MVN_r_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=1, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x43C10000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch32_mvn_r_t1_a_combo_0_0_43c00000() {
    // Thumb encoding (32): 0x43C00000
    // Test aarch32_MVN_r_T1_A field combination: Rm=0, Rd=0
    // ISET: T32
    // Fields: Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x43C00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_mvn_r_t2_a_field_s_0_min_0_ea6f0000() {
    // Thumb encoding (32): 0xEA6F0000
    // Test aarch32_MVN_r_T2_A field S = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, S=0, imm3=0, type1=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA6F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_mvn_r_t2_a_field_s_1_max_0_ea7f0000() {
    // Thumb encoding (32): 0xEA7F0000
    // Test aarch32_MVN_r_T2_A field S = 1 (Max)
    // ISET: T32
    // Fields: Rd=0, type1=0, S=1, imm3=0, imm2=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA7F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_mvn_r_t2_a_field_imm3_0_zero_0_ea6f0000() {
    // Thumb encoding (32): 0xEA6F0000
    // Test aarch32_MVN_r_T2_A field imm3 = 0 (Zero)
    // ISET: T32
    // Fields: imm2=0, type1=0, imm3=0, Rd=0, S=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA6F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_mvn_r_t2_a_field_imm3_1_poweroftwo_0_ea6f1000() {
    // Thumb encoding (32): 0xEA6F1000
    // Test aarch32_MVN_r_T2_A field imm3 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, S=0, imm3=1, type1=0, Rm=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA6F1000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch32_mvn_r_t2_a_field_imm3_3_poweroftwominusone_0_ea6f3000() {
    // Thumb encoding (32): 0xEA6F3000
    // Test aarch32_MVN_r_T2_A field imm3 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm3=3, Rd=0, imm2=0, S=0, type1=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA6F3000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch32_mvn_r_t2_a_field_imm3_7_max_0_ea6f7000() {
    // Thumb encoding (32): 0xEA6F7000
    // Test aarch32_MVN_r_T2_A field imm3 = 7 (Max)
    // ISET: T32
    // Fields: Rm=0, imm2=0, imm3=7, Rd=0, type1=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA6F7000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mvn_r_t2_a_field_rd_0_min_0_ea6f0000() {
    // Thumb encoding (32): 0xEA6F0000
    // Test aarch32_MVN_r_T2_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: type1=0, S=0, Rd=0, imm3=0, Rm=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA6F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mvn_r_t2_a_field_rd_1_poweroftwo_0_ea6f0100() {
    // Thumb encoding (32): 0xEA6F0100
    // Test aarch32_MVN_r_T2_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm2=0, imm3=0, type1=0, Rd=1, Rm=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA6F0100;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `field imm2 6 +: 2`
/// Requirement: FieldBoundary { field: "imm2", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_mvn_r_t2_a_field_imm2_0_zero_0_ea6f0000() {
    // Thumb encoding (32): 0xEA6F0000
    // Test aarch32_MVN_r_T2_A field imm2 = 0 (Zero)
    // ISET: T32
    // Fields: Rm=0, S=0, Rd=0, imm2=0, type1=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA6F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `field imm2 6 +: 2`
/// Requirement: FieldBoundary { field: "imm2", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_mvn_r_t2_a_field_imm2_1_poweroftwo_0_ea6f0040() {
    // Thumb encoding (32): 0xEA6F0040
    // Test aarch32_MVN_r_T2_A field imm2 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, Rd=0, Rm=0, imm3=0, imm2=1, type1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA6F0040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `field imm2 6 +: 2`
/// Requirement: FieldBoundary { field: "imm2", value: 3, boundary: Max }
/// maximum immediate (3)
#[test]
fn test_aarch32_mvn_r_t2_a_field_imm2_3_max_0_ea6f00c0() {
    // Thumb encoding (32): 0xEA6F00C0
    // Test aarch32_MVN_r_T2_A field imm2 = 3 (Max)
    // ISET: T32
    // Fields: S=0, Rm=0, type1=0, Rd=0, imm3=0, imm2=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA6F00C0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `field type1 4 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mvn_r_t2_a_field_type1_0_min_0_ea6f0000() {
    // Thumb encoding (32): 0xEA6F0000
    // Test aarch32_MVN_r_T2_A field type1 = 0 (Min)
    // ISET: T32
    // Fields: imm3=0, S=0, Rm=0, imm2=0, type1=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA6F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `field type1 4 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mvn_r_t2_a_field_type1_1_poweroftwo_0_ea6f0010() {
    // Thumb encoding (32): 0xEA6F0010
    // Test aarch32_MVN_r_T2_A field type1 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm2=0, Rm=0, Rd=0, imm3=0, type1=1, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA6F0010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `field type1 4 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_mvn_r_t2_a_field_type1_3_max_0_ea6f0030() {
    // Thumb encoding (32): 0xEA6F0030
    // Test aarch32_MVN_r_T2_A field type1 = 3 (Max)
    // ISET: T32
    // Fields: type1=3, Rm=0, imm2=0, S=0, imm3=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA6F0030;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mvn_r_t2_a_field_rm_0_min_0_ea6f0000() {
    // Thumb encoding (32): 0xEA6F0000
    // Test aarch32_MVN_r_T2_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: imm2=0, S=0, Rm=0, Rd=0, imm3=0, type1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA6F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mvn_r_t2_a_field_rm_1_poweroftwo_0_ea6f0001() {
    // Thumb encoding (32): 0xEA6F0001
    // Test aarch32_MVN_r_T2_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1, imm2=0, Rd=0, imm3=0, type1=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA6F0001;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=0 (8-bit / byte size)
#[test]
fn test_aarch32_mvn_r_t2_a_combo_0_0_ea6f0000() {
    // Thumb encoding (32): 0xEA6F0000
    // Test aarch32_MVN_r_T2_A field combination: S=0, imm3=0, Rd=0, imm2=0, type1=0, Rm=0
    // ISET: T32
    // Fields: imm3=0, Rm=0, imm2=0, type1=0, S=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA6F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_mvn_r_t2_a_special_s_0_size_variant_0_0_ea6f0000() {
    // Thumb encoding (32): 0xEA6F0000
    // Test aarch32_MVN_r_T2_A special value S = 0 (Size variant 0)
    // ISET: T32
    // Fields: S=0, imm3=0, Rm=0, imm2=0, type1=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA6F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_mvn_r_t2_a_special_s_1_size_variant_1_0_ea7f0000() {
    // Thumb encoding (32): 0xEA7F0000
    // Test aarch32_MVN_r_T2_A special value S = 1 (Size variant 1)
    // ISET: T32
    // Fields: imm3=0, type1=0, imm2=0, S=1, Rd=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA7F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mvn_r_t2_a_invalid_0_0_ea6f0000() {
    // Thumb encoding (32): 0xEA6F0000
    // Test aarch32_MVN_r_T2_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: imm3=0, imm2=0, Rd=0, type1=0, S=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA6F0000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mvn_r_t2_a_invalid_1_0_ea6f0000() {
    // Thumb encoding (32): 0xEA6F0000
    // Test aarch32_MVN_r_T2_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: imm2=0, Rd=0, type1=0, Rm=0, S=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA6F0000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_mvn_r_a1_a_flags_zeroresult_0_01f00002() {
    // Test aarch32_MVN_r_A1_A flag computation: ZeroResult
    // Encoding: 0x01F00002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x01F00002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_mvn_r_a1_a_flags_zeroresult_1_01f00002() {
    // Test aarch32_MVN_r_A1_A flag computation: ZeroResult
    // Encoding: 0x01F00002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x01F00002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_mvn_r_a1_a_flags_negativeresult_2_01f00002() {
    // Test aarch32_MVN_r_A1_A flag computation: NegativeResult
    // Encoding: 0x01F00002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x01F00002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_mvn_r_a1_a_flags_unsignedoverflow_3_01f00002() {
    // Test aarch32_MVN_r_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x01F00002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x01F00002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_mvn_r_a1_a_flags_unsignedoverflow_4_01f00002() {
    // Test aarch32_MVN_r_A1_A flag computation: UnsignedOverflow
    // Encoding: 0x01F00002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x01F00002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_mvn_r_a1_a_flags_signedoverflow_5_01f00002() {
    // Test aarch32_MVN_r_A1_A flag computation: SignedOverflow
    // Encoding: 0x01F00002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x01F00002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_mvn_r_a1_a_flags_signedoverflow_6_01f00002() {
    // Test aarch32_MVN_r_A1_A flag computation: SignedOverflow
    // Encoding: 0x01F00002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x01F00002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MVN_r_A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_mvn_r_a1_a_flags_positiveresult_7_01f00002() {
    // Test aarch32_MVN_r_A1_A flag computation: PositiveResult
    // Encoding: 0x01F00002
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u32 = 0x01F00002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 0 (32)
#[test]
fn test_aarch32_mvn_r_t1_a_lslv_oracle_32_0_43c20020() {
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

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 0 (64)
#[test]
fn test_aarch32_mvn_r_t1_a_lslv_oracle_64_0_c3c20020() {
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

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4 (32)
#[test]
fn test_aarch32_mvn_r_t1_a_lslv_oracle_32_1_43c20020() {
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

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 4 (64)
#[test]
fn test_aarch32_mvn_r_t1_a_lslv_oracle_64_1_c3c20020() {
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

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 8 (32)
#[test]
fn test_aarch32_mvn_r_t1_a_lslv_oracle_32_2_43c20020() {
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

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 8 (64)
#[test]
fn test_aarch32_mvn_r_t1_a_lslv_oracle_64_2_c3c20020() {
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

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1 (32)
#[test]
fn test_aarch32_mvn_r_t1_a_lslv_oracle_32_3_43c20020() {
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

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// MSB set, shift 1 (64)
#[test]
fn test_aarch32_mvn_r_t1_a_lslv_oracle_64_3_c3c20020() {
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

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSB set, max shift (32)
#[test]
fn test_aarch32_mvn_r_t1_a_lslv_oracle_32_4_43c20020() {
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

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSB set, max shift (64)
#[test]
fn test_aarch32_mvn_r_t1_a_lslv_oracle_64_4_c3c20020() {
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

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// all ones, shift 32 (32)
#[test]
fn test_aarch32_mvn_r_t1_a_lslv_oracle_32_5_43c20020() {
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

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all ones, shift 32 (64)
#[test]
fn test_aarch32_mvn_r_t1_a_lslv_oracle_64_5_c3c20020() {
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

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift
#[test]
fn test_aarch32_mvn_r_t1_a_t16_oracle_0_43d00000() {
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

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4
#[test]
fn test_aarch32_mvn_r_t1_a_t16_oracle_1_43d00000() {
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

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1
#[test]
fn test_aarch32_mvn_r_t1_a_t16_oracle_2_43d00000() {
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

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift to MSB
#[test]
fn test_aarch32_mvn_r_t1_a_t16_oracle_3_43d00000() {
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

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_mvn_r_t1_a_flags_zeroresult_0_43d00000() {
    // Test aarch32_MVN_r_T1_A flag computation: ZeroResult
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

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_mvn_r_t1_a_flags_zeroresult_1_43d00000() {
    // Test aarch32_MVN_r_T1_A flag computation: ZeroResult
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
}

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_mvn_r_t1_a_flags_negativeresult_2_43d00000() {
    // Test aarch32_MVN_r_T1_A flag computation: NegativeResult
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

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_mvn_r_t1_a_flags_unsignedoverflow_3_43d00000() {
    // Test aarch32_MVN_r_T1_A flag computation: UnsignedOverflow
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

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_mvn_r_t1_a_flags_unsignedoverflow_4_43d00000() {
    // Test aarch32_MVN_r_T1_A flag computation: UnsignedOverflow
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
}

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_mvn_r_t1_a_flags_signedoverflow_5_43d00000() {
    // Test aarch32_MVN_r_T1_A flag computation: SignedOverflow
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

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_mvn_r_t1_a_flags_signedoverflow_6_43d00000() {
    // Test aarch32_MVN_r_T1_A flag computation: SignedOverflow
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

/// Provenance: aarch32_MVN_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_mvn_r_t1_a_flags_positiveresult_7_43d00000() {
    // Test aarch32_MVN_r_T1_A flag computation: PositiveResult
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

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_mvn_r_t2_a_flags_zeroresult_0_ea7f0002() {
    // Test aarch32_MVN_r_T2_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xEA7F0002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_mvn_r_t2_a_flags_zeroresult_1_ea7f0002() {
    // Test aarch32_MVN_r_T2_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0xEA7F0002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_mvn_r_t2_a_flags_negativeresult_2_ea7f0002() {
    // Test aarch32_MVN_r_T2_A flag computation: NegativeResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xEA7F0002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_mvn_r_t2_a_flags_unsignedoverflow_3_ea7f0002() {
    // Test aarch32_MVN_r_T2_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xEA7F0002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_mvn_r_t2_a_flags_unsignedoverflow_4_ea7f0002() {
    // Test aarch32_MVN_r_T2_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u32 = 0xEA7F0002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_mvn_r_t2_a_flags_signedoverflow_5_ea7f0002() {
    // Test aarch32_MVN_r_T2_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xEA7F0002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_mvn_r_t2_a_flags_signedoverflow_6_ea7f0002() {
    // Test aarch32_MVN_r_T2_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xEA7F0002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_MVN_r_T2_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_mvn_r_t2_a_flags_positiveresult_7_ea7f0002() {
    // Test aarch32_MVN_r_T2_A flag computation: PositiveResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u32 = 0xEA7F0002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}
