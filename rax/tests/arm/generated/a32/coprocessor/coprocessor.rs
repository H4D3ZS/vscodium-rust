//! A32 coprocessor coprocessor tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers_32::*;

// ============================================================================
// aarch32_STC_A Tests
// ============================================================================

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_stc_t1a1_a_field_cond_0_min_5e00_0c005e00() {
    // Encoding: 0x0C005E00
    // Test aarch32_STC_T1A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: P=0, W=0, U=0, cond=0, Rn=0, imm8=0
    let encoding: u32 = 0x0C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_stc_t1a1_a_field_cond_1_poweroftwo_5e00_1c005e00() {
    // Encoding: 0x1C005E00
    // Test aarch32_STC_T1A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, W=0, U=0, imm8=0, P=0, Rn=0
    let encoding: u32 = 0x1C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_stc_t1a1_a_field_cond_2_poweroftwo_5e00_2c005e00() {
    // Encoding: 0x2C005E00
    // Test aarch32_STC_T1A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: U=0, cond=2, imm8=0, Rn=0, W=0, P=0
    let encoding: u32 = 0x2C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_stc_t1a1_a_field_cond_3_poweroftwo_5e00_3c005e00() {
    // Encoding: 0x3C005E00
    // Test aarch32_STC_T1A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, P=0, U=0, cond=3, W=0, imm8=0
    let encoding: u32 = 0x3C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_stc_t1a1_a_field_cond_4_poweroftwo_5e00_4c005e00() {
    // Encoding: 0x4C005E00
    // Test aarch32_STC_T1A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: W=0, Rn=0, U=0, cond=4, imm8=0, P=0
    let encoding: u32 = 0x4C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_stc_t1a1_a_field_cond_5_poweroftwo_5e00_5c005e00() {
    // Encoding: 0x5C005E00
    // Test aarch32_STC_T1A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: U=0, Rn=0, imm8=0, cond=5, P=0, W=0
    let encoding: u32 = 0x5C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_stc_t1a1_a_field_cond_6_poweroftwo_5e00_6c005e00() {
    // Encoding: 0x6C005E00
    // Test aarch32_STC_T1A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: P=0, Rn=0, cond=6, U=0, W=0, imm8=0
    let encoding: u32 = 0x6C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_stc_t1a1_a_field_cond_7_poweroftwo_5e00_7c005e00() {
    // Encoding: 0x7C005E00
    // Test aarch32_STC_T1A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: P=0, cond=7, Rn=0, U=0, W=0, imm8=0
    let encoding: u32 = 0x7C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_stc_t1a1_a_field_cond_8_poweroftwo_5e00_8c005e00() {
    // Encoding: 0x8C005E00
    // Test aarch32_STC_T1A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: W=0, cond=8, Rn=0, U=0, imm8=0, P=0
    let encoding: u32 = 0x8C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_stc_t1a1_a_field_cond_9_poweroftwo_5e00_9c005e00() {
    // Encoding: 0x9C005E00
    // Test aarch32_STC_T1A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: P=0, cond=9, Rn=0, imm8=0, W=0, U=0
    let encoding: u32 = 0x9C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_stc_t1a1_a_field_cond_10_poweroftwo_5e00_ac005e00() {
    // Encoding: 0xAC005E00
    // Test aarch32_STC_T1A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, W=0, P=0, U=0, Rn=0, imm8=0
    let encoding: u32 = 0xAC005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_stc_t1a1_a_field_cond_11_poweroftwo_5e00_bc005e00() {
    // Encoding: 0xBC005E00
    // Test aarch32_STC_T1A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: U=0, imm8=0, P=0, W=0, cond=11, Rn=0
    let encoding: u32 = 0xBC005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_stc_t1a1_a_field_cond_12_poweroftwo_5e00_cc005e00() {
    // Encoding: 0xCC005E00
    // Test aarch32_STC_T1A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, P=0, W=0, U=0, imm8=0, cond=12
    let encoding: u32 = 0xCC005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_stc_t1a1_a_field_cond_13_poweroftwo_5e00_dc005e00() {
    // Encoding: 0xDC005E00
    // Test aarch32_STC_T1A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: U=0, cond=13, P=0, W=0, Rn=0, imm8=0
    let encoding: u32 = 0xDC005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_stc_t1a1_a_field_cond_14_poweroftwo_5e00_ec005e00() {
    // Encoding: 0xEC005E00
    // Test aarch32_STC_T1A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: P=0, cond=14, W=0, U=0, Rn=0, imm8=0
    let encoding: u32 = 0xEC005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_stc_t1a1_a_field_cond_15_max_5e00_fc005e00() {
    // Encoding: 0xFC005E00
    // Test aarch32_STC_T1A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: P=0, U=0, Rn=0, imm8=0, W=0, cond=15
    let encoding: u32 = 0xFC005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field P 24 +: 1`
/// Requirement: FieldBoundary { field: "P", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_stc_t1a1_a_field_p_0_min_5e00_0c005e00() {
    // Encoding: 0x0C005E00
    // Test aarch32_STC_T1A1_A field P = 0 (Min)
    // ISET: A32
    // Fields: imm8=0, W=0, U=0, cond=0, Rn=0, P=0
    let encoding: u32 = 0x0C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field P 24 +: 1`
/// Requirement: FieldBoundary { field: "P", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_stc_t1a1_a_field_p_1_max_5e00_0d005e00() {
    // Encoding: 0x0D005E00
    // Test aarch32_STC_T1A1_A field P = 1 (Max)
    // ISET: A32
    // Fields: U=0, W=0, Rn=0, imm8=0, cond=0, P=1
    let encoding: u32 = 0x0D005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field U 23 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_stc_t1a1_a_field_u_0_min_5e00_0c005e00() {
    // Encoding: 0x0C005E00
    // Test aarch32_STC_T1A1_A field U = 0 (Min)
    // ISET: A32
    // Fields: P=0, W=0, imm8=0, cond=0, Rn=0, U=0
    let encoding: u32 = 0x0C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field U 23 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_stc_t1a1_a_field_u_1_max_5e00_0c805e00() {
    // Encoding: 0x0C805E00
    // Test aarch32_STC_T1A1_A field U = 1 (Max)
    // ISET: A32
    // Fields: imm8=0, cond=0, W=0, U=1, Rn=0, P=0
    let encoding: u32 = 0x0C805E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field W 21 +: 1`
/// Requirement: FieldBoundary { field: "W", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_stc_t1a1_a_field_w_0_min_5e00_0c005e00() {
    // Encoding: 0x0C005E00
    // Test aarch32_STC_T1A1_A field W = 0 (Min)
    // ISET: A32
    // Fields: cond=0, P=0, W=0, U=0, Rn=0, imm8=0
    let encoding: u32 = 0x0C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field W 21 +: 1`
/// Requirement: FieldBoundary { field: "W", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_stc_t1a1_a_field_w_1_max_5e00_0c205e00() {
    // Encoding: 0x0C205E00
    // Test aarch32_STC_T1A1_A field W = 1 (Max)
    // ISET: A32
    // Fields: P=0, Rn=0, W=1, cond=0, imm8=0, U=0
    let encoding: u32 = 0x0C205E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_stc_t1a1_a_field_rn_0_min_5e00_0c005e00() {
    // Encoding: 0x0C005E00
    // Test aarch32_STC_T1A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, W=0, U=0, Rn=0, imm8=0, P=0
    let encoding: u32 = 0x0C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_stc_t1a1_a_field_rn_1_poweroftwo_5e00_0c015e00() {
    // Encoding: 0x0C015E00
    // Test aarch32_STC_T1A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: U=0, W=0, cond=0, Rn=1, imm8=0, P=0
    let encoding: u32 = 0x0C015E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_0_zero_5e00_0c005e00() {
    // Encoding: 0x0C005E00
    // Test aarch32_STC_T1A1_A field imm8 = 0 (Zero)
    // ISET: A32
    // Fields: W=0, cond=0, U=0, Rn=0, imm8=0, P=0
    let encoding: u32 = 0x0C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_1_poweroftwo_5e00_0c005e01() {
    // Encoding: 0x0C005E01
    // Test aarch32_STC_T1A1_A field imm8 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: U=0, imm8=1, cond=0, P=0, Rn=0, W=0
    let encoding: u32 = 0x0C005E01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_3_poweroftwominusone_5e00_0c005e03() {
    // Encoding: 0x0C005E03
    // Test aarch32_STC_T1A1_A field imm8 = 3 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, Rn=0, W=0, imm8=3, U=0, P=0
    let encoding: u32 = 0x0C005E03;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_4_poweroftwo_5e00_0c005e04() {
    // Encoding: 0x0C005E04
    // Test aarch32_STC_T1A1_A field imm8 = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, U=0, cond=0, W=0, P=0, imm8=4
    let encoding: u32 = 0x0C005E04;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_7_poweroftwominusone_5e00_0c005e07() {
    // Encoding: 0x0C005E07
    // Test aarch32_STC_T1A1_A field imm8 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: U=0, W=0, P=0, imm8=7, cond=0, Rn=0
    let encoding: u32 = 0x0C005E07;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_8_poweroftwo_5e00_0c005e08() {
    // Encoding: 0x0C005E08
    // Test aarch32_STC_T1A1_A field imm8 = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: imm8=8, cond=0, Rn=0, U=0, P=0, W=0
    let encoding: u32 = 0x0C005E08;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_15_poweroftwominusone_5e00_0c005e0f() {
    // Encoding: 0x0C005E0F
    // Test aarch32_STC_T1A1_A field imm8 = 15 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: U=0, imm8=15, W=0, P=0, Rn=0, cond=0
    let encoding: u32 = 0x0C005E0F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_16_poweroftwo_5e00_0c005e10() {
    // Encoding: 0x0C005E10
    // Test aarch32_STC_T1A1_A field imm8 = 16 (PowerOfTwo)
    // ISET: A32
    // Fields: W=0, U=0, P=0, imm8=16, Rn=0, cond=0
    let encoding: u32 = 0x0C005E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_31_poweroftwominusone_5e00_0c005e1f() {
    // Encoding: 0x0C005E1F
    // Test aarch32_STC_T1A1_A field imm8 = 31 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, P=0, W=0, Rn=0, imm8=31, U=0
    let encoding: u32 = 0x0C005E1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_32_poweroftwo_5e00_0c005e20() {
    // Encoding: 0x0C005E20
    // Test aarch32_STC_T1A1_A field imm8 = 32 (PowerOfTwo)
    // ISET: A32
    // Fields: imm8=32, U=0, Rn=0, P=0, W=0, cond=0
    let encoding: u32 = 0x0C005E20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_63_poweroftwominusone_5e00_0c005e3f() {
    // Encoding: 0x0C005E3F
    // Test aarch32_STC_T1A1_A field imm8 = 63 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: W=0, imm8=63, Rn=0, cond=0, P=0, U=0
    let encoding: u32 = 0x0C005E3F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_64_poweroftwo_5e00_0c005e40() {
    // Encoding: 0x0C005E40
    // Test aarch32_STC_T1A1_A field imm8 = 64 (PowerOfTwo)
    // ISET: A32
    // Fields: P=0, imm8=64, W=0, Rn=0, cond=0, U=0
    let encoding: u32 = 0x0C005E40;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 127, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (127)
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_127_poweroftwominusone_5e00_0c005e7f() {
    // Encoding: 0x0C005E7F
    // Test aarch32_STC_T1A1_A field imm8 = 127 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: U=0, Rn=0, W=0, imm8=127, P=0, cond=0
    let encoding: u32 = 0x0C005E7F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_128_poweroftwo_5e00_0c005e80() {
    // Encoding: 0x0C005E80
    // Test aarch32_STC_T1A1_A field imm8 = 128 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm8=128, W=0, Rn=0, P=0, U=0
    let encoding: u32 = 0x0C005E80;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 255, boundary: Max }
/// maximum immediate (255)
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_255_max_5e00_0c005eff() {
    // Encoding: 0x0C005EFF
    // Test aarch32_STC_T1A1_A field imm8 = 255 (Max)
    // ISET: A32
    // Fields: U=0, cond=0, imm8=255, W=0, Rn=0, P=0
    let encoding: u32 = 0x0C005EFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_stc_t1a1_a_combo_0_5e00_0c005e00() {
    // Encoding: 0x0C005E00
    // Test aarch32_STC_T1A1_A field combination: cond=0, P=0, U=0, W=0, Rn=0, imm8=0
    // ISET: A32
    // Fields: P=0, W=0, Rn=0, imm8=0, U=0, cond=0
    let encoding: u32 = 0x0C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_stc_t1a1_a_special_cond_0_condition_eq_24064_0c005e00() {
    // Encoding: 0x0C005E00
    // Test aarch32_STC_T1A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: cond=0, P=0, U=0, W=0, imm8=0, Rn=0
    let encoding: u32 = 0x0C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_stc_t1a1_a_special_cond_1_condition_ne_24064_1c005e00() {
    // Encoding: 0x1C005E00
    // Test aarch32_STC_T1A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: P=0, imm8=0, U=0, W=0, Rn=0, cond=1
    let encoding: u32 = 0x1C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_stc_t1a1_a_special_cond_2_condition_cs_hs_24064_2c005e00() {
    // Encoding: 0x2C005E00
    // Test aarch32_STC_T1A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, P=0, U=0, W=0, Rn=0, imm8=0
    let encoding: u32 = 0x2C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_stc_t1a1_a_special_cond_3_condition_cc_lo_24064_3c005e00() {
    // Encoding: 0x3C005E00
    // Test aarch32_STC_T1A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: W=0, U=0, P=0, cond=3, Rn=0, imm8=0
    let encoding: u32 = 0x3C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_stc_t1a1_a_special_cond_4_condition_mi_24064_4c005e00() {
    // Encoding: 0x4C005E00
    // Test aarch32_STC_T1A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: U=0, P=0, imm8=0, Rn=0, W=0, cond=4
    let encoding: u32 = 0x4C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_stc_t1a1_a_special_cond_5_condition_pl_24064_5c005e00() {
    // Encoding: 0x5C005E00
    // Test aarch32_STC_T1A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, U=0, P=0, W=0, Rn=0, imm8=0
    let encoding: u32 = 0x5C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_stc_t1a1_a_special_cond_6_condition_vs_24064_6c005e00() {
    // Encoding: 0x6C005E00
    // Test aarch32_STC_T1A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: W=0, cond=6, P=0, Rn=0, U=0, imm8=0
    let encoding: u32 = 0x6C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_stc_t1a1_a_special_cond_7_condition_vc_24064_7c005e00() {
    // Encoding: 0x7C005E00
    // Test aarch32_STC_T1A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, imm8=0, U=0, W=0, P=0, Rn=0
    let encoding: u32 = 0x7C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_stc_t1a1_a_special_cond_8_condition_hi_24064_8c005e00() {
    // Encoding: 0x8C005E00
    // Test aarch32_STC_T1A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: P=0, cond=8, W=0, U=0, Rn=0, imm8=0
    let encoding: u32 = 0x8C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_stc_t1a1_a_special_cond_9_condition_ls_24064_9c005e00() {
    // Encoding: 0x9C005E00
    // Test aarch32_STC_T1A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: W=0, U=0, cond=9, Rn=0, imm8=0, P=0
    let encoding: u32 = 0x9C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_stc_t1a1_a_special_cond_10_condition_ge_24064_ac005e00() {
    // Encoding: 0xAC005E00
    // Test aarch32_STC_T1A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, P=0, imm8=0, W=0, U=0, Rn=0
    let encoding: u32 = 0xAC005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_stc_t1a1_a_special_cond_11_condition_lt_24064_bc005e00() {
    // Encoding: 0xBC005E00
    // Test aarch32_STC_T1A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: P=0, imm8=0, cond=11, U=0, W=0, Rn=0
    let encoding: u32 = 0xBC005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_stc_t1a1_a_special_cond_12_condition_gt_24064_cc005e00() {
    // Encoding: 0xCC005E00
    // Test aarch32_STC_T1A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: P=0, Rn=0, W=0, cond=12, U=0, imm8=0
    let encoding: u32 = 0xCC005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_stc_t1a1_a_special_cond_13_condition_le_24064_dc005e00() {
    // Encoding: 0xDC005E00
    // Test aarch32_STC_T1A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: W=0, U=0, P=0, cond=13, imm8=0, Rn=0
    let encoding: u32 = 0xDC005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_stc_t1a1_a_special_cond_14_condition_al_24064_ec005e00() {
    // Encoding: 0xEC005E00
    // Test aarch32_STC_T1A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: W=0, cond=14, U=0, imm8=0, P=0, Rn=0
    let encoding: u32 = 0xEC005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_stc_t1a1_a_special_cond_15_condition_nv_24064_fc005e00() {
    // Encoding: 0xFC005E00
    // Test aarch32_STC_T1A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: W=0, U=0, Rn=0, P=0, imm8=0, cond=15
    let encoding: u32 = 0xFC005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "P" }), rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "U" }) } }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "W" }) } }, rhs: LitBits([false]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"P\" }), rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"U\" }) } }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"W\" }) } }, rhs: LitBits([false]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_stc_t1a1_a_invalid_0_5e00_0c005e00() {
    // Encoding: 0x0C005E00
    // Test aarch32_STC_T1A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "P" }), rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "U" }) } }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "W" }) } }, rhs: LitBits([false]) }
    // ISET: A32
    // Fields: imm8=0, P=0, W=0, cond=0, U=0, Rn=0
    let encoding: u32 = 0x0C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_stc_t1a1_a_invalid_1_5e00_0c005e00() {
    // Encoding: 0x0C005E00
    // Test aarch32_STC_T1A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: P=0, cond=0, imm8=0, W=0, Rn=0, U=0
    let encoding: u32 = 0x0C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: Binary { op: And, lhs: LitInt(15), rhs: Binary { op: Ne, lhs: Binary { op: Or, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "wback" }), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "CurrentInstrSet" }, args: [] } }, rhs: Var(QualifiedIdentifier { qualifier: Any, name: "InstrSet_A32" }) } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }), rhs: Binary { op: And, lhs: LitInt(15), rhs: Binary { op: Ne, lhs: Binary { op: Or, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"wback\" }), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"CurrentInstrSet\" }, args: [] } }, rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"InstrSet_A32\" }) } } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_stc_t1a1_a_invalid_2_5e00_0c005e00() {
    // Encoding: 0x0C005E00
    // Test aarch32_STC_T1A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: Binary { op: And, lhs: LitInt(15), rhs: Binary { op: Ne, lhs: Binary { op: Or, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "wback" }), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "CurrentInstrSet" }, args: [] } }, rhs: Var(QualifiedIdentifier { qualifier: Any, name: "InstrSet_A32" }) } } }
    // ISET: A32
    // Fields: P=0, cond=0, U=0, Rn=0, imm8=0, W=0
    let encoding: u32 = 0x0C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_stc_t1a1_a_invalid_3_5e00_0c005e00() {
    // Encoding: 0x0C005E00
    // Test aarch32_STC_T1A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: cond=0, U=0, P=0, W=0, imm8=0, Rn=0
    let encoding: u32 = 0x0C005E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field P 24 +: 1`
/// Requirement: FieldBoundary { field: "P", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_stc_t1a1_a_field_p_0_min_5e00_ec005e00() {
    // Thumb encoding (32): 0xEC005E00
    // Test aarch32_STC_T1A1_A field P = 0 (Min)
    // ISET: T32
    // Fields: P=0, U=0, imm8=0, Rn=0, W=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC005E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field P 24 +: 1`
/// Requirement: FieldBoundary { field: "P", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_stc_t1a1_a_field_p_1_max_5e00_ed005e00() {
    // Thumb encoding (32): 0xED005E00
    // Test aarch32_STC_T1A1_A field P = 1 (Max)
    // ISET: T32
    // Fields: P=1, imm8=0, Rn=0, W=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xED005E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field U 23 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_stc_t1a1_a_field_u_0_min_5e00_ec005e00() {
    // Thumb encoding (32): 0xEC005E00
    // Test aarch32_STC_T1A1_A field U = 0 (Min)
    // ISET: T32
    // Fields: P=0, U=0, W=0, imm8=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC005E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field U 23 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_stc_t1a1_a_field_u_1_max_5e00_ec805e00() {
    // Thumb encoding (32): 0xEC805E00
    // Test aarch32_STC_T1A1_A field U = 1 (Max)
    // ISET: T32
    // Fields: W=0, P=0, Rn=0, U=1, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC805E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field W 21 +: 1`
/// Requirement: FieldBoundary { field: "W", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_stc_t1a1_a_field_w_0_min_5e00_ec005e00() {
    // Thumb encoding (32): 0xEC005E00
    // Test aarch32_STC_T1A1_A field W = 0 (Min)
    // ISET: T32
    // Fields: W=0, imm8=0, Rn=0, P=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC005E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field W 21 +: 1`
/// Requirement: FieldBoundary { field: "W", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_stc_t1a1_a_field_w_1_max_5e00_ec205e00() {
    // Thumb encoding (32): 0xEC205E00
    // Test aarch32_STC_T1A1_A field W = 1 (Max)
    // ISET: T32
    // Fields: W=1, Rn=0, imm8=0, U=0, P=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC205E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_stc_t1a1_a_field_rn_0_min_5e00_ec005e00() {
    // Thumb encoding (32): 0xEC005E00
    // Test aarch32_STC_T1A1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: imm8=0, W=0, U=0, P=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC005E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_stc_t1a1_a_field_rn_1_poweroftwo_5e00_ec015e00() {
    // Thumb encoding (32): 0xEC015E00
    // Test aarch32_STC_T1A1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: P=0, U=0, Rn=1, imm8=0, W=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC015E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_0_zero_5e00_ec005e00() {
    // Thumb encoding (32): 0xEC005E00
    // Test aarch32_STC_T1A1_A field imm8 = 0 (Zero)
    // ISET: T32
    // Fields: imm8=0, U=0, Rn=0, P=0, W=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC005E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_1_poweroftwo_5e00_ec005e01() {
    // Thumb encoding (32): 0xEC005E01
    // Test aarch32_STC_T1A1_A field imm8 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=1, Rn=0, W=0, U=0, P=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC005E01;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_3_poweroftwominusone_5e00_ec005e03() {
    // Thumb encoding (32): 0xEC005E03
    // Test aarch32_STC_T1A1_A field imm8 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: U=0, W=0, P=0, Rn=0, imm8=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC005E03;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_4_poweroftwo_5e00_ec005e04() {
    // Thumb encoding (32): 0xEC005E04
    // Test aarch32_STC_T1A1_A field imm8 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: U=0, P=0, imm8=4, W=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC005E04;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_7_poweroftwominusone_5e00_ec005e07() {
    // Thumb encoding (32): 0xEC005E07
    // Test aarch32_STC_T1A1_A field imm8 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: U=0, P=0, W=0, Rn=0, imm8=7
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC005E07;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_8_poweroftwo_5e00_ec005e08() {
    // Thumb encoding (32): 0xEC005E08
    // Test aarch32_STC_T1A1_A field imm8 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=8, P=0, W=0, U=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC005E08;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_15_poweroftwominusone_5e00_ec005e0f() {
    // Thumb encoding (32): 0xEC005E0F
    // Test aarch32_STC_T1A1_A field imm8 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: W=0, imm8=15, Rn=0, P=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC005E0F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_16_poweroftwo_5e00_ec005e10() {
    // Thumb encoding (32): 0xEC005E10
    // Test aarch32_STC_T1A1_A field imm8 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: W=0, P=0, U=0, imm8=16, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC005E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_31_poweroftwominusone_5e00_ec005e1f() {
    // Thumb encoding (32): 0xEC005E1F
    // Test aarch32_STC_T1A1_A field imm8 = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: P=0, W=0, imm8=31, U=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC005E1F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_32_poweroftwo_5e00_ec005e20() {
    // Thumb encoding (32): 0xEC005E20
    // Test aarch32_STC_T1A1_A field imm8 = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=32, P=0, Rn=0, U=0, W=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC005E20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_63_poweroftwominusone_5e00_ec005e3f() {
    // Thumb encoding (32): 0xEC005E3F
    // Test aarch32_STC_T1A1_A field imm8 = 63 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: U=0, P=0, imm8=63, Rn=0, W=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC005E3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_64_poweroftwo_5e00_ec005e40() {
    // Thumb encoding (32): 0xEC005E40
    // Test aarch32_STC_T1A1_A field imm8 = 64 (PowerOfTwo)
    // ISET: T32
    // Fields: W=0, Rn=0, P=0, imm8=64, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC005E40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 127, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (127)
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_127_poweroftwominusone_5e00_ec005e7f() {
    // Thumb encoding (32): 0xEC005E7F
    // Test aarch32_STC_T1A1_A field imm8 = 127 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=127, W=0, Rn=0, U=0, P=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC005E7F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_128_poweroftwo_5e00_ec005e80() {
    // Thumb encoding (32): 0xEC005E80
    // Test aarch32_STC_T1A1_A field imm8 = 128 (PowerOfTwo)
    // ISET: T32
    // Fields: U=0, W=0, P=0, Rn=0, imm8=128
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC005E80;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 255, boundary: Max }
/// maximum immediate (255)
#[test]
fn test_aarch32_stc_t1a1_a_field_imm8_255_max_5e00_ec005eff() {
    // Thumb encoding (32): 0xEC005EFF
    // Test aarch32_STC_T1A1_A field imm8 = 255 (Max)
    // ISET: T32
    // Fields: imm8=255, P=0, U=0, W=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC005EFF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// P=0 (minimum value)
#[test]
fn test_aarch32_stc_t1a1_a_combo_0_5e00_ec005e00() {
    // Thumb encoding (32): 0xEC005E00
    // Test aarch32_STC_T1A1_A field combination: P=0, U=0, W=0, Rn=0, imm8=0
    // ISET: T32
    // Fields: P=0, imm8=0, Rn=0, W=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC005E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "P" }), rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "U" }) } }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "W" }) } }, rhs: LitBits([false]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"P\" }), rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"U\" }) } }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"W\" }) } }, rhs: LitBits([false]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_stc_t1a1_a_invalid_0_5e00_ec005e00() {
    // Thumb encoding (32): 0xEC005E00
    // Test aarch32_STC_T1A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "P" }), rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "U" }) } }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "W" }) } }, rhs: LitBits([false]) }
    // ISET: T32
    // Fields: P=0, Rn=0, imm8=0, U=0, W=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC005E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_stc_t1a1_a_invalid_1_5e00_ec005e00() {
    // Thumb encoding (32): 0xEC005E00
    // Test aarch32_STC_T1A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: U=0, W=0, Rn=0, P=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC005E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: Binary { op: And, lhs: LitInt(15), rhs: Binary { op: Ne, lhs: Binary { op: Or, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "wback" }), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "CurrentInstrSet" }, args: [] } }, rhs: Var(QualifiedIdentifier { qualifier: Any, name: "InstrSet_A32" }) } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"n\" }), rhs: Binary { op: And, lhs: LitInt(15), rhs: Binary { op: Ne, lhs: Binary { op: Or, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"wback\" }), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"CurrentInstrSet\" }, args: [] } }, rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"InstrSet_A32\" }) } } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_stc_t1a1_a_invalid_2_5e00_ec005e00() {
    // Thumb encoding (32): 0xEC005E00
    // Test aarch32_STC_T1A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "n" }), rhs: Binary { op: And, lhs: LitInt(15), rhs: Binary { op: Ne, lhs: Binary { op: Or, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "wback" }), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "CurrentInstrSet" }, args: [] } }, rhs: Var(QualifiedIdentifier { qualifier: Any, name: "InstrSet_A32" }) } } }
    // ISET: T32
    // Fields: P=0, Rn=0, imm8=0, U=0, W=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC005E00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_stc_t1a1_a_invalid_3_5e00_ec005e00() {
    // Thumb encoding (32): 0xEC005E00
    // Test aarch32_STC_T1A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: U=0, P=0, W=0, imm8=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC005E00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch32_stc_t1a1_a_store_0_0c015e00() {
    // Test aarch32_STC_T1A1_A memory store: 8 bytes
    // Encoding: 0x0C015E00
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 0, 0xCAFEBABE);
    let encoding: u32 = 0x0C015E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch32_STC_T1A1_A
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch32_stc_t1a1_a_store_0_ec015e00() {
    // Test aarch32_STC_T1A1_A memory store: 8 bytes
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 0, 0xCAFEBABE);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xEC015E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// aarch32_MCRR_A Tests
// ============================================================================

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_cond_0_min_e00_0c400e00() {
    // Encoding: 0x0C400E00
    // Test aarch32_MCRR_T1A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rt=0, cond=0, opc1=0, Rt2=0, CRm=0, coproc=0
    let encoding: u32 = 0x0C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_cond_1_poweroftwo_e00_1c400e00() {
    // Encoding: 0x1C400E00
    // Test aarch32_MCRR_T1A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: opc1=0, CRm=0, cond=1, Rt2=0, Rt=0, coproc=0
    let encoding: u32 = 0x1C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_cond_2_poweroftwo_e00_2c400e00() {
    // Encoding: 0x2C400E00
    // Test aarch32_MCRR_T1A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: CRm=0, Rt=0, Rt2=0, coproc=0, cond=2, opc1=0
    let encoding: u32 = 0x2C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_cond_3_poweroftwo_e00_3c400e00() {
    // Encoding: 0x3C400E00
    // Test aarch32_MCRR_T1A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: coproc=0, Rt2=0, opc1=0, CRm=0, cond=3, Rt=0
    let encoding: u32 = 0x3C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_cond_4_poweroftwo_e00_4c400e00() {
    // Encoding: 0x4C400E00
    // Test aarch32_MCRR_T1A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: CRm=0, cond=4, Rt2=0, Rt=0, coproc=0, opc1=0
    let encoding: u32 = 0x4C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_cond_5_poweroftwo_e00_5c400e00() {
    // Encoding: 0x5C400E00
    // Test aarch32_MCRR_T1A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rt2=0, cond=5, CRm=0, opc1=0, Rt=0, coproc=0
    let encoding: u32 = 0x5C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_cond_6_poweroftwo_e00_6c400e00() {
    // Encoding: 0x6C400E00
    // Test aarch32_MCRR_T1A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, coproc=0, opc1=0, Rt2=0, Rt=0, CRm=0
    let encoding: u32 = 0x6C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_cond_7_poweroftwo_e00_7c400e00() {
    // Encoding: 0x7C400E00
    // Test aarch32_MCRR_T1A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: CRm=0, cond=7, opc1=0, coproc=0, Rt=0, Rt2=0
    let encoding: u32 = 0x7C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_cond_8_poweroftwo_e00_8c400e00() {
    // Encoding: 0x8C400E00
    // Test aarch32_MCRR_T1A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: CRm=0, cond=8, Rt=0, opc1=0, Rt2=0, coproc=0
    let encoding: u32 = 0x8C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_cond_9_poweroftwo_e00_9c400e00() {
    // Encoding: 0x9C400E00
    // Test aarch32_MCRR_T1A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, Rt=0, Rt2=0, opc1=0, coproc=0, CRm=0
    let encoding: u32 = 0x9C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_cond_10_poweroftwo_e00_ac400e00() {
    // Encoding: 0xAC400E00
    // Test aarch32_MCRR_T1A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: CRm=0, coproc=0, opc1=0, Rt2=0, Rt=0, cond=10
    let encoding: u32 = 0xAC400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_cond_11_poweroftwo_e00_bc400e00() {
    // Encoding: 0xBC400E00
    // Test aarch32_MCRR_T1A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: coproc=0, opc1=0, CRm=0, Rt2=0, Rt=0, cond=11
    let encoding: u32 = 0xBC400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_cond_12_poweroftwo_e00_cc400e00() {
    // Encoding: 0xCC400E00
    // Test aarch32_MCRR_T1A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: CRm=0, Rt=0, cond=12, Rt2=0, coproc=0, opc1=0
    let encoding: u32 = 0xCC400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_cond_13_poweroftwo_e00_dc400e00() {
    // Encoding: 0xDC400E00
    // Test aarch32_MCRR_T1A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=13, Rt=0, Rt2=0, CRm=0, coproc=0, opc1=0
    let encoding: u32 = 0xDC400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_cond_14_poweroftwo_e00_ec400e00() {
    // Encoding: 0xEC400E00
    // Test aarch32_MCRR_T1A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: opc1=0, CRm=0, Rt2=0, cond=14, Rt=0, coproc=0
    let encoding: u32 = 0xEC400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_cond_15_max_e00_fc400e00() {
    // Encoding: 0xFC400E00
    // Test aarch32_MCRR_T1A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rt=0, cond=15, Rt2=0, coproc=0, opc1=0, CRm=0
    let encoding: u32 = 0xFC400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field Rt2 16 +: 4`
/// Requirement: FieldBoundary { field: "Rt2", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_rt2_0_min_e00_0c400e00() {
    // Encoding: 0x0C400E00
    // Test aarch32_MCRR_T1A1_A field Rt2 = 0 (Min)
    // ISET: A32
    // Fields: opc1=0, cond=0, CRm=0, Rt2=0, coproc=0, Rt=0
    let encoding: u32 = 0x0C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field Rt2 16 +: 4`
/// Requirement: FieldBoundary { field: "Rt2", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_rt2_1_poweroftwo_e00_0c410e00() {
    // Encoding: 0x0C410E00
    // Test aarch32_MCRR_T1A1_A field Rt2 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: coproc=0, opc1=0, CRm=0, Rt=0, Rt2=1, cond=0
    let encoding: u32 = 0x0C410E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field Rt 12 +: 4`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_rt_0_min_e00_0c400e00() {
    // Encoding: 0x0C400E00
    // Test aarch32_MCRR_T1A1_A field Rt = 0 (Min)
    // ISET: A32
    // Fields: Rt2=0, coproc=0, Rt=0, cond=0, CRm=0, opc1=0
    let encoding: u32 = 0x0C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field Rt 12 +: 4`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_rt_1_poweroftwo_e00_0c401e00() {
    // Encoding: 0x0C401E00
    // Test aarch32_MCRR_T1A1_A field Rt = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: CRm=0, Rt2=0, coproc=0, cond=0, Rt=1, opc1=0
    let encoding: u32 = 0x0C401E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field coproc 9 +: 0`
/// Requirement: FieldBoundary { field: "coproc", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mcrr_t1a1_a_field_coproc_0_min_e00_0c400e00() {
    // Encoding: 0x0C400E00
    // Test aarch32_MCRR_T1A1_A field coproc = 0 (Min)
    // ISET: A32
    // Fields: CRm=0, cond=0, Rt2=0, Rt=0, coproc=0, opc1=0
    let encoding: u32 = 0x0C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field opc1 4 +: 4`
/// Requirement: FieldBoundary { field: "opc1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mcrr_t1a1_a_field_opc1_0_min_e00_0c400e00() {
    // Encoding: 0x0C400E00
    // Test aarch32_MCRR_T1A1_A field opc1 = 0 (Min)
    // ISET: A32
    // Fields: Rt=0, CRm=0, cond=0, Rt2=0, coproc=0, opc1=0
    let encoding: u32 = 0x0C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field opc1 4 +: 4`
/// Requirement: FieldBoundary { field: "opc1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mcrr_t1a1_a_field_opc1_1_poweroftwo_e00_0c400e10() {
    // Encoding: 0x0C400E10
    // Test aarch32_MCRR_T1A1_A field opc1 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rt2=0, Rt=0, CRm=0, cond=0, coproc=0, opc1=1
    let encoding: u32 = 0x0C400E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field opc1 4 +: 4`
/// Requirement: FieldBoundary { field: "opc1", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_opc1_7_poweroftwominusone_e00_0c400e70() {
    // Encoding: 0x0C400E70
    // Test aarch32_MCRR_T1A1_A field opc1 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: opc1=7, coproc=0, CRm=0, cond=0, Rt2=0, Rt=0
    let encoding: u32 = 0x0C400E70;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field opc1 4 +: 4`
/// Requirement: FieldBoundary { field: "opc1", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_opc1_15_max_e00_0c400ef0() {
    // Encoding: 0x0C400EF0
    // Test aarch32_MCRR_T1A1_A field opc1 = 15 (Max)
    // ISET: A32
    // Fields: coproc=0, opc1=15, CRm=0, Rt2=0, Rt=0, cond=0
    let encoding: u32 = 0x0C400EF0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mcrr_t1a1_a_field_crm_0_min_e00_0c400e00() {
    // Encoding: 0x0C400E00
    // Test aarch32_MCRR_T1A1_A field CRm = 0 (Min)
    // ISET: A32
    // Fields: CRm=0, Rt2=0, coproc=0, opc1=0, cond=0, Rt=0
    let encoding: u32 = 0x0C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mcrr_t1a1_a_field_crm_1_poweroftwo_e00_0c400e01() {
    // Encoding: 0x0C400E01
    // Test aarch32_MCRR_T1A1_A field CRm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: coproc=0, Rt=0, cond=0, Rt2=0, opc1=0, CRm=1
    let encoding: u32 = 0x0C400E01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_crm_7_poweroftwominusone_e00_0c400e07() {
    // Encoding: 0x0C400E07
    // Test aarch32_MCRR_T1A1_A field CRm = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, coproc=0, Rt2=0, opc1=0, CRm=7, Rt=0
    let encoding: u32 = 0x0C400E07;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_crm_15_max_e00_0c400e0f() {
    // Encoding: 0x0C400E0F
    // Test aarch32_MCRR_T1A1_A field CRm = 15 (Max)
    // ISET: A32
    // Fields: Rt2=0, Rt=0, coproc=0, CRm=15, opc1=0, cond=0
    let encoding: u32 = 0x0C400E0F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_mcrr_t1a1_a_combo_0_e00_0c400e00() {
    // Encoding: 0x0C400E00
    // Test aarch32_MCRR_T1A1_A field combination: cond=0, Rt2=0, Rt=0, coproc=0, opc1=0, CRm=0
    // ISET: A32
    // Fields: Rt2=0, Rt=0, cond=0, CRm=0, opc1=0, coproc=0
    let encoding: u32 = 0x0C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_mcrr_t1a1_a_special_cond_0_condition_eq_3584_0c400e00() {
    // Encoding: 0x0C400E00
    // Test aarch32_MCRR_T1A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rt=0, cond=0, coproc=0, opc1=0, CRm=0, Rt2=0
    let encoding: u32 = 0x0C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_mcrr_t1a1_a_special_cond_1_condition_ne_3584_1c400e00() {
    // Encoding: 0x1C400E00
    // Test aarch32_MCRR_T1A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: Rt2=0, cond=1, coproc=0, opc1=0, CRm=0, Rt=0
    let encoding: u32 = 0x1C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_mcrr_t1a1_a_special_cond_2_condition_cs_hs_3584_2c400e00() {
    // Encoding: 0x2C400E00
    // Test aarch32_MCRR_T1A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rt=0, Rt2=0, coproc=0, opc1=0, cond=2, CRm=0
    let encoding: u32 = 0x2C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_mcrr_t1a1_a_special_cond_3_condition_cc_lo_3584_3c400e00() {
    // Encoding: 0x3C400E00
    // Test aarch32_MCRR_T1A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, coproc=0, Rt2=0, Rt=0, CRm=0, opc1=0
    let encoding: u32 = 0x3C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_mcrr_t1a1_a_special_cond_4_condition_mi_3584_4c400e00() {
    // Encoding: 0x4C400E00
    // Test aarch32_MCRR_T1A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, Rt=0, coproc=0, opc1=0, CRm=0, Rt2=0
    let encoding: u32 = 0x4C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_mcrr_t1a1_a_special_cond_5_condition_pl_3584_5c400e00() {
    // Encoding: 0x5C400E00
    // Test aarch32_MCRR_T1A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rt2=0, cond=5, Rt=0, CRm=0, opc1=0, coproc=0
    let encoding: u32 = 0x5C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_mcrr_t1a1_a_special_cond_6_condition_vs_3584_6c400e00() {
    // Encoding: 0x6C400E00
    // Test aarch32_MCRR_T1A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: opc1=0, Rt2=0, CRm=0, coproc=0, cond=6, Rt=0
    let encoding: u32 = 0x6C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_mcrr_t1a1_a_special_cond_7_condition_vc_3584_7c400e00() {
    // Encoding: 0x7C400E00
    // Test aarch32_MCRR_T1A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: coproc=0, CRm=0, opc1=0, Rt2=0, cond=7, Rt=0
    let encoding: u32 = 0x7C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_mcrr_t1a1_a_special_cond_8_condition_hi_3584_8c400e00() {
    // Encoding: 0x8C400E00
    // Test aarch32_MCRR_T1A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rt2=0, cond=8, coproc=0, Rt=0, CRm=0, opc1=0
    let encoding: u32 = 0x8C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_mcrr_t1a1_a_special_cond_9_condition_ls_3584_9c400e00() {
    // Encoding: 0x9C400E00
    // Test aarch32_MCRR_T1A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: opc1=0, Rt2=0, coproc=0, cond=9, CRm=0, Rt=0
    let encoding: u32 = 0x9C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_mcrr_t1a1_a_special_cond_10_condition_ge_3584_ac400e00() {
    // Encoding: 0xAC400E00
    // Test aarch32_MCRR_T1A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, Rt=0, opc1=0, CRm=0, coproc=0, Rt2=0
    let encoding: u32 = 0xAC400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_mcrr_t1a1_a_special_cond_11_condition_lt_3584_bc400e00() {
    // Encoding: 0xBC400E00
    // Test aarch32_MCRR_T1A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: opc1=0, coproc=0, Rt=0, cond=11, Rt2=0, CRm=0
    let encoding: u32 = 0xBC400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_mcrr_t1a1_a_special_cond_12_condition_gt_3584_cc400e00() {
    // Encoding: 0xCC400E00
    // Test aarch32_MCRR_T1A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rt2=0, coproc=0, opc1=0, CRm=0, cond=12, Rt=0
    let encoding: u32 = 0xCC400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_mcrr_t1a1_a_special_cond_13_condition_le_3584_dc400e00() {
    // Encoding: 0xDC400E00
    // Test aarch32_MCRR_T1A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: opc1=0, coproc=0, Rt2=0, CRm=0, cond=13, Rt=0
    let encoding: u32 = 0xDC400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_mcrr_t1a1_a_special_cond_14_condition_al_3584_ec400e00() {
    // Encoding: 0xEC400E00
    // Test aarch32_MCRR_T1A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: opc1=0, Rt=0, Rt2=0, CRm=0, cond=14, coproc=0
    let encoding: u32 = 0xEC400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_mcrr_t1a1_a_special_cond_15_condition_nv_3584_fc400e00() {
    // Encoding: 0xFC400E00
    // Test aarch32_MCRR_T1A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: opc1=0, CRm=0, cond=15, coproc=0, Rt=0, Rt2=0
    let encoding: u32 = 0xFC400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "t" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "t2" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"t\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"t2\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mcrr_t1a1_a_invalid_0_e00_0c400e00() {
    // Encoding: 0x0C400E00
    // Test aarch32_MCRR_T1A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "t" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "t2" }) } }, rhs: LitInt(15) }
    // ISET: A32
    // Fields: Rt2=0, Rt=0, CRm=0, cond=0, coproc=0, opc1=0
    let encoding: u32 = 0x0C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mcrr_t1a1_a_invalid_1_e00_0c400e00() {
    // Encoding: 0x0C400E00
    // Test aarch32_MCRR_T1A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rt2=0, coproc=0, Rt=0, cond=0, opc1=0, CRm=0
    let encoding: u32 = 0x0C400E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field Rt2 16 +: 4`
/// Requirement: FieldBoundary { field: "Rt2", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_rt2_0_min_e00_ec400e00() {
    // Thumb encoding (32): 0xEC400E00
    // Test aarch32_MCRR_T1A1_A field Rt2 = 0 (Min)
    // ISET: T32
    // Fields: CRm=0, opc1=0, coproc=0, Rt=0, Rt2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC400E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field Rt2 16 +: 4`
/// Requirement: FieldBoundary { field: "Rt2", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_rt2_1_poweroftwo_e00_ec410e00() {
    // Thumb encoding (32): 0xEC410E00
    // Test aarch32_MCRR_T1A1_A field Rt2 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: CRm=0, Rt2=1, opc1=0, Rt=0, coproc=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC410E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field Rt 12 +: 4`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_rt_0_min_e00_ec400e00() {
    // Thumb encoding (32): 0xEC400E00
    // Test aarch32_MCRR_T1A1_A field Rt = 0 (Min)
    // ISET: T32
    // Fields: opc1=0, Rt2=0, CRm=0, Rt=0, coproc=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC400E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field Rt 12 +: 4`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_rt_1_poweroftwo_e00_ec401e00() {
    // Thumb encoding (32): 0xEC401E00
    // Test aarch32_MCRR_T1A1_A field Rt = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: CRm=0, Rt=1, coproc=0, opc1=0, Rt2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC401E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field coproc 9 +: 0`
/// Requirement: FieldBoundary { field: "coproc", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mcrr_t1a1_a_field_coproc_0_min_e00_ec400e00() {
    // Thumb encoding (32): 0xEC400E00
    // Test aarch32_MCRR_T1A1_A field coproc = 0 (Min)
    // ISET: T32
    // Fields: Rt=0, Rt2=0, coproc=0, CRm=0, opc1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC400E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field opc1 4 +: 4`
/// Requirement: FieldBoundary { field: "opc1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mcrr_t1a1_a_field_opc1_0_min_e00_ec400e00() {
    // Thumb encoding (32): 0xEC400E00
    // Test aarch32_MCRR_T1A1_A field opc1 = 0 (Min)
    // ISET: T32
    // Fields: Rt=0, opc1=0, CRm=0, Rt2=0, coproc=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC400E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field opc1 4 +: 4`
/// Requirement: FieldBoundary { field: "opc1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mcrr_t1a1_a_field_opc1_1_poweroftwo_e00_ec400e10() {
    // Thumb encoding (32): 0xEC400E10
    // Test aarch32_MCRR_T1A1_A field opc1 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: opc1=1, Rt2=0, coproc=0, Rt=0, CRm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC400E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field opc1 4 +: 4`
/// Requirement: FieldBoundary { field: "opc1", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_opc1_7_poweroftwominusone_e00_ec400e70() {
    // Thumb encoding (32): 0xEC400E70
    // Test aarch32_MCRR_T1A1_A field opc1 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rt2=0, Rt=0, coproc=0, opc1=7, CRm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC400E70;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field opc1 4 +: 4`
/// Requirement: FieldBoundary { field: "opc1", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_opc1_15_max_e00_ec400ef0() {
    // Thumb encoding (32): 0xEC400EF0
    // Test aarch32_MCRR_T1A1_A field opc1 = 15 (Max)
    // ISET: T32
    // Fields: Rt2=0, CRm=0, coproc=0, Rt=0, opc1=15
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC400EF0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mcrr_t1a1_a_field_crm_0_min_e00_ec400e00() {
    // Thumb encoding (32): 0xEC400E00
    // Test aarch32_MCRR_T1A1_A field CRm = 0 (Min)
    // ISET: T32
    // Fields: coproc=0, Rt2=0, CRm=0, opc1=0, Rt=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC400E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mcrr_t1a1_a_field_crm_1_poweroftwo_e00_ec400e01() {
    // Thumb encoding (32): 0xEC400E01
    // Test aarch32_MCRR_T1A1_A field CRm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: CRm=1, opc1=0, coproc=0, Rt=0, Rt2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC400E01;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_crm_7_poweroftwominusone_e00_ec400e07() {
    // Thumb encoding (32): 0xEC400E07
    // Test aarch32_MCRR_T1A1_A field CRm = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: coproc=0, Rt2=0, Rt=0, opc1=0, CRm=7
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC400E07;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch32_mcrr_t1a1_a_field_crm_15_max_e00_ec400e0f() {
    // Thumb encoding (32): 0xEC400E0F
    // Test aarch32_MCRR_T1A1_A field CRm = 15 (Max)
    // ISET: T32
    // Fields: Rt=0, coproc=0, Rt2=0, opc1=0, CRm=15
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC400E0F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=0 (register index 0 (first register))
#[test]
fn test_aarch32_mcrr_t1a1_a_combo_0_e00_ec400e00() {
    // Thumb encoding (32): 0xEC400E00
    // Test aarch32_MCRR_T1A1_A field combination: Rt2=0, Rt=0, coproc=0, opc1=0, CRm=0
    // ISET: T32
    // Fields: opc1=0, CRm=0, coproc=0, Rt2=0, Rt=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC400E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "t" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "t2" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"t\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"t2\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mcrr_t1a1_a_invalid_0_e00_ec400e00() {
    // Thumb encoding (32): 0xEC400E00
    // Test aarch32_MCRR_T1A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "t" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "t2" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: CRm=0, coproc=0, Rt2=0, Rt=0, opc1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC400E00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_MCRR_T1A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mcrr_t1a1_a_invalid_1_e00_ec400e00() {
    // Thumb encoding (32): 0xEC400E00
    // Test aarch32_MCRR_T1A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rt=0, coproc=0, opc1=0, Rt2=0, CRm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC400E00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_MRC_A Tests
// ============================================================================

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_mrc_t1a1_a_field_cond_0_min_e10_0e100e10() {
    // Encoding: 0x0E100E10
    // Test aarch32_MRC_T1A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rt=0, coproc=0, CRm=0, CRn=0, opc2=0, cond=0, opc1=0
    let encoding: u32 = 0x0E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_mrc_t1a1_a_field_cond_1_poweroftwo_e10_1e100e10() {
    // Encoding: 0x1E100E10
    // Test aarch32_MRC_T1A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rt=0, coproc=0, cond=1, CRn=0, opc2=0, opc1=0, CRm=0
    let encoding: u32 = 0x1E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_mrc_t1a1_a_field_cond_2_poweroftwo_e10_2e100e10() {
    // Encoding: 0x2E100E10
    // Test aarch32_MRC_T1A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rt=0, cond=2, opc1=0, CRn=0, coproc=0, opc2=0, CRm=0
    let encoding: u32 = 0x2E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_mrc_t1a1_a_field_cond_3_poweroftwo_e10_3e100e10() {
    // Encoding: 0x3E100E10
    // Test aarch32_MRC_T1A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rt=0, coproc=0, opc2=0, opc1=0, CRm=0, CRn=0, cond=3
    let encoding: u32 = 0x3E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_mrc_t1a1_a_field_cond_4_poweroftwo_e10_4e100e10() {
    // Encoding: 0x4E100E10
    // Test aarch32_MRC_T1A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: opc2=0, cond=4, CRm=0, opc1=0, CRn=0, Rt=0, coproc=0
    let encoding: u32 = 0x4E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_mrc_t1a1_a_field_cond_5_poweroftwo_e10_5e100e10() {
    // Encoding: 0x5E100E10
    // Test aarch32_MRC_T1A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rt=0, opc2=0, CRm=0, CRn=0, cond=5, opc1=0, coproc=0
    let encoding: u32 = 0x5E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_mrc_t1a1_a_field_cond_6_poweroftwo_e10_6e100e10() {
    // Encoding: 0x6E100E10
    // Test aarch32_MRC_T1A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: coproc=0, opc1=0, cond=6, opc2=0, CRn=0, Rt=0, CRm=0
    let encoding: u32 = 0x6E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_mrc_t1a1_a_field_cond_7_poweroftwo_e10_7e100e10() {
    // Encoding: 0x7E100E10
    // Test aarch32_MRC_T1A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: CRm=0, CRn=0, cond=7, opc1=0, Rt=0, opc2=0, coproc=0
    let encoding: u32 = 0x7E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_mrc_t1a1_a_field_cond_8_poweroftwo_e10_8e100e10() {
    // Encoding: 0x8E100E10
    // Test aarch32_MRC_T1A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=8, Rt=0, opc2=0, CRn=0, opc1=0, coproc=0, CRm=0
    let encoding: u32 = 0x8E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_mrc_t1a1_a_field_cond_9_poweroftwo_e10_9e100e10() {
    // Encoding: 0x9E100E10
    // Test aarch32_MRC_T1A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: CRm=0, CRn=0, Rt=0, coproc=0, opc1=0, cond=9, opc2=0
    let encoding: u32 = 0x9E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_mrc_t1a1_a_field_cond_10_poweroftwo_e10_ae100e10() {
    // Encoding: 0xAE100E10
    // Test aarch32_MRC_T1A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: opc1=0, CRn=0, coproc=0, opc2=0, cond=10, Rt=0, CRm=0
    let encoding: u32 = 0xAE100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_mrc_t1a1_a_field_cond_11_poweroftwo_e10_be100e10() {
    // Encoding: 0xBE100E10
    // Test aarch32_MRC_T1A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=11, coproc=0, CRn=0, opc2=0, opc1=0, CRm=0, Rt=0
    let encoding: u32 = 0xBE100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_mrc_t1a1_a_field_cond_12_poweroftwo_e10_ce100e10() {
    // Encoding: 0xCE100E10
    // Test aarch32_MRC_T1A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: opc1=0, CRn=0, cond=12, Rt=0, coproc=0, CRm=0, opc2=0
    let encoding: u32 = 0xCE100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_mrc_t1a1_a_field_cond_13_poweroftwo_e10_de100e10() {
    // Encoding: 0xDE100E10
    // Test aarch32_MRC_T1A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: CRm=0, CRn=0, Rt=0, coproc=0, cond=13, opc1=0, opc2=0
    let encoding: u32 = 0xDE100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_mrc_t1a1_a_field_cond_14_poweroftwo_e10_ee100e10() {
    // Encoding: 0xEE100E10
    // Test aarch32_MRC_T1A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rt=0, CRm=0, opc1=0, CRn=0, coproc=0, cond=14, opc2=0
    let encoding: u32 = 0xEE100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_mrc_t1a1_a_field_cond_15_max_e10_fe100e10() {
    // Encoding: 0xFE100E10
    // Test aarch32_MRC_T1A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: opc2=0, CRm=0, CRn=0, opc1=0, cond=15, Rt=0, coproc=0
    let encoding: u32 = 0xFE100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field opc1 21 +: 3`
/// Requirement: FieldBoundary { field: "opc1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mrc_t1a1_a_field_opc1_0_min_e10_0e100e10() {
    // Encoding: 0x0E100E10
    // Test aarch32_MRC_T1A1_A field opc1 = 0 (Min)
    // ISET: A32
    // Fields: opc1=0, Rt=0, CRm=0, CRn=0, cond=0, opc2=0, coproc=0
    let encoding: u32 = 0x0E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field opc1 21 +: 3`
/// Requirement: FieldBoundary { field: "opc1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mrc_t1a1_a_field_opc1_1_poweroftwo_e10_0e300e10() {
    // Encoding: 0x0E300E10
    // Test aarch32_MRC_T1A1_A field opc1 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, CRn=0, Rt=0, opc1=1, coproc=0, opc2=0, CRm=0
    let encoding: u32 = 0x0E300E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field opc1 21 +: 3`
/// Requirement: FieldBoundary { field: "opc1", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch32_mrc_t1a1_a_field_opc1_7_max_e10_0ef00e10() {
    // Encoding: 0x0EF00E10
    // Test aarch32_MRC_T1A1_A field opc1 = 7 (Max)
    // ISET: A32
    // Fields: CRm=0, coproc=0, opc2=0, opc1=7, CRn=0, Rt=0, cond=0
    let encoding: u32 = 0x0EF00E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field CRn 16 +: 4`
/// Requirement: FieldBoundary { field: "CRn", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mrc_t1a1_a_field_crn_0_min_e10_0e100e10() {
    // Encoding: 0x0E100E10
    // Test aarch32_MRC_T1A1_A field CRn = 0 (Min)
    // ISET: A32
    // Fields: CRm=0, opc1=0, cond=0, Rt=0, coproc=0, opc2=0, CRn=0
    let encoding: u32 = 0x0E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field CRn 16 +: 4`
/// Requirement: FieldBoundary { field: "CRn", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mrc_t1a1_a_field_crn_1_poweroftwo_e10_0e110e10() {
    // Encoding: 0x0E110E10
    // Test aarch32_MRC_T1A1_A field CRn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: opc1=0, CRn=1, Rt=0, coproc=0, CRm=0, opc2=0, cond=0
    let encoding: u32 = 0x0E110E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field CRn 16 +: 4`
/// Requirement: FieldBoundary { field: "CRn", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch32_mrc_t1a1_a_field_crn_7_poweroftwominusone_e10_0e170e10() {
    // Encoding: 0x0E170E10
    // Test aarch32_MRC_T1A1_A field CRn = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: opc2=0, CRm=0, CRn=7, Rt=0, coproc=0, cond=0, opc1=0
    let encoding: u32 = 0x0E170E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field CRn 16 +: 4`
/// Requirement: FieldBoundary { field: "CRn", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch32_mrc_t1a1_a_field_crn_15_max_e10_0e1f0e10() {
    // Encoding: 0x0E1F0E10
    // Test aarch32_MRC_T1A1_A field CRn = 15 (Max)
    // ISET: A32
    // Fields: cond=0, CRm=0, CRn=15, Rt=0, opc1=0, coproc=0, opc2=0
    let encoding: u32 = 0x0E1F0E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field Rt 12 +: 4`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mrc_t1a1_a_field_rt_0_min_e10_0e100e10() {
    // Encoding: 0x0E100E10
    // Test aarch32_MRC_T1A1_A field Rt = 0 (Min)
    // ISET: A32
    // Fields: coproc=0, opc2=0, Rt=0, CRn=0, opc1=0, cond=0, CRm=0
    let encoding: u32 = 0x0E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field Rt 12 +: 4`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mrc_t1a1_a_field_rt_1_poweroftwo_e10_0e101e10() {
    // Encoding: 0x0E101E10
    // Test aarch32_MRC_T1A1_A field Rt = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, opc1=0, CRn=0, opc2=0, Rt=1, CRm=0, coproc=0
    let encoding: u32 = 0x0E101E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field coproc 9 +: 0`
/// Requirement: FieldBoundary { field: "coproc", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mrc_t1a1_a_field_coproc_0_min_e10_0e100e10() {
    // Encoding: 0x0E100E10
    // Test aarch32_MRC_T1A1_A field coproc = 0 (Min)
    // ISET: A32
    // Fields: CRn=0, cond=0, opc1=0, Rt=0, coproc=0, CRm=0, opc2=0
    let encoding: u32 = 0x0E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field opc2 5 +: 3`
/// Requirement: FieldBoundary { field: "opc2", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mrc_t1a1_a_field_opc2_0_min_e10_0e100e10() {
    // Encoding: 0x0E100E10
    // Test aarch32_MRC_T1A1_A field opc2 = 0 (Min)
    // ISET: A32
    // Fields: CRm=0, CRn=0, Rt=0, coproc=0, opc1=0, cond=0, opc2=0
    let encoding: u32 = 0x0E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field opc2 5 +: 3`
/// Requirement: FieldBoundary { field: "opc2", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mrc_t1a1_a_field_opc2_1_poweroftwo_e10_0e100e30() {
    // Encoding: 0x0E100E30
    // Test aarch32_MRC_T1A1_A field opc2 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rt=0, cond=0, coproc=0, opc2=1, CRm=0, opc1=0, CRn=0
    let encoding: u32 = 0x0E100E30;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field opc2 5 +: 3`
/// Requirement: FieldBoundary { field: "opc2", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch32_mrc_t1a1_a_field_opc2_7_max_e10_0e100ef0() {
    // Encoding: 0x0E100EF0
    // Test aarch32_MRC_T1A1_A field opc2 = 7 (Max)
    // ISET: A32
    // Fields: CRm=0, opc1=0, CRn=0, Rt=0, opc2=7, cond=0, coproc=0
    let encoding: u32 = 0x0E100EF0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mrc_t1a1_a_field_crm_0_min_e10_0e100e10() {
    // Encoding: 0x0E100E10
    // Test aarch32_MRC_T1A1_A field CRm = 0 (Min)
    // ISET: A32
    // Fields: opc2=0, cond=0, opc1=0, Rt=0, coproc=0, CRn=0, CRm=0
    let encoding: u32 = 0x0E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mrc_t1a1_a_field_crm_1_poweroftwo_e10_0e100e11() {
    // Encoding: 0x0E100E11
    // Test aarch32_MRC_T1A1_A field CRm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: opc1=0, cond=0, coproc=0, opc2=0, CRm=1, CRn=0, Rt=0
    let encoding: u32 = 0x0E100E11;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch32_mrc_t1a1_a_field_crm_7_poweroftwominusone_e10_0e100e17() {
    // Encoding: 0x0E100E17
    // Test aarch32_MRC_T1A1_A field CRm = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: opc1=0, Rt=0, CRm=7, opc2=0, CRn=0, coproc=0, cond=0
    let encoding: u32 = 0x0E100E17;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch32_mrc_t1a1_a_field_crm_15_max_e10_0e100e1f() {
    // Encoding: 0x0E100E1F
    // Test aarch32_MRC_T1A1_A field CRm = 15 (Max)
    // ISET: A32
    // Fields: coproc=0, CRn=0, opc1=0, cond=0, Rt=0, opc2=0, CRm=15
    let encoding: u32 = 0x0E100E1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_mrc_t1a1_a_combo_0_e10_0e100e10() {
    // Encoding: 0x0E100E10
    // Test aarch32_MRC_T1A1_A field combination: cond=0, opc1=0, CRn=0, Rt=0, coproc=0, opc2=0, CRm=0
    // ISET: A32
    // Fields: CRn=0, Rt=0, cond=0, opc1=0, opc2=0, CRm=0, coproc=0
    let encoding: u32 = 0x0E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_mrc_t1a1_a_special_cond_0_condition_eq_3600_0e100e10() {
    // Encoding: 0x0E100E10
    // Test aarch32_MRC_T1A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: CRn=0, opc2=0, opc1=0, Rt=0, cond=0, coproc=0, CRm=0
    let encoding: u32 = 0x0E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_mrc_t1a1_a_special_cond_1_condition_ne_3600_1e100e10() {
    // Encoding: 0x1E100E10
    // Test aarch32_MRC_T1A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, Rt=0, coproc=0, CRm=0, opc1=0, opc2=0, CRn=0
    let encoding: u32 = 0x1E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_mrc_t1a1_a_special_cond_2_condition_cs_hs_3600_2e100e10() {
    // Encoding: 0x2E100E10
    // Test aarch32_MRC_T1A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rt=0, CRn=0, coproc=0, CRm=0, opc2=0, opc1=0, cond=2
    let encoding: u32 = 0x2E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_mrc_t1a1_a_special_cond_3_condition_cc_lo_3600_3e100e10() {
    // Encoding: 0x3E100E10
    // Test aarch32_MRC_T1A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: opc1=0, cond=3, coproc=0, opc2=0, CRm=0, Rt=0, CRn=0
    let encoding: u32 = 0x3E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_mrc_t1a1_a_special_cond_4_condition_mi_3600_4e100e10() {
    // Encoding: 0x4E100E10
    // Test aarch32_MRC_T1A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, CRn=0, Rt=0, coproc=0, opc2=0, CRm=0, opc1=0
    let encoding: u32 = 0x4E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_mrc_t1a1_a_special_cond_5_condition_pl_3600_5e100e10() {
    // Encoding: 0x5E100E10
    // Test aarch32_MRC_T1A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rt=0, coproc=0, opc2=0, CRn=0, CRm=0, cond=5, opc1=0
    let encoding: u32 = 0x5E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_mrc_t1a1_a_special_cond_6_condition_vs_3600_6e100e10() {
    // Encoding: 0x6E100E10
    // Test aarch32_MRC_T1A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, Rt=0, coproc=0, opc2=0, CRm=0, CRn=0, opc1=0
    let encoding: u32 = 0x6E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_mrc_t1a1_a_special_cond_7_condition_vc_3600_7e100e10() {
    // Encoding: 0x7E100E10
    // Test aarch32_MRC_T1A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rt=0, coproc=0, CRn=0, opc2=0, CRm=0, opc1=0, cond=7
    let encoding: u32 = 0x7E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_mrc_t1a1_a_special_cond_8_condition_hi_3600_8e100e10() {
    // Encoding: 0x8E100E10
    // Test aarch32_MRC_T1A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rt=0, opc2=0, cond=8, CRn=0, opc1=0, coproc=0, CRm=0
    let encoding: u32 = 0x8E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_mrc_t1a1_a_special_cond_9_condition_ls_3600_9e100e10() {
    // Encoding: 0x9E100E10
    // Test aarch32_MRC_T1A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, opc1=0, CRn=0, Rt=0, coproc=0, opc2=0, CRm=0
    let encoding: u32 = 0x9E100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_mrc_t1a1_a_special_cond_10_condition_ge_3600_ae100e10() {
    // Encoding: 0xAE100E10
    // Test aarch32_MRC_T1A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, opc2=0, CRm=0, CRn=0, opc1=0, coproc=0, Rt=0
    let encoding: u32 = 0xAE100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_mrc_t1a1_a_special_cond_11_condition_lt_3600_be100e10() {
    // Encoding: 0xBE100E10
    // Test aarch32_MRC_T1A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, Rt=0, coproc=0, opc2=0, CRm=0, opc1=0, CRn=0
    let encoding: u32 = 0xBE100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_mrc_t1a1_a_special_cond_12_condition_gt_3600_ce100e10() {
    // Encoding: 0xCE100E10
    // Test aarch32_MRC_T1A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rt=0, opc1=0, coproc=0, cond=12, CRn=0, opc2=0, CRm=0
    let encoding: u32 = 0xCE100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_mrc_t1a1_a_special_cond_13_condition_le_3600_de100e10() {
    // Encoding: 0xDE100E10
    // Test aarch32_MRC_T1A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: opc2=0, cond=13, Rt=0, CRn=0, CRm=0, opc1=0, coproc=0
    let encoding: u32 = 0xDE100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_mrc_t1a1_a_special_cond_14_condition_al_3600_ee100e10() {
    // Encoding: 0xEE100E10
    // Test aarch32_MRC_T1A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: coproc=0, opc2=0, CRn=0, Rt=0, opc1=0, CRm=0, cond=14
    let encoding: u32 = 0xEE100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_mrc_t1a1_a_special_cond_15_condition_nv_3600_fe100e10() {
    // Encoding: 0xFE100E10
    // Test aarch32_MRC_T1A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: opc2=0, Rt=0, CRm=0, opc1=0, CRn=0, cond=15, coproc=0
    let encoding: u32 = 0xFE100E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field opc1 21 +: 3`
/// Requirement: FieldBoundary { field: "opc1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mrc_t1a1_a_field_opc1_0_min_e10_ee100e10() {
    // Thumb encoding (32): 0xEE100E10
    // Test aarch32_MRC_T1A1_A field opc1 = 0 (Min)
    // ISET: T32
    // Fields: opc2=0, opc1=0, coproc=0, CRm=0, CRn=0, Rt=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE100E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field opc1 21 +: 3`
/// Requirement: FieldBoundary { field: "opc1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mrc_t1a1_a_field_opc1_1_poweroftwo_e10_ee300e10() {
    // Thumb encoding (32): 0xEE300E10
    // Test aarch32_MRC_T1A1_A field opc1 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rt=0, CRm=0, CRn=0, opc1=1, coproc=0, opc2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE300E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field opc1 21 +: 3`
/// Requirement: FieldBoundary { field: "opc1", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch32_mrc_t1a1_a_field_opc1_7_max_e10_eef00e10() {
    // Thumb encoding (32): 0xEEF00E10
    // Test aarch32_MRC_T1A1_A field opc1 = 7 (Max)
    // ISET: T32
    // Fields: coproc=0, CRm=0, Rt=0, opc1=7, opc2=0, CRn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEF00E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field CRn 16 +: 4`
/// Requirement: FieldBoundary { field: "CRn", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mrc_t1a1_a_field_crn_0_min_e10_ee100e10() {
    // Thumb encoding (32): 0xEE100E10
    // Test aarch32_MRC_T1A1_A field CRn = 0 (Min)
    // ISET: T32
    // Fields: Rt=0, opc1=0, opc2=0, CRm=0, CRn=0, coproc=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE100E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field CRn 16 +: 4`
/// Requirement: FieldBoundary { field: "CRn", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mrc_t1a1_a_field_crn_1_poweroftwo_e10_ee110e10() {
    // Thumb encoding (32): 0xEE110E10
    // Test aarch32_MRC_T1A1_A field CRn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: CRn=1, opc1=0, coproc=0, opc2=0, Rt=0, CRm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE110E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field CRn 16 +: 4`
/// Requirement: FieldBoundary { field: "CRn", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch32_mrc_t1a1_a_field_crn_7_poweroftwominusone_e10_ee170e10() {
    // Thumb encoding (32): 0xEE170E10
    // Test aarch32_MRC_T1A1_A field CRn = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rt=0, opc1=0, CRn=7, coproc=0, opc2=0, CRm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE170E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field CRn 16 +: 4`
/// Requirement: FieldBoundary { field: "CRn", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch32_mrc_t1a1_a_field_crn_15_max_e10_ee1f0e10() {
    // Thumb encoding (32): 0xEE1F0E10
    // Test aarch32_MRC_T1A1_A field CRn = 15 (Max)
    // ISET: T32
    // Fields: CRm=0, opc1=0, opc2=0, CRn=15, Rt=0, coproc=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE1F0E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field Rt 12 +: 4`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mrc_t1a1_a_field_rt_0_min_e10_ee100e10() {
    // Thumb encoding (32): 0xEE100E10
    // Test aarch32_MRC_T1A1_A field Rt = 0 (Min)
    // ISET: T32
    // Fields: opc1=0, CRm=0, opc2=0, CRn=0, coproc=0, Rt=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE100E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field Rt 12 +: 4`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mrc_t1a1_a_field_rt_1_poweroftwo_e10_ee101e10() {
    // Thumb encoding (32): 0xEE101E10
    // Test aarch32_MRC_T1A1_A field Rt = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: opc2=0, Rt=1, CRm=0, opc1=0, CRn=0, coproc=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE101E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field coproc 9 +: 0`
/// Requirement: FieldBoundary { field: "coproc", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mrc_t1a1_a_field_coproc_0_min_e10_ee100e10() {
    // Thumb encoding (32): 0xEE100E10
    // Test aarch32_MRC_T1A1_A field coproc = 0 (Min)
    // ISET: T32
    // Fields: opc1=0, Rt=0, CRn=0, CRm=0, coproc=0, opc2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE100E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field opc2 5 +: 3`
/// Requirement: FieldBoundary { field: "opc2", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mrc_t1a1_a_field_opc2_0_min_e10_ee100e10() {
    // Thumb encoding (32): 0xEE100E10
    // Test aarch32_MRC_T1A1_A field opc2 = 0 (Min)
    // ISET: T32
    // Fields: coproc=0, Rt=0, CRn=0, opc1=0, opc2=0, CRm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE100E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field opc2 5 +: 3`
/// Requirement: FieldBoundary { field: "opc2", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mrc_t1a1_a_field_opc2_1_poweroftwo_e10_ee100e30() {
    // Thumb encoding (32): 0xEE100E30
    // Test aarch32_MRC_T1A1_A field opc2 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: coproc=0, Rt=0, opc2=1, opc1=0, CRn=0, CRm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE100E30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field opc2 5 +: 3`
/// Requirement: FieldBoundary { field: "opc2", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch32_mrc_t1a1_a_field_opc2_7_max_e10_ee100ef0() {
    // Thumb encoding (32): 0xEE100EF0
    // Test aarch32_MRC_T1A1_A field opc2 = 7 (Max)
    // ISET: T32
    // Fields: CRn=0, opc2=7, Rt=0, coproc=0, CRm=0, opc1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE100EF0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mrc_t1a1_a_field_crm_0_min_e10_ee100e10() {
    // Thumb encoding (32): 0xEE100E10
    // Test aarch32_MRC_T1A1_A field CRm = 0 (Min)
    // ISET: T32
    // Fields: Rt=0, opc1=0, coproc=0, opc2=0, CRm=0, CRn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE100E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mrc_t1a1_a_field_crm_1_poweroftwo_e10_ee100e11() {
    // Thumb encoding (32): 0xEE100E11
    // Test aarch32_MRC_T1A1_A field CRm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: opc1=0, coproc=0, Rt=0, CRn=0, CRm=1, opc2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE100E11;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch32_mrc_t1a1_a_field_crm_7_poweroftwominusone_e10_ee100e17() {
    // Thumb encoding (32): 0xEE100E17
    // Test aarch32_MRC_T1A1_A field CRm = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: CRn=0, coproc=0, opc2=0, opc1=0, CRm=7, Rt=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE100E17;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch32_mrc_t1a1_a_field_crm_15_max_e10_ee100e1f() {
    // Thumb encoding (32): 0xEE100E1F
    // Test aarch32_MRC_T1A1_A field CRm = 15 (Max)
    // ISET: T32
    // Fields: opc1=0, CRm=15, coproc=0, opc2=0, Rt=0, CRn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE100E1F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc1=0 (minimum value)
#[test]
fn test_aarch32_mrc_t1a1_a_combo_0_e10_ee100e10() {
    // Thumb encoding (32): 0xEE100E10
    // Test aarch32_MRC_T1A1_A field combination: opc1=0, CRn=0, Rt=0, coproc=0, opc2=0, CRm=0
    // ISET: T32
    // Fields: Rt=0, opc2=0, CRm=0, CRn=0, coproc=0, opc1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE100E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_mrc_t1a1_a_flags_zeroresult_0_0e100e10() {
    // Test aarch32_MRC_T1A1_A flag computation: ZeroResult
    // Encoding: 0x0E100E10
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x0E100E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_mrc_t1a1_a_flags_zeroresult_1_0e100e10() {
    // Test aarch32_MRC_T1A1_A flag computation: ZeroResult
    // Encoding: 0x0E100E10
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0x0E100E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_mrc_t1a1_a_flags_negativeresult_2_0e100e10() {
    // Test aarch32_MRC_T1A1_A flag computation: NegativeResult
    // Encoding: 0x0E100E10
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x0E100E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_mrc_t1a1_a_flags_unsignedoverflow_3_0e100e10() {
    // Test aarch32_MRC_T1A1_A flag computation: UnsignedOverflow
    // Encoding: 0x0E100E10
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x0E100E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_mrc_t1a1_a_flags_unsignedoverflow_4_0e100e10() {
    // Test aarch32_MRC_T1A1_A flag computation: UnsignedOverflow
    // Encoding: 0x0E100E10
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x0E100E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_mrc_t1a1_a_flags_signedoverflow_5_0e100e10() {
    // Test aarch32_MRC_T1A1_A flag computation: SignedOverflow
    // Encoding: 0x0E100E10
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x0E100E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_mrc_t1a1_a_flags_signedoverflow_6_0e100e10() {
    // Test aarch32_MRC_T1A1_A flag computation: SignedOverflow
    // Encoding: 0x0E100E10
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0x0E100E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_mrc_t1a1_a_flags_positiveresult_7_0e100e10() {
    // Test aarch32_MRC_T1A1_A flag computation: PositiveResult
    // Encoding: 0x0E100E10
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x32);
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0x0E100E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_mrc_t1a1_a_flags_zeroresult_0_ee100e10() {
    // Test aarch32_MRC_T1A1_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xEE100E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_mrc_t1a1_a_flags_zeroresult_1_ee100e10() {
    // Test aarch32_MRC_T1A1_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0xEE100E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_mrc_t1a1_a_flags_negativeresult_2_ee100e10() {
    // Test aarch32_MRC_T1A1_A flag computation: NegativeResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xEE100E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_mrc_t1a1_a_flags_unsignedoverflow_3_ee100e10() {
    // Test aarch32_MRC_T1A1_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xEE100E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_mrc_t1a1_a_flags_unsignedoverflow_4_ee100e10() {
    // Test aarch32_MRC_T1A1_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x2);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xEE100E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_mrc_t1a1_a_flags_signedoverflow_5_ee100e10() {
    // Test aarch32_MRC_T1A1_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xEE100E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_mrc_t1a1_a_flags_signedoverflow_6_ee100e10() {
    // Test aarch32_MRC_T1A1_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0xEE100E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, true, "V should be true");
}

/// Provenance: aarch32_MRC_T1A1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_mrc_t1a1_a_flags_positiveresult_7_ee100e10() {
    // Test aarch32_MRC_T1A1_A flag computation: PositiveResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u32 = 0xEE100E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

// ============================================================================
// aarch32_MCR_A Tests
// ============================================================================

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_mcr_t1a1_a_field_cond_0_min_e10_0e000e10() {
    // Encoding: 0x0E000E10
    // Test aarch32_MCR_T1A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: opc1=0, CRn=0, opc2=0, Rt=0, coproc=0, cond=0, CRm=0
    let encoding: u32 = 0x0E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_mcr_t1a1_a_field_cond_1_poweroftwo_e10_1e000e10() {
    // Encoding: 0x1E000E10
    // Test aarch32_MCR_T1A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: coproc=0, CRn=0, Rt=0, opc2=0, cond=1, opc1=0, CRm=0
    let encoding: u32 = 0x1E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_mcr_t1a1_a_field_cond_2_poweroftwo_e10_2e000e10() {
    // Encoding: 0x2E000E10
    // Test aarch32_MCR_T1A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: CRn=0, opc2=0, Rt=0, CRm=0, cond=2, opc1=0, coproc=0
    let encoding: u32 = 0x2E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_mcr_t1a1_a_field_cond_3_poweroftwo_e10_3e000e10() {
    // Encoding: 0x3E000E10
    // Test aarch32_MCR_T1A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: opc2=0, opc1=0, CRm=0, coproc=0, CRn=0, Rt=0, cond=3
    let encoding: u32 = 0x3E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_mcr_t1a1_a_field_cond_4_poweroftwo_e10_4e000e10() {
    // Encoding: 0x4E000E10
    // Test aarch32_MCR_T1A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=4, Rt=0, opc2=0, CRn=0, coproc=0, opc1=0, CRm=0
    let encoding: u32 = 0x4E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_mcr_t1a1_a_field_cond_5_poweroftwo_e10_5e000e10() {
    // Encoding: 0x5E000E10
    // Test aarch32_MCR_T1A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: CRm=0, opc1=0, cond=5, coproc=0, opc2=0, CRn=0, Rt=0
    let encoding: u32 = 0x5E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_mcr_t1a1_a_field_cond_6_poweroftwo_e10_6e000e10() {
    // Encoding: 0x6E000E10
    // Test aarch32_MCR_T1A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: CRm=0, CRn=0, opc2=0, opc1=0, cond=6, coproc=0, Rt=0
    let encoding: u32 = 0x6E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_mcr_t1a1_a_field_cond_7_poweroftwo_e10_7e000e10() {
    // Encoding: 0x7E000E10
    // Test aarch32_MCR_T1A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: opc1=0, CRn=0, cond=7, coproc=0, opc2=0, Rt=0, CRm=0
    let encoding: u32 = 0x7E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_mcr_t1a1_a_field_cond_8_poweroftwo_e10_8e000e10() {
    // Encoding: 0x8E000E10
    // Test aarch32_MCR_T1A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Rt=0, cond=8, CRm=0, CRn=0, opc2=0, coproc=0, opc1=0
    let encoding: u32 = 0x8E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_mcr_t1a1_a_field_cond_9_poweroftwo_e10_9e000e10() {
    // Encoding: 0x9E000E10
    // Test aarch32_MCR_T1A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: CRn=0, coproc=0, CRm=0, cond=9, opc2=0, opc1=0, Rt=0
    let encoding: u32 = 0x9E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_mcr_t1a1_a_field_cond_10_poweroftwo_e10_ae000e10() {
    // Encoding: 0xAE000E10
    // Test aarch32_MCR_T1A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: CRm=0, cond=10, CRn=0, Rt=0, opc1=0, coproc=0, opc2=0
    let encoding: u32 = 0xAE000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_mcr_t1a1_a_field_cond_11_poweroftwo_e10_be000e10() {
    // Encoding: 0xBE000E10
    // Test aarch32_MCR_T1A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=11, Rt=0, opc2=0, coproc=0, CRm=0, CRn=0, opc1=0
    let encoding: u32 = 0xBE000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_mcr_t1a1_a_field_cond_12_poweroftwo_e10_ce000e10() {
    // Encoding: 0xCE000E10
    // Test aarch32_MCR_T1A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: CRm=0, cond=12, opc2=0, opc1=0, CRn=0, Rt=0, coproc=0
    let encoding: u32 = 0xCE000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_mcr_t1a1_a_field_cond_13_poweroftwo_e10_de000e10() {
    // Encoding: 0xDE000E10
    // Test aarch32_MCR_T1A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: coproc=0, CRm=0, opc1=0, CRn=0, opc2=0, cond=13, Rt=0
    let encoding: u32 = 0xDE000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_mcr_t1a1_a_field_cond_14_poweroftwo_e10_ee000e10() {
    // Encoding: 0xEE000E10
    // Test aarch32_MCR_T1A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: CRm=0, Rt=0, opc1=0, cond=14, CRn=0, coproc=0, opc2=0
    let encoding: u32 = 0xEE000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_mcr_t1a1_a_field_cond_15_max_e10_fe000e10() {
    // Encoding: 0xFE000E10
    // Test aarch32_MCR_T1A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: CRn=0, coproc=0, cond=15, opc2=0, CRm=0, Rt=0, opc1=0
    let encoding: u32 = 0xFE000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field opc1 21 +: 3`
/// Requirement: FieldBoundary { field: "opc1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mcr_t1a1_a_field_opc1_0_min_e10_0e000e10() {
    // Encoding: 0x0E000E10
    // Test aarch32_MCR_T1A1_A field opc1 = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rt=0, CRn=0, coproc=0, opc2=0, opc1=0, CRm=0
    let encoding: u32 = 0x0E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field opc1 21 +: 3`
/// Requirement: FieldBoundary { field: "opc1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mcr_t1a1_a_field_opc1_1_poweroftwo_e10_0e200e10() {
    // Encoding: 0x0E200E10
    // Test aarch32_MCR_T1A1_A field opc1 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: coproc=0, opc2=0, CRm=0, Rt=0, CRn=0, cond=0, opc1=1
    let encoding: u32 = 0x0E200E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field opc1 21 +: 3`
/// Requirement: FieldBoundary { field: "opc1", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch32_mcr_t1a1_a_field_opc1_7_max_e10_0ee00e10() {
    // Encoding: 0x0EE00E10
    // Test aarch32_MCR_T1A1_A field opc1 = 7 (Max)
    // ISET: A32
    // Fields: CRm=0, cond=0, Rt=0, coproc=0, CRn=0, opc1=7, opc2=0
    let encoding: u32 = 0x0EE00E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field CRn 16 +: 4`
/// Requirement: FieldBoundary { field: "CRn", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mcr_t1a1_a_field_crn_0_min_e10_0e000e10() {
    // Encoding: 0x0E000E10
    // Test aarch32_MCR_T1A1_A field CRn = 0 (Min)
    // ISET: A32
    // Fields: cond=0, coproc=0, opc2=0, CRm=0, opc1=0, CRn=0, Rt=0
    let encoding: u32 = 0x0E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field CRn 16 +: 4`
/// Requirement: FieldBoundary { field: "CRn", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mcr_t1a1_a_field_crn_1_poweroftwo_e10_0e010e10() {
    // Encoding: 0x0E010E10
    // Test aarch32_MCR_T1A1_A field CRn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: opc1=0, opc2=0, CRm=0, cond=0, Rt=0, coproc=0, CRn=1
    let encoding: u32 = 0x0E010E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field CRn 16 +: 4`
/// Requirement: FieldBoundary { field: "CRn", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch32_mcr_t1a1_a_field_crn_7_poweroftwominusone_e10_0e070e10() {
    // Encoding: 0x0E070E10
    // Test aarch32_MCR_T1A1_A field CRn = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: CRn=7, coproc=0, Rt=0, opc1=0, cond=0, opc2=0, CRm=0
    let encoding: u32 = 0x0E070E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field CRn 16 +: 4`
/// Requirement: FieldBoundary { field: "CRn", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch32_mcr_t1a1_a_field_crn_15_max_e10_0e0f0e10() {
    // Encoding: 0x0E0F0E10
    // Test aarch32_MCR_T1A1_A field CRn = 15 (Max)
    // ISET: A32
    // Fields: opc2=0, CRm=0, Rt=0, coproc=0, opc1=0, cond=0, CRn=15
    let encoding: u32 = 0x0E0F0E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field Rt 12 +: 4`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mcr_t1a1_a_field_rt_0_min_e10_0e000e10() {
    // Encoding: 0x0E000E10
    // Test aarch32_MCR_T1A1_A field Rt = 0 (Min)
    // ISET: A32
    // Fields: CRn=0, Rt=0, opc2=0, opc1=0, cond=0, coproc=0, CRm=0
    let encoding: u32 = 0x0E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field Rt 12 +: 4`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mcr_t1a1_a_field_rt_1_poweroftwo_e10_0e001e10() {
    // Encoding: 0x0E001E10
    // Test aarch32_MCR_T1A1_A field Rt = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: opc1=0, cond=0, CRn=0, CRm=0, opc2=0, Rt=1, coproc=0
    let encoding: u32 = 0x0E001E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field coproc 9 +: 0`
/// Requirement: FieldBoundary { field: "coproc", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mcr_t1a1_a_field_coproc_0_min_e10_0e000e10() {
    // Encoding: 0x0E000E10
    // Test aarch32_MCR_T1A1_A field coproc = 0 (Min)
    // ISET: A32
    // Fields: opc1=0, Rt=0, coproc=0, opc2=0, CRm=0, cond=0, CRn=0
    let encoding: u32 = 0x0E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field opc2 5 +: 3`
/// Requirement: FieldBoundary { field: "opc2", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mcr_t1a1_a_field_opc2_0_min_e10_0e000e10() {
    // Encoding: 0x0E000E10
    // Test aarch32_MCR_T1A1_A field opc2 = 0 (Min)
    // ISET: A32
    // Fields: cond=0, coproc=0, Rt=0, CRn=0, opc1=0, opc2=0, CRm=0
    let encoding: u32 = 0x0E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field opc2 5 +: 3`
/// Requirement: FieldBoundary { field: "opc2", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mcr_t1a1_a_field_opc2_1_poweroftwo_e10_0e000e30() {
    // Encoding: 0x0E000E30
    // Test aarch32_MCR_T1A1_A field opc2 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, CRn=0, Rt=0, opc1=0, opc2=1, CRm=0, coproc=0
    let encoding: u32 = 0x0E000E30;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field opc2 5 +: 3`
/// Requirement: FieldBoundary { field: "opc2", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch32_mcr_t1a1_a_field_opc2_7_max_e10_0e000ef0() {
    // Encoding: 0x0E000EF0
    // Test aarch32_MCR_T1A1_A field opc2 = 7 (Max)
    // ISET: A32
    // Fields: CRm=0, Rt=0, opc2=7, cond=0, CRn=0, coproc=0, opc1=0
    let encoding: u32 = 0x0E000EF0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mcr_t1a1_a_field_crm_0_min_e10_0e000e10() {
    // Encoding: 0x0E000E10
    // Test aarch32_MCR_T1A1_A field CRm = 0 (Min)
    // ISET: A32
    // Fields: opc2=0, CRm=0, opc1=0, cond=0, CRn=0, coproc=0, Rt=0
    let encoding: u32 = 0x0E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mcr_t1a1_a_field_crm_1_poweroftwo_e10_0e000e11() {
    // Encoding: 0x0E000E11
    // Test aarch32_MCR_T1A1_A field CRm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: opc1=0, Rt=0, CRm=1, coproc=0, cond=0, opc2=0, CRn=0
    let encoding: u32 = 0x0E000E11;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch32_mcr_t1a1_a_field_crm_7_poweroftwominusone_e10_0e000e17() {
    // Encoding: 0x0E000E17
    // Test aarch32_MCR_T1A1_A field CRm = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, Rt=0, opc1=0, opc2=0, CRm=7, coproc=0, CRn=0
    let encoding: u32 = 0x0E000E17;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch32_mcr_t1a1_a_field_crm_15_max_e10_0e000e1f() {
    // Encoding: 0x0E000E1F
    // Test aarch32_MCR_T1A1_A field CRm = 15 (Max)
    // ISET: A32
    // Fields: coproc=0, opc1=0, cond=0, CRn=0, opc2=0, CRm=15, Rt=0
    let encoding: u32 = 0x0E000E1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_mcr_t1a1_a_combo_0_e10_0e000e10() {
    // Encoding: 0x0E000E10
    // Test aarch32_MCR_T1A1_A field combination: cond=0, opc1=0, CRn=0, Rt=0, coproc=0, opc2=0, CRm=0
    // ISET: A32
    // Fields: opc2=0, CRm=0, CRn=0, cond=0, opc1=0, Rt=0, coproc=0
    let encoding: u32 = 0x0E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_mcr_t1a1_a_special_cond_0_condition_eq_3600_0e000e10() {
    // Encoding: 0x0E000E10
    // Test aarch32_MCR_T1A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: CRn=0, coproc=0, Rt=0, CRm=0, opc2=0, cond=0, opc1=0
    let encoding: u32 = 0x0E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_mcr_t1a1_a_special_cond_1_condition_ne_3600_1e000e10() {
    // Encoding: 0x1E000E10
    // Test aarch32_MCR_T1A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: opc1=0, coproc=0, opc2=0, Rt=0, cond=1, CRm=0, CRn=0
    let encoding: u32 = 0x1E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_mcr_t1a1_a_special_cond_2_condition_cs_hs_3600_2e000e10() {
    // Encoding: 0x2E000E10
    // Test aarch32_MCR_T1A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, CRn=0, CRm=0, Rt=0, coproc=0, opc2=0, opc1=0
    let encoding: u32 = 0x2E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_mcr_t1a1_a_special_cond_3_condition_cc_lo_3600_3e000e10() {
    // Encoding: 0x3E000E10
    // Test aarch32_MCR_T1A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, opc1=0, CRn=0, Rt=0, opc2=0, CRm=0, coproc=0
    let encoding: u32 = 0x3E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_mcr_t1a1_a_special_cond_4_condition_mi_3600_4e000e10() {
    // Encoding: 0x4E000E10
    // Test aarch32_MCR_T1A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rt=0, coproc=0, CRm=0, opc1=0, opc2=0, CRn=0, cond=4
    let encoding: u32 = 0x4E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_mcr_t1a1_a_special_cond_5_condition_pl_3600_5e000e10() {
    // Encoding: 0x5E000E10
    // Test aarch32_MCR_T1A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: opc2=0, coproc=0, CRn=0, Rt=0, CRm=0, cond=5, opc1=0
    let encoding: u32 = 0x5E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_mcr_t1a1_a_special_cond_6_condition_vs_3600_6e000e10() {
    // Encoding: 0x6E000E10
    // Test aarch32_MCR_T1A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: CRm=0, coproc=0, CRn=0, cond=6, opc1=0, Rt=0, opc2=0
    let encoding: u32 = 0x6E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_mcr_t1a1_a_special_cond_7_condition_vc_3600_7e000e10() {
    // Encoding: 0x7E000E10
    // Test aarch32_MCR_T1A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: opc1=0, CRn=0, coproc=0, CRm=0, Rt=0, opc2=0, cond=7
    let encoding: u32 = 0x7E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_mcr_t1a1_a_special_cond_8_condition_hi_3600_8e000e10() {
    // Encoding: 0x8E000E10
    // Test aarch32_MCR_T1A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rt=0, CRn=0, opc2=0, CRm=0, opc1=0, cond=8, coproc=0
    let encoding: u32 = 0x8E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_mcr_t1a1_a_special_cond_9_condition_ls_3600_9e000e10() {
    // Encoding: 0x9E000E10
    // Test aarch32_MCR_T1A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: CRm=0, opc1=0, coproc=0, opc2=0, Rt=0, CRn=0, cond=9
    let encoding: u32 = 0x9E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_mcr_t1a1_a_special_cond_10_condition_ge_3600_ae000e10() {
    // Encoding: 0xAE000E10
    // Test aarch32_MCR_T1A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, Rt=0, opc1=0, opc2=0, CRm=0, coproc=0, CRn=0
    let encoding: u32 = 0xAE000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_mcr_t1a1_a_special_cond_11_condition_lt_3600_be000e10() {
    // Encoding: 0xBE000E10
    // Test aarch32_MCR_T1A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: opc2=0, CRm=0, opc1=0, cond=11, Rt=0, coproc=0, CRn=0
    let encoding: u32 = 0xBE000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_mcr_t1a1_a_special_cond_12_condition_gt_3600_ce000e10() {
    // Encoding: 0xCE000E10
    // Test aarch32_MCR_T1A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: coproc=0, cond=12, opc1=0, Rt=0, opc2=0, CRm=0, CRn=0
    let encoding: u32 = 0xCE000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_mcr_t1a1_a_special_cond_13_condition_le_3600_de000e10() {
    // Encoding: 0xDE000E10
    // Test aarch32_MCR_T1A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: opc1=0, CRn=0, CRm=0, opc2=0, cond=13, Rt=0, coproc=0
    let encoding: u32 = 0xDE000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_mcr_t1a1_a_special_cond_14_condition_al_3600_ee000e10() {
    // Encoding: 0xEE000E10
    // Test aarch32_MCR_T1A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: CRm=0, CRn=0, opc1=0, coproc=0, Rt=0, cond=14, opc2=0
    let encoding: u32 = 0xEE000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_mcr_t1a1_a_special_cond_15_condition_nv_3600_fe000e10() {
    // Encoding: 0xFE000E10
    // Test aarch32_MCR_T1A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: opc1=0, CRn=0, Rt=0, coproc=0, cond=15, opc2=0, CRm=0
    let encoding: u32 = 0xFE000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "t" }), rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"t\" }), rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mcr_t1a1_a_invalid_0_e10_0e000e10() {
    // Encoding: 0x0E000E10
    // Test aarch32_MCR_T1A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "t" }), rhs: LitInt(15) }
    // ISET: A32
    // Fields: opc1=0, CRn=0, Rt=0, coproc=0, opc2=0, CRm=0, cond=0
    let encoding: u32 = 0x0E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mcr_t1a1_a_invalid_1_e10_0e000e10() {
    // Encoding: 0x0E000E10
    // Test aarch32_MCR_T1A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rt=0, cond=0, CRn=0, coproc=0, opc2=0, CRm=0, opc1=0
    let encoding: u32 = 0x0E000E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field opc1 21 +: 3`
/// Requirement: FieldBoundary { field: "opc1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mcr_t1a1_a_field_opc1_0_min_e10_ee000e10() {
    // Thumb encoding (32): 0xEE000E10
    // Test aarch32_MCR_T1A1_A field opc1 = 0 (Min)
    // ISET: T32
    // Fields: opc1=0, CRn=0, coproc=0, CRm=0, opc2=0, Rt=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE000E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field opc1 21 +: 3`
/// Requirement: FieldBoundary { field: "opc1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mcr_t1a1_a_field_opc1_1_poweroftwo_e10_ee200e10() {
    // Thumb encoding (32): 0xEE200E10
    // Test aarch32_MCR_T1A1_A field opc1 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: CRm=0, Rt=0, coproc=0, opc2=0, opc1=1, CRn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE200E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field opc1 21 +: 3`
/// Requirement: FieldBoundary { field: "opc1", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch32_mcr_t1a1_a_field_opc1_7_max_e10_eee00e10() {
    // Thumb encoding (32): 0xEEE00E10
    // Test aarch32_MCR_T1A1_A field opc1 = 7 (Max)
    // ISET: T32
    // Fields: CRn=0, opc2=0, CRm=0, opc1=7, Rt=0, coproc=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEE00E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field CRn 16 +: 4`
/// Requirement: FieldBoundary { field: "CRn", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mcr_t1a1_a_field_crn_0_min_e10_ee000e10() {
    // Thumb encoding (32): 0xEE000E10
    // Test aarch32_MCR_T1A1_A field CRn = 0 (Min)
    // ISET: T32
    // Fields: opc1=0, CRn=0, CRm=0, coproc=0, opc2=0, Rt=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE000E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field CRn 16 +: 4`
/// Requirement: FieldBoundary { field: "CRn", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mcr_t1a1_a_field_crn_1_poweroftwo_e10_ee010e10() {
    // Thumb encoding (32): 0xEE010E10
    // Test aarch32_MCR_T1A1_A field CRn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: opc1=0, CRn=1, coproc=0, opc2=0, CRm=0, Rt=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE010E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field CRn 16 +: 4`
/// Requirement: FieldBoundary { field: "CRn", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch32_mcr_t1a1_a_field_crn_7_poweroftwominusone_e10_ee070e10() {
    // Thumb encoding (32): 0xEE070E10
    // Test aarch32_MCR_T1A1_A field CRn = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: opc1=0, coproc=0, Rt=0, CRn=7, opc2=0, CRm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE070E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field CRn 16 +: 4`
/// Requirement: FieldBoundary { field: "CRn", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch32_mcr_t1a1_a_field_crn_15_max_e10_ee0f0e10() {
    // Thumb encoding (32): 0xEE0F0E10
    // Test aarch32_MCR_T1A1_A field CRn = 15 (Max)
    // ISET: T32
    // Fields: opc2=0, Rt=0, coproc=0, CRm=0, opc1=0, CRn=15
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE0F0E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field Rt 12 +: 4`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mcr_t1a1_a_field_rt_0_min_e10_ee000e10() {
    // Thumb encoding (32): 0xEE000E10
    // Test aarch32_MCR_T1A1_A field Rt = 0 (Min)
    // ISET: T32
    // Fields: CRn=0, opc1=0, Rt=0, coproc=0, opc2=0, CRm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE000E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field Rt 12 +: 4`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mcr_t1a1_a_field_rt_1_poweroftwo_e10_ee001e10() {
    // Thumb encoding (32): 0xEE001E10
    // Test aarch32_MCR_T1A1_A field Rt = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rt=1, opc1=0, opc2=0, CRm=0, CRn=0, coproc=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE001E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field coproc 9 +: 0`
/// Requirement: FieldBoundary { field: "coproc", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mcr_t1a1_a_field_coproc_0_min_e10_ee000e10() {
    // Thumb encoding (32): 0xEE000E10
    // Test aarch32_MCR_T1A1_A field coproc = 0 (Min)
    // ISET: T32
    // Fields: CRm=0, Rt=0, CRn=0, opc1=0, coproc=0, opc2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE000E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field opc2 5 +: 3`
/// Requirement: FieldBoundary { field: "opc2", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mcr_t1a1_a_field_opc2_0_min_e10_ee000e10() {
    // Thumb encoding (32): 0xEE000E10
    // Test aarch32_MCR_T1A1_A field opc2 = 0 (Min)
    // ISET: T32
    // Fields: opc1=0, CRn=0, coproc=0, Rt=0, opc2=0, CRm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE000E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field opc2 5 +: 3`
/// Requirement: FieldBoundary { field: "opc2", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mcr_t1a1_a_field_opc2_1_poweroftwo_e10_ee000e30() {
    // Thumb encoding (32): 0xEE000E30
    // Test aarch32_MCR_T1A1_A field opc2 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: coproc=0, CRm=0, opc1=0, opc2=1, CRn=0, Rt=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE000E30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field opc2 5 +: 3`
/// Requirement: FieldBoundary { field: "opc2", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch32_mcr_t1a1_a_field_opc2_7_max_e10_ee000ef0() {
    // Thumb encoding (32): 0xEE000EF0
    // Test aarch32_MCR_T1A1_A field opc2 = 7 (Max)
    // ISET: T32
    // Fields: opc2=7, opc1=0, CRn=0, CRm=0, Rt=0, coproc=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE000EF0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mcr_t1a1_a_field_crm_0_min_e10_ee000e10() {
    // Thumb encoding (32): 0xEE000E10
    // Test aarch32_MCR_T1A1_A field CRm = 0 (Min)
    // ISET: T32
    // Fields: CRm=0, opc1=0, opc2=0, Rt=0, coproc=0, CRn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE000E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mcr_t1a1_a_field_crm_1_poweroftwo_e10_ee000e11() {
    // Thumb encoding (32): 0xEE000E11
    // Test aarch32_MCR_T1A1_A field CRm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: CRm=1, CRn=0, opc1=0, Rt=0, coproc=0, opc2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE000E11;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch32_mcr_t1a1_a_field_crm_7_poweroftwominusone_e10_ee000e17() {
    // Thumb encoding (32): 0xEE000E17
    // Test aarch32_MCR_T1A1_A field CRm = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: opc1=0, coproc=0, CRm=7, CRn=0, opc2=0, Rt=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE000E17;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch32_mcr_t1a1_a_field_crm_15_max_e10_ee000e1f() {
    // Thumb encoding (32): 0xEE000E1F
    // Test aarch32_MCR_T1A1_A field CRm = 15 (Max)
    // ISET: T32
    // Fields: CRm=15, coproc=0, CRn=0, opc1=0, opc2=0, Rt=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE000E1F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc1=0 (minimum value)
#[test]
fn test_aarch32_mcr_t1a1_a_combo_0_e10_ee000e10() {
    // Thumb encoding (32): 0xEE000E10
    // Test aarch32_MCR_T1A1_A field combination: opc1=0, CRn=0, Rt=0, coproc=0, opc2=0, CRm=0
    // ISET: T32
    // Fields: Rt=0, coproc=0, CRn=0, opc2=0, CRm=0, opc1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE000E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "t" }), rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"t\" }), rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mcr_t1a1_a_invalid_0_e10_ee000e10() {
    // Thumb encoding (32): 0xEE000E10
    // Test aarch32_MCR_T1A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "t" }), rhs: LitInt(15) }
    // ISET: T32
    // Fields: coproc=0, opc1=0, CRn=0, Rt=0, CRm=0, opc2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE000E10;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_MCR_T1A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mcr_t1a1_a_invalid_1_e10_ee000e10() {
    // Thumb encoding (32): 0xEE000E10
    // Test aarch32_MCR_T1A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: CRn=0, coproc=0, opc2=0, CRm=0, Rt=0, opc1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEE000E10;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_LDC_i_A Tests
// ============================================================================

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_cond_0_min_5e00_0c105e00() {
    // Encoding: 0x0C105E00
    // Test aarch32_LDC_i_T1A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: W=0, P=0, cond=0, Rn=0, U=0, imm8=0
    let encoding: u32 = 0x0C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_cond_1_poweroftwo_5e00_1c105e00() {
    // Encoding: 0x1C105E00
    // Test aarch32_LDC_i_T1A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: U=0, Rn=0, W=0, cond=1, imm8=0, P=0
    let encoding: u32 = 0x1C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_cond_2_poweroftwo_5e00_2c105e00() {
    // Encoding: 0x2C105E00
    // Test aarch32_LDC_i_T1A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm8=0, U=0, P=0, cond=2, W=0
    let encoding: u32 = 0x2C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_cond_3_poweroftwo_5e00_3c105e00() {
    // Encoding: 0x3C105E00
    // Test aarch32_LDC_i_T1A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: imm8=0, cond=3, P=0, Rn=0, U=0, W=0
    let encoding: u32 = 0x3C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_cond_4_poweroftwo_5e00_4c105e00() {
    // Encoding: 0x4C105E00
    // Test aarch32_LDC_i_T1A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=4, U=0, P=0, W=0, Rn=0, imm8=0
    let encoding: u32 = 0x4C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_cond_5_poweroftwo_5e00_5c105e00() {
    // Encoding: 0x5C105E00
    // Test aarch32_LDC_i_T1A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=5, W=0, P=0, U=0, Rn=0, imm8=0
    let encoding: u32 = 0x5C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_cond_6_poweroftwo_5e00_6c105e00() {
    // Encoding: 0x6C105E00
    // Test aarch32_LDC_i_T1A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: P=0, W=0, cond=6, Rn=0, U=0, imm8=0
    let encoding: u32 = 0x6C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_cond_7_poweroftwo_5e00_7c105e00() {
    // Encoding: 0x7C105E00
    // Test aarch32_LDC_i_T1A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm8=0, cond=7, P=0, U=0, W=0
    let encoding: u32 = 0x7C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_cond_8_poweroftwo_5e00_8c105e00() {
    // Encoding: 0x8C105E00
    // Test aarch32_LDC_i_T1A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: imm8=0, U=0, cond=8, W=0, P=0, Rn=0
    let encoding: u32 = 0x8C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_cond_9_poweroftwo_5e00_9c105e00() {
    // Encoding: 0x9C105E00
    // Test aarch32_LDC_i_T1A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, Rn=0, W=0, imm8=0, P=0, U=0
    let encoding: u32 = 0x9C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_cond_10_poweroftwo_5e00_ac105e00() {
    // Encoding: 0xAC105E00
    // Test aarch32_LDC_i_T1A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: U=0, P=0, W=0, imm8=0, Rn=0, cond=10
    let encoding: u32 = 0xAC105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_cond_11_poweroftwo_5e00_bc105e00() {
    // Encoding: 0xBC105E00
    // Test aarch32_LDC_i_T1A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, cond=11, U=0, imm8=0, P=0, W=0
    let encoding: u32 = 0xBC105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_cond_12_poweroftwo_5e00_cc105e00() {
    // Encoding: 0xCC105E00
    // Test aarch32_LDC_i_T1A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, imm8=0, U=0, W=0, P=0, Rn=0
    let encoding: u32 = 0xCC105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_cond_13_poweroftwo_5e00_dc105e00() {
    // Encoding: 0xDC105E00
    // Test aarch32_LDC_i_T1A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: imm8=0, U=0, P=0, cond=13, W=0, Rn=0
    let encoding: u32 = 0xDC105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_cond_14_poweroftwo_5e00_ec105e00() {
    // Encoding: 0xEC105E00
    // Test aarch32_LDC_i_T1A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: P=0, cond=14, W=0, imm8=0, Rn=0, U=0
    let encoding: u32 = 0xEC105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_cond_15_max_5e00_fc105e00() {
    // Encoding: 0xFC105E00
    // Test aarch32_LDC_i_T1A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, P=0, W=0, Rn=0, imm8=0, U=0
    let encoding: u32 = 0xFC105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field P 24 +: 1`
/// Requirement: FieldBoundary { field: "P", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_p_0_min_5e00_0c105e00() {
    // Encoding: 0x0C105E00
    // Test aarch32_LDC_i_T1A1_A field P = 0 (Min)
    // ISET: A32
    // Fields: Rn=0, P=0, W=0, U=0, imm8=0, cond=0
    let encoding: u32 = 0x0C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field P 24 +: 1`
/// Requirement: FieldBoundary { field: "P", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_p_1_max_5e00_0d105e00() {
    // Encoding: 0x0D105E00
    // Test aarch32_LDC_i_T1A1_A field P = 1 (Max)
    // ISET: A32
    // Fields: P=1, W=0, imm8=0, U=0, cond=0, Rn=0
    let encoding: u32 = 0x0D105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field U 23 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_u_0_min_5e00_0c105e00() {
    // Encoding: 0x0C105E00
    // Test aarch32_LDC_i_T1A1_A field U = 0 (Min)
    // ISET: A32
    // Fields: P=0, W=0, Rn=0, U=0, imm8=0, cond=0
    let encoding: u32 = 0x0C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field U 23 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_u_1_max_5e00_0c905e00() {
    // Encoding: 0x0C905E00
    // Test aarch32_LDC_i_T1A1_A field U = 1 (Max)
    // ISET: A32
    // Fields: U=1, P=0, cond=0, W=0, Rn=0, imm8=0
    let encoding: u32 = 0x0C905E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field W 21 +: 1`
/// Requirement: FieldBoundary { field: "W", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_w_0_min_5e00_0c105e00() {
    // Encoding: 0x0C105E00
    // Test aarch32_LDC_i_T1A1_A field W = 0 (Min)
    // ISET: A32
    // Fields: imm8=0, W=0, Rn=0, cond=0, P=0, U=0
    let encoding: u32 = 0x0C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field W 21 +: 1`
/// Requirement: FieldBoundary { field: "W", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_w_1_max_5e00_0c305e00() {
    // Encoding: 0x0C305E00
    // Test aarch32_LDC_i_T1A1_A field W = 1 (Max)
    // ISET: A32
    // Fields: Rn=0, cond=0, W=1, imm8=0, P=0, U=0
    let encoding: u32 = 0x0C305E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_rn_0_min_5e00_0c105e00() {
    // Encoding: 0x0C105E00
    // Test aarch32_LDC_i_T1A1_A field Rn = 0 (Min)
    // ISET: A32
    // Fields: W=0, U=0, Rn=0, imm8=0, cond=0, P=0
    let encoding: u32 = 0x0C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_rn_1_poweroftwo_5e00_0c115e00() {
    // Encoding: 0x0C115E00
    // Test aarch32_LDC_i_T1A1_A field Rn = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: P=0, Rn=1, imm8=0, U=0, cond=0, W=0
    let encoding: u32 = 0x0C115E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_0_zero_5e00_0c105e00() {
    // Encoding: 0x0C105E00
    // Test aarch32_LDC_i_T1A1_A field imm8 = 0 (Zero)
    // ISET: A32
    // Fields: imm8=0, P=0, cond=0, U=0, W=0, Rn=0
    let encoding: u32 = 0x0C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_1_poweroftwo_5e00_0c105e01() {
    // Encoding: 0x0C105E01
    // Test aarch32_LDC_i_T1A1_A field imm8 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: U=0, cond=0, W=0, Rn=0, imm8=1, P=0
    let encoding: u32 = 0x0C105E01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_3_poweroftwominusone_5e00_0c105e03() {
    // Encoding: 0x0C105E03
    // Test aarch32_LDC_i_T1A1_A field imm8 = 3 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm8=3, P=0, U=0, W=0, Rn=0
    let encoding: u32 = 0x0C105E03;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_4_poweroftwo_5e00_0c105e04() {
    // Encoding: 0x0C105E04
    // Test aarch32_LDC_i_T1A1_A field imm8 = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: W=0, U=0, cond=0, Rn=0, imm8=4, P=0
    let encoding: u32 = 0x0C105E04;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_7_poweroftwominusone_5e00_0c105e07() {
    // Encoding: 0x0C105E07
    // Test aarch32_LDC_i_T1A1_A field imm8 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: W=0, Rn=0, imm8=7, cond=0, U=0, P=0
    let encoding: u32 = 0x0C105E07;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_8_poweroftwo_5e00_0c105e08() {
    // Encoding: 0x0C105E08
    // Test aarch32_LDC_i_T1A1_A field imm8 = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: U=0, cond=0, Rn=0, imm8=8, W=0, P=0
    let encoding: u32 = 0x0C105E08;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_15_poweroftwominusone_5e00_0c105e0f() {
    // Encoding: 0x0C105E0F
    // Test aarch32_LDC_i_T1A1_A field imm8 = 15 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: U=0, W=0, Rn=0, imm8=15, P=0, cond=0
    let encoding: u32 = 0x0C105E0F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_16_poweroftwo_5e00_0c105e10() {
    // Encoding: 0x0C105E10
    // Test aarch32_LDC_i_T1A1_A field imm8 = 16 (PowerOfTwo)
    // ISET: A32
    // Fields: P=0, cond=0, Rn=0, imm8=16, W=0, U=0
    let encoding: u32 = 0x0C105E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_31_poweroftwominusone_5e00_0c105e1f() {
    // Encoding: 0x0C105E1F
    // Test aarch32_LDC_i_T1A1_A field imm8 = 31 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: Rn=0, imm8=31, P=0, cond=0, U=0, W=0
    let encoding: u32 = 0x0C105E1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_32_poweroftwo_5e00_0c105e20() {
    // Encoding: 0x0C105E20
    // Test aarch32_LDC_i_T1A1_A field imm8 = 32 (PowerOfTwo)
    // ISET: A32
    // Fields: U=0, Rn=0, imm8=32, cond=0, W=0, P=0
    let encoding: u32 = 0x0C105E20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_63_poweroftwominusone_5e00_0c105e3f() {
    // Encoding: 0x0C105E3F
    // Test aarch32_LDC_i_T1A1_A field imm8 = 63 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: W=0, cond=0, U=0, Rn=0, P=0, imm8=63
    let encoding: u32 = 0x0C105E3F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_64_poweroftwo_5e00_0c105e40() {
    // Encoding: 0x0C105E40
    // Test aarch32_LDC_i_T1A1_A field imm8 = 64 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm8=64, U=0, P=0, W=0, Rn=0
    let encoding: u32 = 0x0C105E40;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 127, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (127)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_127_poweroftwominusone_5e00_0c105e7f() {
    // Encoding: 0x0C105E7F
    // Test aarch32_LDC_i_T1A1_A field imm8 = 127 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, P=0, imm8=127, Rn=0, U=0, W=0
    let encoding: u32 = 0x0C105E7F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_128_poweroftwo_5e00_0c105e80() {
    // Encoding: 0x0C105E80
    // Test aarch32_LDC_i_T1A1_A field imm8 = 128 (PowerOfTwo)
    // ISET: A32
    // Fields: Rn=0, imm8=128, P=0, cond=0, U=0, W=0
    let encoding: u32 = 0x0C105E80;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 255, boundary: Max }
/// maximum immediate (255)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_255_max_5e00_0c105eff() {
    // Encoding: 0x0C105EFF
    // Test aarch32_LDC_i_T1A1_A field imm8 = 255 (Max)
    // ISET: A32
    // Fields: W=0, U=0, Rn=0, imm8=255, cond=0, P=0
    let encoding: u32 = 0x0C105EFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_ldc_i_t1a1_a_combo_0_5e00_0c105e00() {
    // Encoding: 0x0C105E00
    // Test aarch32_LDC_i_T1A1_A field combination: cond=0, P=0, U=0, W=0, Rn=0, imm8=0
    // ISET: A32
    // Fields: Rn=0, U=0, P=0, W=0, imm8=0, cond=0
    let encoding: u32 = 0x0C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_ldc_i_t1a1_a_special_cond_0_condition_eq_24064_0c105e00() {
    // Encoding: 0x0C105E00
    // Test aarch32_LDC_i_T1A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: imm8=0, P=0, Rn=0, cond=0, W=0, U=0
    let encoding: u32 = 0x0C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_ldc_i_t1a1_a_special_cond_1_condition_ne_24064_1c105e00() {
    // Encoding: 0x1C105E00
    // Test aarch32_LDC_i_T1A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: W=0, Rn=0, P=0, imm8=0, U=0, cond=1
    let encoding: u32 = 0x1C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_ldc_i_t1a1_a_special_cond_2_condition_cs_hs_24064_2c105e00() {
    // Encoding: 0x2C105E00
    // Test aarch32_LDC_i_T1A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Rn=0, imm8=0, cond=2, U=0, W=0, P=0
    let encoding: u32 = 0x2C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_ldc_i_t1a1_a_special_cond_3_condition_cc_lo_24064_3c105e00() {
    // Encoding: 0x3C105E00
    // Test aarch32_LDC_i_T1A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: P=0, W=0, Rn=0, cond=3, imm8=0, U=0
    let encoding: u32 = 0x3C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_ldc_i_t1a1_a_special_cond_4_condition_mi_24064_4c105e00() {
    // Encoding: 0x4C105E00
    // Test aarch32_LDC_i_T1A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, W=0, imm8=0, P=0, Rn=0, U=0
    let encoding: u32 = 0x4C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_ldc_i_t1a1_a_special_cond_5_condition_pl_24064_5c105e00() {
    // Encoding: 0x5C105E00
    // Test aarch32_LDC_i_T1A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: imm8=0, U=0, P=0, cond=5, W=0, Rn=0
    let encoding: u32 = 0x5C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_ldc_i_t1a1_a_special_cond_6_condition_vs_24064_6c105e00() {
    // Encoding: 0x6C105E00
    // Test aarch32_LDC_i_T1A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: imm8=0, P=0, U=0, W=0, Rn=0, cond=6
    let encoding: u32 = 0x6C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_ldc_i_t1a1_a_special_cond_7_condition_vc_24064_7c105e00() {
    // Encoding: 0x7C105E00
    // Test aarch32_LDC_i_T1A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: U=0, W=0, P=0, Rn=0, cond=7, imm8=0
    let encoding: u32 = 0x7C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_ldc_i_t1a1_a_special_cond_8_condition_hi_24064_8c105e00() {
    // Encoding: 0x8C105E00
    // Test aarch32_LDC_i_T1A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: P=0, W=0, Rn=0, U=0, imm8=0, cond=8
    let encoding: u32 = 0x8C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_ldc_i_t1a1_a_special_cond_9_condition_ls_24064_9c105e00() {
    // Encoding: 0x9C105E00
    // Test aarch32_LDC_i_T1A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: imm8=0, cond=9, W=0, P=0, U=0, Rn=0
    let encoding: u32 = 0x9C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_ldc_i_t1a1_a_special_cond_10_condition_ge_24064_ac105e00() {
    // Encoding: 0xAC105E00
    // Test aarch32_LDC_i_T1A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rn=0, P=0, W=0, imm8=0, cond=10, U=0
    let encoding: u32 = 0xAC105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_ldc_i_t1a1_a_special_cond_11_condition_lt_24064_bc105e00() {
    // Encoding: 0xBC105E00
    // Test aarch32_LDC_i_T1A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: imm8=0, P=0, U=0, cond=11, W=0, Rn=0
    let encoding: u32 = 0xBC105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_ldc_i_t1a1_a_special_cond_12_condition_gt_24064_cc105e00() {
    // Encoding: 0xCC105E00
    // Test aarch32_LDC_i_T1A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: W=0, P=0, cond=12, U=0, imm8=0, Rn=0
    let encoding: u32 = 0xCC105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_ldc_i_t1a1_a_special_cond_13_condition_le_24064_dc105e00() {
    // Encoding: 0xDC105E00
    // Test aarch32_LDC_i_T1A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: cond=13, P=0, Rn=0, W=0, imm8=0, U=0
    let encoding: u32 = 0xDC105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_ldc_i_t1a1_a_special_cond_14_condition_al_24064_ec105e00() {
    // Encoding: 0xEC105E00
    // Test aarch32_LDC_i_T1A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: P=0, Rn=0, imm8=0, cond=14, U=0, W=0
    let encoding: u32 = 0xEC105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_ldc_i_t1a1_a_special_cond_15_condition_nv_24064_fc105e00() {
    // Encoding: 0xFC105E00
    // Test aarch32_LDC_i_T1A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: imm8=0, Rn=0, cond=15, W=0, U=0, P=0
    let encoding: u32 = 0xFC105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "P" }), rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "U" }) } }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "W" }) } }, rhs: LitBits([false]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"P\" }), rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"U\" }) } }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"W\" }) } }, rhs: LitBits([false]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_ldc_i_t1a1_a_invalid_0_5e00_0c105e00() {
    // Encoding: 0x0C105E00
    // Test aarch32_LDC_i_T1A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "P" }), rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "U" }) } }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "W" }) } }, rhs: LitBits([false]) }
    // ISET: A32
    // Fields: cond=0, Rn=0, imm8=0, P=0, W=0, U=0
    let encoding: u32 = 0x0C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_ldc_i_t1a1_a_invalid_1_5e00_0c105e00() {
    // Encoding: 0x0C105E00
    // Test aarch32_LDC_i_T1A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: U=0, W=0, cond=0, Rn=0, imm8=0, P=0
    let encoding: u32 = 0x0C105E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field P 24 +: 1`
/// Requirement: FieldBoundary { field: "P", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_p_0_min_5e00_ec105e00() {
    // Thumb encoding (32): 0xEC105E00
    // Test aarch32_LDC_i_T1A1_A field P = 0 (Min)
    // ISET: T32
    // Fields: U=0, imm8=0, Rn=0, W=0, P=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC105E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field P 24 +: 1`
/// Requirement: FieldBoundary { field: "P", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_p_1_max_5e00_ed105e00() {
    // Thumb encoding (32): 0xED105E00
    // Test aarch32_LDC_i_T1A1_A field P = 1 (Max)
    // ISET: T32
    // Fields: P=1, U=0, Rn=0, imm8=0, W=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xED105E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field U 23 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_u_0_min_5e00_ec105e00() {
    // Thumb encoding (32): 0xEC105E00
    // Test aarch32_LDC_i_T1A1_A field U = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, imm8=0, P=0, U=0, W=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC105E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field U 23 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_u_1_max_5e00_ec905e00() {
    // Thumb encoding (32): 0xEC905E00
    // Test aarch32_LDC_i_T1A1_A field U = 1 (Max)
    // ISET: T32
    // Fields: Rn=0, P=0, U=1, imm8=0, W=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC905E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field W 21 +: 1`
/// Requirement: FieldBoundary { field: "W", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_w_0_min_5e00_ec105e00() {
    // Thumb encoding (32): 0xEC105E00
    // Test aarch32_LDC_i_T1A1_A field W = 0 (Min)
    // ISET: T32
    // Fields: U=0, Rn=0, P=0, W=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC105E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field W 21 +: 1`
/// Requirement: FieldBoundary { field: "W", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_w_1_max_5e00_ec305e00() {
    // Thumb encoding (32): 0xEC305E00
    // Test aarch32_LDC_i_T1A1_A field W = 1 (Max)
    // ISET: T32
    // Fields: U=0, W=1, Rn=0, P=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC305E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_rn_0_min_5e00_ec105e00() {
    // Thumb encoding (32): 0xEC105E00
    // Test aarch32_LDC_i_T1A1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, U=0, P=0, imm8=0, W=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC105E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_rn_1_poweroftwo_5e00_ec115e00() {
    // Thumb encoding (32): 0xEC115E00
    // Test aarch32_LDC_i_T1A1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=1, U=0, W=0, imm8=0, P=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC115E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_0_zero_5e00_ec105e00() {
    // Thumb encoding (32): 0xEC105E00
    // Test aarch32_LDC_i_T1A1_A field imm8 = 0 (Zero)
    // ISET: T32
    // Fields: U=0, W=0, P=0, imm8=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC105E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_1_poweroftwo_5e00_ec105e01() {
    // Thumb encoding (32): 0xEC105E01
    // Test aarch32_LDC_i_T1A1_A field imm8 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: U=0, P=0, W=0, Rn=0, imm8=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC105E01;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_3_poweroftwominusone_5e00_ec105e03() {
    // Thumb encoding (32): 0xEC105E03
    // Test aarch32_LDC_i_T1A1_A field imm8 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: P=0, U=0, W=0, imm8=3, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC105E03;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_4_poweroftwo_5e00_ec105e04() {
    // Thumb encoding (32): 0xEC105E04
    // Test aarch32_LDC_i_T1A1_A field imm8 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, imm8=4, W=0, U=0, P=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC105E04;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_7_poweroftwominusone_5e00_ec105e07() {
    // Thumb encoding (32): 0xEC105E07
    // Test aarch32_LDC_i_T1A1_A field imm8 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: P=0, Rn=0, W=0, U=0, imm8=7
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC105E07;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_8_poweroftwo_5e00_ec105e08() {
    // Thumb encoding (32): 0xEC105E08
    // Test aarch32_LDC_i_T1A1_A field imm8 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=8, P=0, U=0, W=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC105E08;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_15_poweroftwominusone_5e00_ec105e0f() {
    // Thumb encoding (32): 0xEC105E0F
    // Test aarch32_LDC_i_T1A1_A field imm8 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rn=0, W=0, imm8=15, P=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC105E0F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_16_poweroftwo_5e00_ec105e10() {
    // Thumb encoding (32): 0xEC105E10
    // Test aarch32_LDC_i_T1A1_A field imm8 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, P=0, U=0, W=0, imm8=16
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC105E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_31_poweroftwominusone_5e00_ec105e1f() {
    // Thumb encoding (32): 0xEC105E1F
    // Test aarch32_LDC_i_T1A1_A field imm8 = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rn=0, U=0, W=0, P=0, imm8=31
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC105E1F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_32_poweroftwo_5e00_ec105e20() {
    // Thumb encoding (32): 0xEC105E20
    // Test aarch32_LDC_i_T1A1_A field imm8 = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: W=0, P=0, Rn=0, imm8=32, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC105E20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_63_poweroftwominusone_5e00_ec105e3f() {
    // Thumb encoding (32): 0xEC105E3F
    // Test aarch32_LDC_i_T1A1_A field imm8 = 63 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: P=0, W=0, Rn=0, imm8=63, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC105E3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_64_poweroftwo_5e00_ec105e40() {
    // Thumb encoding (32): 0xEC105E40
    // Test aarch32_LDC_i_T1A1_A field imm8 = 64 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, imm8=64, P=0, W=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC105E40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 127, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (127)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_127_poweroftwominusone_5e00_ec105e7f() {
    // Thumb encoding (32): 0xEC105E7F
    // Test aarch32_LDC_i_T1A1_A field imm8 = 127 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rn=0, imm8=127, U=0, W=0, P=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC105E7F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_128_poweroftwo_5e00_ec105e80() {
    // Thumb encoding (32): 0xEC105E80
    // Test aarch32_LDC_i_T1A1_A field imm8 = 128 (PowerOfTwo)
    // ISET: T32
    // Fields: W=0, P=0, Rn=0, U=0, imm8=128
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC105E80;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 255, boundary: Max }
/// maximum immediate (255)
#[test]
fn test_aarch32_ldc_i_t1a1_a_field_imm8_255_max_5e00_ec105eff() {
    // Thumb encoding (32): 0xEC105EFF
    // Test aarch32_LDC_i_T1A1_A field imm8 = 255 (Max)
    // ISET: T32
    // Fields: Rn=0, W=0, imm8=255, P=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC105EFF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// P=0 (minimum value)
#[test]
fn test_aarch32_ldc_i_t1a1_a_combo_0_5e00_ec105e00() {
    // Thumb encoding (32): 0xEC105E00
    // Test aarch32_LDC_i_T1A1_A field combination: P=0, U=0, W=0, Rn=0, imm8=0
    // ISET: T32
    // Fields: imm8=0, P=0, U=0, W=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC105E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "P" }), rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "U" }) } }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "W" }) } }, rhs: LitBits([false]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"P\" }), rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"U\" }) } }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"W\" }) } }, rhs: LitBits([false]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_ldc_i_t1a1_a_invalid_0_5e00_ec105e00() {
    // Thumb encoding (32): 0xEC105E00
    // Test aarch32_LDC_i_T1A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "P" }), rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "U" }) } }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "W" }) } }, rhs: LitBits([false]) }
    // ISET: T32
    // Fields: P=0, U=0, W=0, imm8=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC105E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_LDC_i_T1A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_ldc_i_t1a1_a_invalid_1_5e00_ec105e00() {
    // Thumb encoding (32): 0xEC105E00
    // Test aarch32_LDC_i_T1A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: W=0, P=0, imm8=0, U=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC105E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

// ============================================================================
// aarch32_MRRC_A Tests
// ============================================================================

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_cond_0_min_e00_0c500e00() {
    // Encoding: 0x0C500E00
    // Test aarch32_MRRC_T1A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rt2=0, CRm=0, opc1=0, cond=0, Rt=0, coproc=0
    let encoding: u32 = 0x0C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_cond_1_poweroftwo_e00_1c500e00() {
    // Encoding: 0x1C500E00
    // Test aarch32_MRRC_T1A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: CRm=0, opc1=0, cond=1, Rt2=0, coproc=0, Rt=0
    let encoding: u32 = 0x1C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_cond_2_poweroftwo_e00_2c500e00() {
    // Encoding: 0x2C500E00
    // Test aarch32_MRRC_T1A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: coproc=0, CRm=0, Rt=0, opc1=0, Rt2=0, cond=2
    let encoding: u32 = 0x2C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_cond_3_poweroftwo_e00_3c500e00() {
    // Encoding: 0x3C500E00
    // Test aarch32_MRRC_T1A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: opc1=0, Rt2=0, cond=3, CRm=0, Rt=0, coproc=0
    let encoding: u32 = 0x3C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_cond_4_poweroftwo_e00_4c500e00() {
    // Encoding: 0x4C500E00
    // Test aarch32_MRRC_T1A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: opc1=0, CRm=0, cond=4, Rt2=0, Rt=0, coproc=0
    let encoding: u32 = 0x4C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_cond_5_poweroftwo_e00_5c500e00() {
    // Encoding: 0x5C500E00
    // Test aarch32_MRRC_T1A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: opc1=0, CRm=0, Rt2=0, cond=5, Rt=0, coproc=0
    let encoding: u32 = 0x5C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_cond_6_poweroftwo_e00_6c500e00() {
    // Encoding: 0x6C500E00
    // Test aarch32_MRRC_T1A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rt2=0, cond=6, Rt=0, opc1=0, CRm=0, coproc=0
    let encoding: u32 = 0x6C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_cond_7_poweroftwo_e00_7c500e00() {
    // Encoding: 0x7C500E00
    // Test aarch32_MRRC_T1A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=7, Rt2=0, Rt=0, coproc=0, CRm=0, opc1=0
    let encoding: u32 = 0x7C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_cond_8_poweroftwo_e00_8c500e00() {
    // Encoding: 0x8C500E00
    // Test aarch32_MRRC_T1A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: opc1=0, CRm=0, cond=8, Rt2=0, Rt=0, coproc=0
    let encoding: u32 = 0x8C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_cond_9_poweroftwo_e00_9c500e00() {
    // Encoding: 0x9C500E00
    // Test aarch32_MRRC_T1A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, opc1=0, CRm=0, Rt2=0, Rt=0, coproc=0
    let encoding: u32 = 0x9C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_cond_10_poweroftwo_e00_ac500e00() {
    // Encoding: 0xAC500E00
    // Test aarch32_MRRC_T1A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: CRm=0, Rt=0, coproc=0, opc1=0, Rt2=0, cond=10
    let encoding: u32 = 0xAC500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_cond_11_poweroftwo_e00_bc500e00() {
    // Encoding: 0xBC500E00
    // Test aarch32_MRRC_T1A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rt2=0, Rt=0, CRm=0, cond=11, opc1=0, coproc=0
    let encoding: u32 = 0xBC500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_cond_12_poweroftwo_e00_cc500e00() {
    // Encoding: 0xCC500E00
    // Test aarch32_MRRC_T1A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, opc1=0, Rt2=0, coproc=0, CRm=0, Rt=0
    let encoding: u32 = 0xCC500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_cond_13_poweroftwo_e00_dc500e00() {
    // Encoding: 0xDC500E00
    // Test aarch32_MRRC_T1A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: CRm=0, cond=13, Rt=0, coproc=0, opc1=0, Rt2=0
    let encoding: u32 = 0xDC500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_cond_14_poweroftwo_e00_ec500e00() {
    // Encoding: 0xEC500E00
    // Test aarch32_MRRC_T1A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: coproc=0, cond=14, Rt=0, Rt2=0, opc1=0, CRm=0
    let encoding: u32 = 0xEC500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_cond_15_max_e00_fc500e00() {
    // Encoding: 0xFC500E00
    // Test aarch32_MRRC_T1A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rt=0, CRm=0, coproc=0, Rt2=0, cond=15, opc1=0
    let encoding: u32 = 0xFC500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field Rt2 16 +: 4`
/// Requirement: FieldBoundary { field: "Rt2", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_rt2_0_min_e00_0c500e00() {
    // Encoding: 0x0C500E00
    // Test aarch32_MRRC_T1A1_A field Rt2 = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rt=0, CRm=0, Rt2=0, coproc=0, opc1=0
    let encoding: u32 = 0x0C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field Rt2 16 +: 4`
/// Requirement: FieldBoundary { field: "Rt2", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_rt2_1_poweroftwo_e00_0c510e00() {
    // Encoding: 0x0C510E00
    // Test aarch32_MRRC_T1A1_A field Rt2 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: CRm=0, cond=0, Rt=0, coproc=0, Rt2=1, opc1=0
    let encoding: u32 = 0x0C510E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field Rt 12 +: 4`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_rt_0_min_e00_0c500e00() {
    // Encoding: 0x0C500E00
    // Test aarch32_MRRC_T1A1_A field Rt = 0 (Min)
    // ISET: A32
    // Fields: Rt2=0, coproc=0, Rt=0, opc1=0, CRm=0, cond=0
    let encoding: u32 = 0x0C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field Rt 12 +: 4`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_rt_1_poweroftwo_e00_0c501e00() {
    // Encoding: 0x0C501E00
    // Test aarch32_MRRC_T1A1_A field Rt = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rt2=0, coproc=0, CRm=0, opc1=0, Rt=1
    let encoding: u32 = 0x0C501E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field coproc 9 +: 0`
/// Requirement: FieldBoundary { field: "coproc", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mrrc_t1a1_a_field_coproc_0_min_e00_0c500e00() {
    // Encoding: 0x0C500E00
    // Test aarch32_MRRC_T1A1_A field coproc = 0 (Min)
    // ISET: A32
    // Fields: coproc=0, opc1=0, CRm=0, cond=0, Rt2=0, Rt=0
    let encoding: u32 = 0x0C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field opc1 4 +: 4`
/// Requirement: FieldBoundary { field: "opc1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mrrc_t1a1_a_field_opc1_0_min_e00_0c500e00() {
    // Encoding: 0x0C500E00
    // Test aarch32_MRRC_T1A1_A field opc1 = 0 (Min)
    // ISET: A32
    // Fields: Rt2=0, coproc=0, Rt=0, opc1=0, CRm=0, cond=0
    let encoding: u32 = 0x0C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field opc1 4 +: 4`
/// Requirement: FieldBoundary { field: "opc1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mrrc_t1a1_a_field_opc1_1_poweroftwo_e00_0c500e10() {
    // Encoding: 0x0C500E10
    // Test aarch32_MRRC_T1A1_A field opc1 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: coproc=0, Rt=0, cond=0, Rt2=0, opc1=1, CRm=0
    let encoding: u32 = 0x0C500E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field opc1 4 +: 4`
/// Requirement: FieldBoundary { field: "opc1", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_opc1_7_poweroftwominusone_e00_0c500e70() {
    // Encoding: 0x0C500E70
    // Test aarch32_MRRC_T1A1_A field opc1 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: coproc=0, Rt=0, Rt2=0, cond=0, CRm=0, opc1=7
    let encoding: u32 = 0x0C500E70;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field opc1 4 +: 4`
/// Requirement: FieldBoundary { field: "opc1", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_opc1_15_max_e00_0c500ef0() {
    // Encoding: 0x0C500EF0
    // Test aarch32_MRRC_T1A1_A field opc1 = 15 (Max)
    // ISET: A32
    // Fields: cond=0, coproc=0, CRm=0, Rt=0, opc1=15, Rt2=0
    let encoding: u32 = 0x0C500EF0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mrrc_t1a1_a_field_crm_0_min_e00_0c500e00() {
    // Encoding: 0x0C500E00
    // Test aarch32_MRRC_T1A1_A field CRm = 0 (Min)
    // ISET: A32
    // Fields: coproc=0, Rt=0, Rt2=0, opc1=0, CRm=0, cond=0
    let encoding: u32 = 0x0C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mrrc_t1a1_a_field_crm_1_poweroftwo_e00_0c500e01() {
    // Encoding: 0x0C500E01
    // Test aarch32_MRRC_T1A1_A field CRm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: CRm=1, cond=0, coproc=0, opc1=0, Rt2=0, Rt=0
    let encoding: u32 = 0x0C500E01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_crm_7_poweroftwominusone_e00_0c500e07() {
    // Encoding: 0x0C500E07
    // Test aarch32_MRRC_T1A1_A field CRm = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: CRm=7, Rt=0, cond=0, Rt2=0, coproc=0, opc1=0
    let encoding: u32 = 0x0C500E07;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_crm_15_max_e00_0c500e0f() {
    // Encoding: 0x0C500E0F
    // Test aarch32_MRRC_T1A1_A field CRm = 15 (Max)
    // ISET: A32
    // Fields: cond=0, Rt2=0, Rt=0, CRm=15, coproc=0, opc1=0
    let encoding: u32 = 0x0C500E0F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_mrrc_t1a1_a_combo_0_e00_0c500e00() {
    // Encoding: 0x0C500E00
    // Test aarch32_MRRC_T1A1_A field combination: cond=0, Rt2=0, Rt=0, coproc=0, opc1=0, CRm=0
    // ISET: A32
    // Fields: cond=0, Rt=0, coproc=0, Rt2=0, opc1=0, CRm=0
    let encoding: u32 = 0x0C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_mrrc_t1a1_a_special_cond_0_condition_eq_3584_0c500e00() {
    // Encoding: 0x0C500E00
    // Test aarch32_MRRC_T1A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: coproc=0, cond=0, Rt=0, Rt2=0, opc1=0, CRm=0
    let encoding: u32 = 0x0C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_mrrc_t1a1_a_special_cond_1_condition_ne_3584_1c500e00() {
    // Encoding: 0x1C500E00
    // Test aarch32_MRRC_T1A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, Rt2=0, opc1=0, coproc=0, Rt=0, CRm=0
    let encoding: u32 = 0x1C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_mrrc_t1a1_a_special_cond_2_condition_cs_hs_3584_2c500e00() {
    // Encoding: 0x2C500E00
    // Test aarch32_MRRC_T1A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, Rt=0, coproc=0, Rt2=0, opc1=0, CRm=0
    let encoding: u32 = 0x2C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_mrrc_t1a1_a_special_cond_3_condition_cc_lo_3584_3c500e00() {
    // Encoding: 0x3C500E00
    // Test aarch32_MRRC_T1A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, Rt=0, opc1=0, CRm=0, coproc=0, Rt2=0
    let encoding: u32 = 0x3C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_mrrc_t1a1_a_special_cond_4_condition_mi_3584_4c500e00() {
    // Encoding: 0x4C500E00
    // Test aarch32_MRRC_T1A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rt2=0, Rt=0, opc1=0, CRm=0, cond=4, coproc=0
    let encoding: u32 = 0x4C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_mrrc_t1a1_a_special_cond_5_condition_pl_3584_5c500e00() {
    // Encoding: 0x5C500E00
    // Test aarch32_MRRC_T1A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, Rt=0, CRm=0, Rt2=0, opc1=0, coproc=0
    let encoding: u32 = 0x5C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_mrrc_t1a1_a_special_cond_6_condition_vs_3584_6c500e00() {
    // Encoding: 0x6C500E00
    // Test aarch32_MRRC_T1A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: CRm=0, coproc=0, Rt=0, opc1=0, Rt2=0, cond=6
    let encoding: u32 = 0x6C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_mrrc_t1a1_a_special_cond_7_condition_vc_3584_7c500e00() {
    // Encoding: 0x7C500E00
    // Test aarch32_MRRC_T1A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: CRm=0, Rt2=0, Rt=0, coproc=0, cond=7, opc1=0
    let encoding: u32 = 0x7C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_mrrc_t1a1_a_special_cond_8_condition_hi_3584_8c500e00() {
    // Encoding: 0x8C500E00
    // Test aarch32_MRRC_T1A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: CRm=0, coproc=0, opc1=0, Rt2=0, cond=8, Rt=0
    let encoding: u32 = 0x8C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_mrrc_t1a1_a_special_cond_9_condition_ls_3584_9c500e00() {
    // Encoding: 0x9C500E00
    // Test aarch32_MRRC_T1A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, coproc=0, CRm=0, Rt2=0, opc1=0, Rt=0
    let encoding: u32 = 0x9C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_mrrc_t1a1_a_special_cond_10_condition_ge_3584_ac500e00() {
    // Encoding: 0xAC500E00
    // Test aarch32_MRRC_T1A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rt=0, opc1=0, coproc=0, CRm=0, cond=10, Rt2=0
    let encoding: u32 = 0xAC500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_mrrc_t1a1_a_special_cond_11_condition_lt_3584_bc500e00() {
    // Encoding: 0xBC500E00
    // Test aarch32_MRRC_T1A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: coproc=0, Rt=0, Rt2=0, opc1=0, CRm=0, cond=11
    let encoding: u32 = 0xBC500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_mrrc_t1a1_a_special_cond_12_condition_gt_3584_cc500e00() {
    // Encoding: 0xCC500E00
    // Test aarch32_MRRC_T1A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: coproc=0, opc1=0, Rt2=0, Rt=0, CRm=0, cond=12
    let encoding: u32 = 0xCC500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_mrrc_t1a1_a_special_cond_13_condition_le_3584_dc500e00() {
    // Encoding: 0xDC500E00
    // Test aarch32_MRRC_T1A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: CRm=0, cond=13, opc1=0, Rt=0, coproc=0, Rt2=0
    let encoding: u32 = 0xDC500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_mrrc_t1a1_a_special_cond_14_condition_al_3584_ec500e00() {
    // Encoding: 0xEC500E00
    // Test aarch32_MRRC_T1A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: coproc=0, Rt=0, opc1=0, Rt2=0, cond=14, CRm=0
    let encoding: u32 = 0xEC500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_mrrc_t1a1_a_special_cond_15_condition_nv_3584_fc500e00() {
    // Encoding: 0xFC500E00
    // Test aarch32_MRRC_T1A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rt2=0, Rt=0, CRm=0, coproc=0, cond=15, opc1=0
    let encoding: u32 = 0xFC500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "t" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "t2" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "t" }) } }, rhs: Var(QualifiedIdentifier { qualifier: Any, name: "t2" }) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"t\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"t2\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"t\" }) } }, rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"t2\" }) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mrrc_t1a1_a_invalid_0_e00_0c500e00() {
    // Encoding: 0x0C500E00
    // Test aarch32_MRRC_T1A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "t" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "t2" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "t" }) } }, rhs: Var(QualifiedIdentifier { qualifier: Any, name: "t2" }) }
    // ISET: A32
    // Fields: opc1=0, CRm=0, cond=0, coproc=0, Rt=0, Rt2=0
    let encoding: u32 = 0x0C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mrrc_t1a1_a_invalid_1_e00_0c500e00() {
    // Encoding: 0x0C500E00
    // Test aarch32_MRRC_T1A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rt2=0, opc1=0, CRm=0, cond=0, coproc=0, Rt=0
    let encoding: u32 = 0x0C500E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field Rt2 16 +: 4`
/// Requirement: FieldBoundary { field: "Rt2", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_rt2_0_min_e00_ec500e00() {
    // Thumb encoding (32): 0xEC500E00
    // Test aarch32_MRRC_T1A1_A field Rt2 = 0 (Min)
    // ISET: T32
    // Fields: Rt2=0, CRm=0, coproc=0, Rt=0, opc1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC500E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field Rt2 16 +: 4`
/// Requirement: FieldBoundary { field: "Rt2", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_rt2_1_poweroftwo_e00_ec510e00() {
    // Thumb encoding (32): 0xEC510E00
    // Test aarch32_MRRC_T1A1_A field Rt2 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: coproc=0, Rt2=1, opc1=0, Rt=0, CRm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC510E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field Rt 12 +: 4`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_rt_0_min_e00_ec500e00() {
    // Thumb encoding (32): 0xEC500E00
    // Test aarch32_MRRC_T1A1_A field Rt = 0 (Min)
    // ISET: T32
    // Fields: CRm=0, Rt=0, Rt2=0, coproc=0, opc1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC500E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field Rt 12 +: 4`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_rt_1_poweroftwo_e00_ec501e00() {
    // Thumb encoding (32): 0xEC501E00
    // Test aarch32_MRRC_T1A1_A field Rt = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: coproc=0, opc1=0, Rt2=0, Rt=1, CRm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC501E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field coproc 9 +: 0`
/// Requirement: FieldBoundary { field: "coproc", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mrrc_t1a1_a_field_coproc_0_min_e00_ec500e00() {
    // Thumb encoding (32): 0xEC500E00
    // Test aarch32_MRRC_T1A1_A field coproc = 0 (Min)
    // ISET: T32
    // Fields: Rt2=0, Rt=0, CRm=0, opc1=0, coproc=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC500E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field opc1 4 +: 4`
/// Requirement: FieldBoundary { field: "opc1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mrrc_t1a1_a_field_opc1_0_min_e00_ec500e00() {
    // Thumb encoding (32): 0xEC500E00
    // Test aarch32_MRRC_T1A1_A field opc1 = 0 (Min)
    // ISET: T32
    // Fields: opc1=0, CRm=0, Rt=0, coproc=0, Rt2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC500E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field opc1 4 +: 4`
/// Requirement: FieldBoundary { field: "opc1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mrrc_t1a1_a_field_opc1_1_poweroftwo_e00_ec500e10() {
    // Thumb encoding (32): 0xEC500E10
    // Test aarch32_MRRC_T1A1_A field opc1 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: opc1=1, coproc=0, CRm=0, Rt2=0, Rt=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC500E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field opc1 4 +: 4`
/// Requirement: FieldBoundary { field: "opc1", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_opc1_7_poweroftwominusone_e00_ec500e70() {
    // Thumb encoding (32): 0xEC500E70
    // Test aarch32_MRRC_T1A1_A field opc1 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: coproc=0, opc1=7, Rt2=0, Rt=0, CRm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC500E70;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field opc1 4 +: 4`
/// Requirement: FieldBoundary { field: "opc1", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_opc1_15_max_e00_ec500ef0() {
    // Thumb encoding (32): 0xEC500EF0
    // Test aarch32_MRRC_T1A1_A field opc1 = 15 (Max)
    // ISET: T32
    // Fields: Rt=0, Rt2=0, opc1=15, CRm=0, coproc=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC500EF0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_mrrc_t1a1_a_field_crm_0_min_e00_ec500e00() {
    // Thumb encoding (32): 0xEC500E00
    // Test aarch32_MRRC_T1A1_A field CRm = 0 (Min)
    // ISET: T32
    // Fields: Rt2=0, opc1=0, coproc=0, CRm=0, Rt=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC500E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_mrrc_t1a1_a_field_crm_1_poweroftwo_e00_ec500e01() {
    // Thumb encoding (32): 0xEC500E01
    // Test aarch32_MRRC_T1A1_A field CRm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rt2=0, Rt=0, opc1=0, CRm=1, coproc=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC500E01;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_crm_7_poweroftwominusone_e00_ec500e07() {
    // Thumb encoding (32): 0xEC500E07
    // Test aarch32_MRRC_T1A1_A field CRm = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rt2=0, opc1=0, CRm=7, Rt=0, coproc=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC500E07;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field CRm 0 +: 4`
/// Requirement: FieldBoundary { field: "CRm", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch32_mrrc_t1a1_a_field_crm_15_max_e00_ec500e0f() {
    // Thumb encoding (32): 0xEC500E0F
    // Test aarch32_MRRC_T1A1_A field CRm = 15 (Max)
    // ISET: T32
    // Fields: CRm=15, Rt=0, coproc=0, Rt2=0, opc1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC500E0F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=0 (register index 0 (first register))
#[test]
fn test_aarch32_mrrc_t1a1_a_combo_0_e00_ec500e00() {
    // Thumb encoding (32): 0xEC500E00
    // Test aarch32_MRRC_T1A1_A field combination: Rt2=0, Rt=0, coproc=0, opc1=0, CRm=0
    // ISET: T32
    // Fields: Rt=0, opc1=0, Rt2=0, CRm=0, coproc=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC500E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "t" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "t2" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "t" }) } }, rhs: Var(QualifiedIdentifier { qualifier: Any, name: "t2" }) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"t\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"t2\" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"t\" }) } }, rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"t2\" }) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mrrc_t1a1_a_invalid_0_e00_ec500e00() {
    // Thumb encoding (32): 0xEC500E00
    // Test aarch32_MRRC_T1A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "t" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "t2" }) } }, rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "t" }) } }, rhs: Var(QualifiedIdentifier { qualifier: Any, name: "t2" }) }
    // ISET: T32
    // Fields: opc1=0, Rt2=0, coproc=0, Rt=0, CRm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC500E00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_MRRC_T1A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_mrrc_t1a1_a_invalid_1_e00_ec500e00() {
    // Thumb encoding (32): 0xEC500E00
    // Test aarch32_MRRC_T1A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: opc1=0, coproc=0, Rt=0, Rt2=0, CRm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC500E00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_LDC_l_A Tests
// ============================================================================

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_cond_0_min_5e00_0c1f5e00() {
    // Encoding: 0x0C1F5E00
    // Test aarch32_LDC_l_T1A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, P=0, imm8=0, W=0, U=0
    let encoding: u32 = 0x0C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_cond_1_poweroftwo_5e00_1c1f5e00() {
    // Encoding: 0x1C1F5E00
    // Test aarch32_LDC_l_T1A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: U=0, imm8=0, cond=1, W=0, P=0
    let encoding: u32 = 0x1C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_cond_2_poweroftwo_5e00_2c1f5e00() {
    // Encoding: 0x2C1F5E00
    // Test aarch32_LDC_l_T1A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: P=0, W=0, imm8=0, cond=2, U=0
    let encoding: u32 = 0x2C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_cond_3_poweroftwo_5e00_3c1f5e00() {
    // Encoding: 0x3C1F5E00
    // Test aarch32_LDC_l_T1A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: P=0, U=0, W=0, imm8=0, cond=3
    let encoding: u32 = 0x3C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_cond_4_poweroftwo_5e00_4c1f5e00() {
    // Encoding: 0x4C1F5E00
    // Test aarch32_LDC_l_T1A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: W=0, imm8=0, U=0, cond=4, P=0
    let encoding: u32 = 0x4C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_cond_5_poweroftwo_5e00_5c1f5e00() {
    // Encoding: 0x5C1F5E00
    // Test aarch32_LDC_l_T1A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: imm8=0, U=0, cond=5, W=0, P=0
    let encoding: u32 = 0x5C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_cond_6_poweroftwo_5e00_6c1f5e00() {
    // Encoding: 0x6C1F5E00
    // Test aarch32_LDC_l_T1A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: P=0, W=0, imm8=0, U=0, cond=6
    let encoding: u32 = 0x6C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_cond_7_poweroftwo_5e00_7c1f5e00() {
    // Encoding: 0x7C1F5E00
    // Test aarch32_LDC_l_T1A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: imm8=0, U=0, cond=7, W=0, P=0
    let encoding: u32 = 0x7C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_cond_8_poweroftwo_5e00_8c1f5e00() {
    // Encoding: 0x8C1F5E00
    // Test aarch32_LDC_l_T1A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: imm8=0, P=0, cond=8, U=0, W=0
    let encoding: u32 = 0x8C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_cond_9_poweroftwo_5e00_9c1f5e00() {
    // Encoding: 0x9C1F5E00
    // Test aarch32_LDC_l_T1A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, P=0, W=0, U=0, imm8=0
    let encoding: u32 = 0x9C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_cond_10_poweroftwo_5e00_ac1f5e00() {
    // Encoding: 0xAC1F5E00
    // Test aarch32_LDC_l_T1A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, P=0, W=0, U=0, imm8=0
    let encoding: u32 = 0xAC1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_cond_11_poweroftwo_5e00_bc1f5e00() {
    // Encoding: 0xBC1F5E00
    // Test aarch32_LDC_l_T1A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=11, U=0, imm8=0, W=0, P=0
    let encoding: u32 = 0xBC1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_cond_12_poweroftwo_5e00_cc1f5e00() {
    // Encoding: 0xCC1F5E00
    // Test aarch32_LDC_l_T1A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: U=0, P=0, cond=12, W=0, imm8=0
    let encoding: u32 = 0xCC1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_cond_13_poweroftwo_5e00_dc1f5e00() {
    // Encoding: 0xDC1F5E00
    // Test aarch32_LDC_l_T1A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: imm8=0, cond=13, P=0, W=0, U=0
    let encoding: u32 = 0xDC1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_cond_14_poweroftwo_5e00_ec1f5e00() {
    // Encoding: 0xEC1F5E00
    // Test aarch32_LDC_l_T1A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, W=0, imm8=0, U=0, P=0
    let encoding: u32 = 0xEC1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_cond_15_max_5e00_fc1f5e00() {
    // Encoding: 0xFC1F5E00
    // Test aarch32_LDC_l_T1A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: imm8=0, U=0, cond=15, P=0, W=0
    let encoding: u32 = 0xFC1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field P 24 +: 1`
/// Requirement: FieldBoundary { field: "P", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_p_0_min_5e00_0c1f5e00() {
    // Encoding: 0x0C1F5E00
    // Test aarch32_LDC_l_T1A1_A field P = 0 (Min)
    // ISET: A32
    // Fields: W=0, P=0, cond=0, imm8=0, U=0
    let encoding: u32 = 0x0C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field P 24 +: 1`
/// Requirement: FieldBoundary { field: "P", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_p_1_max_5e00_0d1f5e00() {
    // Encoding: 0x0D1F5E00
    // Test aarch32_LDC_l_T1A1_A field P = 1 (Max)
    // ISET: A32
    // Fields: P=1, W=0, U=0, cond=0, imm8=0
    let encoding: u32 = 0x0D1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field U 23 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_u_0_min_5e00_0c1f5e00() {
    // Encoding: 0x0C1F5E00
    // Test aarch32_LDC_l_T1A1_A field U = 0 (Min)
    // ISET: A32
    // Fields: imm8=0, cond=0, P=0, U=0, W=0
    let encoding: u32 = 0x0C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field U 23 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_u_1_max_5e00_0c9f5e00() {
    // Encoding: 0x0C9F5E00
    // Test aarch32_LDC_l_T1A1_A field U = 1 (Max)
    // ISET: A32
    // Fields: cond=0, P=0, U=1, W=0, imm8=0
    let encoding: u32 = 0x0C9F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field W 21 +: 1`
/// Requirement: FieldBoundary { field: "W", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_w_0_min_5e00_0c1f5e00() {
    // Encoding: 0x0C1F5E00
    // Test aarch32_LDC_l_T1A1_A field W = 0 (Min)
    // ISET: A32
    // Fields: P=0, U=0, cond=0, W=0, imm8=0
    let encoding: u32 = 0x0C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field W 21 +: 1`
/// Requirement: FieldBoundary { field: "W", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_w_1_max_5e00_0c3f5e00() {
    // Encoding: 0x0C3F5E00
    // Test aarch32_LDC_l_T1A1_A field W = 1 (Max)
    // ISET: A32
    // Fields: cond=0, imm8=0, W=1, P=0, U=0
    let encoding: u32 = 0x0C3F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_0_zero_5e00_0c1f5e00() {
    // Encoding: 0x0C1F5E00
    // Test aarch32_LDC_l_T1A1_A field imm8 = 0 (Zero)
    // ISET: A32
    // Fields: imm8=0, cond=0, U=0, P=0, W=0
    let encoding: u32 = 0x0C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_1_poweroftwo_5e00_0c1f5e01() {
    // Encoding: 0x0C1F5E01
    // Test aarch32_LDC_l_T1A1_A field imm8 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: U=0, cond=0, W=0, imm8=1, P=0
    let encoding: u32 = 0x0C1F5E01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_3_poweroftwominusone_5e00_0c1f5e03() {
    // Encoding: 0x0C1F5E03
    // Test aarch32_LDC_l_T1A1_A field imm8 = 3 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm8=3, cond=0, P=0, W=0, U=0
    let encoding: u32 = 0x0C1F5E03;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_4_poweroftwo_5e00_0c1f5e04() {
    // Encoding: 0x0C1F5E04
    // Test aarch32_LDC_l_T1A1_A field imm8 = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: imm8=4, cond=0, W=0, P=0, U=0
    let encoding: u32 = 0x0C1F5E04;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_7_poweroftwominusone_5e00_0c1f5e07() {
    // Encoding: 0x0C1F5E07
    // Test aarch32_LDC_l_T1A1_A field imm8 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, W=0, U=0, P=0, imm8=7
    let encoding: u32 = 0x0C1F5E07;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_8_poweroftwo_5e00_0c1f5e08() {
    // Encoding: 0x0C1F5E08
    // Test aarch32_LDC_l_T1A1_A field imm8 = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: P=0, imm8=8, cond=0, U=0, W=0
    let encoding: u32 = 0x0C1F5E08;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_15_poweroftwominusone_5e00_0c1f5e0f() {
    // Encoding: 0x0C1F5E0F
    // Test aarch32_LDC_l_T1A1_A field imm8 = 15 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, U=0, W=0, P=0, imm8=15
    let encoding: u32 = 0x0C1F5E0F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_16_poweroftwo_5e00_0c1f5e10() {
    // Encoding: 0x0C1F5E10
    // Test aarch32_LDC_l_T1A1_A field imm8 = 16 (PowerOfTwo)
    // ISET: A32
    // Fields: P=0, W=0, cond=0, imm8=16, U=0
    let encoding: u32 = 0x0C1F5E10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_31_poweroftwominusone_5e00_0c1f5e1f() {
    // Encoding: 0x0C1F5E1F
    // Test aarch32_LDC_l_T1A1_A field imm8 = 31 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, P=0, U=0, W=0, imm8=31
    let encoding: u32 = 0x0C1F5E1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_32_poweroftwo_5e00_0c1f5e20() {
    // Encoding: 0x0C1F5E20
    // Test aarch32_LDC_l_T1A1_A field imm8 = 32 (PowerOfTwo)
    // ISET: A32
    // Fields: imm8=32, P=0, U=0, W=0, cond=0
    let encoding: u32 = 0x0C1F5E20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_63_poweroftwominusone_5e00_0c1f5e3f() {
    // Encoding: 0x0C1F5E3F
    // Test aarch32_LDC_l_T1A1_A field imm8 = 63 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: W=0, U=0, P=0, cond=0, imm8=63
    let encoding: u32 = 0x0C1F5E3F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_64_poweroftwo_5e00_0c1f5e40() {
    // Encoding: 0x0C1F5E40
    // Test aarch32_LDC_l_T1A1_A field imm8 = 64 (PowerOfTwo)
    // ISET: A32
    // Fields: imm8=64, U=0, P=0, W=0, cond=0
    let encoding: u32 = 0x0C1F5E40;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 127, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (127)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_127_poweroftwominusone_5e00_0c1f5e7f() {
    // Encoding: 0x0C1F5E7F
    // Test aarch32_LDC_l_T1A1_A field imm8 = 127 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, U=0, imm8=127, W=0, P=0
    let encoding: u32 = 0x0C1F5E7F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_128_poweroftwo_5e00_0c1f5e80() {
    // Encoding: 0x0C1F5E80
    // Test aarch32_LDC_l_T1A1_A field imm8 = 128 (PowerOfTwo)
    // ISET: A32
    // Fields: U=0, cond=0, P=0, W=0, imm8=128
    let encoding: u32 = 0x0C1F5E80;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 255, boundary: Max }
/// maximum immediate (255)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_255_max_5e00_0c1f5eff() {
    // Encoding: 0x0C1F5EFF
    // Test aarch32_LDC_l_T1A1_A field imm8 = 255 (Max)
    // ISET: A32
    // Fields: W=0, cond=0, imm8=255, P=0, U=0
    let encoding: u32 = 0x0C1F5EFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_ldc_l_t1a1_a_combo_0_5e00_0c1f5e00() {
    // Encoding: 0x0C1F5E00
    // Test aarch32_LDC_l_T1A1_A field combination: cond=0, P=0, U=0, W=0, imm8=0
    // ISET: A32
    // Fields: W=0, cond=0, U=0, imm8=0, P=0
    let encoding: u32 = 0x0C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_ldc_l_t1a1_a_special_cond_0_condition_eq_24064_0c1f5e00() {
    // Encoding: 0x0C1F5E00
    // Test aarch32_LDC_l_T1A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: cond=0, U=0, W=0, imm8=0, P=0
    let encoding: u32 = 0x0C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_ldc_l_t1a1_a_special_cond_1_condition_ne_24064_1c1f5e00() {
    // Encoding: 0x1C1F5E00
    // Test aarch32_LDC_l_T1A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: W=0, U=0, cond=1, imm8=0, P=0
    let encoding: u32 = 0x1C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_ldc_l_t1a1_a_special_cond_2_condition_cs_hs_24064_2c1f5e00() {
    // Encoding: 0x2C1F5E00
    // Test aarch32_LDC_l_T1A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: imm8=0, W=0, U=0, cond=2, P=0
    let encoding: u32 = 0x2C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_ldc_l_t1a1_a_special_cond_3_condition_cc_lo_24064_3c1f5e00() {
    // Encoding: 0x3C1F5E00
    // Test aarch32_LDC_l_T1A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: imm8=0, U=0, P=0, cond=3, W=0
    let encoding: u32 = 0x3C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_ldc_l_t1a1_a_special_cond_4_condition_mi_24064_4c1f5e00() {
    // Encoding: 0x4C1F5E00
    // Test aarch32_LDC_l_T1A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, U=0, imm8=0, P=0, W=0
    let encoding: u32 = 0x4C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_ldc_l_t1a1_a_special_cond_5_condition_pl_24064_5c1f5e00() {
    // Encoding: 0x5C1F5E00
    // Test aarch32_LDC_l_T1A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, imm8=0, U=0, P=0, W=0
    let encoding: u32 = 0x5C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_ldc_l_t1a1_a_special_cond_6_condition_vs_24064_6c1f5e00() {
    // Encoding: 0x6C1F5E00
    // Test aarch32_LDC_l_T1A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: U=0, imm8=0, cond=6, W=0, P=0
    let encoding: u32 = 0x6C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_ldc_l_t1a1_a_special_cond_7_condition_vc_24064_7c1f5e00() {
    // Encoding: 0x7C1F5E00
    // Test aarch32_LDC_l_T1A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: U=0, W=0, P=0, cond=7, imm8=0
    let encoding: u32 = 0x7C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_ldc_l_t1a1_a_special_cond_8_condition_hi_24064_8c1f5e00() {
    // Encoding: 0x8C1F5E00
    // Test aarch32_LDC_l_T1A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: U=0, P=0, W=0, cond=8, imm8=0
    let encoding: u32 = 0x8C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_ldc_l_t1a1_a_special_cond_9_condition_ls_24064_9c1f5e00() {
    // Encoding: 0x9C1F5E00
    // Test aarch32_LDC_l_T1A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: P=0, U=0, imm8=0, cond=9, W=0
    let encoding: u32 = 0x9C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_ldc_l_t1a1_a_special_cond_10_condition_ge_24064_ac1f5e00() {
    // Encoding: 0xAC1F5E00
    // Test aarch32_LDC_l_T1A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, P=0, U=0, W=0, imm8=0
    let encoding: u32 = 0xAC1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_ldc_l_t1a1_a_special_cond_11_condition_lt_24064_bc1f5e00() {
    // Encoding: 0xBC1F5E00
    // Test aarch32_LDC_l_T1A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: U=0, W=0, P=0, cond=11, imm8=0
    let encoding: u32 = 0xBC1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_ldc_l_t1a1_a_special_cond_12_condition_gt_24064_cc1f5e00() {
    // Encoding: 0xCC1F5E00
    // Test aarch32_LDC_l_T1A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: cond=12, W=0, P=0, imm8=0, U=0
    let encoding: u32 = 0xCC1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_ldc_l_t1a1_a_special_cond_13_condition_le_24064_dc1f5e00() {
    // Encoding: 0xDC1F5E00
    // Test aarch32_LDC_l_T1A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: U=0, P=0, cond=13, imm8=0, W=0
    let encoding: u32 = 0xDC1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_ldc_l_t1a1_a_special_cond_14_condition_al_24064_ec1f5e00() {
    // Encoding: 0xEC1F5E00
    // Test aarch32_LDC_l_T1A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: P=0, cond=14, U=0, W=0, imm8=0
    let encoding: u32 = 0xEC1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_ldc_l_t1a1_a_special_cond_15_condition_nv_24064_fc1f5e00() {
    // Encoding: 0xFC1F5E00
    // Test aarch32_LDC_l_T1A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, W=0, imm8=0, P=0, U=0
    let encoding: u32 = 0xFC1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "P" }), rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "U" }) } }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "W" }) } }, rhs: LitBits([false]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"P\" }), rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"U\" }) } }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"W\" }) } }, rhs: LitBits([false]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_ldc_l_t1a1_a_invalid_0_5e00_0c1f5e00() {
    // Encoding: 0x0C1F5E00
    // Test aarch32_LDC_l_T1A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "P" }), rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "U" }) } }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "W" }) } }, rhs: LitBits([false]) }
    // ISET: A32
    // Fields: U=0, cond=0, P=0, W=0, imm8=0
    let encoding: u32 = 0x0C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_ldc_l_t1a1_a_invalid_1_5e00_0c1f5e00() {
    // Encoding: 0x0C1F5E00
    // Test aarch32_LDC_l_T1A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: cond=0, P=0, W=0, imm8=0, U=0
    let encoding: u32 = 0x0C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "W" }), rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "P" }), rhs: Binary { op: And, lhs: LitBits([false]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "CurrentInstrSet" }, args: [] } } }, rhs: Var(QualifiedIdentifier { qualifier: Any, name: "InstrSet_A32" }) } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"W\" }), rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"P\" }), rhs: Binary { op: And, lhs: LitBits([false]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"CurrentInstrSet\" }, args: [] } } }, rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"InstrSet_A32\" }) } } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_ldc_l_t1a1_a_invalid_2_5e00_0c1f5e00() {
    // Encoding: 0x0C1F5E00
    // Test aarch32_LDC_l_T1A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "W" }), rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "P" }), rhs: Binary { op: And, lhs: LitBits([false]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "CurrentInstrSet" }, args: [] } } }, rhs: Var(QualifiedIdentifier { qualifier: Any, name: "InstrSet_A32" }) } } }
    // ISET: A32
    // Fields: W=0, P=0, cond=0, U=0, imm8=0
    let encoding: u32 = 0x0C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_ldc_l_t1a1_a_invalid_3_5e00_0c1f5e00() {
    // Encoding: 0x0C1F5E00
    // Test aarch32_LDC_l_T1A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: U=0, cond=0, P=0, W=0, imm8=0
    let encoding: u32 = 0x0C1F5E00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field P 24 +: 1`
/// Requirement: FieldBoundary { field: "P", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_p_0_min_5e00_ec1f5e00() {
    // Thumb encoding (32): 0xEC1F5E00
    // Test aarch32_LDC_l_T1A1_A field P = 0 (Min)
    // ISET: T32
    // Fields: U=0, W=0, imm8=0, P=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC1F5E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field P 24 +: 1`
/// Requirement: FieldBoundary { field: "P", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_p_1_max_5e00_ed1f5e00() {
    // Thumb encoding (32): 0xED1F5E00
    // Test aarch32_LDC_l_T1A1_A field P = 1 (Max)
    // ISET: T32
    // Fields: imm8=0, P=1, W=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xED1F5E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field U 23 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_u_0_min_5e00_ec1f5e00() {
    // Thumb encoding (32): 0xEC1F5E00
    // Test aarch32_LDC_l_T1A1_A field U = 0 (Min)
    // ISET: T32
    // Fields: W=0, imm8=0, P=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC1F5E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field U 23 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_u_1_max_5e00_ec9f5e00() {
    // Thumb encoding (32): 0xEC9F5E00
    // Test aarch32_LDC_l_T1A1_A field U = 1 (Max)
    // ISET: T32
    // Fields: U=1, W=0, P=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC9F5E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field W 21 +: 1`
/// Requirement: FieldBoundary { field: "W", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_w_0_min_5e00_ec1f5e00() {
    // Thumb encoding (32): 0xEC1F5E00
    // Test aarch32_LDC_l_T1A1_A field W = 0 (Min)
    // ISET: T32
    // Fields: W=0, imm8=0, U=0, P=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC1F5E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field W 21 +: 1`
/// Requirement: FieldBoundary { field: "W", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_w_1_max_5e00_ec3f5e00() {
    // Thumb encoding (32): 0xEC3F5E00
    // Test aarch32_LDC_l_T1A1_A field W = 1 (Max)
    // ISET: T32
    // Fields: imm8=0, U=0, W=1, P=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC3F5E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_0_zero_5e00_ec1f5e00() {
    // Thumb encoding (32): 0xEC1F5E00
    // Test aarch32_LDC_l_T1A1_A field imm8 = 0 (Zero)
    // ISET: T32
    // Fields: P=0, U=0, imm8=0, W=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC1F5E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_1_poweroftwo_5e00_ec1f5e01() {
    // Thumb encoding (32): 0xEC1F5E01
    // Test aarch32_LDC_l_T1A1_A field imm8 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: P=0, imm8=1, W=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC1F5E01;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_3_poweroftwominusone_5e00_ec1f5e03() {
    // Thumb encoding (32): 0xEC1F5E03
    // Test aarch32_LDC_l_T1A1_A field imm8 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: W=0, P=0, U=0, imm8=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC1F5E03;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_4_poweroftwo_5e00_ec1f5e04() {
    // Thumb encoding (32): 0xEC1F5E04
    // Test aarch32_LDC_l_T1A1_A field imm8 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: W=0, P=0, U=0, imm8=4
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC1F5E04;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_7_poweroftwominusone_5e00_ec1f5e07() {
    // Thumb encoding (32): 0xEC1F5E07
    // Test aarch32_LDC_l_T1A1_A field imm8 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: P=0, W=0, U=0, imm8=7
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC1F5E07;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_8_poweroftwo_5e00_ec1f5e08() {
    // Thumb encoding (32): 0xEC1F5E08
    // Test aarch32_LDC_l_T1A1_A field imm8 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=8, P=0, W=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC1F5E08;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_15_poweroftwominusone_5e00_ec1f5e0f() {
    // Thumb encoding (32): 0xEC1F5E0F
    // Test aarch32_LDC_l_T1A1_A field imm8 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: W=0, imm8=15, P=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC1F5E0F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_16_poweroftwo_5e00_ec1f5e10() {
    // Thumb encoding (32): 0xEC1F5E10
    // Test aarch32_LDC_l_T1A1_A field imm8 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: W=0, imm8=16, P=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC1F5E10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_31_poweroftwominusone_5e00_ec1f5e1f() {
    // Thumb encoding (32): 0xEC1F5E1F
    // Test aarch32_LDC_l_T1A1_A field imm8 = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=31, P=0, U=0, W=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC1F5E1F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_32_poweroftwo_5e00_ec1f5e20() {
    // Thumb encoding (32): 0xEC1F5E20
    // Test aarch32_LDC_l_T1A1_A field imm8 = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=32, U=0, W=0, P=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC1F5E20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_63_poweroftwominusone_5e00_ec1f5e3f() {
    // Thumb encoding (32): 0xEC1F5E3F
    // Test aarch32_LDC_l_T1A1_A field imm8 = 63 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: U=0, imm8=63, W=0, P=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC1F5E3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_64_poweroftwo_5e00_ec1f5e40() {
    // Thumb encoding (32): 0xEC1F5E40
    // Test aarch32_LDC_l_T1A1_A field imm8 = 64 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=64, P=0, W=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC1F5E40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 127, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (127)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_127_poweroftwominusone_5e00_ec1f5e7f() {
    // Thumb encoding (32): 0xEC1F5E7F
    // Test aarch32_LDC_l_T1A1_A field imm8 = 127 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=127, P=0, U=0, W=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC1F5E7F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_128_poweroftwo_5e00_ec1f5e80() {
    // Thumb encoding (32): 0xEC1F5E80
    // Test aarch32_LDC_l_T1A1_A field imm8 = 128 (PowerOfTwo)
    // ISET: T32
    // Fields: P=0, W=0, imm8=128, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC1F5E80;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 255, boundary: Max }
/// maximum immediate (255)
#[test]
fn test_aarch32_ldc_l_t1a1_a_field_imm8_255_max_5e00_ec1f5eff() {
    // Thumb encoding (32): 0xEC1F5EFF
    // Test aarch32_LDC_l_T1A1_A field imm8 = 255 (Max)
    // ISET: T32
    // Fields: U=0, P=0, W=0, imm8=255
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC1F5EFF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// P=0 (minimum value)
#[test]
fn test_aarch32_ldc_l_t1a1_a_combo_0_5e00_ec1f5e00() {
    // Thumb encoding (32): 0xEC1F5E00
    // Test aarch32_LDC_l_T1A1_A field combination: P=0, U=0, W=0, imm8=0
    // ISET: T32
    // Fields: imm8=0, W=0, U=0, P=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC1F5E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "P" }), rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "U" }) } }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "W" }) } }, rhs: LitBits([false]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"P\" }), rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"U\" }) } }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"W\" }) } }, rhs: LitBits([false]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_ldc_l_t1a1_a_invalid_0_5e00_ec1f5e00() {
    // Thumb encoding (32): 0xEC1F5E00
    // Test aarch32_LDC_l_T1A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "P" }), rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "U" }) } }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "W" }) } }, rhs: LitBits([false]) }
    // ISET: T32
    // Fields: W=0, P=0, U=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC1F5E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_ldc_l_t1a1_a_invalid_1_5e00_ec1f5e00() {
    // Thumb encoding (32): 0xEC1F5E00
    // Test aarch32_LDC_l_T1A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: imm8=0, P=0, W=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC1F5E00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "W" }), rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "P" }), rhs: Binary { op: And, lhs: LitBits([false]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "CurrentInstrSet" }, args: [] } } }, rhs: Var(QualifiedIdentifier { qualifier: Any, name: "InstrSet_A32" }) } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"W\" }), rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"P\" }), rhs: Binary { op: And, lhs: LitBits([false]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"CurrentInstrSet\" }, args: [] } } }, rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"InstrSet_A32\" }) } } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_ldc_l_t1a1_a_invalid_2_5e00_ec1f5e00() {
    // Thumb encoding (32): 0xEC1F5E00
    // Test aarch32_LDC_l_T1A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "W" }), rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "P" }), rhs: Binary { op: And, lhs: LitBits([false]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "CurrentInstrSet" }, args: [] } } }, rhs: Var(QualifiedIdentifier { qualifier: Any, name: "InstrSet_A32" }) } } }
    // ISET: T32
    // Fields: U=0, imm8=0, W=0, P=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC1F5E00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_LDC_l_T1A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_ldc_l_t1a1_a_invalid_3_5e00_ec1f5e00() {
    // Thumb encoding (32): 0xEC1F5E00
    // Test aarch32_LDC_l_T1A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: P=0, W=0, imm8=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEC1F5E00;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}
