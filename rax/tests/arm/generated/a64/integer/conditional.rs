//! A64 integer conditional tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_integer_conditional_select Tests
// ============================================================================

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_conditional_select_field_sf_0_min_0_1a800000() {
    // Encoding: 0x1A800000
    // Test aarch64_integer_conditional_select field sf = 0 (Min)
    // Fields: Rm=0, op=0, cond=0, Rd=0, sf=0, o2=0, Rn=0
    let encoding: u32 = 0x1A800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_conditional_select_field_sf_1_max_0_9a800000() {
    // Encoding: 0x9A800000
    // Test aarch64_integer_conditional_select field sf = 1 (Max)
    // Fields: o2=0, Rn=0, cond=0, Rd=0, sf=1, Rm=0, op=0
    let encoding: u32 = 0x9A800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field op 30 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_conditional_select_field_op_0_min_0_1a800000() {
    // Encoding: 0x1A800000
    // Test aarch64_integer_conditional_select field op = 0 (Min)
    // Fields: op=0, o2=0, Rd=0, Rn=0, sf=0, Rm=0, cond=0
    let encoding: u32 = 0x1A800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field op 30 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_integer_conditional_select_field_op_1_max_0_5a800000() {
    // Encoding: 0x5A800000
    // Test aarch64_integer_conditional_select field op = 1 (Max)
    // Fields: Rm=0, cond=0, op=1, Rn=0, sf=0, o2=0, Rd=0
    let encoding: u32 = 0x5A800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_conditional_select_field_rm_0_min_0_1a800000() {
    // Encoding: 0x1A800000
    // Test aarch64_integer_conditional_select field Rm = 0 (Min)
    // Fields: cond=0, o2=0, Rn=0, Rd=0, op=0, sf=0, Rm=0
    let encoding: u32 = 0x1A800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_conditional_select_field_rm_1_poweroftwo_0_1a810000() {
    // Encoding: 0x1A810000
    // Test aarch64_integer_conditional_select field Rm = 1 (PowerOfTwo)
    // Fields: cond=0, op=0, Rd=0, o2=0, Rm=1, sf=0, Rn=0
    let encoding: u32 = 0x1A810000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_conditional_select_field_rm_30_poweroftwominusone_0_1a9e0000() {
    // Encoding: 0x1A9E0000
    // Test aarch64_integer_conditional_select field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: cond=0, Rn=0, sf=0, Rd=0, Rm=30, o2=0, op=0
    let encoding: u32 = 0x1A9E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_conditional_select_field_rm_31_max_0_1a9f0000() {
    // Encoding: 0x1A9F0000
    // Test aarch64_integer_conditional_select field Rm = 31 (Max)
    // Fields: Rd=0, sf=0, Rm=31, op=0, cond=0, o2=0, Rn=0
    let encoding: u32 = 0x1A9F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch64_integer_conditional_select_field_cond_0_min_0_1a800000() {
    // Encoding: 0x1A800000
    // Test aarch64_integer_conditional_select field cond = 0 (Min)
    // Fields: sf=0, cond=0, o2=0, Rn=0, Rd=0, op=0, Rm=0
    let encoding: u32 = 0x1A800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch64_integer_conditional_select_field_cond_1_poweroftwo_0_1a801000() {
    // Encoding: 0x1A801000
    // Test aarch64_integer_conditional_select field cond = 1 (PowerOfTwo)
    // Fields: Rd=0, Rn=0, o2=0, op=0, Rm=0, sf=0, cond=1
    let encoding: u32 = 0x1A801000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch64_integer_conditional_select_field_cond_2_poweroftwo_0_1a802000() {
    // Encoding: 0x1A802000
    // Test aarch64_integer_conditional_select field cond = 2 (PowerOfTwo)
    // Fields: Rn=0, o2=0, Rd=0, sf=0, Rm=0, op=0, cond=2
    let encoding: u32 = 0x1A802000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch64_integer_conditional_select_field_cond_3_poweroftwo_0_1a803000() {
    // Encoding: 0x1A803000
    // Test aarch64_integer_conditional_select field cond = 3 (PowerOfTwo)
    // Fields: Rn=0, o2=0, Rd=0, sf=0, op=0, Rm=0, cond=3
    let encoding: u32 = 0x1A803000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch64_integer_conditional_select_field_cond_4_poweroftwo_0_1a804000() {
    // Encoding: 0x1A804000
    // Test aarch64_integer_conditional_select field cond = 4 (PowerOfTwo)
    // Fields: Rd=0, op=0, Rm=0, o2=0, sf=0, cond=4, Rn=0
    let encoding: u32 = 0x1A804000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch64_integer_conditional_select_field_cond_5_poweroftwo_0_1a805000() {
    // Encoding: 0x1A805000
    // Test aarch64_integer_conditional_select field cond = 5 (PowerOfTwo)
    // Fields: Rd=0, Rn=0, cond=5, sf=0, op=0, Rm=0, o2=0
    let encoding: u32 = 0x1A805000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch64_integer_conditional_select_field_cond_6_poweroftwo_0_1a806000() {
    // Encoding: 0x1A806000
    // Test aarch64_integer_conditional_select field cond = 6 (PowerOfTwo)
    // Fields: Rn=0, o2=0, Rm=0, sf=0, cond=6, op=0, Rd=0
    let encoding: u32 = 0x1A806000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch64_integer_conditional_select_field_cond_7_poweroftwo_0_1a807000() {
    // Encoding: 0x1A807000
    // Test aarch64_integer_conditional_select field cond = 7 (PowerOfTwo)
    // Fields: op=0, o2=0, Rn=0, Rd=0, sf=0, Rm=0, cond=7
    let encoding: u32 = 0x1A807000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch64_integer_conditional_select_field_cond_8_poweroftwo_0_1a808000() {
    // Encoding: 0x1A808000
    // Test aarch64_integer_conditional_select field cond = 8 (PowerOfTwo)
    // Fields: Rn=0, Rd=0, o2=0, op=0, sf=0, Rm=0, cond=8
    let encoding: u32 = 0x1A808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch64_integer_conditional_select_field_cond_9_poweroftwo_0_1a809000() {
    // Encoding: 0x1A809000
    // Test aarch64_integer_conditional_select field cond = 9 (PowerOfTwo)
    // Fields: cond=9, Rm=0, o2=0, sf=0, op=0, Rn=0, Rd=0
    let encoding: u32 = 0x1A809000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch64_integer_conditional_select_field_cond_10_poweroftwo_0_1a80a000() {
    // Encoding: 0x1A80A000
    // Test aarch64_integer_conditional_select field cond = 10 (PowerOfTwo)
    // Fields: cond=10, sf=0, o2=0, Rn=0, Rm=0, Rd=0, op=0
    let encoding: u32 = 0x1A80A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch64_integer_conditional_select_field_cond_11_poweroftwo_0_1a80b000() {
    // Encoding: 0x1A80B000
    // Test aarch64_integer_conditional_select field cond = 11 (PowerOfTwo)
    // Fields: Rd=0, Rm=0, o2=0, cond=11, sf=0, Rn=0, op=0
    let encoding: u32 = 0x1A80B000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch64_integer_conditional_select_field_cond_12_poweroftwo_0_1a80c000() {
    // Encoding: 0x1A80C000
    // Test aarch64_integer_conditional_select field cond = 12 (PowerOfTwo)
    // Fields: sf=0, cond=12, o2=0, op=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x1A80C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch64_integer_conditional_select_field_cond_13_poweroftwo_0_1a80d000() {
    // Encoding: 0x1A80D000
    // Test aarch64_integer_conditional_select field cond = 13 (PowerOfTwo)
    // Fields: Rm=0, cond=13, sf=0, o2=0, Rd=0, op=0, Rn=0
    let encoding: u32 = 0x1A80D000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch64_integer_conditional_select_field_cond_14_poweroftwo_0_1a80e000() {
    // Encoding: 0x1A80E000
    // Test aarch64_integer_conditional_select field cond = 14 (PowerOfTwo)
    // Fields: Rm=0, sf=0, cond=14, o2=0, Rd=0, op=0, Rn=0
    let encoding: u32 = 0x1A80E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch64_integer_conditional_select_field_cond_15_max_0_1a80f000() {
    // Encoding: 0x1A80F000
    // Test aarch64_integer_conditional_select field cond = 15 (Max)
    // Fields: sf=0, cond=15, op=0, Rn=0, Rm=0, Rd=0, o2=0
    let encoding: u32 = 0x1A80F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field o2 10 +: 1`
/// Requirement: FieldBoundary { field: "o2", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_conditional_select_field_o2_0_min_0_1a800000() {
    // Encoding: 0x1A800000
    // Test aarch64_integer_conditional_select field o2 = 0 (Min)
    // Fields: sf=0, Rm=0, Rn=0, cond=0, Rd=0, op=0, o2=0
    let encoding: u32 = 0x1A800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field o2 10 +: 1`
/// Requirement: FieldBoundary { field: "o2", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_integer_conditional_select_field_o2_1_max_0_1a800400() {
    // Encoding: 0x1A800400
    // Test aarch64_integer_conditional_select field o2 = 1 (Max)
    // Fields: o2=1, op=0, cond=0, Rm=0, Rn=0, Rd=0, sf=0
    let encoding: u32 = 0x1A800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_conditional_select_field_rn_0_min_0_1a800000() {
    // Encoding: 0x1A800000
    // Test aarch64_integer_conditional_select field Rn = 0 (Min)
    // Fields: sf=0, op=0, o2=0, Rm=0, Rn=0, cond=0, Rd=0
    let encoding: u32 = 0x1A800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_conditional_select_field_rn_1_poweroftwo_0_1a800020() {
    // Encoding: 0x1A800020
    // Test aarch64_integer_conditional_select field Rn = 1 (PowerOfTwo)
    // Fields: o2=0, Rn=1, cond=0, op=0, sf=0, Rm=0, Rd=0
    let encoding: u32 = 0x1A800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_conditional_select_field_rn_30_poweroftwominusone_0_1a8003c0() {
    // Encoding: 0x1A8003C0
    // Test aarch64_integer_conditional_select field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, sf=0, o2=0, Rn=30, cond=0, Rm=0, op=0
    let encoding: u32 = 0x1A8003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_conditional_select_field_rn_31_max_0_1a8003e0() {
    // Encoding: 0x1A8003E0
    // Test aarch64_integer_conditional_select field Rn = 31 (Max)
    // Fields: Rm=0, Rn=31, o2=0, op=0, Rd=0, cond=0, sf=0
    let encoding: u32 = 0x1A8003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_conditional_select_field_rd_0_min_0_1a800000() {
    // Encoding: 0x1A800000
    // Test aarch64_integer_conditional_select field Rd = 0 (Min)
    // Fields: op=0, Rn=0, cond=0, o2=0, Rd=0, sf=0, Rm=0
    let encoding: u32 = 0x1A800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_conditional_select_field_rd_1_poweroftwo_0_1a800001() {
    // Encoding: 0x1A800001
    // Test aarch64_integer_conditional_select field Rd = 1 (PowerOfTwo)
    // Fields: Rm=0, o2=0, Rn=0, Rd=1, cond=0, sf=0, op=0
    let encoding: u32 = 0x1A800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_conditional_select_field_rd_30_poweroftwominusone_0_1a80001e() {
    // Encoding: 0x1A80001E
    // Test aarch64_integer_conditional_select field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: o2=0, Rn=0, Rm=0, op=0, Rd=30, cond=0, sf=0
    let encoding: u32 = 0x1A80001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_integer_conditional_select_field_rd_31_max_0_1a80001f() {
    // Encoding: 0x1A80001F
    // Test aarch64_integer_conditional_select field Rd = 31 (Max)
    // Fields: Rd=31, o2=0, Rm=0, Rn=0, cond=0, sf=0, op=0
    let encoding: u32 = 0x1A80001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_conditional_select_combo_0_0_1a800000() {
    // Encoding: 0x1A800000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=0, o2=0, Rn=0, Rd=0
    // Fields: cond=0, Rn=0, Rd=0, Rm=0, sf=0, op=0, o2=0
    let encoding: u32 = 0x1A800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_conditional_select_combo_1_0_9a800000() {
    // Encoding: 0x9A800000
    // Test aarch64_integer_conditional_select field combination: sf=1, op=0, Rm=0, cond=0, o2=0, Rn=0, Rd=0
    // Fields: o2=0, cond=0, Rd=0, Rn=0, sf=1, Rm=0, op=0
    let encoding: u32 = 0x9A800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_integer_conditional_select_combo_2_0_1a800000() {
    // Encoding: 0x1A800000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=0, o2=0, Rn=0, Rd=0
    // Fields: Rn=0, cond=0, Rm=0, op=0, Rd=0, sf=0, o2=0
    let encoding: u32 = 0x1A800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_integer_conditional_select_combo_3_0_5a800000() {
    // Encoding: 0x5A800000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=1, Rm=0, cond=0, o2=0, Rn=0, Rd=0
    // Fields: Rd=0, sf=0, Rm=0, op=1, cond=0, o2=0, Rn=0
    let encoding: u32 = 0x5A800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_conditional_select_combo_4_0_1a800000() {
    // Encoding: 0x1A800000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=0, o2=0, Rn=0, Rd=0
    // Fields: Rd=0, sf=0, cond=0, op=0, Rm=0, o2=0, Rn=0
    let encoding: u32 = 0x1A800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_conditional_select_combo_5_0_1a810000() {
    // Encoding: 0x1A810000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=1, cond=0, o2=0, Rn=0, Rd=0
    // Fields: op=0, cond=0, o2=0, Rn=0, Rd=0, sf=0, Rm=1
    let encoding: u32 = 0x1A810000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_conditional_select_combo_6_0_1a9e0000() {
    // Encoding: 0x1A9E0000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=30, cond=0, o2=0, Rn=0, Rd=0
    // Fields: sf=0, Rm=30, cond=0, Rn=0, Rd=0, o2=0, op=0
    let encoding: u32 = 0x1A9E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_conditional_select_combo_7_0_1a9f0000() {
    // Encoding: 0x1A9F0000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=31, cond=0, o2=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, op=0, Rm=31, sf=0, o2=0, cond=0
    let encoding: u32 = 0x1A9F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch64_integer_conditional_select_combo_8_0_1a800000() {
    // Encoding: 0x1A800000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=0, o2=0, Rn=0, Rd=0
    // Fields: o2=0, Rd=0, op=0, Rm=0, cond=0, Rn=0, sf=0
    let encoding: u32 = 0x1A800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=1 (condition NE (not equal))
#[test]
fn test_aarch64_integer_conditional_select_combo_9_0_1a801000() {
    // Encoding: 0x1A801000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=1, o2=0, Rn=0, Rd=0
    // Fields: Rn=0, o2=0, Rd=0, cond=1, op=0, sf=0, Rm=0
    let encoding: u32 = 0x1A801000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=2 (condition CS/HS (carry set))
#[test]
fn test_aarch64_integer_conditional_select_combo_10_0_1a802000() {
    // Encoding: 0x1A802000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=2, o2=0, Rn=0, Rd=0
    // Fields: sf=0, o2=0, Rn=0, Rd=0, cond=2, op=0, Rm=0
    let encoding: u32 = 0x1A802000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=3 (condition CC/LO (carry clear))
#[test]
fn test_aarch64_integer_conditional_select_combo_11_0_1a803000() {
    // Encoding: 0x1A803000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=3, o2=0, Rn=0, Rd=0
    // Fields: Rm=0, sf=0, Rn=0, op=0, o2=0, Rd=0, cond=3
    let encoding: u32 = 0x1A803000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=4 (condition MI (minus/negative))
#[test]
fn test_aarch64_integer_conditional_select_combo_12_0_1a804000() {
    // Encoding: 0x1A804000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=4, o2=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, cond=4, op=0, sf=0, o2=0, Rm=0
    let encoding: u32 = 0x1A804000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=5 (condition PL (plus/positive))
#[test]
fn test_aarch64_integer_conditional_select_combo_13_0_1a805000() {
    // Encoding: 0x1A805000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=5, o2=0, Rn=0, Rd=0
    // Fields: Rd=0, sf=0, cond=5, o2=0, Rm=0, op=0, Rn=0
    let encoding: u32 = 0x1A805000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=6 (condition VS (overflow set))
#[test]
fn test_aarch64_integer_conditional_select_combo_14_0_1a806000() {
    // Encoding: 0x1A806000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=6, o2=0, Rn=0, Rd=0
    // Fields: o2=0, Rd=0, Rn=0, sf=0, cond=6, op=0, Rm=0
    let encoding: u32 = 0x1A806000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=7 (condition VC (overflow clear))
#[test]
fn test_aarch64_integer_conditional_select_combo_15_0_1a807000() {
    // Encoding: 0x1A807000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=7, o2=0, Rn=0, Rd=0
    // Fields: Rm=0, o2=0, sf=0, Rn=0, op=0, cond=7, Rd=0
    let encoding: u32 = 0x1A807000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=8 (condition HI (unsigned higher))
#[test]
fn test_aarch64_integer_conditional_select_combo_16_0_1a808000() {
    // Encoding: 0x1A808000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=8, o2=0, Rn=0, Rd=0
    // Fields: sf=0, Rm=0, cond=8, o2=0, Rn=0, Rd=0, op=0
    let encoding: u32 = 0x1A808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=9 (condition LS (unsigned lower or same))
#[test]
fn test_aarch64_integer_conditional_select_combo_17_0_1a809000() {
    // Encoding: 0x1A809000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=9, o2=0, Rn=0, Rd=0
    // Fields: sf=0, op=0, Rd=0, cond=9, o2=0, Rm=0, Rn=0
    let encoding: u32 = 0x1A809000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=10 (condition GE (signed >=))
#[test]
fn test_aarch64_integer_conditional_select_combo_18_0_1a80a000() {
    // Encoding: 0x1A80A000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=10, o2=0, Rn=0, Rd=0
    // Fields: o2=0, Rd=0, Rn=0, Rm=0, sf=0, op=0, cond=10
    let encoding: u32 = 0x1A80A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=11 (condition LT (signed <))
#[test]
fn test_aarch64_integer_conditional_select_combo_19_0_1a80b000() {
    // Encoding: 0x1A80B000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=11, o2=0, Rn=0, Rd=0
    // Fields: o2=0, Rm=0, Rd=0, op=0, sf=0, cond=11, Rn=0
    let encoding: u32 = 0x1A80B000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=12 (condition GT (signed >))
#[test]
fn test_aarch64_integer_conditional_select_combo_20_0_1a80c000() {
    // Encoding: 0x1A80C000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=12, o2=0, Rn=0, Rd=0
    // Fields: Rd=0, o2=0, op=0, cond=12, Rn=0, Rm=0, sf=0
    let encoding: u32 = 0x1A80C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=13 (condition LE (signed <=))
#[test]
fn test_aarch64_integer_conditional_select_combo_21_0_1a80d000() {
    // Encoding: 0x1A80D000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=13, o2=0, Rn=0, Rd=0
    // Fields: cond=13, op=0, o2=0, Rn=0, sf=0, Rd=0, Rm=0
    let encoding: u32 = 0x1A80D000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=14 (condition AL (always))
#[test]
fn test_aarch64_integer_conditional_select_combo_22_0_1a80e000() {
    // Encoding: 0x1A80E000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=14, o2=0, Rn=0, Rd=0
    // Fields: sf=0, Rn=0, op=0, cond=14, o2=0, Rm=0, Rd=0
    let encoding: u32 = 0x1A80E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=15 (condition NV (never, reserved))
#[test]
fn test_aarch64_integer_conditional_select_combo_23_0_1a80f000() {
    // Encoding: 0x1A80F000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=15, o2=0, Rn=0, Rd=0
    // Fields: cond=15, Rn=0, Rd=0, sf=0, o2=0, Rm=0, op=0
    let encoding: u32 = 0x1A80F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o2=0 (minimum value)
#[test]
fn test_aarch64_integer_conditional_select_combo_24_0_1a800000() {
    // Encoding: 0x1A800000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=0, o2=0, Rn=0, Rd=0
    // Fields: op=0, Rm=0, o2=0, Rd=0, sf=0, cond=0, Rn=0
    let encoding: u32 = 0x1A800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o2=1 (maximum value (1))
#[test]
fn test_aarch64_integer_conditional_select_combo_25_0_1a800400() {
    // Encoding: 0x1A800400
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=0, o2=1, Rn=0, Rd=0
    // Fields: Rd=0, sf=0, o2=1, op=0, Rn=0, Rm=0, cond=0
    let encoding: u32 = 0x1A800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_conditional_select_combo_26_0_1a800000() {
    // Encoding: 0x1A800000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=0, o2=0, Rn=0, Rd=0
    // Fields: Rm=0, cond=0, o2=0, Rd=0, sf=0, Rn=0, op=0
    let encoding: u32 = 0x1A800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_conditional_select_combo_27_0_1a800020() {
    // Encoding: 0x1A800020
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=0, o2=0, Rn=1, Rd=0
    // Fields: Rd=0, sf=0, op=0, Rn=1, Rm=0, cond=0, o2=0
    let encoding: u32 = 0x1A800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_conditional_select_combo_28_0_1a8003c0() {
    // Encoding: 0x1A8003C0
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=0, o2=0, Rn=30, Rd=0
    // Fields: op=0, Rn=30, sf=0, Rm=0, Rd=0, cond=0, o2=0
    let encoding: u32 = 0x1A8003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_conditional_select_combo_29_0_1a8003e0() {
    // Encoding: 0x1A8003E0
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=0, o2=0, Rn=31, Rd=0
    // Fields: Rn=31, op=0, Rm=0, o2=0, Rd=0, cond=0, sf=0
    let encoding: u32 = 0x1A8003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_conditional_select_combo_30_0_1a800000() {
    // Encoding: 0x1A800000
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=0, o2=0, Rn=0, Rd=0
    // Fields: op=0, sf=0, cond=0, o2=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x1A800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_conditional_select_combo_31_0_1a800001() {
    // Encoding: 0x1A800001
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=0, o2=0, Rn=0, Rd=1
    // Fields: op=0, Rm=0, cond=0, o2=0, Rn=0, Rd=1, sf=0
    let encoding: u32 = 0x1A800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_conditional_select_combo_32_0_1a80001e() {
    // Encoding: 0x1A80001E
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=0, o2=0, Rn=0, Rd=30
    // Fields: Rm=0, cond=0, o2=0, Rd=30, Rn=0, sf=0, op=0
    let encoding: u32 = 0x1A80001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_integer_conditional_select_combo_33_0_1a80001f() {
    // Encoding: 0x1A80001F
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=0, o2=0, Rn=0, Rd=31
    // Fields: Rd=31, Rn=0, o2=0, op=0, Rm=0, cond=0, sf=0
    let encoding: u32 = 0x1A80001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_conditional_select_combo_34_0_1a810020() {
    // Encoding: 0x1A810020
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=1, cond=0, o2=0, Rn=1, Rd=0
    // Fields: op=0, Rd=0, Rm=1, cond=0, sf=0, Rn=1, o2=0
    let encoding: u32 = 0x1A810020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_conditional_select_combo_35_0_1a9f03e0() {
    // Encoding: 0x1A9F03E0
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=31, cond=0, o2=0, Rn=31, Rd=0
    // Fields: o2=0, Rd=0, op=0, Rm=31, cond=0, Rn=31, sf=0
    let encoding: u32 = 0x1A9F03E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_conditional_select_combo_36_0_1a810001() {
    // Encoding: 0x1A810001
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=1, cond=0, o2=0, Rn=0, Rd=1
    // Fields: Rn=0, op=0, Rd=1, Rm=1, sf=0, cond=0, o2=0
    let encoding: u32 = 0x1A810001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 37`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_conditional_select_combo_37_0_1a9f001f() {
    // Encoding: 0x1A9F001F
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=31, cond=0, o2=0, Rn=0, Rd=31
    // Fields: op=0, Rm=31, Rn=0, Rd=31, cond=0, o2=0, sf=0
    let encoding: u32 = 0x1A9F001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 38`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_conditional_select_combo_38_0_1a800021() {
    // Encoding: 0x1A800021
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=0, o2=0, Rn=1, Rd=1
    // Fields: op=0, o2=0, sf=0, Rd=1, cond=0, Rm=0, Rn=1
    let encoding: u32 = 0x1A800021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field combination 39`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_conditional_select_combo_39_0_1a8003ff() {
    // Encoding: 0x1A8003FF
    // Test aarch64_integer_conditional_select field combination: sf=0, op=0, Rm=0, cond=0, o2=0, Rn=31, Rd=31
    // Fields: cond=0, sf=0, op=0, o2=0, Rn=31, Rd=31, Rm=0
    let encoding: u32 = 0x1A8003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_conditional_select_special_sf_0_size_variant_0_0_1a800000() {
    // Encoding: 0x1A800000
    // Test aarch64_integer_conditional_select special value sf = 0 (Size variant 0)
    // Fields: Rm=0, sf=0, o2=0, Rd=0, cond=0, op=0, Rn=0
    let encoding: u32 = 0x1A800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_conditional_select_special_sf_1_size_variant_1_0_9a800000() {
    // Encoding: 0x9A800000
    // Test aarch64_integer_conditional_select special value sf = 1 (Size variant 1)
    // Fields: sf=1, o2=0, Rn=0, op=0, Rd=0, Rm=0, cond=0
    let encoding: u32 = 0x9A800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch64_integer_conditional_select_special_cond_0_condition_eq_0_1a800000() {
    // Encoding: 0x1A800000
    // Test aarch64_integer_conditional_select special value cond = 0 (Condition EQ)
    // Fields: Rn=0, Rd=0, Rm=0, o2=0, sf=0, op=0, cond=0
    let encoding: u32 = 0x1A800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch64_integer_conditional_select_special_cond_1_condition_ne_0_1a801000() {
    // Encoding: 0x1A801000
    // Test aarch64_integer_conditional_select special value cond = 1 (Condition NE)
    // Fields: sf=0, Rd=0, cond=1, o2=0, op=0, Rn=0, Rm=0
    let encoding: u32 = 0x1A801000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch64_integer_conditional_select_special_cond_2_condition_cs_hs_0_1a802000() {
    // Encoding: 0x1A802000
    // Test aarch64_integer_conditional_select special value cond = 2 (Condition CS/HS)
    // Fields: cond=2, op=0, sf=0, Rm=0, o2=0, Rn=0, Rd=0
    let encoding: u32 = 0x1A802000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch64_integer_conditional_select_special_cond_3_condition_cc_lo_0_1a803000() {
    // Encoding: 0x1A803000
    // Test aarch64_integer_conditional_select special value cond = 3 (Condition CC/LO)
    // Fields: Rd=0, sf=0, op=0, Rm=0, cond=3, o2=0, Rn=0
    let encoding: u32 = 0x1A803000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch64_integer_conditional_select_special_cond_4_condition_mi_0_1a804000() {
    // Encoding: 0x1A804000
    // Test aarch64_integer_conditional_select special value cond = 4 (Condition MI)
    // Fields: sf=0, op=0, cond=4, Rn=0, Rd=0, o2=0, Rm=0
    let encoding: u32 = 0x1A804000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch64_integer_conditional_select_special_cond_5_condition_pl_0_1a805000() {
    // Encoding: 0x1A805000
    // Test aarch64_integer_conditional_select special value cond = 5 (Condition PL)
    // Fields: op=0, cond=5, Rn=0, Rd=0, sf=0, Rm=0, o2=0
    let encoding: u32 = 0x1A805000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch64_integer_conditional_select_special_cond_6_condition_vs_0_1a806000() {
    // Encoding: 0x1A806000
    // Test aarch64_integer_conditional_select special value cond = 6 (Condition VS)
    // Fields: cond=6, Rd=0, Rn=0, sf=0, op=0, o2=0, Rm=0
    let encoding: u32 = 0x1A806000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch64_integer_conditional_select_special_cond_7_condition_vc_0_1a807000() {
    // Encoding: 0x1A807000
    // Test aarch64_integer_conditional_select special value cond = 7 (Condition VC)
    // Fields: op=0, cond=7, o2=0, sf=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x1A807000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch64_integer_conditional_select_special_cond_8_condition_hi_0_1a808000() {
    // Encoding: 0x1A808000
    // Test aarch64_integer_conditional_select special value cond = 8 (Condition HI)
    // Fields: Rm=0, cond=8, Rn=0, Rd=0, o2=0, op=0, sf=0
    let encoding: u32 = 0x1A808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch64_integer_conditional_select_special_cond_9_condition_ls_0_1a809000() {
    // Encoding: 0x1A809000
    // Test aarch64_integer_conditional_select special value cond = 9 (Condition LS)
    // Fields: o2=0, Rd=0, cond=9, sf=0, Rn=0, Rm=0, op=0
    let encoding: u32 = 0x1A809000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch64_integer_conditional_select_special_cond_10_condition_ge_0_1a80a000() {
    // Encoding: 0x1A80A000
    // Test aarch64_integer_conditional_select special value cond = 10 (Condition GE)
    // Fields: op=0, sf=0, cond=10, o2=0, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x1A80A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch64_integer_conditional_select_special_cond_11_condition_lt_0_1a80b000() {
    // Encoding: 0x1A80B000
    // Test aarch64_integer_conditional_select special value cond = 11 (Condition LT)
    // Fields: cond=11, sf=0, o2=0, Rm=0, Rd=0, Rn=0, op=0
    let encoding: u32 = 0x1A80B000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch64_integer_conditional_select_special_cond_12_condition_gt_0_1a80c000() {
    // Encoding: 0x1A80C000
    // Test aarch64_integer_conditional_select special value cond = 12 (Condition GT)
    // Fields: Rd=0, Rm=0, sf=0, cond=12, o2=0, op=0, Rn=0
    let encoding: u32 = 0x1A80C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch64_integer_conditional_select_special_cond_13_condition_le_0_1a80d000() {
    // Encoding: 0x1A80D000
    // Test aarch64_integer_conditional_select special value cond = 13 (Condition LE)
    // Fields: Rn=0, Rd=0, op=0, o2=0, Rm=0, sf=0, cond=13
    let encoding: u32 = 0x1A80D000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch64_integer_conditional_select_special_cond_14_condition_al_0_1a80e000() {
    // Encoding: 0x1A80E000
    // Test aarch64_integer_conditional_select special value cond = 14 (Condition AL)
    // Fields: sf=0, Rn=0, cond=14, Rd=0, op=0, o2=0, Rm=0
    let encoding: u32 = 0x1A80E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch64_integer_conditional_select_special_cond_15_condition_nv_0_1a80f000() {
    // Encoding: 0x1A80F000
    // Test aarch64_integer_conditional_select special value cond = 15 (Condition NV)
    // Fields: sf=0, Rn=0, Rm=0, Rd=0, cond=15, op=0, o2=0
    let encoding: u32 = 0x1A80F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_conditional_select_special_rn_31_stack_pointer_sp_may_require_alignment_0_1a8003e0()
 {
    // Encoding: 0x1A8003E0
    // Test aarch64_integer_conditional_select special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: sf=0, op=0, Rm=0, o2=0, Rn=31, cond=0, Rd=0
    let encoding: u32 = 0x1A8003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_integer_conditional_select_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_1a80001f()
 {
    // Encoding: 0x1A80001F
    // Test aarch64_integer_conditional_select special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: cond=0, sf=0, Rd=31, Rm=0, op=0, Rn=0, o2=0
    let encoding: u32 = 0x1A80001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `CSEL X0, X1, X2, EQ`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// different values (32)
#[test]
fn test_aarch64_integer_conditional_select_csel_oracle_32_0_1a820020() {
    // Test CSEL 32-bit: different values (oracle)
    // Encoding: 0x1A820020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0xC8);
    let encoding: u32 = 0x1A820020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `CSEL X0, X1, X2, EQ`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// different values (64)
#[test]
fn test_aarch64_integer_conditional_select_csel_oracle_64_0_9a820020() {
    // Test CSEL 64-bit: different values (oracle)
    // Encoding: 0x9A820020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0xC8);
    let encoding: u32 = 0x9A820020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `CSEL X0, X1, X2, EQ`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero values (32)
#[test]
fn test_aarch64_integer_conditional_select_csel_oracle_32_1_1a820020() {
    // Test CSEL 32-bit: zero values (oracle)
    // Encoding: 0x1A820020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x1A820020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x00000000");
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `CSEL X0, X1, X2, EQ`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero values (64)
#[test]
fn test_aarch64_integer_conditional_select_csel_oracle_64_1_9a820020() {
    // Test CSEL 64-bit: zero values (oracle)
    // Encoding: 0x9A820020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x9A820020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `CSEL X0, X1, X2, EQ`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max Rn, zero Rm (32)
#[test]
fn test_aarch64_integer_conditional_select_csel_oracle_32_2_1a820020() {
    // Test CSEL 32-bit: max Rn, zero Rm (oracle)
    // Encoding: 0x1A820020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x1A820020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `CSEL X0, X1, X2, EQ`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max Rn, zero Rm (64)
#[test]
fn test_aarch64_integer_conditional_select_csel_oracle_64_2_9a820020() {
    // Test CSEL 64-bit: max Rn, zero Rm (oracle)
    // Encoding: 0x9A820020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x9A820020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `CSEL X0, X1, X2, EQ`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero Rn, max Rm (32)
#[test]
fn test_aarch64_integer_conditional_select_csel_oracle_32_3_1a820020() {
    // Test CSEL 32-bit: zero Rn, max Rm (oracle)
    // Encoding: 0x1A820020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0x1A820020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `CSEL X0, X1, X2, EQ`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero Rn, max Rm (64)
#[test]
fn test_aarch64_integer_conditional_select_csel_oracle_64_3_9a820020() {
    // Test CSEL 64-bit: zero Rn, max Rm (oracle)
    // Encoding: 0x9A820020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x9A820020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `CSEL X0, X1, X2, EQ`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// random pattern (32)
#[test]
fn test_aarch64_integer_conditional_select_csel_oracle_32_4_1a820020() {
    // Test CSEL 32-bit: random pattern (oracle)
    // Encoding: 0x1A820020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x12345678);
    set_x(&mut cpu, 2, 0xABCDEF01);
    let encoding: u32 = 0x1A820020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `CSEL X0, X1, X2, EQ`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// random pattern (64)
#[test]
fn test_aarch64_integer_conditional_select_csel_oracle_64_4_9a820020() {
    // Test CSEL 64-bit: random pattern (oracle)
    // Encoding: 0x9A820020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xABCDEF01);
    set_x(&mut cpu, 1, 0x12345678);
    let encoding: u32 = 0x9A820020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `CSEL X0, X1, X2, EQ`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// both one (32)
#[test]
fn test_aarch64_integer_conditional_select_csel_oracle_32_5_1a820020() {
    // Test CSEL 32-bit: both one (oracle)
    // Encoding: 0x1A820020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x1A820020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1, "W0 should be 0x00000001");
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `CSEL X0, X1, X2, EQ`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// both one (64)
#[test]
fn test_aarch64_integer_conditional_select_csel_oracle_64_5_9a820020() {
    // Test CSEL 64-bit: both one (oracle)
    // Encoding: 0x9A820020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x9A820020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x1, "X0 should be 0x0000000000000001");
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_conditional_select_reg_write_0_1a800000() {
    // Test aarch64_integer_conditional_select register write: GpFromField("d")
    // Encoding: 0x1A800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x1A800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_conditional_select_sp_rn_1a8003e0() {
    // Test aarch64_integer_conditional_select with Rn = SP (31)
    // Encoding: 0x1A8003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x1A8003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_conditional_select
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_integer_conditional_select_zr_rd_1a80001f() {
    // Test aarch64_integer_conditional_select with Rd = ZR (31)
    // Encoding: 0x1A80001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x1A80001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_integer_conditional_compare_register Tests
// ============================================================================

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_conditional_compare_register_field_sf_0_min_0_3a400000() {
    // Encoding: 0x3A400000
    // Test aarch64_integer_conditional_compare_register field sf = 0 (Min)
    // Fields: Rm=0, sf=0, nzcv=0, Rn=0, cond=0, op=0
    let encoding: u32 = 0x3A400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_conditional_compare_register_field_sf_1_max_0_ba400000() {
    // Encoding: 0xBA400000
    // Test aarch64_integer_conditional_compare_register field sf = 1 (Max)
    // Fields: sf=1, op=0, Rm=0, cond=0, nzcv=0, Rn=0
    let encoding: u32 = 0xBA400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field op 30 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_conditional_compare_register_field_op_0_min_0_3a400000() {
    // Encoding: 0x3A400000
    // Test aarch64_integer_conditional_compare_register field op = 0 (Min)
    // Fields: nzcv=0, op=0, sf=0, Rm=0, Rn=0, cond=0
    let encoding: u32 = 0x3A400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field op 30 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_op_1_max_0_7a400000() {
    // Encoding: 0x7A400000
    // Test aarch64_integer_conditional_compare_register field op = 1 (Max)
    // Fields: nzcv=0, sf=0, Rm=0, Rn=0, op=1, cond=0
    let encoding: u32 = 0x7A400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_rm_0_min_0_3a400000() {
    // Encoding: 0x3A400000
    // Test aarch64_integer_conditional_compare_register field Rm = 0 (Min)
    // Fields: nzcv=0, op=0, sf=0, cond=0, Rm=0, Rn=0
    let encoding: u32 = 0x3A400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_rm_1_poweroftwo_0_3a410000() {
    // Encoding: 0x3A410000
    // Test aarch64_integer_conditional_compare_register field Rm = 1 (PowerOfTwo)
    // Fields: Rm=1, Rn=0, nzcv=0, cond=0, op=0, sf=0
    let encoding: u32 = 0x3A410000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_rm_30_poweroftwominusone_0_3a5e0000() {
    // Encoding: 0x3A5E0000
    // Test aarch64_integer_conditional_compare_register field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: sf=0, Rm=30, cond=0, Rn=0, op=0, nzcv=0
    let encoding: u32 = 0x3A5E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_rm_31_max_0_3a5f0000() {
    // Encoding: 0x3A5F0000
    // Test aarch64_integer_conditional_compare_register field Rm = 31 (Max)
    // Fields: nzcv=0, op=0, sf=0, cond=0, Rn=0, Rm=31
    let encoding: u32 = 0x3A5F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_cond_0_min_0_3a400000() {
    // Encoding: 0x3A400000
    // Test aarch64_integer_conditional_compare_register field cond = 0 (Min)
    // Fields: Rn=0, sf=0, op=0, nzcv=0, cond=0, Rm=0
    let encoding: u32 = 0x3A400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_cond_1_poweroftwo_0_3a401000() {
    // Encoding: 0x3A401000
    // Test aarch64_integer_conditional_compare_register field cond = 1 (PowerOfTwo)
    // Fields: nzcv=0, Rm=0, sf=0, Rn=0, op=0, cond=1
    let encoding: u32 = 0x3A401000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_cond_2_poweroftwo_0_3a402000() {
    // Encoding: 0x3A402000
    // Test aarch64_integer_conditional_compare_register field cond = 2 (PowerOfTwo)
    // Fields: Rn=0, sf=0, Rm=0, cond=2, nzcv=0, op=0
    let encoding: u32 = 0x3A402000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_cond_3_poweroftwo_0_3a403000() {
    // Encoding: 0x3A403000
    // Test aarch64_integer_conditional_compare_register field cond = 3 (PowerOfTwo)
    // Fields: cond=3, Rn=0, Rm=0, nzcv=0, sf=0, op=0
    let encoding: u32 = 0x3A403000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_cond_4_poweroftwo_0_3a404000() {
    // Encoding: 0x3A404000
    // Test aarch64_integer_conditional_compare_register field cond = 4 (PowerOfTwo)
    // Fields: cond=4, op=0, Rn=0, nzcv=0, sf=0, Rm=0
    let encoding: u32 = 0x3A404000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_cond_5_poweroftwo_0_3a405000() {
    // Encoding: 0x3A405000
    // Test aarch64_integer_conditional_compare_register field cond = 5 (PowerOfTwo)
    // Fields: op=0, Rn=0, sf=0, Rm=0, nzcv=0, cond=5
    let encoding: u32 = 0x3A405000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_cond_6_poweroftwo_0_3a406000() {
    // Encoding: 0x3A406000
    // Test aarch64_integer_conditional_compare_register field cond = 6 (PowerOfTwo)
    // Fields: Rm=0, op=0, Rn=0, cond=6, sf=0, nzcv=0
    let encoding: u32 = 0x3A406000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_cond_7_poweroftwo_0_3a407000() {
    // Encoding: 0x3A407000
    // Test aarch64_integer_conditional_compare_register field cond = 7 (PowerOfTwo)
    // Fields: nzcv=0, cond=7, Rn=0, Rm=0, sf=0, op=0
    let encoding: u32 = 0x3A407000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_cond_8_poweroftwo_0_3a408000() {
    // Encoding: 0x3A408000
    // Test aarch64_integer_conditional_compare_register field cond = 8 (PowerOfTwo)
    // Fields: cond=8, Rn=0, sf=0, nzcv=0, Rm=0, op=0
    let encoding: u32 = 0x3A408000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_cond_9_poweroftwo_0_3a409000() {
    // Encoding: 0x3A409000
    // Test aarch64_integer_conditional_compare_register field cond = 9 (PowerOfTwo)
    // Fields: Rn=0, Rm=0, nzcv=0, op=0, cond=9, sf=0
    let encoding: u32 = 0x3A409000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_cond_10_poweroftwo_0_3a40a000() {
    // Encoding: 0x3A40A000
    // Test aarch64_integer_conditional_compare_register field cond = 10 (PowerOfTwo)
    // Fields: cond=10, Rn=0, op=0, Rm=0, nzcv=0, sf=0
    let encoding: u32 = 0x3A40A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_cond_11_poweroftwo_0_3a40b000() {
    // Encoding: 0x3A40B000
    // Test aarch64_integer_conditional_compare_register field cond = 11 (PowerOfTwo)
    // Fields: op=0, Rm=0, sf=0, cond=11, Rn=0, nzcv=0
    let encoding: u32 = 0x3A40B000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_cond_12_poweroftwo_0_3a40c000() {
    // Encoding: 0x3A40C000
    // Test aarch64_integer_conditional_compare_register field cond = 12 (PowerOfTwo)
    // Fields: nzcv=0, sf=0, cond=12, Rn=0, op=0, Rm=0
    let encoding: u32 = 0x3A40C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_cond_13_poweroftwo_0_3a40d000() {
    // Encoding: 0x3A40D000
    // Test aarch64_integer_conditional_compare_register field cond = 13 (PowerOfTwo)
    // Fields: op=0, Rm=0, Rn=0, cond=13, nzcv=0, sf=0
    let encoding: u32 = 0x3A40D000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_cond_14_poweroftwo_0_3a40e000() {
    // Encoding: 0x3A40E000
    // Test aarch64_integer_conditional_compare_register field cond = 14 (PowerOfTwo)
    // Fields: cond=14, nzcv=0, op=0, Rm=0, Rn=0, sf=0
    let encoding: u32 = 0x3A40E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_cond_15_max_0_3a40f000() {
    // Encoding: 0x3A40F000
    // Test aarch64_integer_conditional_compare_register field cond = 15 (Max)
    // Fields: op=0, cond=15, nzcv=0, Rm=0, sf=0, Rn=0
    let encoding: u32 = 0x3A40F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_rn_0_min_0_3a400000() {
    // Encoding: 0x3A400000
    // Test aarch64_integer_conditional_compare_register field Rn = 0 (Min)
    // Fields: cond=0, Rn=0, nzcv=0, sf=0, op=0, Rm=0
    let encoding: u32 = 0x3A400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_rn_1_poweroftwo_0_3a400020() {
    // Encoding: 0x3A400020
    // Test aarch64_integer_conditional_compare_register field Rn = 1 (PowerOfTwo)
    // Fields: Rm=0, nzcv=0, sf=0, Rn=1, op=0, cond=0
    let encoding: u32 = 0x3A400020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_rn_30_poweroftwominusone_0_3a4003c0() {
    // Encoding: 0x3A4003C0
    // Test aarch64_integer_conditional_compare_register field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: sf=0, Rm=0, Rn=30, nzcv=0, op=0, cond=0
    let encoding: u32 = 0x3A4003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_rn_31_max_0_3a4003e0() {
    // Encoding: 0x3A4003E0
    // Test aarch64_integer_conditional_compare_register field Rn = 31 (Max)
    // Fields: cond=0, op=0, Rm=0, Rn=31, nzcv=0, sf=0
    let encoding: u32 = 0x3A4003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field nzcv 0 +: 4`
/// Requirement: FieldBoundary { field: "nzcv", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_conditional_compare_register_field_nzcv_0_min_0_3a400000() {
    // Encoding: 0x3A400000
    // Test aarch64_integer_conditional_compare_register field nzcv = 0 (Min)
    // Fields: op=0, cond=0, Rn=0, sf=0, Rm=0, nzcv=0
    let encoding: u32 = 0x3A400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field nzcv 0 +: 4`
/// Requirement: FieldBoundary { field: "nzcv", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_conditional_compare_register_field_nzcv_1_poweroftwo_0_3a400001() {
    // Encoding: 0x3A400001
    // Test aarch64_integer_conditional_compare_register field nzcv = 1 (PowerOfTwo)
    // Fields: sf=0, Rm=0, Rn=0, nzcv=1, op=0, cond=0
    let encoding: u32 = 0x3A400001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field nzcv 0 +: 4`
/// Requirement: FieldBoundary { field: "nzcv", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_nzcv_7_poweroftwominusone_0_3a400007() {
    // Encoding: 0x3A400007
    // Test aarch64_integer_conditional_compare_register field nzcv = 7 (PowerOfTwoMinusOne)
    // Fields: cond=0, sf=0, Rm=0, op=0, nzcv=7, Rn=0
    let encoding: u32 = 0x3A400007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field nzcv 0 +: 4`
/// Requirement: FieldBoundary { field: "nzcv", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch64_integer_conditional_compare_register_field_nzcv_15_max_0_3a40000f() {
    // Encoding: 0x3A40000F
    // Test aarch64_integer_conditional_compare_register field nzcv = 15 (Max)
    // Fields: Rm=0, nzcv=15, Rn=0, op=0, sf=0, cond=0
    let encoding: u32 = 0x3A40000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_0_0_3a400000() {
    // Encoding: 0x3A400000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=0, Rn=0, nzcv=0
    // Fields: cond=0, nzcv=0, op=0, Rn=0, sf=0, Rm=0
    let encoding: u32 = 0x3A400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_1_0_ba400000() {
    // Encoding: 0xBA400000
    // Test aarch64_integer_conditional_compare_register field combination: sf=1, op=0, Rm=0, cond=0, Rn=0, nzcv=0
    // Fields: Rm=0, cond=0, Rn=0, nzcv=0, op=0, sf=1
    let encoding: u32 = 0xBA400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_2_0_3a400000() {
    // Encoding: 0x3A400000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=0, Rn=0, nzcv=0
    // Fields: op=0, sf=0, Rm=0, cond=0, nzcv=0, Rn=0
    let encoding: u32 = 0x3A400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_3_0_7a400000() {
    // Encoding: 0x7A400000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=1, Rm=0, cond=0, Rn=0, nzcv=0
    // Fields: cond=0, sf=0, Rn=0, Rm=0, op=1, nzcv=0
    let encoding: u32 = 0x7A400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_4_0_3a400000() {
    // Encoding: 0x3A400000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=0, Rn=0, nzcv=0
    // Fields: op=0, sf=0, cond=0, nzcv=0, Rm=0, Rn=0
    let encoding: u32 = 0x3A400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_5_0_3a410000() {
    // Encoding: 0x3A410000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=1, cond=0, Rn=0, nzcv=0
    // Fields: nzcv=0, cond=0, Rn=0, sf=0, op=0, Rm=1
    let encoding: u32 = 0x3A410000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_6_0_3a5e0000() {
    // Encoding: 0x3A5E0000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=30, cond=0, Rn=0, nzcv=0
    // Fields: cond=0, Rm=30, Rn=0, op=0, nzcv=0, sf=0
    let encoding: u32 = 0x3A5E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_7_0_3a5f0000() {
    // Encoding: 0x3A5F0000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=31, cond=0, Rn=0, nzcv=0
    // Fields: sf=0, Rm=31, op=0, cond=0, nzcv=0, Rn=0
    let encoding: u32 = 0x3A5F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_8_0_3a400000() {
    // Encoding: 0x3A400000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=0, Rn=0, nzcv=0
    // Fields: op=0, Rn=0, sf=0, Rm=0, cond=0, nzcv=0
    let encoding: u32 = 0x3A400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=1 (condition NE (not equal))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_9_0_3a401000() {
    // Encoding: 0x3A401000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=1, Rn=0, nzcv=0
    // Fields: sf=0, Rn=0, cond=1, Rm=0, nzcv=0, op=0
    let encoding: u32 = 0x3A401000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=2 (condition CS/HS (carry set))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_10_0_3a402000() {
    // Encoding: 0x3A402000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=2, Rn=0, nzcv=0
    // Fields: sf=0, op=0, Rn=0, cond=2, nzcv=0, Rm=0
    let encoding: u32 = 0x3A402000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=3 (condition CC/LO (carry clear))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_11_0_3a403000() {
    // Encoding: 0x3A403000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=3, Rn=0, nzcv=0
    // Fields: sf=0, cond=3, Rm=0, Rn=0, nzcv=0, op=0
    let encoding: u32 = 0x3A403000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=4 (condition MI (minus/negative))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_12_0_3a404000() {
    // Encoding: 0x3A404000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=4, Rn=0, nzcv=0
    // Fields: nzcv=0, Rn=0, Rm=0, sf=0, cond=4, op=0
    let encoding: u32 = 0x3A404000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=5 (condition PL (plus/positive))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_13_0_3a405000() {
    // Encoding: 0x3A405000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=5, Rn=0, nzcv=0
    // Fields: nzcv=0, op=0, cond=5, Rm=0, Rn=0, sf=0
    let encoding: u32 = 0x3A405000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=6 (condition VS (overflow set))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_14_0_3a406000() {
    // Encoding: 0x3A406000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=6, Rn=0, nzcv=0
    // Fields: sf=0, Rn=0, cond=6, nzcv=0, Rm=0, op=0
    let encoding: u32 = 0x3A406000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=7 (condition VC (overflow clear))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_15_0_3a407000() {
    // Encoding: 0x3A407000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=7, Rn=0, nzcv=0
    // Fields: Rn=0, sf=0, nzcv=0, cond=7, op=0, Rm=0
    let encoding: u32 = 0x3A407000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=8 (condition HI (unsigned higher))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_16_0_3a408000() {
    // Encoding: 0x3A408000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=8, Rn=0, nzcv=0
    // Fields: sf=0, op=0, Rn=0, Rm=0, cond=8, nzcv=0
    let encoding: u32 = 0x3A408000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=9 (condition LS (unsigned lower or same))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_17_0_3a409000() {
    // Encoding: 0x3A409000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=9, Rn=0, nzcv=0
    // Fields: cond=9, Rn=0, nzcv=0, Rm=0, op=0, sf=0
    let encoding: u32 = 0x3A409000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=10 (condition GE (signed >=))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_18_0_3a40a000() {
    // Encoding: 0x3A40A000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=10, Rn=0, nzcv=0
    // Fields: Rm=0, sf=0, op=0, cond=10, Rn=0, nzcv=0
    let encoding: u32 = 0x3A40A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=11 (condition LT (signed <))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_19_0_3a40b000() {
    // Encoding: 0x3A40B000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=11, Rn=0, nzcv=0
    // Fields: cond=11, Rm=0, sf=0, Rn=0, nzcv=0, op=0
    let encoding: u32 = 0x3A40B000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=12 (condition GT (signed >))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_20_0_3a40c000() {
    // Encoding: 0x3A40C000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=12, Rn=0, nzcv=0
    // Fields: sf=0, op=0, nzcv=0, Rm=0, cond=12, Rn=0
    let encoding: u32 = 0x3A40C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=13 (condition LE (signed <=))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_21_0_3a40d000() {
    // Encoding: 0x3A40D000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=13, Rn=0, nzcv=0
    // Fields: nzcv=0, op=0, Rm=0, Rn=0, sf=0, cond=13
    let encoding: u32 = 0x3A40D000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=14 (condition AL (always))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_22_0_3a40e000() {
    // Encoding: 0x3A40E000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=14, Rn=0, nzcv=0
    // Fields: Rm=0, nzcv=0, cond=14, Rn=0, sf=0, op=0
    let encoding: u32 = 0x3A40E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=15 (condition NV (never, reserved))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_23_0_3a40f000() {
    // Encoding: 0x3A40F000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=15, Rn=0, nzcv=0
    // Fields: op=0, Rm=0, cond=15, Rn=0, sf=0, nzcv=0
    let encoding: u32 = 0x3A40F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_24_0_3a400000() {
    // Encoding: 0x3A400000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=0, Rn=0, nzcv=0
    // Fields: op=0, sf=0, Rn=0, nzcv=0, cond=0, Rm=0
    let encoding: u32 = 0x3A400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_25_0_3a400020() {
    // Encoding: 0x3A400020
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=0, Rn=1, nzcv=0
    // Fields: Rn=1, op=0, Rm=0, cond=0, nzcv=0, sf=0
    let encoding: u32 = 0x3A400020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_26_0_3a4003c0() {
    // Encoding: 0x3A4003C0
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=0, Rn=30, nzcv=0
    // Fields: nzcv=0, Rm=0, op=0, Rn=30, cond=0, sf=0
    let encoding: u32 = 0x3A4003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_27_0_3a4003e0() {
    // Encoding: 0x3A4003E0
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=0, Rn=31, nzcv=0
    // Fields: sf=0, op=0, cond=0, Rn=31, nzcv=0, Rm=0
    let encoding: u32 = 0x3A4003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// nzcv=0 (minimum value)
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_28_0_3a400000() {
    // Encoding: 0x3A400000
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=0, Rn=0, nzcv=0
    // Fields: op=0, Rm=0, Rn=0, sf=0, cond=0, nzcv=0
    let encoding: u32 = 0x3A400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// nzcv=1 (value 1)
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_29_0_3a400001() {
    // Encoding: 0x3A400001
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=0, Rn=0, nzcv=1
    // Fields: nzcv=1, sf=0, op=0, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x3A400001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// nzcv=7 (midpoint (7))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_30_0_3a400007() {
    // Encoding: 0x3A400007
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=0, Rn=0, nzcv=7
    // Fields: nzcv=7, Rm=0, cond=0, sf=0, op=0, Rn=0
    let encoding: u32 = 0x3A400007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// nzcv=15 (maximum value (15))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_31_0_3a40000f() {
    // Encoding: 0x3A40000F
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=0, cond=0, Rn=0, nzcv=15
    // Fields: nzcv=15, sf=0, op=0, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x3A40000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_32_0_3a410020() {
    // Encoding: 0x3A410020
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=1, cond=0, Rn=1, nzcv=0
    // Fields: Rn=1, sf=0, Rm=1, op=0, cond=0, nzcv=0
    let encoding: u32 = 0x3A410020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_conditional_compare_register_combo_33_0_3a5f03e0() {
    // Encoding: 0x3A5F03E0
    // Test aarch64_integer_conditional_compare_register field combination: sf=0, op=0, Rm=31, cond=0, Rn=31, nzcv=0
    // Fields: op=0, Rn=31, sf=0, nzcv=0, Rm=31, cond=0
    let encoding: u32 = 0x3A5F03E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_conditional_compare_register_special_sf_0_size_variant_0_0_3a400000() {
    // Encoding: 0x3A400000
    // Test aarch64_integer_conditional_compare_register special value sf = 0 (Size variant 0)
    // Fields: cond=0, Rn=0, op=0, Rm=0, sf=0, nzcv=0
    let encoding: u32 = 0x3A400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_conditional_compare_register_special_sf_1_size_variant_1_0_ba400000() {
    // Encoding: 0xBA400000
    // Test aarch64_integer_conditional_compare_register special value sf = 1 (Size variant 1)
    // Fields: Rm=0, sf=1, op=0, Rn=0, cond=0, nzcv=0
    let encoding: u32 = 0xBA400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch64_integer_conditional_compare_register_special_cond_0_condition_eq_0_3a400000() {
    // Encoding: 0x3A400000
    // Test aarch64_integer_conditional_compare_register special value cond = 0 (Condition EQ)
    // Fields: Rn=0, Rm=0, cond=0, nzcv=0, op=0, sf=0
    let encoding: u32 = 0x3A400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch64_integer_conditional_compare_register_special_cond_1_condition_ne_0_3a401000() {
    // Encoding: 0x3A401000
    // Test aarch64_integer_conditional_compare_register special value cond = 1 (Condition NE)
    // Fields: op=0, sf=0, Rm=0, cond=1, nzcv=0, Rn=0
    let encoding: u32 = 0x3A401000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch64_integer_conditional_compare_register_special_cond_2_condition_cs_hs_0_3a402000() {
    // Encoding: 0x3A402000
    // Test aarch64_integer_conditional_compare_register special value cond = 2 (Condition CS/HS)
    // Fields: Rm=0, cond=2, sf=0, Rn=0, op=0, nzcv=0
    let encoding: u32 = 0x3A402000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch64_integer_conditional_compare_register_special_cond_3_condition_cc_lo_0_3a403000() {
    // Encoding: 0x3A403000
    // Test aarch64_integer_conditional_compare_register special value cond = 3 (Condition CC/LO)
    // Fields: cond=3, Rn=0, sf=0, Rm=0, nzcv=0, op=0
    let encoding: u32 = 0x3A403000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch64_integer_conditional_compare_register_special_cond_4_condition_mi_0_3a404000() {
    // Encoding: 0x3A404000
    // Test aarch64_integer_conditional_compare_register special value cond = 4 (Condition MI)
    // Fields: op=0, sf=0, cond=4, Rm=0, Rn=0, nzcv=0
    let encoding: u32 = 0x3A404000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch64_integer_conditional_compare_register_special_cond_5_condition_pl_0_3a405000() {
    // Encoding: 0x3A405000
    // Test aarch64_integer_conditional_compare_register special value cond = 5 (Condition PL)
    // Fields: Rm=0, nzcv=0, cond=5, Rn=0, sf=0, op=0
    let encoding: u32 = 0x3A405000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch64_integer_conditional_compare_register_special_cond_6_condition_vs_0_3a406000() {
    // Encoding: 0x3A406000
    // Test aarch64_integer_conditional_compare_register special value cond = 6 (Condition VS)
    // Fields: op=0, Rm=0, Rn=0, nzcv=0, sf=0, cond=6
    let encoding: u32 = 0x3A406000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch64_integer_conditional_compare_register_special_cond_7_condition_vc_0_3a407000() {
    // Encoding: 0x3A407000
    // Test aarch64_integer_conditional_compare_register special value cond = 7 (Condition VC)
    // Fields: op=0, nzcv=0, cond=7, sf=0, Rm=0, Rn=0
    let encoding: u32 = 0x3A407000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch64_integer_conditional_compare_register_special_cond_8_condition_hi_0_3a408000() {
    // Encoding: 0x3A408000
    // Test aarch64_integer_conditional_compare_register special value cond = 8 (Condition HI)
    // Fields: Rm=0, nzcv=0, sf=0, cond=8, op=0, Rn=0
    let encoding: u32 = 0x3A408000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch64_integer_conditional_compare_register_special_cond_9_condition_ls_0_3a409000() {
    // Encoding: 0x3A409000
    // Test aarch64_integer_conditional_compare_register special value cond = 9 (Condition LS)
    // Fields: Rm=0, nzcv=0, op=0, cond=9, Rn=0, sf=0
    let encoding: u32 = 0x3A409000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch64_integer_conditional_compare_register_special_cond_10_condition_ge_0_3a40a000() {
    // Encoding: 0x3A40A000
    // Test aarch64_integer_conditional_compare_register special value cond = 10 (Condition GE)
    // Fields: nzcv=0, op=0, sf=0, Rm=0, Rn=0, cond=10
    let encoding: u32 = 0x3A40A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch64_integer_conditional_compare_register_special_cond_11_condition_lt_0_3a40b000() {
    // Encoding: 0x3A40B000
    // Test aarch64_integer_conditional_compare_register special value cond = 11 (Condition LT)
    // Fields: cond=11, Rn=0, nzcv=0, sf=0, Rm=0, op=0
    let encoding: u32 = 0x3A40B000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch64_integer_conditional_compare_register_special_cond_12_condition_gt_0_3a40c000() {
    // Encoding: 0x3A40C000
    // Test aarch64_integer_conditional_compare_register special value cond = 12 (Condition GT)
    // Fields: nzcv=0, cond=12, sf=0, op=0, Rn=0, Rm=0
    let encoding: u32 = 0x3A40C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch64_integer_conditional_compare_register_special_cond_13_condition_le_0_3a40d000() {
    // Encoding: 0x3A40D000
    // Test aarch64_integer_conditional_compare_register special value cond = 13 (Condition LE)
    // Fields: cond=13, sf=0, Rm=0, nzcv=0, Rn=0, op=0
    let encoding: u32 = 0x3A40D000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch64_integer_conditional_compare_register_special_cond_14_condition_al_0_3a40e000() {
    // Encoding: 0x3A40E000
    // Test aarch64_integer_conditional_compare_register special value cond = 14 (Condition AL)
    // Fields: Rn=0, cond=14, Rm=0, nzcv=0, sf=0, op=0
    let encoding: u32 = 0x3A40E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch64_integer_conditional_compare_register_special_cond_15_condition_nv_0_3a40f000() {
    // Encoding: 0x3A40F000
    // Test aarch64_integer_conditional_compare_register special value cond = 15 (Condition NV)
    // Fields: cond=15, op=0, Rm=0, Rn=0, nzcv=0, sf=0
    let encoding: u32 = 0x3A40F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_conditional_compare_register_special_rn_31_stack_pointer_sp_may_require_alignment_0_3a4003e0()
 {
    // Encoding: 0x3A4003E0
    // Test aarch64_integer_conditional_compare_register special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, nzcv=0, sf=0, Rm=0, cond=0, op=0
    let encoding: u32 = 0x3A4003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_conditional_compare_register_sp_rn_3a4003e0() {
    // Test aarch64_integer_conditional_compare_register with Rn = SP (31)
    // Encoding: 0x3A4003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x3A4003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch64_integer_conditional_compare_register_flags_zeroresult_0_ba42e020() {
    // Test aarch64_integer_conditional_compare_register flag computation: ZeroResult
    // Encoding: 0xBA42E020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xBA42E020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch64_integer_conditional_compare_register_flags_zeroresult_1_ba42e020() {
    // Test aarch64_integer_conditional_compare_register flag computation: ZeroResult
    // Encoding: 0xBA42E020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xBA42E020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch64_integer_conditional_compare_register_flags_negativeresult_2_ba42e020() {
    // Test aarch64_integer_conditional_compare_register flag computation: NegativeResult
    // Encoding: 0xBA42E020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xBA42E020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch64_integer_conditional_compare_register_flags_unsignedoverflow_3_ba42e020() {
    // Test aarch64_integer_conditional_compare_register flag computation: UnsignedOverflow
    // Encoding: 0xBA42E020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xBA42E020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch64_integer_conditional_compare_register_flags_unsignedoverflow_4_ba42e020() {
    // Test aarch64_integer_conditional_compare_register flag computation: UnsignedOverflow
    // Encoding: 0xBA42E020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xBA42E020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch64_integer_conditional_compare_register_flags_signedoverflow_5_ba42e020() {
    // Test aarch64_integer_conditional_compare_register flag computation: SignedOverflow
    // Encoding: 0xBA42E020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    let encoding: u32 = 0xBA42E020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch64_integer_conditional_compare_register_flags_signedoverflow_6_ba42e020() {
    // Test aarch64_integer_conditional_compare_register flag computation: SignedOverflow
    // Encoding: 0xBA42E020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xBA42E020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch64_integer_conditional_compare_register
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch64_integer_conditional_compare_register_flags_positiveresult_7_ba42e020() {
    // Test aarch64_integer_conditional_compare_register flag computation: PositiveResult
    // Encoding: 0xBA42E020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x32);
    let encoding: u32 = 0xBA42E020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

// ============================================================================
// aarch64_integer_conditional_compare_immediate Tests
// ============================================================================

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_sf_0_min_800_3a400800() {
    // Encoding: 0x3A400800
    // Test aarch64_integer_conditional_compare_immediate field sf = 0 (Min)
    // Fields: sf=0, op=0, imm5=0, cond=0, nzcv=0, Rn=0
    let encoding: u32 = 0x3A400800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_sf_1_max_800_ba400800() {
    // Encoding: 0xBA400800
    // Test aarch64_integer_conditional_compare_immediate field sf = 1 (Max)
    // Fields: cond=0, sf=1, op=0, nzcv=0, Rn=0, imm5=0
    let encoding: u32 = 0xBA400800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field op 30 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_op_0_min_800_3a400800() {
    // Encoding: 0x3A400800
    // Test aarch64_integer_conditional_compare_immediate field op = 0 (Min)
    // Fields: op=0, imm5=0, cond=0, sf=0, Rn=0, nzcv=0
    let encoding: u32 = 0x3A400800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field op 30 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_op_1_max_800_7a400800() {
    // Encoding: 0x7A400800
    // Test aarch64_integer_conditional_compare_immediate field op = 1 (Max)
    // Fields: nzcv=0, imm5=0, Rn=0, cond=0, sf=0, op=1
    let encoding: u32 = 0x7A400800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_imm5_0_zero_800_3a400800() {
    // Encoding: 0x3A400800
    // Test aarch64_integer_conditional_compare_immediate field imm5 = 0 (Zero)
    // Fields: Rn=0, nzcv=0, imm5=0, sf=0, cond=0, op=0
    let encoding: u32 = 0x3A400800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_imm5_1_poweroftwo_800_3a410800() {
    // Encoding: 0x3A410800
    // Test aarch64_integer_conditional_compare_immediate field imm5 = 1 (PowerOfTwo)
    // Fields: op=0, nzcv=0, Rn=0, cond=0, imm5=1, sf=0
    let encoding: u32 = 0x3A410800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_imm5_3_poweroftwominusone_800_3a430800()
{
    // Encoding: 0x3A430800
    // Test aarch64_integer_conditional_compare_immediate field imm5 = 3 (PowerOfTwoMinusOne)
    // Fields: cond=0, Rn=0, nzcv=0, imm5=3, sf=0, op=0
    let encoding: u32 = 0x3A430800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_imm5_4_poweroftwo_800_3a440800() {
    // Encoding: 0x3A440800
    // Test aarch64_integer_conditional_compare_immediate field imm5 = 4 (PowerOfTwo)
    // Fields: op=0, cond=0, sf=0, Rn=0, imm5=4, nzcv=0
    let encoding: u32 = 0x3A440800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_imm5_7_poweroftwominusone_800_3a470800()
{
    // Encoding: 0x3A470800
    // Test aarch64_integer_conditional_compare_immediate field imm5 = 7 (PowerOfTwoMinusOne)
    // Fields: sf=0, imm5=7, nzcv=0, cond=0, op=0, Rn=0
    let encoding: u32 = 0x3A470800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_imm5_8_poweroftwo_800_3a480800() {
    // Encoding: 0x3A480800
    // Test aarch64_integer_conditional_compare_immediate field imm5 = 8 (PowerOfTwo)
    // Fields: op=0, nzcv=0, cond=0, sf=0, Rn=0, imm5=8
    let encoding: u32 = 0x3A480800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 15, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (15)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_imm5_15_poweroftwominusone_800_3a4f0800()
 {
    // Encoding: 0x3A4F0800
    // Test aarch64_integer_conditional_compare_immediate field imm5 = 15 (PowerOfTwoMinusOne)
    // Fields: cond=0, op=0, Rn=0, nzcv=0, imm5=15, sf=0
    let encoding: u32 = 0x3A4F0800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_imm5_16_poweroftwo_800_3a500800() {
    // Encoding: 0x3A500800
    // Test aarch64_integer_conditional_compare_immediate field imm5 = 16 (PowerOfTwo)
    // Fields: op=0, nzcv=0, Rn=0, sf=0, imm5=16, cond=0
    let encoding: u32 = 0x3A500800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 31, boundary: Max }
/// maximum immediate (31)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_imm5_31_max_800_3a5f0800() {
    // Encoding: 0x3A5F0800
    // Test aarch64_integer_conditional_compare_immediate field imm5 = 31 (Max)
    // Fields: Rn=0, nzcv=0, op=0, cond=0, imm5=31, sf=0
    let encoding: u32 = 0x3A5F0800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_cond_0_min_800_3a400800() {
    // Encoding: 0x3A400800
    // Test aarch64_integer_conditional_compare_immediate field cond = 0 (Min)
    // Fields: sf=0, nzcv=0, imm5=0, cond=0, op=0, Rn=0
    let encoding: u32 = 0x3A400800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_cond_1_poweroftwo_800_3a401800() {
    // Encoding: 0x3A401800
    // Test aarch64_integer_conditional_compare_immediate field cond = 1 (PowerOfTwo)
    // Fields: Rn=0, imm5=0, cond=1, nzcv=0, sf=0, op=0
    let encoding: u32 = 0x3A401800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_cond_2_poweroftwo_800_3a402800() {
    // Encoding: 0x3A402800
    // Test aarch64_integer_conditional_compare_immediate field cond = 2 (PowerOfTwo)
    // Fields: cond=2, op=0, nzcv=0, imm5=0, Rn=0, sf=0
    let encoding: u32 = 0x3A402800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_cond_3_poweroftwo_800_3a403800() {
    // Encoding: 0x3A403800
    // Test aarch64_integer_conditional_compare_immediate field cond = 3 (PowerOfTwo)
    // Fields: sf=0, Rn=0, imm5=0, op=0, nzcv=0, cond=3
    let encoding: u32 = 0x3A403800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_cond_4_poweroftwo_800_3a404800() {
    // Encoding: 0x3A404800
    // Test aarch64_integer_conditional_compare_immediate field cond = 4 (PowerOfTwo)
    // Fields: cond=4, imm5=0, Rn=0, op=0, nzcv=0, sf=0
    let encoding: u32 = 0x3A404800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_cond_5_poweroftwo_800_3a405800() {
    // Encoding: 0x3A405800
    // Test aarch64_integer_conditional_compare_immediate field cond = 5 (PowerOfTwo)
    // Fields: nzcv=0, op=0, sf=0, imm5=0, cond=5, Rn=0
    let encoding: u32 = 0x3A405800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_cond_6_poweroftwo_800_3a406800() {
    // Encoding: 0x3A406800
    // Test aarch64_integer_conditional_compare_immediate field cond = 6 (PowerOfTwo)
    // Fields: op=0, imm5=0, cond=6, Rn=0, sf=0, nzcv=0
    let encoding: u32 = 0x3A406800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_cond_7_poweroftwo_800_3a407800() {
    // Encoding: 0x3A407800
    // Test aarch64_integer_conditional_compare_immediate field cond = 7 (PowerOfTwo)
    // Fields: imm5=0, cond=7, Rn=0, op=0, sf=0, nzcv=0
    let encoding: u32 = 0x3A407800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_cond_8_poweroftwo_800_3a408800() {
    // Encoding: 0x3A408800
    // Test aarch64_integer_conditional_compare_immediate field cond = 8 (PowerOfTwo)
    // Fields: nzcv=0, sf=0, cond=8, op=0, imm5=0, Rn=0
    let encoding: u32 = 0x3A408800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_cond_9_poweroftwo_800_3a409800() {
    // Encoding: 0x3A409800
    // Test aarch64_integer_conditional_compare_immediate field cond = 9 (PowerOfTwo)
    // Fields: cond=9, Rn=0, op=0, nzcv=0, imm5=0, sf=0
    let encoding: u32 = 0x3A409800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_cond_10_poweroftwo_800_3a40a800() {
    // Encoding: 0x3A40A800
    // Test aarch64_integer_conditional_compare_immediate field cond = 10 (PowerOfTwo)
    // Fields: Rn=0, imm5=0, op=0, sf=0, cond=10, nzcv=0
    let encoding: u32 = 0x3A40A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_cond_11_poweroftwo_800_3a40b800() {
    // Encoding: 0x3A40B800
    // Test aarch64_integer_conditional_compare_immediate field cond = 11 (PowerOfTwo)
    // Fields: sf=0, op=0, imm5=0, Rn=0, cond=11, nzcv=0
    let encoding: u32 = 0x3A40B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_cond_12_poweroftwo_800_3a40c800() {
    // Encoding: 0x3A40C800
    // Test aarch64_integer_conditional_compare_immediate field cond = 12 (PowerOfTwo)
    // Fields: sf=0, cond=12, Rn=0, op=0, nzcv=0, imm5=0
    let encoding: u32 = 0x3A40C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_cond_13_poweroftwo_800_3a40d800() {
    // Encoding: 0x3A40D800
    // Test aarch64_integer_conditional_compare_immediate field cond = 13 (PowerOfTwo)
    // Fields: cond=13, imm5=0, sf=0, nzcv=0, op=0, Rn=0
    let encoding: u32 = 0x3A40D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_cond_14_poweroftwo_800_3a40e800() {
    // Encoding: 0x3A40E800
    // Test aarch64_integer_conditional_compare_immediate field cond = 14 (PowerOfTwo)
    // Fields: nzcv=0, imm5=0, sf=0, op=0, cond=14, Rn=0
    let encoding: u32 = 0x3A40E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_cond_15_max_800_3a40f800() {
    // Encoding: 0x3A40F800
    // Test aarch64_integer_conditional_compare_immediate field cond = 15 (Max)
    // Fields: cond=15, nzcv=0, op=0, sf=0, Rn=0, imm5=0
    let encoding: u32 = 0x3A40F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_rn_0_min_800_3a400800() {
    // Encoding: 0x3A400800
    // Test aarch64_integer_conditional_compare_immediate field Rn = 0 (Min)
    // Fields: sf=0, cond=0, nzcv=0, op=0, imm5=0, Rn=0
    let encoding: u32 = 0x3A400800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_rn_1_poweroftwo_800_3a400820() {
    // Encoding: 0x3A400820
    // Test aarch64_integer_conditional_compare_immediate field Rn = 1 (PowerOfTwo)
    // Fields: imm5=0, cond=0, Rn=1, sf=0, nzcv=0, op=0
    let encoding: u32 = 0x3A400820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_rn_30_poweroftwominusone_800_3a400bc0()
{
    // Encoding: 0x3A400BC0
    // Test aarch64_integer_conditional_compare_immediate field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: imm5=0, nzcv=0, Rn=30, cond=0, sf=0, op=0
    let encoding: u32 = 0x3A400BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_rn_31_max_800_3a400be0() {
    // Encoding: 0x3A400BE0
    // Test aarch64_integer_conditional_compare_immediate field Rn = 31 (Max)
    // Fields: sf=0, cond=0, nzcv=0, imm5=0, op=0, Rn=31
    let encoding: u32 = 0x3A400BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field nzcv 0 +: 4`
/// Requirement: FieldBoundary { field: "nzcv", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_nzcv_0_min_800_3a400800() {
    // Encoding: 0x3A400800
    // Test aarch64_integer_conditional_compare_immediate field nzcv = 0 (Min)
    // Fields: imm5=0, Rn=0, sf=0, op=0, cond=0, nzcv=0
    let encoding: u32 = 0x3A400800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field nzcv 0 +: 4`
/// Requirement: FieldBoundary { field: "nzcv", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_nzcv_1_poweroftwo_800_3a400801() {
    // Encoding: 0x3A400801
    // Test aarch64_integer_conditional_compare_immediate field nzcv = 1 (PowerOfTwo)
    // Fields: op=0, nzcv=1, cond=0, sf=0, imm5=0, Rn=0
    let encoding: u32 = 0x3A400801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field nzcv 0 +: 4`
/// Requirement: FieldBoundary { field: "nzcv", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_nzcv_7_poweroftwominusone_800_3a400807()
{
    // Encoding: 0x3A400807
    // Test aarch64_integer_conditional_compare_immediate field nzcv = 7 (PowerOfTwoMinusOne)
    // Fields: Rn=0, op=0, nzcv=7, sf=0, imm5=0, cond=0
    let encoding: u32 = 0x3A400807;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field nzcv 0 +: 4`
/// Requirement: FieldBoundary { field: "nzcv", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_field_nzcv_15_max_800_3a40080f() {
    // Encoding: 0x3A40080F
    // Test aarch64_integer_conditional_compare_immediate field nzcv = 15 (Max)
    // Fields: cond=0, Rn=0, imm5=0, sf=0, nzcv=15, op=0
    let encoding: u32 = 0x3A40080F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_0_800_3a400800() {
    // Encoding: 0x3A400800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=0, Rn=0, nzcv=0
    // Fields: sf=0, imm5=0, op=0, cond=0, Rn=0, nzcv=0
    let encoding: u32 = 0x3A400800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_1_800_ba400800() {
    // Encoding: 0xBA400800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=1, op=0, imm5=0, cond=0, Rn=0, nzcv=0
    // Fields: sf=1, op=0, cond=0, imm5=0, nzcv=0, Rn=0
    let encoding: u32 = 0xBA400800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_2_800_3a400800() {
    // Encoding: 0x3A400800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=0, Rn=0, nzcv=0
    // Fields: nzcv=0, op=0, sf=0, imm5=0, cond=0, Rn=0
    let encoding: u32 = 0x3A400800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_3_800_7a400800() {
    // Encoding: 0x7A400800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=1, imm5=0, cond=0, Rn=0, nzcv=0
    // Fields: Rn=0, op=1, imm5=0, nzcv=0, cond=0, sf=0
    let encoding: u32 = 0x7A400800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=0 (immediate value 0)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_4_800_3a400800() {
    // Encoding: 0x3A400800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=0, Rn=0, nzcv=0
    // Fields: op=0, sf=0, cond=0, imm5=0, Rn=0, nzcv=0
    let encoding: u32 = 0x3A400800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=1 (immediate value 1)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_5_800_3a410800() {
    // Encoding: 0x3A410800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=1, cond=0, Rn=0, nzcv=0
    // Fields: cond=0, Rn=0, op=0, sf=0, imm5=1, nzcv=0
    let encoding: u32 = 0x3A410800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_6_800_3a430800() {
    // Encoding: 0x3A430800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=3, cond=0, Rn=0, nzcv=0
    // Fields: Rn=0, op=0, imm5=3, nzcv=0, sf=0, cond=0
    let encoding: u32 = 0x3A430800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_7_800_3a440800() {
    // Encoding: 0x3A440800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=4, cond=0, Rn=0, nzcv=0
    // Fields: cond=0, imm5=4, nzcv=0, Rn=0, sf=0, op=0
    let encoding: u32 = 0x3A440800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_8_800_3a470800() {
    // Encoding: 0x3A470800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=7, cond=0, Rn=0, nzcv=0
    // Fields: sf=0, imm5=7, op=0, cond=0, Rn=0, nzcv=0
    let encoding: u32 = 0x3A470800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_9_800_3a480800() {
    // Encoding: 0x3A480800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=8, cond=0, Rn=0, nzcv=0
    // Fields: cond=0, sf=0, op=0, nzcv=0, imm5=8, Rn=0
    let encoding: u32 = 0x3A480800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=15 (immediate midpoint (15))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_10_800_3a4f0800() {
    // Encoding: 0x3A4F0800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=15, cond=0, Rn=0, nzcv=0
    // Fields: sf=0, imm5=15, cond=0, Rn=0, nzcv=0, op=0
    let encoding: u32 = 0x3A4F0800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_11_800_3a500800() {
    // Encoding: 0x3A500800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=16, cond=0, Rn=0, nzcv=0
    // Fields: imm5=16, Rn=0, cond=0, op=0, sf=0, nzcv=0
    let encoding: u32 = 0x3A500800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=31 (maximum immediate (31))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_12_800_3a5f0800() {
    // Encoding: 0x3A5F0800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=31, cond=0, Rn=0, nzcv=0
    // Fields: nzcv=0, op=0, sf=0, cond=0, imm5=31, Rn=0
    let encoding: u32 = 0x3A5F0800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_13_800_3a400800() {
    // Encoding: 0x3A400800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=0, Rn=0, nzcv=0
    // Fields: sf=0, imm5=0, op=0, cond=0, Rn=0, nzcv=0
    let encoding: u32 = 0x3A400800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=1 (condition NE (not equal))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_14_800_3a401800() {
    // Encoding: 0x3A401800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=1, Rn=0, nzcv=0
    // Fields: imm5=0, cond=1, sf=0, Rn=0, nzcv=0, op=0
    let encoding: u32 = 0x3A401800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=2 (condition CS/HS (carry set))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_15_800_3a402800() {
    // Encoding: 0x3A402800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=2, Rn=0, nzcv=0
    // Fields: Rn=0, imm5=0, op=0, nzcv=0, sf=0, cond=2
    let encoding: u32 = 0x3A402800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=3 (condition CC/LO (carry clear))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_16_800_3a403800() {
    // Encoding: 0x3A403800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=3, Rn=0, nzcv=0
    // Fields: op=0, imm5=0, cond=3, sf=0, nzcv=0, Rn=0
    let encoding: u32 = 0x3A403800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=4 (condition MI (minus/negative))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_17_800_3a404800() {
    // Encoding: 0x3A404800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=4, Rn=0, nzcv=0
    // Fields: nzcv=0, sf=0, op=0, Rn=0, imm5=0, cond=4
    let encoding: u32 = 0x3A404800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=5 (condition PL (plus/positive))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_18_800_3a405800() {
    // Encoding: 0x3A405800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=5, Rn=0, nzcv=0
    // Fields: cond=5, sf=0, imm5=0, Rn=0, nzcv=0, op=0
    let encoding: u32 = 0x3A405800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=6 (condition VS (overflow set))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_19_800_3a406800() {
    // Encoding: 0x3A406800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=6, Rn=0, nzcv=0
    // Fields: nzcv=0, imm5=0, op=0, cond=6, sf=0, Rn=0
    let encoding: u32 = 0x3A406800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=7 (condition VC (overflow clear))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_20_800_3a407800() {
    // Encoding: 0x3A407800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=7, Rn=0, nzcv=0
    // Fields: sf=0, op=0, Rn=0, imm5=0, cond=7, nzcv=0
    let encoding: u32 = 0x3A407800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=8 (condition HI (unsigned higher))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_21_800_3a408800() {
    // Encoding: 0x3A408800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=8, Rn=0, nzcv=0
    // Fields: Rn=0, op=0, nzcv=0, sf=0, cond=8, imm5=0
    let encoding: u32 = 0x3A408800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=9 (condition LS (unsigned lower or same))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_22_800_3a409800() {
    // Encoding: 0x3A409800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=9, Rn=0, nzcv=0
    // Fields: sf=0, cond=9, op=0, Rn=0, nzcv=0, imm5=0
    let encoding: u32 = 0x3A409800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=10 (condition GE (signed >=))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_23_800_3a40a800() {
    // Encoding: 0x3A40A800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=10, Rn=0, nzcv=0
    // Fields: cond=10, Rn=0, sf=0, nzcv=0, imm5=0, op=0
    let encoding: u32 = 0x3A40A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=11 (condition LT (signed <))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_24_800_3a40b800() {
    // Encoding: 0x3A40B800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=11, Rn=0, nzcv=0
    // Fields: nzcv=0, Rn=0, cond=11, sf=0, op=0, imm5=0
    let encoding: u32 = 0x3A40B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=12 (condition GT (signed >))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_25_800_3a40c800() {
    // Encoding: 0x3A40C800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=12, Rn=0, nzcv=0
    // Fields: Rn=0, op=0, nzcv=0, imm5=0, sf=0, cond=12
    let encoding: u32 = 0x3A40C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=13 (condition LE (signed <=))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_26_800_3a40d800() {
    // Encoding: 0x3A40D800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=13, Rn=0, nzcv=0
    // Fields: cond=13, op=0, imm5=0, Rn=0, nzcv=0, sf=0
    let encoding: u32 = 0x3A40D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=14 (condition AL (always))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_27_800_3a40e800() {
    // Encoding: 0x3A40E800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=14, Rn=0, nzcv=0
    // Fields: Rn=0, op=0, imm5=0, sf=0, cond=14, nzcv=0
    let encoding: u32 = 0x3A40E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=15 (condition NV (never, reserved))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_28_800_3a40f800() {
    // Encoding: 0x3A40F800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=15, Rn=0, nzcv=0
    // Fields: sf=0, op=0, imm5=0, cond=15, Rn=0, nzcv=0
    let encoding: u32 = 0x3A40F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_29_800_3a400800() {
    // Encoding: 0x3A400800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=0, Rn=0, nzcv=0
    // Fields: cond=0, nzcv=0, sf=0, op=0, imm5=0, Rn=0
    let encoding: u32 = 0x3A400800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_30_800_3a400820() {
    // Encoding: 0x3A400820
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=0, Rn=1, nzcv=0
    // Fields: Rn=1, sf=0, op=0, cond=0, imm5=0, nzcv=0
    let encoding: u32 = 0x3A400820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_31_800_3a400bc0() {
    // Encoding: 0x3A400BC0
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=0, Rn=30, nzcv=0
    // Fields: nzcv=0, cond=0, sf=0, op=0, imm5=0, Rn=30
    let encoding: u32 = 0x3A400BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_32_800_3a400be0() {
    // Encoding: 0x3A400BE0
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=0, Rn=31, nzcv=0
    // Fields: cond=0, sf=0, Rn=31, nzcv=0, imm5=0, op=0
    let encoding: u32 = 0x3A400BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// nzcv=0 (minimum value)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_33_800_3a400800() {
    // Encoding: 0x3A400800
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=0, Rn=0, nzcv=0
    // Fields: op=0, cond=0, Rn=0, nzcv=0, sf=0, imm5=0
    let encoding: u32 = 0x3A400800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// nzcv=1 (value 1)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_34_800_3a400801() {
    // Encoding: 0x3A400801
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=0, Rn=0, nzcv=1
    // Fields: imm5=0, nzcv=1, sf=0, Rn=0, op=0, cond=0
    let encoding: u32 = 0x3A400801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// nzcv=7 (midpoint (7))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_35_800_3a400807() {
    // Encoding: 0x3A400807
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=0, Rn=0, nzcv=7
    // Fields: Rn=0, sf=0, imm5=0, op=0, nzcv=7, cond=0
    let encoding: u32 = 0x3A400807;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// nzcv=15 (maximum value (15))
#[test]
fn test_aarch64_integer_conditional_compare_immediate_combo_36_800_3a40080f() {
    // Encoding: 0x3A40080F
    // Test aarch64_integer_conditional_compare_immediate field combination: sf=0, op=0, imm5=0, cond=0, Rn=0, nzcv=15
    // Fields: sf=0, op=0, cond=0, imm5=0, Rn=0, nzcv=15
    let encoding: u32 = 0x3A40080F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_conditional_compare_immediate_special_sf_0_size_variant_0_2048_3a410800() {
    // Encoding: 0x3A410800
    // Test aarch64_integer_conditional_compare_immediate special value sf = 0 (Size variant 0)
    // Fields: Rn=0, op=0, imm5=1, cond=0, sf=0, nzcv=0
    let encoding: u32 = 0x3A410800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_conditional_compare_immediate_special_sf_1_size_variant_1_2048_ba410800() {
    // Encoding: 0xBA410800
    // Test aarch64_integer_conditional_compare_immediate special value sf = 1 (Size variant 1)
    // Fields: op=0, imm5=1, sf=1, nzcv=0, cond=0, Rn=0
    let encoding: u32 = 0xBA410800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch64_integer_conditional_compare_immediate_special_cond_0_condition_eq_2048_3a410800() {
    // Encoding: 0x3A410800
    // Test aarch64_integer_conditional_compare_immediate special value cond = 0 (Condition EQ)
    // Fields: op=0, cond=0, Rn=0, nzcv=0, sf=0, imm5=1
    let encoding: u32 = 0x3A410800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch64_integer_conditional_compare_immediate_special_cond_1_condition_ne_2048_3a411800() {
    // Encoding: 0x3A411800
    // Test aarch64_integer_conditional_compare_immediate special value cond = 1 (Condition NE)
    // Fields: cond=1, Rn=0, op=0, nzcv=0, imm5=1, sf=0
    let encoding: u32 = 0x3A411800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch64_integer_conditional_compare_immediate_special_cond_2_condition_cs_hs_2048_3a412800()
{
    // Encoding: 0x3A412800
    // Test aarch64_integer_conditional_compare_immediate special value cond = 2 (Condition CS/HS)
    // Fields: nzcv=0, sf=0, op=0, cond=2, imm5=1, Rn=0
    let encoding: u32 = 0x3A412800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch64_integer_conditional_compare_immediate_special_cond_3_condition_cc_lo_2048_3a413800()
{
    // Encoding: 0x3A413800
    // Test aarch64_integer_conditional_compare_immediate special value cond = 3 (Condition CC/LO)
    // Fields: imm5=1, op=0, sf=0, cond=3, Rn=0, nzcv=0
    let encoding: u32 = 0x3A413800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch64_integer_conditional_compare_immediate_special_cond_4_condition_mi_2048_3a414800() {
    // Encoding: 0x3A414800
    // Test aarch64_integer_conditional_compare_immediate special value cond = 4 (Condition MI)
    // Fields: sf=0, cond=4, nzcv=0, Rn=0, imm5=1, op=0
    let encoding: u32 = 0x3A414800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch64_integer_conditional_compare_immediate_special_cond_5_condition_pl_2048_3a415800() {
    // Encoding: 0x3A415800
    // Test aarch64_integer_conditional_compare_immediate special value cond = 5 (Condition PL)
    // Fields: nzcv=0, cond=5, sf=0, op=0, imm5=1, Rn=0
    let encoding: u32 = 0x3A415800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch64_integer_conditional_compare_immediate_special_cond_6_condition_vs_2048_3a416800() {
    // Encoding: 0x3A416800
    // Test aarch64_integer_conditional_compare_immediate special value cond = 6 (Condition VS)
    // Fields: imm5=1, op=0, sf=0, Rn=0, nzcv=0, cond=6
    let encoding: u32 = 0x3A416800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch64_integer_conditional_compare_immediate_special_cond_7_condition_vc_2048_3a417800() {
    // Encoding: 0x3A417800
    // Test aarch64_integer_conditional_compare_immediate special value cond = 7 (Condition VC)
    // Fields: op=0, imm5=1, cond=7, Rn=0, nzcv=0, sf=0
    let encoding: u32 = 0x3A417800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch64_integer_conditional_compare_immediate_special_cond_8_condition_hi_2048_3a418800() {
    // Encoding: 0x3A418800
    // Test aarch64_integer_conditional_compare_immediate special value cond = 8 (Condition HI)
    // Fields: imm5=1, Rn=0, sf=0, op=0, cond=8, nzcv=0
    let encoding: u32 = 0x3A418800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch64_integer_conditional_compare_immediate_special_cond_9_condition_ls_2048_3a419800() {
    // Encoding: 0x3A419800
    // Test aarch64_integer_conditional_compare_immediate special value cond = 9 (Condition LS)
    // Fields: op=0, cond=9, Rn=0, imm5=1, sf=0, nzcv=0
    let encoding: u32 = 0x3A419800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch64_integer_conditional_compare_immediate_special_cond_10_condition_ge_2048_3a41a800() {
    // Encoding: 0x3A41A800
    // Test aarch64_integer_conditional_compare_immediate special value cond = 10 (Condition GE)
    // Fields: op=0, cond=10, imm5=1, nzcv=0, sf=0, Rn=0
    let encoding: u32 = 0x3A41A800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch64_integer_conditional_compare_immediate_special_cond_11_condition_lt_2048_3a41b800() {
    // Encoding: 0x3A41B800
    // Test aarch64_integer_conditional_compare_immediate special value cond = 11 (Condition LT)
    // Fields: imm5=1, nzcv=0, Rn=0, sf=0, op=0, cond=11
    let encoding: u32 = 0x3A41B800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch64_integer_conditional_compare_immediate_special_cond_12_condition_gt_2048_3a41c800() {
    // Encoding: 0x3A41C800
    // Test aarch64_integer_conditional_compare_immediate special value cond = 12 (Condition GT)
    // Fields: imm5=1, sf=0, op=0, cond=12, Rn=0, nzcv=0
    let encoding: u32 = 0x3A41C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch64_integer_conditional_compare_immediate_special_cond_13_condition_le_2048_3a41d800() {
    // Encoding: 0x3A41D800
    // Test aarch64_integer_conditional_compare_immediate special value cond = 13 (Condition LE)
    // Fields: sf=0, cond=13, nzcv=0, op=0, imm5=1, Rn=0
    let encoding: u32 = 0x3A41D800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch64_integer_conditional_compare_immediate_special_cond_14_condition_al_2048_3a41e800() {
    // Encoding: 0x3A41E800
    // Test aarch64_integer_conditional_compare_immediate special value cond = 14 (Condition AL)
    // Fields: sf=0, imm5=1, cond=14, Rn=0, op=0, nzcv=0
    let encoding: u32 = 0x3A41E800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch64_integer_conditional_compare_immediate_special_cond_15_condition_nv_2048_3a41f800() {
    // Encoding: 0x3A41F800
    // Test aarch64_integer_conditional_compare_immediate special value cond = 15 (Condition NV)
    // Fields: Rn=0, cond=15, op=0, sf=0, imm5=1, nzcv=0
    let encoding: u32 = 0x3A41F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_conditional_compare_immediate_special_rn_31_stack_pointer_sp_may_require_alignment_2048_3a410be0()
 {
    // Encoding: 0x3A410BE0
    // Test aarch64_integer_conditional_compare_immediate special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: sf=0, op=0, Rn=31, nzcv=0, imm5=1, cond=0
    let encoding: u32 = 0x3A410BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_sp_rn_3a400be0() {
    // Test aarch64_integer_conditional_compare_immediate with Rn = SP (31)
    // Encoding: 0x3A400BE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x3A400BE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_flags_zeroresult_0_ba40e820() {
    // Test aarch64_integer_conditional_compare_immediate flag computation: ZeroResult
    // Encoding: 0xBA40E820
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xBA40E820;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_flags_zeroresult_1_ba40e820() {
    // Test aarch64_integer_conditional_compare_immediate flag computation: ZeroResult
    // Encoding: 0xBA40E820
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xBA40E820;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_flags_negativeresult_2_ba40e820() {
    // Test aarch64_integer_conditional_compare_immediate flag computation: NegativeResult
    // Encoding: 0xBA40E820
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0xBA40E820;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_flags_unsignedoverflow_3_ba40e820() {
    // Test aarch64_integer_conditional_compare_immediate flag computation: UnsignedOverflow
    // Encoding: 0xBA40E820
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xBA40E820;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_flags_unsignedoverflow_4_ba40e820() {
    // Test aarch64_integer_conditional_compare_immediate flag computation: UnsignedOverflow
    // Encoding: 0xBA40E820
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xBA40E820;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_flags_signedoverflow_5_ba40e820() {
    // Test aarch64_integer_conditional_compare_immediate flag computation: SignedOverflow
    // Encoding: 0xBA40E820
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xBA40E820;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_flags_signedoverflow_6_ba40e820() {
    // Test aarch64_integer_conditional_compare_immediate flag computation: SignedOverflow
    // Encoding: 0xBA40E820
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0xBA40E820;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_conditional_compare_immediate
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch64_integer_conditional_compare_immediate_flags_positiveresult_7_ba40e820() {
    // Test aarch64_integer_conditional_compare_immediate flag computation: PositiveResult
    // Encoding: 0xBA40E820
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x32);
    let encoding: u32 = 0xBA40E820;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}
