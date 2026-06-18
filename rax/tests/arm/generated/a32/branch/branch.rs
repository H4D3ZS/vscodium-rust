//! A32 branch branch tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers_32::*;

// ============================================================================
// aarch32_BL_i_A Tests
// ============================================================================

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_bl_i_a1_a_field_cond_0_min_0_0b000000() {
    // Encoding: 0x0B000000
    // Test aarch32_BL_i_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, imm24=0
    let encoding: u32 = 0x0B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_bl_i_a1_a_field_cond_1_poweroftwo_0_1b000000() {
    // Encoding: 0x1B000000
    // Test aarch32_BL_i_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=0, cond=1
    let encoding: u32 = 0x1B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_bl_i_a1_a_field_cond_2_poweroftwo_0_2b000000() {
    // Encoding: 0x2B000000
    // Test aarch32_BL_i_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=2, imm24=0
    let encoding: u32 = 0x2B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_bl_i_a1_a_field_cond_3_poweroftwo_0_3b000000() {
    // Encoding: 0x3B000000
    // Test aarch32_BL_i_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=0, cond=3
    let encoding: u32 = 0x3B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_bl_i_a1_a_field_cond_4_poweroftwo_0_4b000000() {
    // Encoding: 0x4B000000
    // Test aarch32_BL_i_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=0, cond=4
    let encoding: u32 = 0x4B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_bl_i_a1_a_field_cond_5_poweroftwo_0_5b000000() {
    // Encoding: 0x5B000000
    // Test aarch32_BL_i_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=0, cond=5
    let encoding: u32 = 0x5B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_bl_i_a1_a_field_cond_6_poweroftwo_0_6b000000() {
    // Encoding: 0x6B000000
    // Test aarch32_BL_i_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, imm24=0
    let encoding: u32 = 0x6B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_bl_i_a1_a_field_cond_7_poweroftwo_0_7b000000() {
    // Encoding: 0x7B000000
    // Test aarch32_BL_i_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=0, cond=7
    let encoding: u32 = 0x7B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_bl_i_a1_a_field_cond_8_poweroftwo_0_8b000000() {
    // Encoding: 0x8B000000
    // Test aarch32_BL_i_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=0, cond=8
    let encoding: u32 = 0x8B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_bl_i_a1_a_field_cond_9_poweroftwo_0_9b000000() {
    // Encoding: 0x9B000000
    // Test aarch32_BL_i_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, imm24=0
    let encoding: u32 = 0x9B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_bl_i_a1_a_field_cond_10_poweroftwo_0_ab000000() {
    // Encoding: 0xAB000000
    // Test aarch32_BL_i_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=0, cond=10
    let encoding: u32 = 0xAB000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_bl_i_a1_a_field_cond_11_poweroftwo_0_bb000000() {
    // Encoding: 0xBB000000
    // Test aarch32_BL_i_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=11, imm24=0
    let encoding: u32 = 0xBB000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_bl_i_a1_a_field_cond_12_poweroftwo_0_cb000000() {
    // Encoding: 0xCB000000
    // Test aarch32_BL_i_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=0, cond=12
    let encoding: u32 = 0xCB000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_bl_i_a1_a_field_cond_13_poweroftwo_0_db000000() {
    // Encoding: 0xDB000000
    // Test aarch32_BL_i_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=0, cond=13
    let encoding: u32 = 0xDB000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_bl_i_a1_a_field_cond_14_poweroftwo_0_eb000000() {
    // Encoding: 0xEB000000
    // Test aarch32_BL_i_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, imm24=0
    let encoding: u32 = 0xEB000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_bl_i_a1_a_field_cond_15_max_0_fb000000() {
    // Encoding: 0xFB000000
    // Test aarch32_BL_i_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, imm24=0
    let encoding: u32 = 0xFB000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_0_zero_0_0b000000() {
    // Encoding: 0x0B000000
    // Test aarch32_BL_i_A1_A field imm24 = 0 (Zero)
    // ISET: A32
    // Fields: cond=0, imm24=0
    let encoding: u32 = 0x0B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_1_poweroftwo_0_0b000001() {
    // Encoding: 0x0B000001
    // Test aarch32_BL_i_A1_A field imm24 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=1
    let encoding: u32 = 0x0B000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_3_poweroftwominusone_0_0b000003() {
    // Encoding: 0x0B000003
    // Test aarch32_BL_i_A1_A field imm24 = 3 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm24=3, cond=0
    let encoding: u32 = 0x0B000003;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_4_poweroftwo_0_0b000004() {
    // Encoding: 0x0B000004
    // Test aarch32_BL_i_A1_A field imm24 = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=4, cond=0
    let encoding: u32 = 0x0B000004;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_7_poweroftwominusone_0_0b000007() {
    // Encoding: 0x0B000007
    // Test aarch32_BL_i_A1_A field imm24 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm24=7, cond=0
    let encoding: u32 = 0x0B000007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_8_poweroftwo_0_0b000008() {
    // Encoding: 0x0B000008
    // Test aarch32_BL_i_A1_A field imm24 = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=8, cond=0
    let encoding: u32 = 0x0B000008;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_15_poweroftwominusone_0_0b00000f() {
    // Encoding: 0x0B00000F
    // Test aarch32_BL_i_A1_A field imm24 = 15 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=15
    let encoding: u32 = 0x0B00000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_16_poweroftwo_0_0b000010() {
    // Encoding: 0x0B000010
    // Test aarch32_BL_i_A1_A field imm24 = 16 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=16, cond=0
    let encoding: u32 = 0x0B000010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_31_poweroftwominusone_0_0b00001f() {
    // Encoding: 0x0B00001F
    // Test aarch32_BL_i_A1_A field imm24 = 31 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm24=31, cond=0
    let encoding: u32 = 0x0B00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_32_poweroftwo_0_0b000020() {
    // Encoding: 0x0B000020
    // Test aarch32_BL_i_A1_A field imm24 = 32 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=32
    let encoding: u32 = 0x0B000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_63_poweroftwominusone_0_0b00003f() {
    // Encoding: 0x0B00003F
    // Test aarch32_BL_i_A1_A field imm24 = 63 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=63
    let encoding: u32 = 0x0B00003F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_64_poweroftwo_0_0b000040() {
    // Encoding: 0x0B000040
    // Test aarch32_BL_i_A1_A field imm24 = 64 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=64, cond=0
    let encoding: u32 = 0x0B000040;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_127_poweroftwominusone_0_0b00007f() {
    // Encoding: 0x0B00007F
    // Test aarch32_BL_i_A1_A field imm24 = 127 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm24=127, cond=0
    let encoding: u32 = 0x0B00007F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_128_poweroftwo_0_0b000080() {
    // Encoding: 0x0B000080
    // Test aarch32_BL_i_A1_A field imm24 = 128 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=128
    let encoding: u32 = 0x0B000080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_255_poweroftwominusone_0_0b0000ff() {
    // Encoding: 0x0B0000FF
    // Test aarch32_BL_i_A1_A field imm24 = 255 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=255
    let encoding: u32 = 0x0B0000FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_256_poweroftwo_0_0b000100() {
    // Encoding: 0x0B000100
    // Test aarch32_BL_i_A1_A field imm24 = 256 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=256
    let encoding: u32 = 0x0B000100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_511_poweroftwominusone_0_0b0001ff() {
    // Encoding: 0x0B0001FF
    // Test aarch32_BL_i_A1_A field imm24 = 511 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm24=511, cond=0
    let encoding: u32 = 0x0B0001FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_512_poweroftwo_0_0b000200() {
    // Encoding: 0x0B000200
    // Test aarch32_BL_i_A1_A field imm24 = 512 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=512
    let encoding: u32 = 0x0B000200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 1023, boundary: PowerOfTwoMinusOne }
/// 2^10 - 1 = 1023
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_1023_poweroftwominusone_0_0b0003ff() {
    // Encoding: 0x0B0003FF
    // Test aarch32_BL_i_A1_A field imm24 = 1023 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=1023
    let encoding: u32 = 0x0B0003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_1024_poweroftwo_0_0b000400() {
    // Encoding: 0x0B000400
    // Test aarch32_BL_i_A1_A field imm24 = 1024 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=1024, cond=0
    let encoding: u32 = 0x0B000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 2047, boundary: PowerOfTwoMinusOne }
/// 2^11 - 1 = 2047
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_2047_poweroftwominusone_0_0b0007ff() {
    // Encoding: 0x0B0007FF
    // Test aarch32_BL_i_A1_A field imm24 = 2047 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=2047
    let encoding: u32 = 0x0B0007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 2048, boundary: PowerOfTwo }
/// power of 2 (2^11 = 2048)
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_2048_poweroftwo_0_0b000800() {
    // Encoding: 0x0B000800
    // Test aarch32_BL_i_A1_A field imm24 = 2048 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=2048, cond=0
    let encoding: u32 = 0x0B000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 4095, boundary: PowerOfTwoMinusOne }
/// 2^12 - 1 = 4095
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_4095_poweroftwominusone_0_0b000fff() {
    // Encoding: 0x0B000FFF
    // Test aarch32_BL_i_A1_A field imm24 = 4095 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=4095
    let encoding: u32 = 0x0B000FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 4096, boundary: PowerOfTwo }
/// power of 2 (2^12 = 4096)
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_4096_poweroftwo_0_0b001000() {
    // Encoding: 0x0B001000
    // Test aarch32_BL_i_A1_A field imm24 = 4096 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=4096, cond=0
    let encoding: u32 = 0x0B001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 8191, boundary: PowerOfTwoMinusOne }
/// 2^13 - 1 = 8191
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_8191_poweroftwominusone_0_0b001fff() {
    // Encoding: 0x0B001FFF
    // Test aarch32_BL_i_A1_A field imm24 = 8191 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=8191
    let encoding: u32 = 0x0B001FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 8192, boundary: PowerOfTwo }
/// power of 2 (2^13 = 8192)
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_8192_poweroftwo_0_0b002000() {
    // Encoding: 0x0B002000
    // Test aarch32_BL_i_A1_A field imm24 = 8192 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=8192
    let encoding: u32 = 0x0B002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 16383, boundary: PowerOfTwoMinusOne }
/// 2^14 - 1 = 16383
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_16383_poweroftwominusone_0_0b003fff() {
    // Encoding: 0x0B003FFF
    // Test aarch32_BL_i_A1_A field imm24 = 16383 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=16383
    let encoding: u32 = 0x0B003FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 16384, boundary: PowerOfTwo }
/// power of 2 (2^14 = 16384)
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_16384_poweroftwo_0_0b004000() {
    // Encoding: 0x0B004000
    // Test aarch32_BL_i_A1_A field imm24 = 16384 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=16384, cond=0
    let encoding: u32 = 0x0B004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 32767, boundary: PowerOfTwoMinusOne }
/// 2^15 - 1 = 32767
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_32767_poweroftwominusone_0_0b007fff() {
    // Encoding: 0x0B007FFF
    // Test aarch32_BL_i_A1_A field imm24 = 32767 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=32767
    let encoding: u32 = 0x0B007FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 32768, boundary: PowerOfTwo }
/// power of 2 (2^15 = 32768)
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_32768_poweroftwo_0_0b008000() {
    // Encoding: 0x0B008000
    // Test aarch32_BL_i_A1_A field imm24 = 32768 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=32768
    let encoding: u32 = 0x0B008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 65535, boundary: PowerOfTwoMinusOne }
/// 2^16 - 1 = 65535
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_65535_poweroftwominusone_0_0b00ffff() {
    // Encoding: 0x0B00FFFF
    // Test aarch32_BL_i_A1_A field imm24 = 65535 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=65535
    let encoding: u32 = 0x0B00FFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 65536, boundary: PowerOfTwo }
/// power of 2 (2^16 = 65536)
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_65536_poweroftwo_0_0b010000() {
    // Encoding: 0x0B010000
    // Test aarch32_BL_i_A1_A field imm24 = 65536 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=65536
    let encoding: u32 = 0x0B010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 131071, boundary: PowerOfTwoMinusOne }
/// 2^17 - 1 = 131071
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_131071_poweroftwominusone_0_0b01ffff() {
    // Encoding: 0x0B01FFFF
    // Test aarch32_BL_i_A1_A field imm24 = 131071 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=131071
    let encoding: u32 = 0x0B01FFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 131072, boundary: PowerOfTwo }
/// power of 2 (2^17 = 131072)
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_131072_poweroftwo_0_0b020000() {
    // Encoding: 0x0B020000
    // Test aarch32_BL_i_A1_A field imm24 = 131072 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=131072
    let encoding: u32 = 0x0B020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 262143, boundary: PowerOfTwoMinusOne }
/// 2^18 - 1 = 262143
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_262143_poweroftwominusone_0_0b03ffff() {
    // Encoding: 0x0B03FFFF
    // Test aarch32_BL_i_A1_A field imm24 = 262143 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=262143
    let encoding: u32 = 0x0B03FFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 262144, boundary: PowerOfTwo }
/// power of 2 (2^18 = 262144)
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_262144_poweroftwo_0_0b040000() {
    // Encoding: 0x0B040000
    // Test aarch32_BL_i_A1_A field imm24 = 262144 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=262144, cond=0
    let encoding: u32 = 0x0B040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 524287, boundary: PowerOfTwoMinusOne }
/// 2^19 - 1 = 524287
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_524287_poweroftwominusone_0_0b07ffff() {
    // Encoding: 0x0B07FFFF
    // Test aarch32_BL_i_A1_A field imm24 = 524287 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=524287
    let encoding: u32 = 0x0B07FFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 524288, boundary: PowerOfTwo }
/// power of 2 (2^19 = 524288)
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_524288_poweroftwo_0_0b080000() {
    // Encoding: 0x0B080000
    // Test aarch32_BL_i_A1_A field imm24 = 524288 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=524288
    let encoding: u32 = 0x0B080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 1048575, boundary: PowerOfTwoMinusOne }
/// 2^20 - 1 = 1048575
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_1048575_poweroftwominusone_0_0b0fffff() {
    // Encoding: 0x0B0FFFFF
    // Test aarch32_BL_i_A1_A field imm24 = 1048575 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm24=1048575, cond=0
    let encoding: u32 = 0x0B0FFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 1048576, boundary: PowerOfTwo }
/// power of 2 (2^20 = 1048576)
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_1048576_poweroftwo_0_0b100000() {
    // Encoding: 0x0B100000
    // Test aarch32_BL_i_A1_A field imm24 = 1048576 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=1048576
    let encoding: u32 = 0x0B100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 2097151, boundary: PowerOfTwoMinusOne }
/// 2^21 - 1 = 2097151
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_2097151_poweroftwominusone_0_0b1fffff() {
    // Encoding: 0x0B1FFFFF
    // Test aarch32_BL_i_A1_A field imm24 = 2097151 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=2097151
    let encoding: u32 = 0x0B1FFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 2097152, boundary: PowerOfTwo }
/// power of 2 (2^21 = 2097152)
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_2097152_poweroftwo_0_0b200000() {
    // Encoding: 0x0B200000
    // Test aarch32_BL_i_A1_A field imm24 = 2097152 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=2097152, cond=0
    let encoding: u32 = 0x0B200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 4194303, boundary: PowerOfTwoMinusOne }
/// 2^22 - 1 = 4194303
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_4194303_poweroftwominusone_0_0b3fffff() {
    // Encoding: 0x0B3FFFFF
    // Test aarch32_BL_i_A1_A field imm24 = 4194303 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm24=4194303, cond=0
    let encoding: u32 = 0x0B3FFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 4194304, boundary: PowerOfTwo }
/// power of 2 (2^22 = 4194304)
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_4194304_poweroftwo_0_0b400000() {
    // Encoding: 0x0B400000
    // Test aarch32_BL_i_A1_A field imm24 = 4194304 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=4194304
    let encoding: u32 = 0x0B400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 8388607, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (8388607)
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_8388607_poweroftwominusone_0_0b7fffff() {
    // Encoding: 0x0B7FFFFF
    // Test aarch32_BL_i_A1_A field imm24 = 8388607 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=8388607
    let encoding: u32 = 0x0B7FFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 8388608, boundary: PowerOfTwo }
/// power of 2 (2^23 = 8388608)
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_8388608_poweroftwo_0_0b800000() {
    // Encoding: 0x0B800000
    // Test aarch32_BL_i_A1_A field imm24 = 8388608 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=8388608
    let encoding: u32 = 0x0B800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 16777215, boundary: Max }
/// maximum immediate (16777215)
#[test]
fn test_aarch32_bl_i_a1_a_field_imm24_16777215_max_0_0bffffff() {
    // Encoding: 0x0BFFFFFF
    // Test aarch32_BL_i_A1_A field imm24 = 16777215 (Max)
    // ISET: A32
    // Fields: cond=0, imm24=16777215
    let encoding: u32 = 0x0BFFFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_bl_i_a1_a_combo_0_0_0b000000() {
    // Encoding: 0x0B000000
    // Test aarch32_BL_i_A1_A field combination: cond=0, imm24=0
    // ISET: A32
    // Fields: imm24=0, cond=0
    let encoding: u32 = 0x0B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_bl_i_a1_a_special_cond_0_condition_eq_0_0b000000() {
    // Encoding: 0x0B000000
    // Test aarch32_BL_i_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: cond=0, imm24=0
    let encoding: u32 = 0x0B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_bl_i_a1_a_special_cond_1_condition_ne_0_1b000000() {
    // Encoding: 0x1B000000
    // Test aarch32_BL_i_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, imm24=0
    let encoding: u32 = 0x1B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_bl_i_a1_a_special_cond_2_condition_cs_hs_0_2b000000() {
    // Encoding: 0x2B000000
    // Test aarch32_BL_i_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, imm24=0
    let encoding: u32 = 0x2B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_bl_i_a1_a_special_cond_3_condition_cc_lo_0_3b000000() {
    // Encoding: 0x3B000000
    // Test aarch32_BL_i_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: imm24=0, cond=3
    let encoding: u32 = 0x3B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_bl_i_a1_a_special_cond_4_condition_mi_0_4b000000() {
    // Encoding: 0x4B000000
    // Test aarch32_BL_i_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: imm24=0, cond=4
    let encoding: u32 = 0x4B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_bl_i_a1_a_special_cond_5_condition_pl_0_5b000000() {
    // Encoding: 0x5B000000
    // Test aarch32_BL_i_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, imm24=0
    let encoding: u32 = 0x5B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_bl_i_a1_a_special_cond_6_condition_vs_0_6b000000() {
    // Encoding: 0x6B000000
    // Test aarch32_BL_i_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, imm24=0
    let encoding: u32 = 0x6B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_bl_i_a1_a_special_cond_7_condition_vc_0_7b000000() {
    // Encoding: 0x7B000000
    // Test aarch32_BL_i_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, imm24=0
    let encoding: u32 = 0x7B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_bl_i_a1_a_special_cond_8_condition_hi_0_8b000000() {
    // Encoding: 0x8B000000
    // Test aarch32_BL_i_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, imm24=0
    let encoding: u32 = 0x8B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_bl_i_a1_a_special_cond_9_condition_ls_0_9b000000() {
    // Encoding: 0x9B000000
    // Test aarch32_BL_i_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, imm24=0
    let encoding: u32 = 0x9B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_bl_i_a1_a_special_cond_10_condition_ge_0_ab000000() {
    // Encoding: 0xAB000000
    // Test aarch32_BL_i_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: imm24=0, cond=10
    let encoding: u32 = 0xAB000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_bl_i_a1_a_special_cond_11_condition_lt_0_bb000000() {
    // Encoding: 0xBB000000
    // Test aarch32_BL_i_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, imm24=0
    let encoding: u32 = 0xBB000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_bl_i_a1_a_special_cond_12_condition_gt_0_cb000000() {
    // Encoding: 0xCB000000
    // Test aarch32_BL_i_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: cond=12, imm24=0
    let encoding: u32 = 0xCB000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_bl_i_a1_a_special_cond_13_condition_le_0_db000000() {
    // Encoding: 0xDB000000
    // Test aarch32_BL_i_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: cond=13, imm24=0
    let encoding: u32 = 0xDB000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_bl_i_a1_a_special_cond_14_condition_al_0_eb000000() {
    // Encoding: 0xEB000000
    // Test aarch32_BL_i_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: cond=14, imm24=0
    let encoding: u32 = 0xEB000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_bl_i_a1_a_special_cond_15_condition_nv_0_fb000000() {
    // Encoding: 0xFB000000
    // Test aarch32_BL_i_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, imm24=0
    let encoding: u32 = 0xFB000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_bl_i_a2_a_field_cond_0_min_0_0a000000() {
    // Encoding: 0x0A000000
    // Test aarch32_BL_i_A2_A field cond = 0 (Min)
    // ISET: A32
    // Fields: imm24=0, cond=0, H=0
    let encoding: u32 = 0x0A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_bl_i_a2_a_field_cond_1_poweroftwo_0_1a000000() {
    // Encoding: 0x1A000000
    // Test aarch32_BL_i_A2_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=0, H=0, cond=1
    let encoding: u32 = 0x1A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_bl_i_a2_a_field_cond_2_poweroftwo_0_2a000000() {
    // Encoding: 0x2A000000
    // Test aarch32_BL_i_A2_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=0, cond=2, H=0
    let encoding: u32 = 0x2A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_bl_i_a2_a_field_cond_3_poweroftwo_0_3a000000() {
    // Encoding: 0x3A000000
    // Test aarch32_BL_i_A2_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, imm24=0, H=0
    let encoding: u32 = 0x3A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_bl_i_a2_a_field_cond_4_poweroftwo_0_4a000000() {
    // Encoding: 0x4A000000
    // Test aarch32_BL_i_A2_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=4, H=0, imm24=0
    let encoding: u32 = 0x4A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_bl_i_a2_a_field_cond_5_poweroftwo_0_5a000000() {
    // Encoding: 0x5A000000
    // Test aarch32_BL_i_A2_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: H=0, imm24=0, cond=5
    let encoding: u32 = 0x5A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_bl_i_a2_a_field_cond_6_poweroftwo_0_6a000000() {
    // Encoding: 0x6A000000
    // Test aarch32_BL_i_A2_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=0, H=0, cond=6
    let encoding: u32 = 0x6A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_bl_i_a2_a_field_cond_7_poweroftwo_0_7a000000() {
    // Encoding: 0x7A000000
    // Test aarch32_BL_i_A2_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=0, cond=7, H=0
    let encoding: u32 = 0x7A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_bl_i_a2_a_field_cond_8_poweroftwo_0_8a000000() {
    // Encoding: 0x8A000000
    // Test aarch32_BL_i_A2_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=8, H=0, imm24=0
    let encoding: u32 = 0x8A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_bl_i_a2_a_field_cond_9_poweroftwo_0_9a000000() {
    // Encoding: 0x9A000000
    // Test aarch32_BL_i_A2_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: H=0, cond=9, imm24=0
    let encoding: u32 = 0x9A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_bl_i_a2_a_field_cond_10_poweroftwo_0_aa000000() {
    // Encoding: 0xAA000000
    // Test aarch32_BL_i_A2_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, H=0, imm24=0
    let encoding: u32 = 0xAA000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_bl_i_a2_a_field_cond_11_poweroftwo_0_ba000000() {
    // Encoding: 0xBA000000
    // Test aarch32_BL_i_A2_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=11, H=0, imm24=0
    let encoding: u32 = 0xBA000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_bl_i_a2_a_field_cond_12_poweroftwo_0_ca000000() {
    // Encoding: 0xCA000000
    // Test aarch32_BL_i_A2_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, H=0, imm24=0
    let encoding: u32 = 0xCA000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_bl_i_a2_a_field_cond_13_poweroftwo_0_da000000() {
    // Encoding: 0xDA000000
    // Test aarch32_BL_i_A2_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=13, H=0, imm24=0
    let encoding: u32 = 0xDA000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_bl_i_a2_a_field_cond_14_poweroftwo_0_ea000000() {
    // Encoding: 0xEA000000
    // Test aarch32_BL_i_A2_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=0, cond=14, H=0
    let encoding: u32 = 0xEA000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_bl_i_a2_a_field_cond_15_max_0_fa000000() {
    // Encoding: 0xFA000000
    // Test aarch32_BL_i_A2_A field cond = 15 (Max)
    // ISET: A32
    // Fields: imm24=0, H=0, cond=15
    let encoding: u32 = 0xFA000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field H 24 +: 1`
/// Requirement: FieldBoundary { field: "H", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_bl_i_a2_a_field_h_0_min_0_0a000000() {
    // Encoding: 0x0A000000
    // Test aarch32_BL_i_A2_A field H = 0 (Min)
    // ISET: A32
    // Fields: H=0, imm24=0, cond=0
    let encoding: u32 = 0x0A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field H 24 +: 1`
/// Requirement: FieldBoundary { field: "H", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_bl_i_a2_a_field_h_1_max_0_0b000000() {
    // Encoding: 0x0B000000
    // Test aarch32_BL_i_A2_A field H = 1 (Max)
    // ISET: A32
    // Fields: imm24=0, H=1, cond=0
    let encoding: u32 = 0x0B000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_0_zero_0_0a000000() {
    // Encoding: 0x0A000000
    // Test aarch32_BL_i_A2_A field imm24 = 0 (Zero)
    // ISET: A32
    // Fields: imm24=0, cond=0, H=0
    let encoding: u32 = 0x0A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_1_poweroftwo_0_0a000001() {
    // Encoding: 0x0A000001
    // Test aarch32_BL_i_A2_A field imm24 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=1, H=0
    let encoding: u32 = 0x0A000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_3_poweroftwominusone_0_0a000003() {
    // Encoding: 0x0A000003
    // Test aarch32_BL_i_A2_A field imm24 = 3 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm24=3, cond=0, H=0
    let encoding: u32 = 0x0A000003;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_4_poweroftwo_0_0a000004() {
    // Encoding: 0x0A000004
    // Test aarch32_BL_i_A2_A field imm24 = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, H=0, imm24=4
    let encoding: u32 = 0x0A000004;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_7_poweroftwominusone_0_0a000007() {
    // Encoding: 0x0A000007
    // Test aarch32_BL_i_A2_A field imm24 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, H=0, imm24=7
    let encoding: u32 = 0x0A000007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_8_poweroftwo_0_0a000008() {
    // Encoding: 0x0A000008
    // Test aarch32_BL_i_A2_A field imm24 = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: H=0, cond=0, imm24=8
    let encoding: u32 = 0x0A000008;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_15_poweroftwominusone_0_0a00000f() {
    // Encoding: 0x0A00000F
    // Test aarch32_BL_i_A2_A field imm24 = 15 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: H=0, cond=0, imm24=15
    let encoding: u32 = 0x0A00000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_16_poweroftwo_0_0a000010() {
    // Encoding: 0x0A000010
    // Test aarch32_BL_i_A2_A field imm24 = 16 (PowerOfTwo)
    // ISET: A32
    // Fields: H=0, imm24=16, cond=0
    let encoding: u32 = 0x0A000010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_31_poweroftwominusone_0_0a00001f() {
    // Encoding: 0x0A00001F
    // Test aarch32_BL_i_A2_A field imm24 = 31 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: H=0, imm24=31, cond=0
    let encoding: u32 = 0x0A00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_32_poweroftwo_0_0a000020() {
    // Encoding: 0x0A000020
    // Test aarch32_BL_i_A2_A field imm24 = 32 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, H=0, imm24=32
    let encoding: u32 = 0x0A000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_63_poweroftwominusone_0_0a00003f() {
    // Encoding: 0x0A00003F
    // Test aarch32_BL_i_A2_A field imm24 = 63 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, H=0, imm24=63
    let encoding: u32 = 0x0A00003F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_64_poweroftwo_0_0a000040() {
    // Encoding: 0x0A000040
    // Test aarch32_BL_i_A2_A field imm24 = 64 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=64, cond=0, H=0
    let encoding: u32 = 0x0A000040;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_127_poweroftwominusone_0_0a00007f() {
    // Encoding: 0x0A00007F
    // Test aarch32_BL_i_A2_A field imm24 = 127 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm24=127, H=0, cond=0
    let encoding: u32 = 0x0A00007F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_128_poweroftwo_0_0a000080() {
    // Encoding: 0x0A000080
    // Test aarch32_BL_i_A2_A field imm24 = 128 (PowerOfTwo)
    // ISET: A32
    // Fields: H=0, imm24=128, cond=0
    let encoding: u32 = 0x0A000080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_255_poweroftwominusone_0_0a0000ff() {
    // Encoding: 0x0A0000FF
    // Test aarch32_BL_i_A2_A field imm24 = 255 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm24=255, cond=0, H=0
    let encoding: u32 = 0x0A0000FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_256_poweroftwo_0_0a000100() {
    // Encoding: 0x0A000100
    // Test aarch32_BL_i_A2_A field imm24 = 256 (PowerOfTwo)
    // ISET: A32
    // Fields: H=0, cond=0, imm24=256
    let encoding: u32 = 0x0A000100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_511_poweroftwominusone_0_0a0001ff() {
    // Encoding: 0x0A0001FF
    // Test aarch32_BL_i_A2_A field imm24 = 511 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm24=511, cond=0, H=0
    let encoding: u32 = 0x0A0001FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_512_poweroftwo_0_0a000200() {
    // Encoding: 0x0A000200
    // Test aarch32_BL_i_A2_A field imm24 = 512 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, H=0, imm24=512
    let encoding: u32 = 0x0A000200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 1023, boundary: PowerOfTwoMinusOne }
/// 2^10 - 1 = 1023
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_1023_poweroftwominusone_0_0a0003ff() {
    // Encoding: 0x0A0003FF
    // Test aarch32_BL_i_A2_A field imm24 = 1023 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm24=1023, cond=0, H=0
    let encoding: u32 = 0x0A0003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_1024_poweroftwo_0_0a000400() {
    // Encoding: 0x0A000400
    // Test aarch32_BL_i_A2_A field imm24 = 1024 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=1024, cond=0, H=0
    let encoding: u32 = 0x0A000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 2047, boundary: PowerOfTwoMinusOne }
/// 2^11 - 1 = 2047
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_2047_poweroftwominusone_0_0a0007ff() {
    // Encoding: 0x0A0007FF
    // Test aarch32_BL_i_A2_A field imm24 = 2047 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=2047, H=0
    let encoding: u32 = 0x0A0007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 2048, boundary: PowerOfTwo }
/// power of 2 (2^11 = 2048)
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_2048_poweroftwo_0_0a000800() {
    // Encoding: 0x0A000800
    // Test aarch32_BL_i_A2_A field imm24 = 2048 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, H=0, imm24=2048
    let encoding: u32 = 0x0A000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 4095, boundary: PowerOfTwoMinusOne }
/// 2^12 - 1 = 4095
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_4095_poweroftwominusone_0_0a000fff() {
    // Encoding: 0x0A000FFF
    // Test aarch32_BL_i_A2_A field imm24 = 4095 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm24=4095, H=0, cond=0
    let encoding: u32 = 0x0A000FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 4096, boundary: PowerOfTwo }
/// power of 2 (2^12 = 4096)
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_4096_poweroftwo_0_0a001000() {
    // Encoding: 0x0A001000
    // Test aarch32_BL_i_A2_A field imm24 = 4096 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, H=0, imm24=4096
    let encoding: u32 = 0x0A001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 8191, boundary: PowerOfTwoMinusOne }
/// 2^13 - 1 = 8191
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_8191_poweroftwominusone_0_0a001fff() {
    // Encoding: 0x0A001FFF
    // Test aarch32_BL_i_A2_A field imm24 = 8191 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, H=0, imm24=8191
    let encoding: u32 = 0x0A001FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 8192, boundary: PowerOfTwo }
/// power of 2 (2^13 = 8192)
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_8192_poweroftwo_0_0a002000() {
    // Encoding: 0x0A002000
    // Test aarch32_BL_i_A2_A field imm24 = 8192 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=8192, cond=0, H=0
    let encoding: u32 = 0x0A002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 16383, boundary: PowerOfTwoMinusOne }
/// 2^14 - 1 = 16383
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_16383_poweroftwominusone_0_0a003fff() {
    // Encoding: 0x0A003FFF
    // Test aarch32_BL_i_A2_A field imm24 = 16383 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: H=0, imm24=16383, cond=0
    let encoding: u32 = 0x0A003FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 16384, boundary: PowerOfTwo }
/// power of 2 (2^14 = 16384)
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_16384_poweroftwo_0_0a004000() {
    // Encoding: 0x0A004000
    // Test aarch32_BL_i_A2_A field imm24 = 16384 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, H=0, imm24=16384
    let encoding: u32 = 0x0A004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 32767, boundary: PowerOfTwoMinusOne }
/// 2^15 - 1 = 32767
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_32767_poweroftwominusone_0_0a007fff() {
    // Encoding: 0x0A007FFF
    // Test aarch32_BL_i_A2_A field imm24 = 32767 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: H=0, cond=0, imm24=32767
    let encoding: u32 = 0x0A007FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 32768, boundary: PowerOfTwo }
/// power of 2 (2^15 = 32768)
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_32768_poweroftwo_0_0a008000() {
    // Encoding: 0x0A008000
    // Test aarch32_BL_i_A2_A field imm24 = 32768 (PowerOfTwo)
    // ISET: A32
    // Fields: H=0, imm24=32768, cond=0
    let encoding: u32 = 0x0A008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 65535, boundary: PowerOfTwoMinusOne }
/// 2^16 - 1 = 65535
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_65535_poweroftwominusone_0_0a00ffff() {
    // Encoding: 0x0A00FFFF
    // Test aarch32_BL_i_A2_A field imm24 = 65535 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm24=65535, cond=0, H=0
    let encoding: u32 = 0x0A00FFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 65536, boundary: PowerOfTwo }
/// power of 2 (2^16 = 65536)
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_65536_poweroftwo_0_0a010000() {
    // Encoding: 0x0A010000
    // Test aarch32_BL_i_A2_A field imm24 = 65536 (PowerOfTwo)
    // ISET: A32
    // Fields: H=0, cond=0, imm24=65536
    let encoding: u32 = 0x0A010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 131071, boundary: PowerOfTwoMinusOne }
/// 2^17 - 1 = 131071
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_131071_poweroftwominusone_0_0a01ffff() {
    // Encoding: 0x0A01FFFF
    // Test aarch32_BL_i_A2_A field imm24 = 131071 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: H=0, imm24=131071, cond=0
    let encoding: u32 = 0x0A01FFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 131072, boundary: PowerOfTwo }
/// power of 2 (2^17 = 131072)
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_131072_poweroftwo_0_0a020000() {
    // Encoding: 0x0A020000
    // Test aarch32_BL_i_A2_A field imm24 = 131072 (PowerOfTwo)
    // ISET: A32
    // Fields: H=0, cond=0, imm24=131072
    let encoding: u32 = 0x0A020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 262143, boundary: PowerOfTwoMinusOne }
/// 2^18 - 1 = 262143
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_262143_poweroftwominusone_0_0a03ffff() {
    // Encoding: 0x0A03FFFF
    // Test aarch32_BL_i_A2_A field imm24 = 262143 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: H=0, imm24=262143, cond=0
    let encoding: u32 = 0x0A03FFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 262144, boundary: PowerOfTwo }
/// power of 2 (2^18 = 262144)
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_262144_poweroftwo_0_0a040000() {
    // Encoding: 0x0A040000
    // Test aarch32_BL_i_A2_A field imm24 = 262144 (PowerOfTwo)
    // ISET: A32
    // Fields: H=0, cond=0, imm24=262144
    let encoding: u32 = 0x0A040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 524287, boundary: PowerOfTwoMinusOne }
/// 2^19 - 1 = 524287
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_524287_poweroftwominusone_0_0a07ffff() {
    // Encoding: 0x0A07FFFF
    // Test aarch32_BL_i_A2_A field imm24 = 524287 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm24=524287, cond=0, H=0
    let encoding: u32 = 0x0A07FFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 524288, boundary: PowerOfTwo }
/// power of 2 (2^19 = 524288)
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_524288_poweroftwo_0_0a080000() {
    // Encoding: 0x0A080000
    // Test aarch32_BL_i_A2_A field imm24 = 524288 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, H=0, imm24=524288
    let encoding: u32 = 0x0A080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 1048575, boundary: PowerOfTwoMinusOne }
/// 2^20 - 1 = 1048575
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_1048575_poweroftwominusone_0_0a0fffff() {
    // Encoding: 0x0A0FFFFF
    // Test aarch32_BL_i_A2_A field imm24 = 1048575 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, H=0, imm24=1048575
    let encoding: u32 = 0x0A0FFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 1048576, boundary: PowerOfTwo }
/// power of 2 (2^20 = 1048576)
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_1048576_poweroftwo_0_0a100000() {
    // Encoding: 0x0A100000
    // Test aarch32_BL_i_A2_A field imm24 = 1048576 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=1048576, H=0, cond=0
    let encoding: u32 = 0x0A100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 2097151, boundary: PowerOfTwoMinusOne }
/// 2^21 - 1 = 2097151
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_2097151_poweroftwominusone_0_0a1fffff() {
    // Encoding: 0x0A1FFFFF
    // Test aarch32_BL_i_A2_A field imm24 = 2097151 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm24=2097151, cond=0, H=0
    let encoding: u32 = 0x0A1FFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 2097152, boundary: PowerOfTwo }
/// power of 2 (2^21 = 2097152)
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_2097152_poweroftwo_0_0a200000() {
    // Encoding: 0x0A200000
    // Test aarch32_BL_i_A2_A field imm24 = 2097152 (PowerOfTwo)
    // ISET: A32
    // Fields: H=0, cond=0, imm24=2097152
    let encoding: u32 = 0x0A200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 4194303, boundary: PowerOfTwoMinusOne }
/// 2^22 - 1 = 4194303
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_4194303_poweroftwominusone_0_0a3fffff() {
    // Encoding: 0x0A3FFFFF
    // Test aarch32_BL_i_A2_A field imm24 = 4194303 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm24=4194303, H=0, cond=0
    let encoding: u32 = 0x0A3FFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 4194304, boundary: PowerOfTwo }
/// power of 2 (2^22 = 4194304)
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_4194304_poweroftwo_0_0a400000() {
    // Encoding: 0x0A400000
    // Test aarch32_BL_i_A2_A field imm24 = 4194304 (PowerOfTwo)
    // ISET: A32
    // Fields: H=0, cond=0, imm24=4194304
    let encoding: u32 = 0x0A400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 8388607, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (8388607)
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_8388607_poweroftwominusone_0_0a7fffff() {
    // Encoding: 0x0A7FFFFF
    // Test aarch32_BL_i_A2_A field imm24 = 8388607 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, H=0, imm24=8388607
    let encoding: u32 = 0x0A7FFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 8388608, boundary: PowerOfTwo }
/// power of 2 (2^23 = 8388608)
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_8388608_poweroftwo_0_0a800000() {
    // Encoding: 0x0A800000
    // Test aarch32_BL_i_A2_A field imm24 = 8388608 (PowerOfTwo)
    // ISET: A32
    // Fields: H=0, cond=0, imm24=8388608
    let encoding: u32 = 0x0A800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 16777215, boundary: Max }
/// maximum immediate (16777215)
#[test]
fn test_aarch32_bl_i_a2_a_field_imm24_16777215_max_0_0affffff() {
    // Encoding: 0x0AFFFFFF
    // Test aarch32_BL_i_A2_A field imm24 = 16777215 (Max)
    // ISET: A32
    // Fields: imm24=16777215, H=0, cond=0
    let encoding: u32 = 0x0AFFFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_bl_i_a2_a_combo_0_0_0a000000() {
    // Encoding: 0x0A000000
    // Test aarch32_BL_i_A2_A field combination: cond=0, H=0, imm24=0
    // ISET: A32
    // Fields: cond=0, imm24=0, H=0
    let encoding: u32 = 0x0A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_bl_i_a2_a_special_cond_0_condition_eq_0_0a000000() {
    // Encoding: 0x0A000000
    // Test aarch32_BL_i_A2_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: imm24=0, H=0, cond=0
    let encoding: u32 = 0x0A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_bl_i_a2_a_special_cond_1_condition_ne_0_1a000000() {
    // Encoding: 0x1A000000
    // Test aarch32_BL_i_A2_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: imm24=0, H=0, cond=1
    let encoding: u32 = 0x1A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_bl_i_a2_a_special_cond_2_condition_cs_hs_0_2a000000() {
    // Encoding: 0x2A000000
    // Test aarch32_BL_i_A2_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: H=0, cond=2, imm24=0
    let encoding: u32 = 0x2A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_bl_i_a2_a_special_cond_3_condition_cc_lo_0_3a000000() {
    // Encoding: 0x3A000000
    // Test aarch32_BL_i_A2_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, H=0, imm24=0
    let encoding: u32 = 0x3A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_bl_i_a2_a_special_cond_4_condition_mi_0_4a000000() {
    // Encoding: 0x4A000000
    // Test aarch32_BL_i_A2_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: imm24=0, H=0, cond=4
    let encoding: u32 = 0x4A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_bl_i_a2_a_special_cond_5_condition_pl_0_5a000000() {
    // Encoding: 0x5A000000
    // Test aarch32_BL_i_A2_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: H=0, cond=5, imm24=0
    let encoding: u32 = 0x5A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_bl_i_a2_a_special_cond_6_condition_vs_0_6a000000() {
    // Encoding: 0x6A000000
    // Test aarch32_BL_i_A2_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: H=0, imm24=0, cond=6
    let encoding: u32 = 0x6A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_bl_i_a2_a_special_cond_7_condition_vc_0_7a000000() {
    // Encoding: 0x7A000000
    // Test aarch32_BL_i_A2_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: H=0, imm24=0, cond=7
    let encoding: u32 = 0x7A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_bl_i_a2_a_special_cond_8_condition_hi_0_8a000000() {
    // Encoding: 0x8A000000
    // Test aarch32_BL_i_A2_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, H=0, imm24=0
    let encoding: u32 = 0x8A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_bl_i_a2_a_special_cond_9_condition_ls_0_9a000000() {
    // Encoding: 0x9A000000
    // Test aarch32_BL_i_A2_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, H=0, imm24=0
    let encoding: u32 = 0x9A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_bl_i_a2_a_special_cond_10_condition_ge_0_aa000000() {
    // Encoding: 0xAA000000
    // Test aarch32_BL_i_A2_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, H=0, imm24=0
    let encoding: u32 = 0xAA000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_bl_i_a2_a_special_cond_11_condition_lt_0_ba000000() {
    // Encoding: 0xBA000000
    // Test aarch32_BL_i_A2_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: imm24=0, cond=11, H=0
    let encoding: u32 = 0xBA000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_bl_i_a2_a_special_cond_12_condition_gt_0_ca000000() {
    // Encoding: 0xCA000000
    // Test aarch32_BL_i_A2_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: cond=12, H=0, imm24=0
    let encoding: u32 = 0xCA000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_bl_i_a2_a_special_cond_13_condition_le_0_da000000() {
    // Encoding: 0xDA000000
    // Test aarch32_BL_i_A2_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: H=0, imm24=0, cond=13
    let encoding: u32 = 0xDA000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_bl_i_a2_a_special_cond_14_condition_al_0_ea000000() {
    // Encoding: 0xEA000000
    // Test aarch32_BL_i_A2_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: H=0, cond=14, imm24=0
    let encoding: u32 = 0xEA000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_A2_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_bl_i_a2_a_special_cond_15_condition_nv_0_fa000000() {
    // Encoding: 0xFA000000
    // Test aarch32_BL_i_A2_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: H=0, imm24=0, cond=15
    let encoding: u32 = 0xFA000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field S 26 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_bl_i_t1_a_field_s_0_min_d000_f000d000() {
    // Thumb encoding (32): 0xF000D000
    // Test aarch32_BL_i_T1_A field S = 0 (Min)
    // ISET: T32
    // Fields: imm10=0, imm11=0, J2=0, S=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field S 26 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_bl_i_t1_a_field_s_1_max_d000_f400d000() {
    // Thumb encoding (32): 0xF400D000
    // Test aarch32_BL_i_T1_A field S = 1 (Max)
    // ISET: T32
    // Fields: imm11=0, J1=0, J2=0, S=1, imm10=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF400D000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_bl_i_t1_a_field_imm10_0_zero_d000_f000d000() {
    // Thumb encoding (32): 0xF000D000
    // Test aarch32_BL_i_T1_A field imm10 = 0 (Zero)
    // ISET: T32
    // Fields: J2=0, imm10=0, S=0, J1=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_bl_i_t1_a_field_imm10_1_poweroftwo_d000_f001d000() {
    // Thumb encoding (32): 0xF001D000
    // Test aarch32_BL_i_T1_A field imm10 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm11=0, imm10=1, J1=0, S=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF001D000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_bl_i_t1_a_field_imm10_3_poweroftwominusone_d000_f003d000() {
    // Thumb encoding (32): 0xF003D000
    // Test aarch32_BL_i_T1_A field imm10 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm11=0, J2=0, J1=0, S=0, imm10=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF003D000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_bl_i_t1_a_field_imm10_4_poweroftwo_d000_f004d000() {
    // Thumb encoding (32): 0xF004D000
    // Test aarch32_BL_i_T1_A field imm10 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, imm11=0, J1=0, imm10=4, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF004D000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_bl_i_t1_a_field_imm10_7_poweroftwominusone_d000_f007d000() {
    // Thumb encoding (32): 0xF007D000
    // Test aarch32_BL_i_T1_A field imm10 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm11=0, S=0, imm10=7, J1=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF007D000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_bl_i_t1_a_field_imm10_8_poweroftwo_d000_f008d000() {
    // Thumb encoding (32): 0xF008D000
    // Test aarch32_BL_i_T1_A field imm10 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: imm11=0, imm10=8, S=0, J2=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF008D000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_bl_i_t1_a_field_imm10_15_poweroftwominusone_d000_f00fd000() {
    // Thumb encoding (32): 0xF00FD000
    // Test aarch32_BL_i_T1_A field imm10 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm10=15, J1=0, imm11=0, J2=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF00FD000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_bl_i_t1_a_field_imm10_16_poweroftwo_d000_f010d000() {
    // Thumb encoding (32): 0xF010D000
    // Test aarch32_BL_i_T1_A field imm10 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: J1=0, imm10=16, S=0, imm11=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF010D000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_bl_i_t1_a_field_imm10_31_poweroftwominusone_d000_f01fd000() {
    // Thumb encoding (32): 0xF01FD000
    // Test aarch32_BL_i_T1_A field imm10 = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm10=31, S=0, J1=0, J2=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF01FD000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_bl_i_t1_a_field_imm10_32_poweroftwo_d000_f020d000() {
    // Thumb encoding (32): 0xF020D000
    // Test aarch32_BL_i_T1_A field imm10 = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: imm10=32, J1=0, S=0, J2=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF020D000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_bl_i_t1_a_field_imm10_63_poweroftwominusone_d000_f03fd000() {
    // Thumb encoding (32): 0xF03FD000
    // Test aarch32_BL_i_T1_A field imm10 = 63 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm10=63, J1=0, S=0, imm11=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF03FD000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_bl_i_t1_a_field_imm10_64_poweroftwo_d000_f040d000() {
    // Thumb encoding (32): 0xF040D000
    // Test aarch32_BL_i_T1_A field imm10 = 64 (PowerOfTwo)
    // ISET: T32
    // Fields: J1=0, S=0, imm11=0, imm10=64, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF040D000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch32_bl_i_t1_a_field_imm10_127_poweroftwominusone_d000_f07fd000() {
    // Thumb encoding (32): 0xF07FD000
    // Test aarch32_BL_i_T1_A field imm10 = 127 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: S=0, imm10=127, J1=0, J2=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF07FD000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_bl_i_t1_a_field_imm10_128_poweroftwo_d000_f080d000() {
    // Thumb encoding (32): 0xF080D000
    // Test aarch32_BL_i_T1_A field imm10 = 128 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, imm10=128, J1=0, J2=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF080D000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch32_bl_i_t1_a_field_imm10_255_poweroftwominusone_d000_f0ffd000() {
    // Thumb encoding (32): 0xF0FFD000
    // Test aarch32_BL_i_T1_A field imm10 = 255 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: J1=0, imm10=255, S=0, J2=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0FFD000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch32_bl_i_t1_a_field_imm10_256_poweroftwo_d000_f100d000() {
    // Thumb encoding (32): 0xF100D000
    // Test aarch32_BL_i_T1_A field imm10 = 256 (PowerOfTwo)
    // ISET: T32
    // Fields: imm10=256, J2=0, S=0, imm11=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF100D000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 511, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (511)
#[test]
fn test_aarch32_bl_i_t1_a_field_imm10_511_poweroftwominusone_d000_f1ffd000() {
    // Thumb encoding (32): 0xF1FFD000
    // Test aarch32_BL_i_T1_A field imm10 = 511 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: J1=0, imm10=511, imm11=0, S=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1FFD000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch32_bl_i_t1_a_field_imm10_512_poweroftwo_d000_f200d000() {
    // Thumb encoding (32): 0xF200D000
    // Test aarch32_BL_i_T1_A field imm10 = 512 (PowerOfTwo)
    // ISET: T32
    // Fields: imm10=512, imm11=0, J1=0, S=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF200D000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 1023, boundary: Max }
/// maximum immediate (1023)
#[test]
fn test_aarch32_bl_i_t1_a_field_imm10_1023_max_d000_f3ffd000() {
    // Thumb encoding (32): 0xF3FFD000
    // Test aarch32_BL_i_T1_A field imm10 = 1023 (Max)
    // ISET: T32
    // Fields: J1=0, imm10=1023, S=0, imm11=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF3FFD000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field J1 13 +: 1`
/// Requirement: FieldBoundary { field: "J1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_bl_i_t1_a_field_j1_0_min_d000_f000d000() {
    // Thumb encoding (32): 0xF000D000
    // Test aarch32_BL_i_T1_A field J1 = 0 (Min)
    // ISET: T32
    // Fields: S=0, J1=0, J2=0, imm11=0, imm10=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field J1 13 +: 1`
/// Requirement: FieldBoundary { field: "J1", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_bl_i_t1_a_field_j1_1_max_d000_f000f000() {
    // Thumb encoding (32): 0xF000F000
    // Test aarch32_BL_i_T1_A field J1 = 1 (Max)
    // ISET: T32
    // Fields: imm11=0, J2=0, J1=1, S=0, imm10=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000F000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field J2 11 +: 1`
/// Requirement: FieldBoundary { field: "J2", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_bl_i_t1_a_field_j2_0_min_d000_f000d000() {
    // Thumb encoding (32): 0xF000D000
    // Test aarch32_BL_i_T1_A field J2 = 0 (Min)
    // ISET: T32
    // Fields: S=0, imm11=0, J1=0, J2=0, imm10=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field J2 11 +: 1`
/// Requirement: FieldBoundary { field: "J2", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_bl_i_t1_a_field_j2_1_max_d000_f000d800() {
    // Thumb encoding (32): 0xF000D800
    // Test aarch32_BL_i_T1_A field J2 = 1 (Max)
    // ISET: T32
    // Fields: J2=1, J1=0, S=0, imm10=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_bl_i_t1_a_field_imm11_0_zero_d000_f000d000() {
    // Thumb encoding (32): 0xF000D000
    // Test aarch32_BL_i_T1_A field imm11 = 0 (Zero)
    // ISET: T32
    // Fields: S=0, imm10=0, J1=0, imm11=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_bl_i_t1_a_field_imm11_1_poweroftwo_d000_f000d001() {
    // Thumb encoding (32): 0xF000D001
    // Test aarch32_BL_i_T1_A field imm11 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: J1=0, S=0, J2=0, imm11=1, imm10=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D001;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_bl_i_t1_a_field_imm11_3_poweroftwominusone_d000_f000d003() {
    // Thumb encoding (32): 0xF000D003
    // Test aarch32_BL_i_T1_A field imm11 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: J1=0, S=0, imm10=0, J2=0, imm11=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D003;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_bl_i_t1_a_field_imm11_4_poweroftwo_d000_f000d004() {
    // Thumb encoding (32): 0xF000D004
    // Test aarch32_BL_i_T1_A field imm11 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, J1=0, imm10=0, J2=0, imm11=4
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D004;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_bl_i_t1_a_field_imm11_7_poweroftwominusone_d000_f000d007() {
    // Thumb encoding (32): 0xF000D007
    // Test aarch32_BL_i_T1_A field imm11 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: J1=0, S=0, J2=0, imm10=0, imm11=7
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D007;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_bl_i_t1_a_field_imm11_8_poweroftwo_d000_f000d008() {
    // Thumb encoding (32): 0xF000D008
    // Test aarch32_BL_i_T1_A field imm11 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, J1=0, imm11=8, imm10=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D008;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_bl_i_t1_a_field_imm11_15_poweroftwominusone_d000_f000d00f() {
    // Thumb encoding (32): 0xF000D00F
    // Test aarch32_BL_i_T1_A field imm11 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm11=15, J2=0, S=0, J1=0, imm10=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D00F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_bl_i_t1_a_field_imm11_16_poweroftwo_d000_f000d010() {
    // Thumb encoding (32): 0xF000D010
    // Test aarch32_BL_i_T1_A field imm11 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: imm10=0, J2=0, imm11=16, S=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_bl_i_t1_a_field_imm11_31_poweroftwominusone_d000_f000d01f() {
    // Thumb encoding (32): 0xF000D01F
    // Test aarch32_BL_i_T1_A field imm11 = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: J1=0, imm10=0, J2=0, imm11=31, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D01F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_bl_i_t1_a_field_imm11_32_poweroftwo_d000_f000d020() {
    // Thumb encoding (32): 0xF000D020
    // Test aarch32_BL_i_T1_A field imm11 = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: J1=0, imm11=32, imm10=0, J2=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_bl_i_t1_a_field_imm11_63_poweroftwominusone_d000_f000d03f() {
    // Thumb encoding (32): 0xF000D03F
    // Test aarch32_BL_i_T1_A field imm11 = 63 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm10=0, J2=0, S=0, J1=0, imm11=63
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D03F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_bl_i_t1_a_field_imm11_64_poweroftwo_d000_f000d040() {
    // Thumb encoding (32): 0xF000D040
    // Test aarch32_BL_i_T1_A field imm11 = 64 (PowerOfTwo)
    // ISET: T32
    // Fields: imm11=64, S=0, J1=0, J2=0, imm10=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch32_bl_i_t1_a_field_imm11_127_poweroftwominusone_d000_f000d07f() {
    // Thumb encoding (32): 0xF000D07F
    // Test aarch32_BL_i_T1_A field imm11 = 127 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm10=0, J1=0, J2=0, imm11=127, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D07F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_bl_i_t1_a_field_imm11_128_poweroftwo_d000_f000d080() {
    // Thumb encoding (32): 0xF000D080
    // Test aarch32_BL_i_T1_A field imm11 = 128 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, J2=0, imm11=128, imm10=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch32_bl_i_t1_a_field_imm11_255_poweroftwominusone_d000_f000d0ff() {
    // Thumb encoding (32): 0xF000D0FF
    // Test aarch32_BL_i_T1_A field imm11 = 255 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: J1=0, S=0, J2=0, imm11=255, imm10=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D0FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch32_bl_i_t1_a_field_imm11_256_poweroftwo_d000_f000d100() {
    // Thumb encoding (32): 0xF000D100
    // Test aarch32_BL_i_T1_A field imm11 = 256 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, J1=0, imm10=0, imm11=256, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D100;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch32_bl_i_t1_a_field_imm11_511_poweroftwominusone_d000_f000d1ff() {
    // Thumb encoding (32): 0xF000D1FF
    // Test aarch32_BL_i_T1_A field imm11 = 511 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: J2=0, imm11=511, imm10=0, S=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D1FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch32_bl_i_t1_a_field_imm11_512_poweroftwo_d000_f000d200() {
    // Thumb encoding (32): 0xF000D200
    // Test aarch32_BL_i_T1_A field imm11 = 512 (PowerOfTwo)
    // ISET: T32
    // Fields: imm10=0, S=0, J1=0, imm11=512, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 1023, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (1023)
#[test]
fn test_aarch32_bl_i_t1_a_field_imm11_1023_poweroftwominusone_d000_f000d3ff() {
    // Thumb encoding (32): 0xF000D3FF
    // Test aarch32_BL_i_T1_A field imm11 = 1023 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm11=1023, S=0, J1=0, imm10=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D3FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch32_bl_i_t1_a_field_imm11_1024_poweroftwo_d000_f000d400() {
    // Thumb encoding (32): 0xF000D400
    // Test aarch32_BL_i_T1_A field imm11 = 1024 (PowerOfTwo)
    // ISET: T32
    // Fields: imm10=0, J1=0, imm11=1024, J2=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 2047, boundary: Max }
/// maximum immediate (2047)
#[test]
fn test_aarch32_bl_i_t1_a_field_imm11_2047_max_d000_f000d7ff() {
    // Thumb encoding (32): 0xF000D7FF
    // Test aarch32_BL_i_T1_A field imm11 = 2047 (Max)
    // ISET: T32
    // Fields: J1=0, S=0, imm10=0, J2=0, imm11=2047
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D7FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=0 (8-bit / byte size)
#[test]
fn test_aarch32_bl_i_t1_a_combo_0_d000_f000d000() {
    // Thumb encoding (32): 0xF000D000
    // Test aarch32_BL_i_T1_A field combination: S=0, imm10=0, J1=0, J2=0, imm11=0
    // ISET: T32
    // Fields: S=0, J1=0, imm10=0, J2=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_bl_i_t1_a_special_s_0_size_variant_0_53248_f000d000() {
    // Thumb encoding (32): 0xF000D000
    // Test aarch32_BL_i_T1_A special value S = 0 (Size variant 0)
    // ISET: T32
    // Fields: imm11=0, S=0, imm10=0, J2=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_bl_i_t1_a_special_s_1_size_variant_1_53248_f400d000() {
    // Thumb encoding (32): 0xF400D000
    // Test aarch32_BL_i_T1_A special value S = 1 (Size variant 1)
    // ISET: T32
    // Fields: S=1, J1=0, imm10=0, J2=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF400D000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `Binary { op: And, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "LastInITBlock" }, args: [] } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: And, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"LastInITBlock\" }, args: [] } } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_bl_i_t1_a_invalid_0_d000_f000d000() {
    // Thumb encoding (32): 0xF000D000
    // Test aarch32_BL_i_T1_A invalid encoding: Binary { op: And, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "LastInITBlock" }, args: [] } } }
    // ISET: T32
    // Fields: imm11=0, imm10=0, J2=0, S=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_BL_i_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_bl_i_t1_a_invalid_1_d000_f000d000() {
    // Thumb encoding (32): 0xF000D000
    // Test aarch32_BL_i_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: S=0, imm10=0, J1=0, imm11=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000D000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field S 26 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_bl_i_t2_a_field_s_0_min_c000_f000c000() {
    // Thumb encoding (32): 0xF000C000
    // Test aarch32_BL_i_T2_A field S = 0 (Min)
    // ISET: T32
    // Fields: S=0, imm10H=0, H=0, J2=0, imm10L=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field S 26 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_bl_i_t2_a_field_s_1_max_c000_f400c000() {
    // Thumb encoding (32): 0xF400C000
    // Test aarch32_BL_i_T2_A field S = 1 (Max)
    // ISET: T32
    // Fields: H=0, J1=0, J2=0, imm10H=0, imm10L=0, S=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF400C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10H 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10H", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10h_0_zero_c000_f000c000() {
    // Thumb encoding (32): 0xF000C000
    // Test aarch32_BL_i_T2_A field imm10H = 0 (Zero)
    // ISET: T32
    // Fields: imm10L=0, H=0, S=0, imm10H=0, J1=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10H 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10H", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10h_1_poweroftwo_c000_f001c000() {
    // Thumb encoding (32): 0xF001C000
    // Test aarch32_BL_i_T2_A field imm10H = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, J1=0, imm10H=1, J2=0, imm10L=0, H=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF001C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10H 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10H", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10h_3_poweroftwominusone_c000_f003c000() {
    // Thumb encoding (32): 0xF003C000
    // Test aarch32_BL_i_T2_A field imm10H = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: J1=0, J2=0, imm10L=0, S=0, imm10H=3, H=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF003C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10H 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10H", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10h_4_poweroftwo_c000_f004c000() {
    // Thumb encoding (32): 0xF004C000
    // Test aarch32_BL_i_T2_A field imm10H = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: imm10H=4, J1=0, S=0, J2=0, H=0, imm10L=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF004C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10H 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10H", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10h_7_poweroftwominusone_c000_f007c000() {
    // Thumb encoding (32): 0xF007C000
    // Test aarch32_BL_i_T2_A field imm10H = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: S=0, imm10H=7, J1=0, J2=0, imm10L=0, H=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF007C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10H 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10H", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10h_8_poweroftwo_c000_f008c000() {
    // Thumb encoding (32): 0xF008C000
    // Test aarch32_BL_i_T2_A field imm10H = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, J2=0, H=0, imm10L=0, imm10H=8, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF008C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10H 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10H", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10h_15_poweroftwominusone_c000_f00fc000() {
    // Thumb encoding (32): 0xF00FC000
    // Test aarch32_BL_i_T2_A field imm10H = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: S=0, imm10L=0, H=0, J1=0, J2=0, imm10H=15
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF00FC000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10H 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10H", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10h_16_poweroftwo_c000_f010c000() {
    // Thumb encoding (32): 0xF010C000
    // Test aarch32_BL_i_T2_A field imm10H = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: J2=0, S=0, imm10H=16, J1=0, imm10L=0, H=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF010C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10H 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10H", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10h_31_poweroftwominusone_c000_f01fc000() {
    // Thumb encoding (32): 0xF01FC000
    // Test aarch32_BL_i_T2_A field imm10H = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: J2=0, imm10L=0, H=0, S=0, imm10H=31, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF01FC000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10H 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10H", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10h_32_poweroftwo_c000_f020c000() {
    // Thumb encoding (32): 0xF020C000
    // Test aarch32_BL_i_T2_A field imm10H = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: imm10H=32, J2=0, H=0, imm10L=0, S=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF020C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10H 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10H", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10h_63_poweroftwominusone_c000_f03fc000() {
    // Thumb encoding (32): 0xF03FC000
    // Test aarch32_BL_i_T2_A field imm10H = 63 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: S=0, imm10H=63, imm10L=0, J1=0, J2=0, H=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF03FC000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10H 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10H", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10h_64_poweroftwo_c000_f040c000() {
    // Thumb encoding (32): 0xF040C000
    // Test aarch32_BL_i_T2_A field imm10H = 64 (PowerOfTwo)
    // ISET: T32
    // Fields: J2=0, H=0, S=0, imm10L=0, imm10H=64, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF040C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10H 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10H", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10h_127_poweroftwominusone_c000_f07fc000() {
    // Thumb encoding (32): 0xF07FC000
    // Test aarch32_BL_i_T2_A field imm10H = 127 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm10H=127, imm10L=0, H=0, S=0, J2=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF07FC000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10H 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10H", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10h_128_poweroftwo_c000_f080c000() {
    // Thumb encoding (32): 0xF080C000
    // Test aarch32_BL_i_T2_A field imm10H = 128 (PowerOfTwo)
    // ISET: T32
    // Fields: imm10H=128, J2=0, imm10L=0, H=0, J1=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF080C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10H 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10H", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10h_255_poweroftwominusone_c000_f0ffc000() {
    // Thumb encoding (32): 0xF0FFC000
    // Test aarch32_BL_i_T2_A field imm10H = 255 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm10H=255, J1=0, J2=0, H=0, S=0, imm10L=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0FFC000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10H 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10H", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10h_256_poweroftwo_c000_f100c000() {
    // Thumb encoding (32): 0xF100C000
    // Test aarch32_BL_i_T2_A field imm10H = 256 (PowerOfTwo)
    // ISET: T32
    // Fields: J2=0, S=0, imm10L=0, J1=0, H=0, imm10H=256
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF100C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10H 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10H", value: 511, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (511)
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10h_511_poweroftwominusone_c000_f1ffc000() {
    // Thumb encoding (32): 0xF1FFC000
    // Test aarch32_BL_i_T2_A field imm10H = 511 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm10L=0, H=0, J1=0, imm10H=511, S=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1FFC000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10H 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10H", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10h_512_poweroftwo_c000_f200c000() {
    // Thumb encoding (32): 0xF200C000
    // Test aarch32_BL_i_T2_A field imm10H = 512 (PowerOfTwo)
    // ISET: T32
    // Fields: imm10H=512, J1=0, S=0, H=0, imm10L=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF200C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10H 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10H", value: 1023, boundary: Max }
/// maximum immediate (1023)
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10h_1023_max_c000_f3ffc000() {
    // Thumb encoding (32): 0xF3FFC000
    // Test aarch32_BL_i_T2_A field imm10H = 1023 (Max)
    // ISET: T32
    // Fields: S=0, J2=0, imm10H=1023, imm10L=0, H=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF3FFC000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field J1 13 +: 1`
/// Requirement: FieldBoundary { field: "J1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_bl_i_t2_a_field_j1_0_min_c000_f000c000() {
    // Thumb encoding (32): 0xF000C000
    // Test aarch32_BL_i_T2_A field J1 = 0 (Min)
    // ISET: T32
    // Fields: imm10H=0, S=0, H=0, J1=0, imm10L=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field J1 13 +: 1`
/// Requirement: FieldBoundary { field: "J1", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_bl_i_t2_a_field_j1_1_max_c000_f000e000() {
    // Thumb encoding (32): 0xF000E000
    // Test aarch32_BL_i_T2_A field J1 = 1 (Max)
    // ISET: T32
    // Fields: imm10H=0, S=0, J2=0, imm10L=0, H=0, J1=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000E000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field J2 11 +: 1`
/// Requirement: FieldBoundary { field: "J2", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_bl_i_t2_a_field_j2_0_min_c000_f000c000() {
    // Thumb encoding (32): 0xF000C000
    // Test aarch32_BL_i_T2_A field J2 = 0 (Min)
    // ISET: T32
    // Fields: imm10L=0, imm10H=0, J1=0, H=0, J2=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field J2 11 +: 1`
/// Requirement: FieldBoundary { field: "J2", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_bl_i_t2_a_field_j2_1_max_c000_f000c800() {
    // Thumb encoding (32): 0xF000C800
    // Test aarch32_BL_i_T2_A field J2 = 1 (Max)
    // ISET: T32
    // Fields: J2=1, S=0, J1=0, imm10H=0, H=0, imm10L=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10L 1 +: 10`
/// Requirement: FieldBoundary { field: "imm10L", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10l_0_zero_c000_f000c000() {
    // Thumb encoding (32): 0xF000C000
    // Test aarch32_BL_i_T2_A field imm10L = 0 (Zero)
    // ISET: T32
    // Fields: J2=0, imm10L=0, S=0, J1=0, imm10H=0, H=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10L 1 +: 10`
/// Requirement: FieldBoundary { field: "imm10L", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10l_1_poweroftwo_c000_f000c002() {
    // Thumb encoding (32): 0xF000C002
    // Test aarch32_BL_i_T2_A field imm10L = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, imm10L=1, J2=0, imm10H=0, H=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10L 1 +: 10`
/// Requirement: FieldBoundary { field: "imm10L", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10l_3_poweroftwominusone_c000_f000c006() {
    // Thumb encoding (32): 0xF000C006
    // Test aarch32_BL_i_T2_A field imm10L = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: J1=0, S=0, imm10H=0, J2=0, H=0, imm10L=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C006;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10L 1 +: 10`
/// Requirement: FieldBoundary { field: "imm10L", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10l_4_poweroftwo_c000_f000c008() {
    // Thumb encoding (32): 0xF000C008
    // Test aarch32_BL_i_T2_A field imm10L = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: imm10H=0, J1=0, imm10L=4, S=0, H=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C008;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10L 1 +: 10`
/// Requirement: FieldBoundary { field: "imm10L", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10l_7_poweroftwominusone_c000_f000c00e() {
    // Thumb encoding (32): 0xF000C00E
    // Test aarch32_BL_i_T2_A field imm10L = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: J2=0, S=0, imm10L=7, imm10H=0, J1=0, H=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C00E;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10L 1 +: 10`
/// Requirement: FieldBoundary { field: "imm10L", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10l_8_poweroftwo_c000_f000c010() {
    // Thumb encoding (32): 0xF000C010
    // Test aarch32_BL_i_T2_A field imm10L = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: imm10L=8, imm10H=0, J2=0, S=0, J1=0, H=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10L 1 +: 10`
/// Requirement: FieldBoundary { field: "imm10L", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10l_15_poweroftwominusone_c000_f000c01e() {
    // Thumb encoding (32): 0xF000C01E
    // Test aarch32_BL_i_T2_A field imm10L = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm10H=0, S=0, J2=0, imm10L=15, H=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C01E;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10L 1 +: 10`
/// Requirement: FieldBoundary { field: "imm10L", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10l_16_poweroftwo_c000_f000c020() {
    // Thumb encoding (32): 0xF000C020
    // Test aarch32_BL_i_T2_A field imm10L = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: imm10L=16, S=0, imm10H=0, J2=0, J1=0, H=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10L 1 +: 10`
/// Requirement: FieldBoundary { field: "imm10L", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10l_31_poweroftwominusone_c000_f000c03e() {
    // Thumb encoding (32): 0xF000C03E
    // Test aarch32_BL_i_T2_A field imm10L = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: H=0, J1=0, J2=0, S=0, imm10L=31, imm10H=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C03E;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10L 1 +: 10`
/// Requirement: FieldBoundary { field: "imm10L", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10l_32_poweroftwo_c000_f000c040() {
    // Thumb encoding (32): 0xF000C040
    // Test aarch32_BL_i_T2_A field imm10L = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: J2=0, S=0, imm10L=32, J1=0, imm10H=0, H=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10L 1 +: 10`
/// Requirement: FieldBoundary { field: "imm10L", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10l_63_poweroftwominusone_c000_f000c07e() {
    // Thumb encoding (32): 0xF000C07E
    // Test aarch32_BL_i_T2_A field imm10L = 63 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm10H=0, S=0, J1=0, J2=0, H=0, imm10L=63
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C07E;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10L 1 +: 10`
/// Requirement: FieldBoundary { field: "imm10L", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10l_64_poweroftwo_c000_f000c080() {
    // Thumb encoding (32): 0xF000C080
    // Test aarch32_BL_i_T2_A field imm10L = 64 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, J1=0, imm10H=0, imm10L=64, H=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10L 1 +: 10`
/// Requirement: FieldBoundary { field: "imm10L", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10l_127_poweroftwominusone_c000_f000c0fe() {
    // Thumb encoding (32): 0xF000C0FE
    // Test aarch32_BL_i_T2_A field imm10L = 127 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: S=0, J1=0, H=0, imm10H=0, imm10L=127, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C0FE;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10L 1 +: 10`
/// Requirement: FieldBoundary { field: "imm10L", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10l_128_poweroftwo_c000_f000c100() {
    // Thumb encoding (32): 0xF000C100
    // Test aarch32_BL_i_T2_A field imm10L = 128 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, J2=0, imm10L=128, H=0, J1=0, imm10H=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C100;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10L 1 +: 10`
/// Requirement: FieldBoundary { field: "imm10L", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10l_255_poweroftwominusone_c000_f000c1fe() {
    // Thumb encoding (32): 0xF000C1FE
    // Test aarch32_BL_i_T2_A field imm10L = 255 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: J1=0, S=0, imm10H=0, J2=0, imm10L=255, H=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C1FE;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10L 1 +: 10`
/// Requirement: FieldBoundary { field: "imm10L", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10l_256_poweroftwo_c000_f000c200() {
    // Thumb encoding (32): 0xF000C200
    // Test aarch32_BL_i_T2_A field imm10L = 256 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, imm10L=256, imm10H=0, H=0, J2=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10L 1 +: 10`
/// Requirement: FieldBoundary { field: "imm10L", value: 511, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (511)
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10l_511_poweroftwominusone_c000_f000c3fe() {
    // Thumb encoding (32): 0xF000C3FE
    // Test aarch32_BL_i_T2_A field imm10L = 511 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: J2=0, imm10H=0, S=0, imm10L=511, H=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C3FE;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10L 1 +: 10`
/// Requirement: FieldBoundary { field: "imm10L", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10l_512_poweroftwo_c000_f000c400() {
    // Thumb encoding (32): 0xF000C400
    // Test aarch32_BL_i_T2_A field imm10L = 512 (PowerOfTwo)
    // ISET: T32
    // Fields: imm10H=0, J1=0, J2=0, imm10L=512, H=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field imm10L 1 +: 10`
/// Requirement: FieldBoundary { field: "imm10L", value: 1023, boundary: Max }
/// maximum immediate (1023)
#[test]
fn test_aarch32_bl_i_t2_a_field_imm10l_1023_max_c000_f000c7fe() {
    // Thumb encoding (32): 0xF000C7FE
    // Test aarch32_BL_i_T2_A field imm10L = 1023 (Max)
    // ISET: T32
    // Fields: imm10H=0, J2=0, S=0, imm10L=1023, H=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C7FE;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field H 0 +: 1`
/// Requirement: FieldBoundary { field: "H", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_bl_i_t2_a_field_h_0_min_c000_f000c000() {
    // Thumb encoding (32): 0xF000C000
    // Test aarch32_BL_i_T2_A field H = 0 (Min)
    // ISET: T32
    // Fields: S=0, J2=0, imm10L=0, H=0, imm10H=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field H 0 +: 1`
/// Requirement: FieldBoundary { field: "H", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_bl_i_t2_a_field_h_1_max_c000_f000c001() {
    // Thumb encoding (32): 0xF000C001
    // Test aarch32_BL_i_T2_A field H = 1 (Max)
    // ISET: T32
    // Fields: S=0, imm10L=0, H=1, J2=0, J1=0, imm10H=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C001;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=0 (8-bit / byte size)
#[test]
fn test_aarch32_bl_i_t2_a_combo_0_c000_f000c000() {
    // Thumb encoding (32): 0xF000C000
    // Test aarch32_BL_i_T2_A field combination: S=0, imm10H=0, J1=0, J2=0, imm10L=0, H=0
    // ISET: T32
    // Fields: H=0, imm10L=0, S=0, imm10H=0, J1=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_bl_i_t2_a_special_s_0_size_variant_0_49152_f000c000() {
    // Thumb encoding (32): 0xF000C000
    // Test aarch32_BL_i_T2_A special value S = 0 (Size variant 0)
    // ISET: T32
    // Fields: J2=0, S=0, H=0, imm10H=0, J1=0, imm10L=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_bl_i_t2_a_special_s_1_size_variant_1_49152_f400c000() {
    // Thumb encoding (32): 0xF400C000
    // Test aarch32_BL_i_T2_A special value S = 1 (Size variant 1)
    // ISET: T32
    // Fields: J1=0, J2=0, imm10H=0, S=1, imm10L=0, H=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF400C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "H" }), rhs: LitBits([true]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"H\" }), rhs: LitBits([true]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_bl_i_t2_a_invalid_0_c000_f000c000() {
    // Thumb encoding (32): 0xF000C000
    // Test aarch32_BL_i_T2_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "H" }), rhs: LitBits([true]) }
    // ISET: T32
    // Fields: H=0, imm10L=0, J2=0, S=0, J1=0, imm10H=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_bl_i_t2_a_invalid_1_c000_f000c000() {
    // Thumb encoding (32): 0xF000C000
    // Test aarch32_BL_i_T2_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: H=0, imm10L=0, S=0, J2=0, J1=0, imm10H=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `Binary { op: And, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "LastInITBlock" }, args: [] } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: And, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"LastInITBlock\" }, args: [] } } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_bl_i_t2_a_invalid_2_c000_f000c000() {
    // Thumb encoding (32): 0xF000C000
    // Test aarch32_BL_i_T2_A invalid encoding: Binary { op: And, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "LastInITBlock" }, args: [] } } }
    // ISET: T32
    // Fields: H=0, imm10H=0, J2=0, J1=0, S=0, imm10L=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_BL_i_T2_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_bl_i_t2_a_invalid_3_c000_f000c000() {
    // Thumb encoding (32): 0xF000C000
    // Test aarch32_BL_i_T2_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: imm10L=0, H=0, J2=0, S=0, imm10H=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000C000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_BXJ_A Tests
// ============================================================================

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_bxj_a1_a_field_cond_0_min_20_01200020() {
    // Encoding: 0x01200020
    // Test aarch32_BXJ_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0
    let encoding: u32 = 0x01200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_bxj_a1_a_field_cond_1_poweroftwo_20_11200020() {
    // Encoding: 0x11200020
    // Test aarch32_BXJ_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=1
    let encoding: u32 = 0x11200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_bxj_a1_a_field_cond_2_poweroftwo_20_21200020() {
    // Encoding: 0x21200020
    // Test aarch32_BXJ_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=2, Rm=0
    let encoding: u32 = 0x21200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_bxj_a1_a_field_cond_3_poweroftwo_20_31200020() {
    // Encoding: 0x31200020
    // Test aarch32_BXJ_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, Rm=0
    let encoding: u32 = 0x31200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_bxj_a1_a_field_cond_4_poweroftwo_20_41200020() {
    // Encoding: 0x41200020
    // Test aarch32_BXJ_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=4, Rm=0
    let encoding: u32 = 0x41200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_bxj_a1_a_field_cond_5_poweroftwo_20_51200020() {
    // Encoding: 0x51200020
    // Test aarch32_BXJ_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=5
    let encoding: u32 = 0x51200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_bxj_a1_a_field_cond_6_poweroftwo_20_61200020() {
    // Encoding: 0x61200020
    // Test aarch32_BXJ_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, Rm=0
    let encoding: u32 = 0x61200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_bxj_a1_a_field_cond_7_poweroftwo_20_71200020() {
    // Encoding: 0x71200020
    // Test aarch32_BXJ_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=7
    let encoding: u32 = 0x71200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_bxj_a1_a_field_cond_8_poweroftwo_20_81200020() {
    // Encoding: 0x81200020
    // Test aarch32_BXJ_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=8, Rm=0
    let encoding: u32 = 0x81200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_bxj_a1_a_field_cond_9_poweroftwo_20_91200020() {
    // Encoding: 0x91200020
    // Test aarch32_BXJ_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, Rm=0
    let encoding: u32 = 0x91200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_bxj_a1_a_field_cond_10_poweroftwo_20_a1200020() {
    // Encoding: 0xA1200020
    // Test aarch32_BXJ_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=10
    let encoding: u32 = 0xA1200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_bxj_a1_a_field_cond_11_poweroftwo_20_b1200020() {
    // Encoding: 0xB1200020
    // Test aarch32_BXJ_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=11, Rm=0
    let encoding: u32 = 0xB1200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_bxj_a1_a_field_cond_12_poweroftwo_20_c1200020() {
    // Encoding: 0xC1200020
    // Test aarch32_BXJ_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=12
    let encoding: u32 = 0xC1200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_bxj_a1_a_field_cond_13_poweroftwo_20_d1200020() {
    // Encoding: 0xD1200020
    // Test aarch32_BXJ_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=13
    let encoding: u32 = 0xD1200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_bxj_a1_a_field_cond_14_poweroftwo_20_e1200020() {
    // Encoding: 0xE1200020
    // Test aarch32_BXJ_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, Rm=0
    let encoding: u32 = 0xE1200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_bxj_a1_a_field_cond_15_max_20_f1200020() {
    // Encoding: 0xF1200020
    // Test aarch32_BXJ_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, Rm=0
    let encoding: u32 = 0xF1200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_bxj_a1_a_field_rm_0_min_20_01200020() {
    // Encoding: 0x01200020
    // Test aarch32_BXJ_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, cond=0
    let encoding: u32 = 0x01200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_bxj_a1_a_field_rm_1_poweroftwo_20_01200021() {
    // Encoding: 0x01200021
    // Test aarch32_BXJ_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rm=1
    let encoding: u32 = 0x01200021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_bxj_a1_a_combo_0_20_01200020() {
    // Encoding: 0x01200020
    // Test aarch32_BXJ_A1_A field combination: cond=0, Rm=0
    // ISET: A32
    // Fields: Rm=0, cond=0
    let encoding: u32 = 0x01200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_bxj_a1_a_special_cond_0_condition_eq_32_01200020() {
    // Encoding: 0x01200020
    // Test aarch32_BXJ_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: cond=0, Rm=0
    let encoding: u32 = 0x01200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_bxj_a1_a_special_cond_1_condition_ne_32_11200020() {
    // Encoding: 0x11200020
    // Test aarch32_BXJ_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, Rm=0
    let encoding: u32 = 0x11200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_bxj_a1_a_special_cond_2_condition_cs_hs_32_21200020() {
    // Encoding: 0x21200020
    // Test aarch32_BXJ_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, Rm=0
    let encoding: u32 = 0x21200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_bxj_a1_a_special_cond_3_condition_cc_lo_32_31200020() {
    // Encoding: 0x31200020
    // Test aarch32_BXJ_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, Rm=0
    let encoding: u32 = 0x31200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_bxj_a1_a_special_cond_4_condition_mi_32_41200020() {
    // Encoding: 0x41200020
    // Test aarch32_BXJ_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, Rm=0
    let encoding: u32 = 0x41200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_bxj_a1_a_special_cond_5_condition_pl_32_51200020() {
    // Encoding: 0x51200020
    // Test aarch32_BXJ_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Rm=0, cond=5
    let encoding: u32 = 0x51200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_bxj_a1_a_special_cond_6_condition_vs_32_61200020() {
    // Encoding: 0x61200020
    // Test aarch32_BXJ_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rm=0, cond=6
    let encoding: u32 = 0x61200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_bxj_a1_a_special_cond_7_condition_vc_32_71200020() {
    // Encoding: 0x71200020
    // Test aarch32_BXJ_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rm=0, cond=7
    let encoding: u32 = 0x71200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_bxj_a1_a_special_cond_8_condition_hi_32_81200020() {
    // Encoding: 0x81200020
    // Test aarch32_BXJ_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Rm=0, cond=8
    let encoding: u32 = 0x81200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_bxj_a1_a_special_cond_9_condition_ls_32_91200020() {
    // Encoding: 0x91200020
    // Test aarch32_BXJ_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, Rm=0
    let encoding: u32 = 0x91200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_bxj_a1_a_special_cond_10_condition_ge_32_a1200020() {
    // Encoding: 0xA1200020
    // Test aarch32_BXJ_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, Rm=0
    let encoding: u32 = 0xA1200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_bxj_a1_a_special_cond_11_condition_lt_32_b1200020() {
    // Encoding: 0xB1200020
    // Test aarch32_BXJ_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Rm=0, cond=11
    let encoding: u32 = 0xB1200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_bxj_a1_a_special_cond_12_condition_gt_32_c1200020() {
    // Encoding: 0xC1200020
    // Test aarch32_BXJ_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rm=0, cond=12
    let encoding: u32 = 0xC1200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_bxj_a1_a_special_cond_13_condition_le_32_d1200020() {
    // Encoding: 0xD1200020
    // Test aarch32_BXJ_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rm=0, cond=13
    let encoding: u32 = 0xD1200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_bxj_a1_a_special_cond_14_condition_al_32_e1200020() {
    // Encoding: 0xE1200020
    // Test aarch32_BXJ_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: Rm=0, cond=14
    let encoding: u32 = 0xE1200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_bxj_a1_a_special_cond_15_condition_nv_32_f1200020() {
    // Encoding: 0xF1200020
    // Test aarch32_BXJ_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Rm=0, cond=15
    let encoding: u32 = 0xF1200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }), rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }), rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_bxj_a1_a_invalid_0_20_01200020() {
    // Encoding: 0x01200020
    // Test aarch32_BXJ_A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }), rhs: LitInt(15) }
    // ISET: A32
    // Fields: cond=0, Rm=0
    let encoding: u32 = 0x01200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_BXJ_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_bxj_a1_a_invalid_1_20_01200020() {
    // Encoding: 0x01200020
    // Test aarch32_BXJ_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Rm=0, cond=0
    let encoding: u32 = 0x01200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_BXJ_T1_A
/// ASL: `field Rm 16 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_bxj_t1_a_field_rm_0_min_8000_f3c08000() {
    // Thumb encoding (32): 0xF3C08000
    // Test aarch32_BXJ_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF3C08000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BXJ_T1_A
/// ASL: `field Rm 16 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_bxj_t1_a_field_rm_1_poweroftwo_8000_f3c18000() {
    // Thumb encoding (32): 0xF3C18000
    // Test aarch32_BXJ_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF3C18000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BXJ_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch32_bxj_t1_a_combo_0_8000_f3c08000() {
    // Thumb encoding (32): 0xF3C08000
    // Test aarch32_BXJ_T1_A field combination: Rm=0
    // ISET: T32
    // Fields: Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF3C08000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BXJ_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }), rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }), rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_bxj_t1_a_invalid_0_8000_f3c08000() {
    // Thumb encoding (32): 0xF3C08000
    // Test aarch32_BXJ_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }), rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF3C08000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_BXJ_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_bxj_t1_a_invalid_1_8000_f3c08000() {
    // Thumb encoding (32): 0xF3C08000
    // Test aarch32_BXJ_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF3C08000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_BXJ_T1_A
/// ASL: `Binary { op: And, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "LastInITBlock" }, args: [] } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: And, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"LastInITBlock\" }, args: [] } } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_bxj_t1_a_invalid_2_8000_f3c08000() {
    // Thumb encoding (32): 0xF3C08000
    // Test aarch32_BXJ_T1_A invalid encoding: Binary { op: And, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "LastInITBlock" }, args: [] } } }
    // ISET: T32
    // Fields: Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF3C08000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_BXJ_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_bxj_t1_a_invalid_3_8000_f3c08000() {
    // Thumb encoding (32): 0xF3C08000
    // Test aarch32_BXJ_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF3C08000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_BX_A Tests
// ============================================================================

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_bx_a1_a_field_cond_0_min_10_01200010() {
    // Encoding: 0x01200010
    // Test aarch32_BX_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0
    let encoding: u32 = 0x01200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_bx_a1_a_field_cond_1_poweroftwo_10_11200010() {
    // Encoding: 0x11200010
    // Test aarch32_BX_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, Rm=0
    let encoding: u32 = 0x11200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_bx_a1_a_field_cond_2_poweroftwo_10_21200010() {
    // Encoding: 0x21200010
    // Test aarch32_BX_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=2, Rm=0
    let encoding: u32 = 0x21200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_bx_a1_a_field_cond_3_poweroftwo_10_31200010() {
    // Encoding: 0x31200010
    // Test aarch32_BX_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=3
    let encoding: u32 = 0x31200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_bx_a1_a_field_cond_4_poweroftwo_10_41200010() {
    // Encoding: 0x41200010
    // Test aarch32_BX_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=4
    let encoding: u32 = 0x41200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_bx_a1_a_field_cond_5_poweroftwo_10_51200010() {
    // Encoding: 0x51200010
    // Test aarch32_BX_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=5
    let encoding: u32 = 0x51200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_bx_a1_a_field_cond_6_poweroftwo_10_61200010() {
    // Encoding: 0x61200010
    // Test aarch32_BX_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=6
    let encoding: u32 = 0x61200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_bx_a1_a_field_cond_7_poweroftwo_10_71200010() {
    // Encoding: 0x71200010
    // Test aarch32_BX_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=7, Rm=0
    let encoding: u32 = 0x71200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_bx_a1_a_field_cond_8_poweroftwo_10_81200010() {
    // Encoding: 0x81200010
    // Test aarch32_BX_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=8, Rm=0
    let encoding: u32 = 0x81200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_bx_a1_a_field_cond_9_poweroftwo_10_91200010() {
    // Encoding: 0x91200010
    // Test aarch32_BX_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=9
    let encoding: u32 = 0x91200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_bx_a1_a_field_cond_10_poweroftwo_10_a1200010() {
    // Encoding: 0xA1200010
    // Test aarch32_BX_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, Rm=0
    let encoding: u32 = 0xA1200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_bx_a1_a_field_cond_11_poweroftwo_10_b1200010() {
    // Encoding: 0xB1200010
    // Test aarch32_BX_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=11
    let encoding: u32 = 0xB1200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_bx_a1_a_field_cond_12_poweroftwo_10_c1200010() {
    // Encoding: 0xC1200010
    // Test aarch32_BX_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=12
    let encoding: u32 = 0xC1200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_bx_a1_a_field_cond_13_poweroftwo_10_d1200010() {
    // Encoding: 0xD1200010
    // Test aarch32_BX_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=13
    let encoding: u32 = 0xD1200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_bx_a1_a_field_cond_14_poweroftwo_10_e1200010() {
    // Encoding: 0xE1200010
    // Test aarch32_BX_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=14
    let encoding: u32 = 0xE1200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_bx_a1_a_field_cond_15_max_10_f1200010() {
    // Encoding: 0xF1200010
    // Test aarch32_BX_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Rm=0, cond=15
    let encoding: u32 = 0xF1200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_bx_a1_a_field_rm_0_min_10_01200010() {
    // Encoding: 0x01200010
    // Test aarch32_BX_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0
    let encoding: u32 = 0x01200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_bx_a1_a_field_rm_1_poweroftwo_10_01200011() {
    // Encoding: 0x01200011
    // Test aarch32_BX_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rm=1
    let encoding: u32 = 0x01200011;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_bx_a1_a_combo_0_10_01200010() {
    // Encoding: 0x01200010
    // Test aarch32_BX_A1_A field combination: cond=0, Rm=0
    // ISET: A32
    // Fields: cond=0, Rm=0
    let encoding: u32 = 0x01200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_bx_a1_a_special_cond_0_condition_eq_16_01200010() {
    // Encoding: 0x01200010
    // Test aarch32_BX_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Rm=0, cond=0
    let encoding: u32 = 0x01200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_bx_a1_a_special_cond_1_condition_ne_16_11200010() {
    // Encoding: 0x11200010
    // Test aarch32_BX_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, Rm=0
    let encoding: u32 = 0x11200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_bx_a1_a_special_cond_2_condition_cs_hs_16_21200010() {
    // Encoding: 0x21200010
    // Test aarch32_BX_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, Rm=0
    let encoding: u32 = 0x21200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_bx_a1_a_special_cond_3_condition_cc_lo_16_31200010() {
    // Encoding: 0x31200010
    // Test aarch32_BX_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Rm=0, cond=3
    let encoding: u32 = 0x31200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_bx_a1_a_special_cond_4_condition_mi_16_41200010() {
    // Encoding: 0x41200010
    // Test aarch32_BX_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Rm=0, cond=4
    let encoding: u32 = 0x41200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_bx_a1_a_special_cond_5_condition_pl_16_51200010() {
    // Encoding: 0x51200010
    // Test aarch32_BX_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, Rm=0
    let encoding: u32 = 0x51200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_bx_a1_a_special_cond_6_condition_vs_16_61200010() {
    // Encoding: 0x61200010
    // Test aarch32_BX_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rm=0, cond=6
    let encoding: u32 = 0x61200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_bx_a1_a_special_cond_7_condition_vc_16_71200010() {
    // Encoding: 0x71200010
    // Test aarch32_BX_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, Rm=0
    let encoding: u32 = 0x71200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_bx_a1_a_special_cond_8_condition_hi_16_81200010() {
    // Encoding: 0x81200010
    // Test aarch32_BX_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, Rm=0
    let encoding: u32 = 0x81200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_bx_a1_a_special_cond_9_condition_ls_16_91200010() {
    // Encoding: 0x91200010
    // Test aarch32_BX_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, Rm=0
    let encoding: u32 = 0x91200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_bx_a1_a_special_cond_10_condition_ge_16_a1200010() {
    // Encoding: 0xA1200010
    // Test aarch32_BX_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, Rm=0
    let encoding: u32 = 0xA1200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_bx_a1_a_special_cond_11_condition_lt_16_b1200010() {
    // Encoding: 0xB1200010
    // Test aarch32_BX_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, Rm=0
    let encoding: u32 = 0xB1200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_bx_a1_a_special_cond_12_condition_gt_16_c1200010() {
    // Encoding: 0xC1200010
    // Test aarch32_BX_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rm=0, cond=12
    let encoding: u32 = 0xC1200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_bx_a1_a_special_cond_13_condition_le_16_d1200010() {
    // Encoding: 0xD1200010
    // Test aarch32_BX_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: cond=13, Rm=0
    let encoding: u32 = 0xD1200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_bx_a1_a_special_cond_14_condition_al_16_e1200010() {
    // Encoding: 0xE1200010
    // Test aarch32_BX_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: cond=14, Rm=0
    let encoding: u32 = 0xE1200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_bx_a1_a_special_cond_15_condition_nv_16_f1200010() {
    // Encoding: 0xF1200010
    // Test aarch32_BX_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, Rm=0
    let encoding: u32 = 0xF1200010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BX_T1_A
/// ASL: `field Rm 19 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_bx_t1_a_field_rm_0_min_0_47000000() {
    // Thumb encoding (32): 0x47000000
    // Test aarch32_BX_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x47000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BX_T1_A
/// ASL: `field Rm 19 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_bx_t1_a_field_rm_1_poweroftwo_0_47080000() {
    // Thumb encoding (32): 0x47080000
    // Test aarch32_BX_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x47080000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BX_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch32_bx_t1_a_combo_0_0_47000000() {
    // Thumb encoding (32): 0x47000000
    // Test aarch32_BX_T1_A field combination: Rm=0
    // ISET: T32
    // Fields: Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x47000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BX_T1_A
/// ASL: `Binary { op: And, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "LastInITBlock" }, args: [] } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: And, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"LastInITBlock\" }, args: [] } } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_bx_t1_a_invalid_0_0_47000000() {
    // Thumb encoding (32): 0x47000000
    // Test aarch32_BX_T1_A invalid encoding: Binary { op: And, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "LastInITBlock" }, args: [] } } }
    // ISET: T32
    // Fields: Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x47000000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_BX_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_bx_t1_a_invalid_1_0_47000000() {
    // Thumb encoding (32): 0x47000000
    // Test aarch32_BX_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x47000000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_BX_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 0 (32)
#[test]
fn test_aarch32_bx_t1_a_lslv_oracle_32_0_47020020() {
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

/// Provenance: aarch32_BX_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 0 (64)
#[test]
fn test_aarch32_bx_t1_a_lslv_oracle_64_0_c7020020() {
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

/// Provenance: aarch32_BX_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4 (32)
#[test]
fn test_aarch32_bx_t1_a_lslv_oracle_32_1_47020020() {
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

/// Provenance: aarch32_BX_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 4 (64)
#[test]
fn test_aarch32_bx_t1_a_lslv_oracle_64_1_c7020020() {
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

/// Provenance: aarch32_BX_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 8 (32)
#[test]
fn test_aarch32_bx_t1_a_lslv_oracle_32_2_47020020() {
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

/// Provenance: aarch32_BX_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 8 (64)
#[test]
fn test_aarch32_bx_t1_a_lslv_oracle_64_2_c7020020() {
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

/// Provenance: aarch32_BX_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1 (32)
#[test]
fn test_aarch32_bx_t1_a_lslv_oracle_32_3_47020020() {
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

/// Provenance: aarch32_BX_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// MSB set, shift 1 (64)
#[test]
fn test_aarch32_bx_t1_a_lslv_oracle_64_3_c7020020() {
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

/// Provenance: aarch32_BX_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSB set, max shift (32)
#[test]
fn test_aarch32_bx_t1_a_lslv_oracle_32_4_47020020() {
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

/// Provenance: aarch32_BX_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSB set, max shift (64)
#[test]
fn test_aarch32_bx_t1_a_lslv_oracle_64_4_c7020020() {
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

/// Provenance: aarch32_BX_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// all ones, shift 32 (32)
#[test]
fn test_aarch32_bx_t1_a_lslv_oracle_32_5_47020020() {
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

/// Provenance: aarch32_BX_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all ones, shift 32 (64)
#[test]
fn test_aarch32_bx_t1_a_lslv_oracle_64_5_c7020020() {
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

/// Provenance: aarch32_BX_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift
#[test]
fn test_aarch32_bx_t1_a_t16_oracle_0_47100000() {
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

/// Provenance: aarch32_BX_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4
#[test]
fn test_aarch32_bx_t1_a_t16_oracle_1_47100000() {
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

/// Provenance: aarch32_BX_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1
#[test]
fn test_aarch32_bx_t1_a_t16_oracle_2_47100000() {
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

/// Provenance: aarch32_BX_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift to MSB
#[test]
fn test_aarch32_bx_t1_a_t16_oracle_3_47100000() {
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
// aarch32_B_A Tests
// ============================================================================

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_b_a1_a_field_cond_0_min_0_0a000000() {
    // Encoding: 0x0A000000
    // Test aarch32_B_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: imm24=0, cond=0
    let encoding: u32 = 0x0A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_b_a1_a_field_cond_1_poweroftwo_0_1a000000() {
    // Encoding: 0x1A000000
    // Test aarch32_B_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=0, cond=1
    let encoding: u32 = 0x1A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_b_a1_a_field_cond_2_poweroftwo_0_2a000000() {
    // Encoding: 0x2A000000
    // Test aarch32_B_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=2, imm24=0
    let encoding: u32 = 0x2A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_b_a1_a_field_cond_3_poweroftwo_0_3a000000() {
    // Encoding: 0x3A000000
    // Test aarch32_B_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, imm24=0
    let encoding: u32 = 0x3A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_b_a1_a_field_cond_4_poweroftwo_0_4a000000() {
    // Encoding: 0x4A000000
    // Test aarch32_B_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=0, cond=4
    let encoding: u32 = 0x4A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_b_a1_a_field_cond_5_poweroftwo_0_5a000000() {
    // Encoding: 0x5A000000
    // Test aarch32_B_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=0, cond=5
    let encoding: u32 = 0x5A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_b_a1_a_field_cond_6_poweroftwo_0_6a000000() {
    // Encoding: 0x6A000000
    // Test aarch32_B_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, imm24=0
    let encoding: u32 = 0x6A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_b_a1_a_field_cond_7_poweroftwo_0_7a000000() {
    // Encoding: 0x7A000000
    // Test aarch32_B_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=7, imm24=0
    let encoding: u32 = 0x7A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_b_a1_a_field_cond_8_poweroftwo_0_8a000000() {
    // Encoding: 0x8A000000
    // Test aarch32_B_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=0, cond=8
    let encoding: u32 = 0x8A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_b_a1_a_field_cond_9_poweroftwo_0_9a000000() {
    // Encoding: 0x9A000000
    // Test aarch32_B_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, imm24=0
    let encoding: u32 = 0x9A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_b_a1_a_field_cond_10_poweroftwo_0_aa000000() {
    // Encoding: 0xAA000000
    // Test aarch32_B_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, imm24=0
    let encoding: u32 = 0xAA000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_b_a1_a_field_cond_11_poweroftwo_0_ba000000() {
    // Encoding: 0xBA000000
    // Test aarch32_B_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=11, imm24=0
    let encoding: u32 = 0xBA000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_b_a1_a_field_cond_12_poweroftwo_0_ca000000() {
    // Encoding: 0xCA000000
    // Test aarch32_B_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=0, cond=12
    let encoding: u32 = 0xCA000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_b_a1_a_field_cond_13_poweroftwo_0_da000000() {
    // Encoding: 0xDA000000
    // Test aarch32_B_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=13, imm24=0
    let encoding: u32 = 0xDA000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_b_a1_a_field_cond_14_poweroftwo_0_ea000000() {
    // Encoding: 0xEA000000
    // Test aarch32_B_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, imm24=0
    let encoding: u32 = 0xEA000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_b_a1_a_field_cond_15_max_0_fa000000() {
    // Encoding: 0xFA000000
    // Test aarch32_B_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: imm24=0, cond=15
    let encoding: u32 = 0xFA000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_b_a1_a_field_imm24_0_zero_0_0a000000() {
    // Encoding: 0x0A000000
    // Test aarch32_B_A1_A field imm24 = 0 (Zero)
    // ISET: A32
    // Fields: imm24=0, cond=0
    let encoding: u32 = 0x0A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_b_a1_a_field_imm24_1_poweroftwo_0_0a000001() {
    // Encoding: 0x0A000001
    // Test aarch32_B_A1_A field imm24 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=1
    let encoding: u32 = 0x0A000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_b_a1_a_field_imm24_3_poweroftwominusone_0_0a000003() {
    // Encoding: 0x0A000003
    // Test aarch32_B_A1_A field imm24 = 3 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm24=3, cond=0
    let encoding: u32 = 0x0A000003;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_b_a1_a_field_imm24_4_poweroftwo_0_0a000004() {
    // Encoding: 0x0A000004
    // Test aarch32_B_A1_A field imm24 = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=4, cond=0
    let encoding: u32 = 0x0A000004;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_b_a1_a_field_imm24_7_poweroftwominusone_0_0a000007() {
    // Encoding: 0x0A000007
    // Test aarch32_B_A1_A field imm24 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm24=7, cond=0
    let encoding: u32 = 0x0A000007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_b_a1_a_field_imm24_8_poweroftwo_0_0a000008() {
    // Encoding: 0x0A000008
    // Test aarch32_B_A1_A field imm24 = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=8, cond=0
    let encoding: u32 = 0x0A000008;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_b_a1_a_field_imm24_15_poweroftwominusone_0_0a00000f() {
    // Encoding: 0x0A00000F
    // Test aarch32_B_A1_A field imm24 = 15 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=15
    let encoding: u32 = 0x0A00000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_b_a1_a_field_imm24_16_poweroftwo_0_0a000010() {
    // Encoding: 0x0A000010
    // Test aarch32_B_A1_A field imm24 = 16 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=16
    let encoding: u32 = 0x0A000010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_b_a1_a_field_imm24_31_poweroftwominusone_0_0a00001f() {
    // Encoding: 0x0A00001F
    // Test aarch32_B_A1_A field imm24 = 31 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=31
    let encoding: u32 = 0x0A00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_b_a1_a_field_imm24_32_poweroftwo_0_0a000020() {
    // Encoding: 0x0A000020
    // Test aarch32_B_A1_A field imm24 = 32 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=32, cond=0
    let encoding: u32 = 0x0A000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_b_a1_a_field_imm24_63_poweroftwominusone_0_0a00003f() {
    // Encoding: 0x0A00003F
    // Test aarch32_B_A1_A field imm24 = 63 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=63
    let encoding: u32 = 0x0A00003F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_b_a1_a_field_imm24_64_poweroftwo_0_0a000040() {
    // Encoding: 0x0A000040
    // Test aarch32_B_A1_A field imm24 = 64 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=64, cond=0
    let encoding: u32 = 0x0A000040;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch32_b_a1_a_field_imm24_127_poweroftwominusone_0_0a00007f() {
    // Encoding: 0x0A00007F
    // Test aarch32_B_A1_A field imm24 = 127 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm24=127, cond=0
    let encoding: u32 = 0x0A00007F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_b_a1_a_field_imm24_128_poweroftwo_0_0a000080() {
    // Encoding: 0x0A000080
    // Test aarch32_B_A1_A field imm24 = 128 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=128
    let encoding: u32 = 0x0A000080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch32_b_a1_a_field_imm24_255_poweroftwominusone_0_0a0000ff() {
    // Encoding: 0x0A0000FF
    // Test aarch32_B_A1_A field imm24 = 255 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=255
    let encoding: u32 = 0x0A0000FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch32_b_a1_a_field_imm24_256_poweroftwo_0_0a000100() {
    // Encoding: 0x0A000100
    // Test aarch32_B_A1_A field imm24 = 256 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=256, cond=0
    let encoding: u32 = 0x0A000100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch32_b_a1_a_field_imm24_511_poweroftwominusone_0_0a0001ff() {
    // Encoding: 0x0A0001FF
    // Test aarch32_B_A1_A field imm24 = 511 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm24=511, cond=0
    let encoding: u32 = 0x0A0001FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch32_b_a1_a_field_imm24_512_poweroftwo_0_0a000200() {
    // Encoding: 0x0A000200
    // Test aarch32_B_A1_A field imm24 = 512 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=512
    let encoding: u32 = 0x0A000200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 1023, boundary: PowerOfTwoMinusOne }
/// 2^10 - 1 = 1023
#[test]
fn test_aarch32_b_a1_a_field_imm24_1023_poweroftwominusone_0_0a0003ff() {
    // Encoding: 0x0A0003FF
    // Test aarch32_B_A1_A field imm24 = 1023 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=1023
    let encoding: u32 = 0x0A0003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch32_b_a1_a_field_imm24_1024_poweroftwo_0_0a000400() {
    // Encoding: 0x0A000400
    // Test aarch32_B_A1_A field imm24 = 1024 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=1024, cond=0
    let encoding: u32 = 0x0A000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 2047, boundary: PowerOfTwoMinusOne }
/// 2^11 - 1 = 2047
#[test]
fn test_aarch32_b_a1_a_field_imm24_2047_poweroftwominusone_0_0a0007ff() {
    // Encoding: 0x0A0007FF
    // Test aarch32_B_A1_A field imm24 = 2047 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=2047
    let encoding: u32 = 0x0A0007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 2048, boundary: PowerOfTwo }
/// power of 2 (2^11 = 2048)
#[test]
fn test_aarch32_b_a1_a_field_imm24_2048_poweroftwo_0_0a000800() {
    // Encoding: 0x0A000800
    // Test aarch32_B_A1_A field imm24 = 2048 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=2048, cond=0
    let encoding: u32 = 0x0A000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 4095, boundary: PowerOfTwoMinusOne }
/// 2^12 - 1 = 4095
#[test]
fn test_aarch32_b_a1_a_field_imm24_4095_poweroftwominusone_0_0a000fff() {
    // Encoding: 0x0A000FFF
    // Test aarch32_B_A1_A field imm24 = 4095 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=4095
    let encoding: u32 = 0x0A000FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 4096, boundary: PowerOfTwo }
/// power of 2 (2^12 = 4096)
#[test]
fn test_aarch32_b_a1_a_field_imm24_4096_poweroftwo_0_0a001000() {
    // Encoding: 0x0A001000
    // Test aarch32_B_A1_A field imm24 = 4096 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=4096
    let encoding: u32 = 0x0A001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 8191, boundary: PowerOfTwoMinusOne }
/// 2^13 - 1 = 8191
#[test]
fn test_aarch32_b_a1_a_field_imm24_8191_poweroftwominusone_0_0a001fff() {
    // Encoding: 0x0A001FFF
    // Test aarch32_B_A1_A field imm24 = 8191 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm24=8191, cond=0
    let encoding: u32 = 0x0A001FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 8192, boundary: PowerOfTwo }
/// power of 2 (2^13 = 8192)
#[test]
fn test_aarch32_b_a1_a_field_imm24_8192_poweroftwo_0_0a002000() {
    // Encoding: 0x0A002000
    // Test aarch32_B_A1_A field imm24 = 8192 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=8192
    let encoding: u32 = 0x0A002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 16383, boundary: PowerOfTwoMinusOne }
/// 2^14 - 1 = 16383
#[test]
fn test_aarch32_b_a1_a_field_imm24_16383_poweroftwominusone_0_0a003fff() {
    // Encoding: 0x0A003FFF
    // Test aarch32_B_A1_A field imm24 = 16383 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=16383
    let encoding: u32 = 0x0A003FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 16384, boundary: PowerOfTwo }
/// power of 2 (2^14 = 16384)
#[test]
fn test_aarch32_b_a1_a_field_imm24_16384_poweroftwo_0_0a004000() {
    // Encoding: 0x0A004000
    // Test aarch32_B_A1_A field imm24 = 16384 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=16384
    let encoding: u32 = 0x0A004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 32767, boundary: PowerOfTwoMinusOne }
/// 2^15 - 1 = 32767
#[test]
fn test_aarch32_b_a1_a_field_imm24_32767_poweroftwominusone_0_0a007fff() {
    // Encoding: 0x0A007FFF
    // Test aarch32_B_A1_A field imm24 = 32767 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm24=32767, cond=0
    let encoding: u32 = 0x0A007FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 32768, boundary: PowerOfTwo }
/// power of 2 (2^15 = 32768)
#[test]
fn test_aarch32_b_a1_a_field_imm24_32768_poweroftwo_0_0a008000() {
    // Encoding: 0x0A008000
    // Test aarch32_B_A1_A field imm24 = 32768 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=32768
    let encoding: u32 = 0x0A008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 65535, boundary: PowerOfTwoMinusOne }
/// 2^16 - 1 = 65535
#[test]
fn test_aarch32_b_a1_a_field_imm24_65535_poweroftwominusone_0_0a00ffff() {
    // Encoding: 0x0A00FFFF
    // Test aarch32_B_A1_A field imm24 = 65535 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=65535
    let encoding: u32 = 0x0A00FFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 65536, boundary: PowerOfTwo }
/// power of 2 (2^16 = 65536)
#[test]
fn test_aarch32_b_a1_a_field_imm24_65536_poweroftwo_0_0a010000() {
    // Encoding: 0x0A010000
    // Test aarch32_B_A1_A field imm24 = 65536 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=65536, cond=0
    let encoding: u32 = 0x0A010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 131071, boundary: PowerOfTwoMinusOne }
/// 2^17 - 1 = 131071
#[test]
fn test_aarch32_b_a1_a_field_imm24_131071_poweroftwominusone_0_0a01ffff() {
    // Encoding: 0x0A01FFFF
    // Test aarch32_B_A1_A field imm24 = 131071 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=131071
    let encoding: u32 = 0x0A01FFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 131072, boundary: PowerOfTwo }
/// power of 2 (2^17 = 131072)
#[test]
fn test_aarch32_b_a1_a_field_imm24_131072_poweroftwo_0_0a020000() {
    // Encoding: 0x0A020000
    // Test aarch32_B_A1_A field imm24 = 131072 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=131072
    let encoding: u32 = 0x0A020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 262143, boundary: PowerOfTwoMinusOne }
/// 2^18 - 1 = 262143
#[test]
fn test_aarch32_b_a1_a_field_imm24_262143_poweroftwominusone_0_0a03ffff() {
    // Encoding: 0x0A03FFFF
    // Test aarch32_B_A1_A field imm24 = 262143 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=262143
    let encoding: u32 = 0x0A03FFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 262144, boundary: PowerOfTwo }
/// power of 2 (2^18 = 262144)
#[test]
fn test_aarch32_b_a1_a_field_imm24_262144_poweroftwo_0_0a040000() {
    // Encoding: 0x0A040000
    // Test aarch32_B_A1_A field imm24 = 262144 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=262144, cond=0
    let encoding: u32 = 0x0A040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 524287, boundary: PowerOfTwoMinusOne }
/// 2^19 - 1 = 524287
#[test]
fn test_aarch32_b_a1_a_field_imm24_524287_poweroftwominusone_0_0a07ffff() {
    // Encoding: 0x0A07FFFF
    // Test aarch32_B_A1_A field imm24 = 524287 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=524287
    let encoding: u32 = 0x0A07FFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 524288, boundary: PowerOfTwo }
/// power of 2 (2^19 = 524288)
#[test]
fn test_aarch32_b_a1_a_field_imm24_524288_poweroftwo_0_0a080000() {
    // Encoding: 0x0A080000
    // Test aarch32_B_A1_A field imm24 = 524288 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=524288, cond=0
    let encoding: u32 = 0x0A080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 1048575, boundary: PowerOfTwoMinusOne }
/// 2^20 - 1 = 1048575
#[test]
fn test_aarch32_b_a1_a_field_imm24_1048575_poweroftwominusone_0_0a0fffff() {
    // Encoding: 0x0A0FFFFF
    // Test aarch32_B_A1_A field imm24 = 1048575 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=1048575
    let encoding: u32 = 0x0A0FFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 1048576, boundary: PowerOfTwo }
/// power of 2 (2^20 = 1048576)
#[test]
fn test_aarch32_b_a1_a_field_imm24_1048576_poweroftwo_0_0a100000() {
    // Encoding: 0x0A100000
    // Test aarch32_B_A1_A field imm24 = 1048576 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=1048576, cond=0
    let encoding: u32 = 0x0A100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 2097151, boundary: PowerOfTwoMinusOne }
/// 2^21 - 1 = 2097151
#[test]
fn test_aarch32_b_a1_a_field_imm24_2097151_poweroftwominusone_0_0a1fffff() {
    // Encoding: 0x0A1FFFFF
    // Test aarch32_B_A1_A field imm24 = 2097151 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=2097151
    let encoding: u32 = 0x0A1FFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 2097152, boundary: PowerOfTwo }
/// power of 2 (2^21 = 2097152)
#[test]
fn test_aarch32_b_a1_a_field_imm24_2097152_poweroftwo_0_0a200000() {
    // Encoding: 0x0A200000
    // Test aarch32_B_A1_A field imm24 = 2097152 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=2097152
    let encoding: u32 = 0x0A200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 4194303, boundary: PowerOfTwoMinusOne }
/// 2^22 - 1 = 4194303
#[test]
fn test_aarch32_b_a1_a_field_imm24_4194303_poweroftwominusone_0_0a3fffff() {
    // Encoding: 0x0A3FFFFF
    // Test aarch32_B_A1_A field imm24 = 4194303 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm24=4194303, cond=0
    let encoding: u32 = 0x0A3FFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 4194304, boundary: PowerOfTwo }
/// power of 2 (2^22 = 4194304)
#[test]
fn test_aarch32_b_a1_a_field_imm24_4194304_poweroftwo_0_0a400000() {
    // Encoding: 0x0A400000
    // Test aarch32_B_A1_A field imm24 = 4194304 (PowerOfTwo)
    // ISET: A32
    // Fields: imm24=4194304, cond=0
    let encoding: u32 = 0x0A400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 8388607, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (8388607)
#[test]
fn test_aarch32_b_a1_a_field_imm24_8388607_poweroftwominusone_0_0a7fffff() {
    // Encoding: 0x0A7FFFFF
    // Test aarch32_B_A1_A field imm24 = 8388607 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm24=8388607
    let encoding: u32 = 0x0A7FFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 8388608, boundary: PowerOfTwo }
/// power of 2 (2^23 = 8388608)
#[test]
fn test_aarch32_b_a1_a_field_imm24_8388608_poweroftwo_0_0a800000() {
    // Encoding: 0x0A800000
    // Test aarch32_B_A1_A field imm24 = 8388608 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, imm24=8388608
    let encoding: u32 = 0x0A800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field imm24 0 +: 24`
/// Requirement: FieldBoundary { field: "imm24", value: 16777215, boundary: Max }
/// maximum immediate (16777215)
#[test]
fn test_aarch32_b_a1_a_field_imm24_16777215_max_0_0affffff() {
    // Encoding: 0x0AFFFFFF
    // Test aarch32_B_A1_A field imm24 = 16777215 (Max)
    // ISET: A32
    // Fields: imm24=16777215, cond=0
    let encoding: u32 = 0x0AFFFFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_b_a1_a_combo_0_0_0a000000() {
    // Encoding: 0x0A000000
    // Test aarch32_B_A1_A field combination: cond=0, imm24=0
    // ISET: A32
    // Fields: cond=0, imm24=0
    let encoding: u32 = 0x0A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_b_a1_a_special_cond_0_condition_eq_0_0a000000() {
    // Encoding: 0x0A000000
    // Test aarch32_B_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: cond=0, imm24=0
    let encoding: u32 = 0x0A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_b_a1_a_special_cond_1_condition_ne_0_1a000000() {
    // Encoding: 0x1A000000
    // Test aarch32_B_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: imm24=0, cond=1
    let encoding: u32 = 0x1A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_b_a1_a_special_cond_2_condition_cs_hs_0_2a000000() {
    // Encoding: 0x2A000000
    // Test aarch32_B_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, imm24=0
    let encoding: u32 = 0x2A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_b_a1_a_special_cond_3_condition_cc_lo_0_3a000000() {
    // Encoding: 0x3A000000
    // Test aarch32_B_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, imm24=0
    let encoding: u32 = 0x3A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_b_a1_a_special_cond_4_condition_mi_0_4a000000() {
    // Encoding: 0x4A000000
    // Test aarch32_B_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, imm24=0
    let encoding: u32 = 0x4A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_b_a1_a_special_cond_5_condition_pl_0_5a000000() {
    // Encoding: 0x5A000000
    // Test aarch32_B_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, imm24=0
    let encoding: u32 = 0x5A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_b_a1_a_special_cond_6_condition_vs_0_6a000000() {
    // Encoding: 0x6A000000
    // Test aarch32_B_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: imm24=0, cond=6
    let encoding: u32 = 0x6A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_b_a1_a_special_cond_7_condition_vc_0_7a000000() {
    // Encoding: 0x7A000000
    // Test aarch32_B_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, imm24=0
    let encoding: u32 = 0x7A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_b_a1_a_special_cond_8_condition_hi_0_8a000000() {
    // Encoding: 0x8A000000
    // Test aarch32_B_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, imm24=0
    let encoding: u32 = 0x8A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_b_a1_a_special_cond_9_condition_ls_0_9a000000() {
    // Encoding: 0x9A000000
    // Test aarch32_B_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, imm24=0
    let encoding: u32 = 0x9A000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_b_a1_a_special_cond_10_condition_ge_0_aa000000() {
    // Encoding: 0xAA000000
    // Test aarch32_B_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, imm24=0
    let encoding: u32 = 0xAA000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_b_a1_a_special_cond_11_condition_lt_0_ba000000() {
    // Encoding: 0xBA000000
    // Test aarch32_B_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, imm24=0
    let encoding: u32 = 0xBA000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_b_a1_a_special_cond_12_condition_gt_0_ca000000() {
    // Encoding: 0xCA000000
    // Test aarch32_B_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: cond=12, imm24=0
    let encoding: u32 = 0xCA000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_b_a1_a_special_cond_13_condition_le_0_da000000() {
    // Encoding: 0xDA000000
    // Test aarch32_B_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: imm24=0, cond=13
    let encoding: u32 = 0xDA000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_b_a1_a_special_cond_14_condition_al_0_ea000000() {
    // Encoding: 0xEA000000
    // Test aarch32_B_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: cond=14, imm24=0
    let encoding: u32 = 0xEA000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_b_a1_a_special_cond_15_condition_nv_0_fa000000() {
    // Encoding: 0xFA000000
    // Test aarch32_B_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: imm24=0, cond=15
    let encoding: u32 = 0xFA000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond 24 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_b_t1_a_field_cond_0_min_0_d0000000() {
    // Thumb encoding (32): 0xD0000000
    // Test aarch32_B_T1_A field cond = 0 (Min)
    // ISET: T32
    // Fields: imm8=0, cond=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD0000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond 24 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_b_t1_a_field_cond_1_poweroftwo_0_d1000000() {
    // Thumb encoding (32): 0xD1000000
    // Test aarch32_B_T1_A field cond = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=0, cond=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD1000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond 24 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_b_t1_a_field_cond_2_poweroftwo_0_d2000000() {
    // Thumb encoding (32): 0xD2000000
    // Test aarch32_B_T1_A field cond = 2 (PowerOfTwo)
    // ISET: T32
    // Fields: cond=2, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD2000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond 24 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_b_t1_a_field_cond_3_poweroftwo_0_d3000000() {
    // Thumb encoding (32): 0xD3000000
    // Test aarch32_B_T1_A field cond = 3 (PowerOfTwo)
    // ISET: T32
    // Fields: cond=3, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD3000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond 24 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_b_t1_a_field_cond_4_poweroftwo_0_d4000000() {
    // Thumb encoding (32): 0xD4000000
    // Test aarch32_B_T1_A field cond = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: cond=4, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD4000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond 24 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_b_t1_a_field_cond_5_poweroftwo_0_d5000000() {
    // Thumb encoding (32): 0xD5000000
    // Test aarch32_B_T1_A field cond = 5 (PowerOfTwo)
    // ISET: T32
    // Fields: cond=5, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD5000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond 24 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_b_t1_a_field_cond_6_poweroftwo_0_d6000000() {
    // Thumb encoding (32): 0xD6000000
    // Test aarch32_B_T1_A field cond = 6 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=0, cond=6
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD6000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond 24 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_b_t1_a_field_cond_7_poweroftwo_0_d7000000() {
    // Thumb encoding (32): 0xD7000000
    // Test aarch32_B_T1_A field cond = 7 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=0, cond=7
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD7000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond 24 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_b_t1_a_field_cond_8_poweroftwo_0_d8000000() {
    // Thumb encoding (32): 0xD8000000
    // Test aarch32_B_T1_A field cond = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=0, cond=8
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD8000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond 24 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_b_t1_a_field_cond_9_poweroftwo_0_d9000000() {
    // Thumb encoding (32): 0xD9000000
    // Test aarch32_B_T1_A field cond = 9 (PowerOfTwo)
    // ISET: T32
    // Fields: cond=9, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD9000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond 24 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_b_t1_a_field_cond_10_poweroftwo_0_da000000() {
    // Thumb encoding (32): 0xDA000000
    // Test aarch32_B_T1_A field cond = 10 (PowerOfTwo)
    // ISET: T32
    // Fields: cond=10, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xDA000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond 24 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_b_t1_a_field_cond_11_poweroftwo_0_db000000() {
    // Thumb encoding (32): 0xDB000000
    // Test aarch32_B_T1_A field cond = 11 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=0, cond=11
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xDB000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond 24 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_b_t1_a_field_cond_12_poweroftwo_0_dc000000() {
    // Thumb encoding (32): 0xDC000000
    // Test aarch32_B_T1_A field cond = 12 (PowerOfTwo)
    // ISET: T32
    // Fields: cond=12, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xDC000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond 24 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_b_t1_a_field_cond_13_poweroftwo_0_dd000000() {
    // Thumb encoding (32): 0xDD000000
    // Test aarch32_B_T1_A field cond = 13 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=0, cond=13
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xDD000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond 24 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_b_t1_a_field_cond_14_poweroftwo_0_de000000() {
    // Thumb encoding (32): 0xDE000000
    // Test aarch32_B_T1_A field cond = 14 (PowerOfTwo)
    // ISET: T32
    // Fields: cond=14, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xDE000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond 24 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_b_t1_a_field_cond_15_max_0_df000000() {
    // Thumb encoding (32): 0xDF000000
    // Test aarch32_B_T1_A field cond = 15 (Max)
    // ISET: T32
    // Fields: imm8=0, cond=15
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xDF000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_b_t1_a_field_imm8_0_zero_0_d0000000() {
    // Thumb encoding (32): 0xD0000000
    // Test aarch32_B_T1_A field imm8 = 0 (Zero)
    // ISET: T32
    // Fields: imm8=0, cond=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD0000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_b_t1_a_field_imm8_1_poweroftwo_0_d0010000() {
    // Thumb encoding (32): 0xD0010000
    // Test aarch32_B_T1_A field imm8 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: cond=0, imm8=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD0010000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_b_t1_a_field_imm8_3_poweroftwominusone_0_d0030000() {
    // Thumb encoding (32): 0xD0030000
    // Test aarch32_B_T1_A field imm8 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: cond=0, imm8=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD0030000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_b_t1_a_field_imm8_4_poweroftwo_0_d0040000() {
    // Thumb encoding (32): 0xD0040000
    // Test aarch32_B_T1_A field imm8 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: cond=0, imm8=4
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD0040000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_b_t1_a_field_imm8_7_poweroftwominusone_0_d0070000() {
    // Thumb encoding (32): 0xD0070000
    // Test aarch32_B_T1_A field imm8 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: cond=0, imm8=7
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD0070000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_b_t1_a_field_imm8_8_poweroftwo_0_d0080000() {
    // Thumb encoding (32): 0xD0080000
    // Test aarch32_B_T1_A field imm8 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=8, cond=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD0080000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_b_t1_a_field_imm8_15_poweroftwominusone_0_d00f0000() {
    // Thumb encoding (32): 0xD00F0000
    // Test aarch32_B_T1_A field imm8 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=15, cond=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD00F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_b_t1_a_field_imm8_16_poweroftwo_0_d0100000() {
    // Thumb encoding (32): 0xD0100000
    // Test aarch32_B_T1_A field imm8 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: cond=0, imm8=16
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD0100000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_b_t1_a_field_imm8_31_poweroftwominusone_0_d01f0000() {
    // Thumb encoding (32): 0xD01F0000
    // Test aarch32_B_T1_A field imm8 = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: cond=0, imm8=31
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD01F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_b_t1_a_field_imm8_32_poweroftwo_0_d0200000() {
    // Thumb encoding (32): 0xD0200000
    // Test aarch32_B_T1_A field imm8 = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: cond=0, imm8=32
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD0200000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_b_t1_a_field_imm8_63_poweroftwominusone_0_d03f0000() {
    // Thumb encoding (32): 0xD03F0000
    // Test aarch32_B_T1_A field imm8 = 63 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=63, cond=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD03F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_b_t1_a_field_imm8_64_poweroftwo_0_d0400000() {
    // Thumb encoding (32): 0xD0400000
    // Test aarch32_B_T1_A field imm8 = 64 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=64, cond=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD0400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 127, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (127)
#[test]
fn test_aarch32_b_t1_a_field_imm8_127_poweroftwominusone_0_d07f0000() {
    // Thumb encoding (32): 0xD07F0000
    // Test aarch32_B_T1_A field imm8 = 127 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=127, cond=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD07F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_b_t1_a_field_imm8_128_poweroftwo_0_d0800000() {
    // Thumb encoding (32): 0xD0800000
    // Test aarch32_B_T1_A field imm8 = 128 (PowerOfTwo)
    // ISET: T32
    // Fields: cond=0, imm8=128
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD0800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field imm8 16 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 255, boundary: Max }
/// maximum immediate (255)
#[test]
fn test_aarch32_b_t1_a_field_imm8_255_max_0_d0ff0000() {
    // Thumb encoding (32): 0xD0FF0000
    // Test aarch32_B_T1_A field imm8 = 255 (Max)
    // ISET: T32
    // Fields: cond=0, imm8=255
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD0FF0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_b_t1_a_combo_0_0_d0000000() {
    // Thumb encoding (32): 0xD0000000
    // Test aarch32_B_T1_A field combination: cond=0, imm8=0
    // ISET: T32
    // Fields: cond=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD0000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_b_t1_a_special_cond_0_condition_eq_0_d0000000() {
    // Thumb encoding (32): 0xD0000000
    // Test aarch32_B_T1_A special value cond = 0 (Condition EQ)
    // ISET: T32
    // Fields: cond=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD0000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_b_t1_a_special_cond_1_condition_ne_0_d1000000() {
    // Thumb encoding (32): 0xD1000000
    // Test aarch32_B_T1_A special value cond = 1 (Condition NE)
    // ISET: T32
    // Fields: cond=1, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD1000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_b_t1_a_special_cond_2_condition_cs_hs_0_d2000000() {
    // Thumb encoding (32): 0xD2000000
    // Test aarch32_B_T1_A special value cond = 2 (Condition CS/HS)
    // ISET: T32
    // Fields: cond=2, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD2000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_b_t1_a_special_cond_3_condition_cc_lo_0_d3000000() {
    // Thumb encoding (32): 0xD3000000
    // Test aarch32_B_T1_A special value cond = 3 (Condition CC/LO)
    // ISET: T32
    // Fields: cond=3, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD3000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_b_t1_a_special_cond_4_condition_mi_0_d4000000() {
    // Thumb encoding (32): 0xD4000000
    // Test aarch32_B_T1_A special value cond = 4 (Condition MI)
    // ISET: T32
    // Fields: cond=4, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD4000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_b_t1_a_special_cond_5_condition_pl_0_d5000000() {
    // Thumb encoding (32): 0xD5000000
    // Test aarch32_B_T1_A special value cond = 5 (Condition PL)
    // ISET: T32
    // Fields: imm8=0, cond=5
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD5000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_b_t1_a_special_cond_6_condition_vs_0_d6000000() {
    // Thumb encoding (32): 0xD6000000
    // Test aarch32_B_T1_A special value cond = 6 (Condition VS)
    // ISET: T32
    // Fields: imm8=0, cond=6
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD6000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_b_t1_a_special_cond_7_condition_vc_0_d7000000() {
    // Thumb encoding (32): 0xD7000000
    // Test aarch32_B_T1_A special value cond = 7 (Condition VC)
    // ISET: T32
    // Fields: cond=7, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD7000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_b_t1_a_special_cond_8_condition_hi_0_d8000000() {
    // Thumb encoding (32): 0xD8000000
    // Test aarch32_B_T1_A special value cond = 8 (Condition HI)
    // ISET: T32
    // Fields: cond=8, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD8000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_b_t1_a_special_cond_9_condition_ls_0_d9000000() {
    // Thumb encoding (32): 0xD9000000
    // Test aarch32_B_T1_A special value cond = 9 (Condition LS)
    // ISET: T32
    // Fields: cond=9, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD9000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_b_t1_a_special_cond_10_condition_ge_0_da000000() {
    // Thumb encoding (32): 0xDA000000
    // Test aarch32_B_T1_A special value cond = 10 (Condition GE)
    // ISET: T32
    // Fields: cond=10, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xDA000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_b_t1_a_special_cond_11_condition_lt_0_db000000() {
    // Thumb encoding (32): 0xDB000000
    // Test aarch32_B_T1_A special value cond = 11 (Condition LT)
    // ISET: T32
    // Fields: cond=11, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xDB000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_b_t1_a_special_cond_12_condition_gt_0_dc000000() {
    // Thumb encoding (32): 0xDC000000
    // Test aarch32_B_T1_A special value cond = 12 (Condition GT)
    // ISET: T32
    // Fields: cond=12, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xDC000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_b_t1_a_special_cond_13_condition_le_0_dd000000() {
    // Thumb encoding (32): 0xDD000000
    // Test aarch32_B_T1_A special value cond = 13 (Condition LE)
    // ISET: T32
    // Fields: imm8=0, cond=13
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xDD000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_b_t1_a_special_cond_14_condition_al_0_de000000() {
    // Thumb encoding (32): 0xDE000000
    // Test aarch32_B_T1_A special value cond = 14 (Condition AL)
    // ISET: T32
    // Fields: cond=14, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xDE000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_b_t1_a_special_cond_15_condition_nv_0_df000000() {
    // Thumb encoding (32): 0xDF000000
    // Test aarch32_B_T1_A special value cond = 15 (Condition NV)
    // ISET: T32
    // Fields: cond=15, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xDF000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T1_A
/// ASL: `Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }`
/// Requirement: UndefinedEncoding { condition: "Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_b_t1_a_invalid_0_0_d0000000() {
    // Thumb encoding (32): 0xD0000000
    // Test aarch32_B_T1_A invalid encoding: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }
    // ISET: T32
    // Fields: cond=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD0000000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_B_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_b_t1_a_invalid_1_0_d0000000() {
    // Thumb encoding (32): 0xD0000000
    // Test aarch32_B_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: imm8=0, cond=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xD0000000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_B_T2_A
/// ASL: `field imm11 16 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_b_t2_a_field_imm11_0_zero_0_e0000000() {
    // Thumb encoding (32): 0xE0000000
    // Test aarch32_B_T2_A field imm11 = 0 (Zero)
    // ISET: T32
    // Fields: imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE0000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T2_A
/// ASL: `field imm11 16 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_b_t2_a_field_imm11_1_poweroftwo_0_e0010000() {
    // Thumb encoding (32): 0xE0010000
    // Test aarch32_B_T2_A field imm11 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm11=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE0010000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T2_A
/// ASL: `field imm11 16 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_b_t2_a_field_imm11_3_poweroftwominusone_0_e0030000() {
    // Thumb encoding (32): 0xE0030000
    // Test aarch32_B_T2_A field imm11 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm11=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE0030000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T2_A
/// ASL: `field imm11 16 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_b_t2_a_field_imm11_4_poweroftwo_0_e0040000() {
    // Thumb encoding (32): 0xE0040000
    // Test aarch32_B_T2_A field imm11 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: imm11=4
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE0040000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T2_A
/// ASL: `field imm11 16 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_b_t2_a_field_imm11_7_poweroftwominusone_0_e0070000() {
    // Thumb encoding (32): 0xE0070000
    // Test aarch32_B_T2_A field imm11 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm11=7
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE0070000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T2_A
/// ASL: `field imm11 16 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_b_t2_a_field_imm11_8_poweroftwo_0_e0080000() {
    // Thumb encoding (32): 0xE0080000
    // Test aarch32_B_T2_A field imm11 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: imm11=8
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE0080000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T2_A
/// ASL: `field imm11 16 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_b_t2_a_field_imm11_15_poweroftwominusone_0_e00f0000() {
    // Thumb encoding (32): 0xE00F0000
    // Test aarch32_B_T2_A field imm11 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm11=15
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE00F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T2_A
/// ASL: `field imm11 16 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_b_t2_a_field_imm11_16_poweroftwo_0_e0100000() {
    // Thumb encoding (32): 0xE0100000
    // Test aarch32_B_T2_A field imm11 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: imm11=16
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE0100000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T2_A
/// ASL: `field imm11 16 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_b_t2_a_field_imm11_31_poweroftwominusone_0_e01f0000() {
    // Thumb encoding (32): 0xE01F0000
    // Test aarch32_B_T2_A field imm11 = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm11=31
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE01F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T2_A
/// ASL: `field imm11 16 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_b_t2_a_field_imm11_32_poweroftwo_0_e0200000() {
    // Thumb encoding (32): 0xE0200000
    // Test aarch32_B_T2_A field imm11 = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: imm11=32
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE0200000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T2_A
/// ASL: `field imm11 16 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_b_t2_a_field_imm11_63_poweroftwominusone_0_e03f0000() {
    // Thumb encoding (32): 0xE03F0000
    // Test aarch32_B_T2_A field imm11 = 63 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm11=63
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE03F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T2_A
/// ASL: `field imm11 16 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_b_t2_a_field_imm11_64_poweroftwo_0_e0400000() {
    // Thumb encoding (32): 0xE0400000
    // Test aarch32_B_T2_A field imm11 = 64 (PowerOfTwo)
    // ISET: T32
    // Fields: imm11=64
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE0400000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T2_A
/// ASL: `field imm11 16 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch32_b_t2_a_field_imm11_127_poweroftwominusone_0_e07f0000() {
    // Thumb encoding (32): 0xE07F0000
    // Test aarch32_B_T2_A field imm11 = 127 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm11=127
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE07F0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T2_A
/// ASL: `field imm11 16 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_b_t2_a_field_imm11_128_poweroftwo_0_e0800000() {
    // Thumb encoding (32): 0xE0800000
    // Test aarch32_B_T2_A field imm11 = 128 (PowerOfTwo)
    // ISET: T32
    // Fields: imm11=128
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE0800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T2_A
/// ASL: `field imm11 16 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch32_b_t2_a_field_imm11_255_poweroftwominusone_0_e0ff0000() {
    // Thumb encoding (32): 0xE0FF0000
    // Test aarch32_B_T2_A field imm11 = 255 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm11=255
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE0FF0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T2_A
/// ASL: `field imm11 16 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch32_b_t2_a_field_imm11_256_poweroftwo_0_e1000000() {
    // Thumb encoding (32): 0xE1000000
    // Test aarch32_B_T2_A field imm11 = 256 (PowerOfTwo)
    // ISET: T32
    // Fields: imm11=256
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE1000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T2_A
/// ASL: `field imm11 16 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch32_b_t2_a_field_imm11_511_poweroftwominusone_0_e1ff0000() {
    // Thumb encoding (32): 0xE1FF0000
    // Test aarch32_B_T2_A field imm11 = 511 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm11=511
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE1FF0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T2_A
/// ASL: `field imm11 16 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch32_b_t2_a_field_imm11_512_poweroftwo_0_e2000000() {
    // Thumb encoding (32): 0xE2000000
    // Test aarch32_B_T2_A field imm11 = 512 (PowerOfTwo)
    // ISET: T32
    // Fields: imm11=512
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE2000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T2_A
/// ASL: `field imm11 16 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 1023, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (1023)
#[test]
fn test_aarch32_b_t2_a_field_imm11_1023_poweroftwominusone_0_e3ff0000() {
    // Thumb encoding (32): 0xE3FF0000
    // Test aarch32_B_T2_A field imm11 = 1023 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm11=1023
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE3FF0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T2_A
/// ASL: `field imm11 16 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch32_b_t2_a_field_imm11_1024_poweroftwo_0_e4000000() {
    // Thumb encoding (32): 0xE4000000
    // Test aarch32_B_T2_A field imm11 = 1024 (PowerOfTwo)
    // ISET: T32
    // Fields: imm11=1024
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE4000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T2_A
/// ASL: `field imm11 16 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 2047, boundary: Max }
/// maximum immediate (2047)
#[test]
fn test_aarch32_b_t2_a_field_imm11_2047_max_0_e7ff0000() {
    // Thumb encoding (32): 0xE7FF0000
    // Test aarch32_B_T2_A field imm11 = 2047 (Max)
    // ISET: T32
    // Fields: imm11=2047
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE7FF0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T2_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm11=0 (immediate value 0)
#[test]
fn test_aarch32_b_t2_a_combo_0_0_e0000000() {
    // Thumb encoding (32): 0xE0000000
    // Test aarch32_B_T2_A field combination: imm11=0
    // ISET: T32
    // Fields: imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE0000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T2_A
/// ASL: `Binary { op: And, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "LastInITBlock" }, args: [] } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: And, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"LastInITBlock\" }, args: [] } } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_b_t2_a_invalid_0_0_e0000000() {
    // Thumb encoding (32): 0xE0000000
    // Test aarch32_B_T2_A invalid encoding: Binary { op: And, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "LastInITBlock" }, args: [] } } }
    // ISET: T32
    // Fields: imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE0000000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_B_T2_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_b_t2_a_invalid_1_0_e0000000() {
    // Thumb encoding (32): 0xE0000000
    // Test aarch32_B_T2_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xE0000000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field S 26 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_b_t3_a_field_s_0_min_8000_f0008000() {
    // Thumb encoding (32): 0xF0008000
    // Test aarch32_B_T3_A field S = 0 (Min)
    // ISET: T32
    // Fields: imm11=0, S=0, cond=0, imm6=0, J1=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0008000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field S 26 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_b_t3_a_field_s_1_max_8000_f4008000() {
    // Thumb encoding (32): 0xF4008000
    // Test aarch32_B_T3_A field S = 1 (Max)
    // ISET: T32
    // Fields: cond=0, imm6=0, J1=0, J2=0, S=1, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF4008000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond 22 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_b_t3_a_field_cond_0_min_8000_f0008000() {
    // Thumb encoding (32): 0xF0008000
    // Test aarch32_B_T3_A field cond = 0 (Min)
    // ISET: T32
    // Fields: J2=0, imm11=0, cond=0, imm6=0, S=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0008000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond 22 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_b_t3_a_field_cond_1_poweroftwo_8000_f0408000() {
    // Thumb encoding (32): 0xF0408000
    // Test aarch32_B_T3_A field cond = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: cond=1, S=0, imm11=0, imm6=0, J1=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0408000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond 22 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_b_t3_a_field_cond_2_poweroftwo_8000_f0808000() {
    // Thumb encoding (32): 0xF0808000
    // Test aarch32_B_T3_A field cond = 2 (PowerOfTwo)
    // ISET: T32
    // Fields: J2=0, J1=0, imm11=0, cond=2, imm6=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0808000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond 22 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_b_t3_a_field_cond_3_poweroftwo_8000_f0c08000() {
    // Thumb encoding (32): 0xF0C08000
    // Test aarch32_B_T3_A field cond = 3 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, J1=0, imm6=0, J2=0, cond=3, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0C08000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond 22 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_b_t3_a_field_cond_4_poweroftwo_8000_f1008000() {
    // Thumb encoding (32): 0xF1008000
    // Test aarch32_B_T3_A field cond = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: J1=0, imm11=0, J2=0, S=0, cond=4, imm6=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1008000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond 22 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_b_t3_a_field_cond_5_poweroftwo_8000_f1408000() {
    // Thumb encoding (32): 0xF1408000
    // Test aarch32_B_T3_A field cond = 5 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, J1=0, imm6=0, imm11=0, J2=0, cond=5
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1408000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond 22 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_b_t3_a_field_cond_6_poweroftwo_8000_f1808000() {
    // Thumb encoding (32): 0xF1808000
    // Test aarch32_B_T3_A field cond = 6 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, cond=6, J2=0, imm6=0, J1=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1808000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond 22 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_b_t3_a_field_cond_7_poweroftwo_8000_f1c08000() {
    // Thumb encoding (32): 0xF1C08000
    // Test aarch32_B_T3_A field cond = 7 (PowerOfTwo)
    // ISET: T32
    // Fields: imm6=0, S=0, cond=7, J2=0, imm11=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1C08000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond 22 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_b_t3_a_field_cond_8_poweroftwo_8000_f2008000() {
    // Thumb encoding (32): 0xF2008000
    // Test aarch32_B_T3_A field cond = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: imm6=0, S=0, cond=8, J2=0, J1=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2008000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond 22 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_b_t3_a_field_cond_9_poweroftwo_8000_f2408000() {
    // Thumb encoding (32): 0xF2408000
    // Test aarch32_B_T3_A field cond = 9 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, imm6=0, cond=9, J2=0, imm11=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2408000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond 22 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_b_t3_a_field_cond_10_poweroftwo_8000_f2808000() {
    // Thumb encoding (32): 0xF2808000
    // Test aarch32_B_T3_A field cond = 10 (PowerOfTwo)
    // ISET: T32
    // Fields: J1=0, cond=10, imm6=0, imm11=0, J2=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2808000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond 22 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_b_t3_a_field_cond_11_poweroftwo_8000_f2c08000() {
    // Thumb encoding (32): 0xF2C08000
    // Test aarch32_B_T3_A field cond = 11 (PowerOfTwo)
    // ISET: T32
    // Fields: imm6=0, cond=11, J2=0, S=0, J1=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C08000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond 22 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_b_t3_a_field_cond_12_poweroftwo_8000_f3008000() {
    // Thumb encoding (32): 0xF3008000
    // Test aarch32_B_T3_A field cond = 12 (PowerOfTwo)
    // ISET: T32
    // Fields: J1=0, cond=12, imm11=0, J2=0, S=0, imm6=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF3008000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond 22 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_b_t3_a_field_cond_13_poweroftwo_8000_f3408000() {
    // Thumb encoding (32): 0xF3408000
    // Test aarch32_B_T3_A field cond = 13 (PowerOfTwo)
    // ISET: T32
    // Fields: cond=13, J1=0, imm11=0, imm6=0, S=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF3408000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond 22 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_b_t3_a_field_cond_14_poweroftwo_8000_f3808000() {
    // Thumb encoding (32): 0xF3808000
    // Test aarch32_B_T3_A field cond = 14 (PowerOfTwo)
    // ISET: T32
    // Fields: cond=14, J2=0, S=0, imm6=0, imm11=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF3808000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond 22 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_b_t3_a_field_cond_15_max_8000_f3c08000() {
    // Thumb encoding (32): 0xF3C08000
    // Test aarch32_B_T3_A field cond = 15 (Max)
    // ISET: T32
    // Fields: cond=15, J1=0, imm11=0, S=0, imm6=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF3C08000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_b_t3_a_field_imm6_0_zero_8000_f0008000() {
    // Thumb encoding (32): 0xF0008000
    // Test aarch32_B_T3_A field imm6 = 0 (Zero)
    // ISET: T32
    // Fields: cond=0, S=0, J1=0, J2=0, imm11=0, imm6=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0008000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_b_t3_a_field_imm6_1_poweroftwo_8000_f0018000() {
    // Thumb encoding (32): 0xF0018000
    // Test aarch32_B_T3_A field imm6 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, J2=0, imm6=1, cond=0, imm11=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0018000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_b_t3_a_field_imm6_3_poweroftwominusone_8000_f0038000() {
    // Thumb encoding (32): 0xF0038000
    // Test aarch32_B_T3_A field imm6 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: J2=0, imm11=0, imm6=3, S=0, J1=0, cond=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0038000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_b_t3_a_field_imm6_4_poweroftwo_8000_f0048000() {
    // Thumb encoding (32): 0xF0048000
    // Test aarch32_B_T3_A field imm6 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: J1=0, cond=0, S=0, imm6=4, J2=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0048000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_b_t3_a_field_imm6_7_poweroftwominusone_8000_f0078000() {
    // Thumb encoding (32): 0xF0078000
    // Test aarch32_B_T3_A field imm6 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: J1=0, S=0, J2=0, imm11=0, cond=0, imm6=7
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0078000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_b_t3_a_field_imm6_8_poweroftwo_8000_f0088000() {
    // Thumb encoding (32): 0xF0088000
    // Test aarch32_B_T3_A field imm6 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: J1=0, cond=0, imm11=0, J2=0, S=0, imm6=8
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0088000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_b_t3_a_field_imm6_15_poweroftwominusone_8000_f00f8000() {
    // Thumb encoding (32): 0xF00F8000
    // Test aarch32_B_T3_A field imm6 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: S=0, cond=0, J2=0, imm6=15, J1=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF00F8000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_b_t3_a_field_imm6_16_poweroftwo_8000_f0108000() {
    // Thumb encoding (32): 0xF0108000
    // Test aarch32_B_T3_A field imm6 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: imm11=0, J1=0, J2=0, imm6=16, S=0, cond=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0108000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 31, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (31)
#[test]
fn test_aarch32_b_t3_a_field_imm6_31_poweroftwominusone_8000_f01f8000() {
    // Thumb encoding (32): 0xF01F8000
    // Test aarch32_B_T3_A field imm6 = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm11=0, S=0, cond=0, imm6=31, J2=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF01F8000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_b_t3_a_field_imm6_32_poweroftwo_8000_f0208000() {
    // Thumb encoding (32): 0xF0208000
    // Test aarch32_B_T3_A field imm6 = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: imm11=0, S=0, imm6=32, cond=0, J1=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0208000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 63, boundary: Max }
/// maximum immediate (63)
#[test]
fn test_aarch32_b_t3_a_field_imm6_63_max_8000_f03f8000() {
    // Thumb encoding (32): 0xF03F8000
    // Test aarch32_B_T3_A field imm6 = 63 (Max)
    // ISET: T32
    // Fields: imm11=0, S=0, cond=0, J1=0, J2=0, imm6=63
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF03F8000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field J1 13 +: 1`
/// Requirement: FieldBoundary { field: "J1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_b_t3_a_field_j1_0_min_8000_f0008000() {
    // Thumb encoding (32): 0xF0008000
    // Test aarch32_B_T3_A field J1 = 0 (Min)
    // ISET: T32
    // Fields: J2=0, imm11=0, cond=0, imm6=0, S=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0008000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field J1 13 +: 1`
/// Requirement: FieldBoundary { field: "J1", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_b_t3_a_field_j1_1_max_8000_f000a000() {
    // Thumb encoding (32): 0xF000A000
    // Test aarch32_B_T3_A field J1 = 1 (Max)
    // ISET: T32
    // Fields: J2=0, J1=1, imm11=0, imm6=0, cond=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000A000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field J2 11 +: 1`
/// Requirement: FieldBoundary { field: "J2", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_b_t3_a_field_j2_0_min_8000_f0008000() {
    // Thumb encoding (32): 0xF0008000
    // Test aarch32_B_T3_A field J2 = 0 (Min)
    // ISET: T32
    // Fields: S=0, imm6=0, J1=0, J2=0, imm11=0, cond=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0008000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field J2 11 +: 1`
/// Requirement: FieldBoundary { field: "J2", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_b_t3_a_field_j2_1_max_8000_f0008800() {
    // Thumb encoding (32): 0xF0008800
    // Test aarch32_B_T3_A field J2 = 1 (Max)
    // ISET: T32
    // Fields: S=0, cond=0, J2=1, imm11=0, imm6=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0008800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_b_t3_a_field_imm11_0_zero_8000_f0008000() {
    // Thumb encoding (32): 0xF0008000
    // Test aarch32_B_T3_A field imm11 = 0 (Zero)
    // ISET: T32
    // Fields: cond=0, imm6=0, J2=0, J1=0, imm11=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0008000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_b_t3_a_field_imm11_1_poweroftwo_8000_f0008001() {
    // Thumb encoding (32): 0xF0008001
    // Test aarch32_B_T3_A field imm11 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: J1=0, J2=0, imm11=1, cond=0, S=0, imm6=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0008001;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_b_t3_a_field_imm11_3_poweroftwominusone_8000_f0008003() {
    // Thumb encoding (32): 0xF0008003
    // Test aarch32_B_T3_A field imm11 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: S=0, J1=0, J2=0, imm6=0, imm11=3, cond=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0008003;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_b_t3_a_field_imm11_4_poweroftwo_8000_f0008004() {
    // Thumb encoding (32): 0xF0008004
    // Test aarch32_B_T3_A field imm11 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: J1=0, J2=0, imm11=4, S=0, cond=0, imm6=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0008004;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_b_t3_a_field_imm11_7_poweroftwominusone_8000_f0008007() {
    // Thumb encoding (32): 0xF0008007
    // Test aarch32_B_T3_A field imm11 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: cond=0, J1=0, J2=0, S=0, imm6=0, imm11=7
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0008007;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_b_t3_a_field_imm11_8_poweroftwo_8000_f0008008() {
    // Thumb encoding (32): 0xF0008008
    // Test aarch32_B_T3_A field imm11 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, imm6=0, J2=0, imm11=8, cond=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0008008;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_b_t3_a_field_imm11_15_poweroftwominusone_8000_f000800f() {
    // Thumb encoding (32): 0xF000800F
    // Test aarch32_B_T3_A field imm11 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: S=0, cond=0, J1=0, J2=0, imm11=15, imm6=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000800F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_b_t3_a_field_imm11_16_poweroftwo_8000_f0008010() {
    // Thumb encoding (32): 0xF0008010
    // Test aarch32_B_T3_A field imm11 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: J1=0, J2=0, imm6=0, S=0, cond=0, imm11=16
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0008010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_b_t3_a_field_imm11_31_poweroftwominusone_8000_f000801f() {
    // Thumb encoding (32): 0xF000801F
    // Test aarch32_B_T3_A field imm11 = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: cond=0, J1=0, imm11=31, S=0, imm6=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000801F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_b_t3_a_field_imm11_32_poweroftwo_8000_f0008020() {
    // Thumb encoding (32): 0xF0008020
    // Test aarch32_B_T3_A field imm11 = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: J2=0, imm11=32, S=0, cond=0, imm6=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0008020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_b_t3_a_field_imm11_63_poweroftwominusone_8000_f000803f() {
    // Thumb encoding (32): 0xF000803F
    // Test aarch32_B_T3_A field imm11 = 63 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: S=0, cond=0, imm6=0, imm11=63, J1=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000803F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_b_t3_a_field_imm11_64_poweroftwo_8000_f0008040() {
    // Thumb encoding (32): 0xF0008040
    // Test aarch32_B_T3_A field imm11 = 64 (PowerOfTwo)
    // ISET: T32
    // Fields: J1=0, imm6=0, S=0, J2=0, imm11=64, cond=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0008040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch32_b_t3_a_field_imm11_127_poweroftwominusone_8000_f000807f() {
    // Thumb encoding (32): 0xF000807F
    // Test aarch32_B_T3_A field imm11 = 127 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: cond=0, J1=0, imm11=127, J2=0, S=0, imm6=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000807F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_b_t3_a_field_imm11_128_poweroftwo_8000_f0008080() {
    // Thumb encoding (32): 0xF0008080
    // Test aarch32_B_T3_A field imm11 = 128 (PowerOfTwo)
    // ISET: T32
    // Fields: imm6=0, J2=0, imm11=128, cond=0, J1=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0008080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch32_b_t3_a_field_imm11_255_poweroftwominusone_8000_f00080ff() {
    // Thumb encoding (32): 0xF00080FF
    // Test aarch32_B_T3_A field imm11 = 255 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: cond=0, imm6=0, J1=0, S=0, J2=0, imm11=255
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF00080FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch32_b_t3_a_field_imm11_256_poweroftwo_8000_f0008100() {
    // Thumb encoding (32): 0xF0008100
    // Test aarch32_B_T3_A field imm11 = 256 (PowerOfTwo)
    // ISET: T32
    // Fields: cond=0, imm11=256, imm6=0, J1=0, S=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0008100;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch32_b_t3_a_field_imm11_511_poweroftwominusone_8000_f00081ff() {
    // Thumb encoding (32): 0xF00081FF
    // Test aarch32_B_T3_A field imm11 = 511 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm11=511, J1=0, cond=0, J2=0, imm6=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF00081FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch32_b_t3_a_field_imm11_512_poweroftwo_8000_f0008200() {
    // Thumb encoding (32): 0xF0008200
    // Test aarch32_B_T3_A field imm11 = 512 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, J2=0, imm6=0, J1=0, imm11=512, cond=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0008200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 1023, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (1023)
#[test]
fn test_aarch32_b_t3_a_field_imm11_1023_poweroftwominusone_8000_f00083ff() {
    // Thumb encoding (32): 0xF00083FF
    // Test aarch32_B_T3_A field imm11 = 1023 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: J1=0, J2=0, cond=0, imm6=0, imm11=1023, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF00083FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch32_b_t3_a_field_imm11_1024_poweroftwo_8000_f0008400() {
    // Thumb encoding (32): 0xF0008400
    // Test aarch32_B_T3_A field imm11 = 1024 (PowerOfTwo)
    // ISET: T32
    // Fields: imm11=1024, imm6=0, S=0, cond=0, J1=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0008400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 2047, boundary: Max }
/// maximum immediate (2047)
#[test]
fn test_aarch32_b_t3_a_field_imm11_2047_max_8000_f00087ff() {
    // Thumb encoding (32): 0xF00087FF
    // Test aarch32_B_T3_A field imm11 = 2047 (Max)
    // ISET: T32
    // Fields: J1=0, J2=0, imm11=2047, imm6=0, cond=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF00087FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=0 (8-bit / byte size)
#[test]
fn test_aarch32_b_t3_a_combo_0_8000_f0008000() {
    // Thumb encoding (32): 0xF0008000
    // Test aarch32_B_T3_A field combination: S=0, cond=0, imm6=0, J1=0, J2=0, imm11=0
    // ISET: T32
    // Fields: cond=0, S=0, J2=0, J1=0, imm11=0, imm6=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0008000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_b_t3_a_special_s_0_size_variant_0_32768_f0008000() {
    // Thumb encoding (32): 0xF0008000
    // Test aarch32_B_T3_A special value S = 0 (Size variant 0)
    // ISET: T32
    // Fields: cond=0, imm11=0, J1=0, J2=0, S=0, imm6=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0008000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_b_t3_a_special_s_1_size_variant_1_32768_f4008000() {
    // Thumb encoding (32): 0xF4008000
    // Test aarch32_B_T3_A special value S = 1 (Size variant 1)
    // ISET: T32
    // Fields: J1=0, imm11=0, imm6=0, S=1, J2=0, cond=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF4008000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_b_t3_a_special_cond_0_condition_eq_32768_f0008000() {
    // Thumb encoding (32): 0xF0008000
    // Test aarch32_B_T3_A special value cond = 0 (Condition EQ)
    // ISET: T32
    // Fields: S=0, cond=0, imm6=0, imm11=0, J2=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0008000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_b_t3_a_special_cond_1_condition_ne_32768_f0408000() {
    // Thumb encoding (32): 0xF0408000
    // Test aarch32_B_T3_A special value cond = 1 (Condition NE)
    // ISET: T32
    // Fields: imm6=0, cond=1, S=0, J2=0, imm11=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0408000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_b_t3_a_special_cond_2_condition_cs_hs_32768_f0808000() {
    // Thumb encoding (32): 0xF0808000
    // Test aarch32_B_T3_A special value cond = 2 (Condition CS/HS)
    // ISET: T32
    // Fields: cond=2, J1=0, imm11=0, imm6=0, J2=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0808000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_b_t3_a_special_cond_3_condition_cc_lo_32768_f0c08000() {
    // Thumb encoding (32): 0xF0C08000
    // Test aarch32_B_T3_A special value cond = 3 (Condition CC/LO)
    // ISET: T32
    // Fields: S=0, imm6=0, cond=3, J1=0, J2=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0C08000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_b_t3_a_special_cond_4_condition_mi_32768_f1008000() {
    // Thumb encoding (32): 0xF1008000
    // Test aarch32_B_T3_A special value cond = 4 (Condition MI)
    // ISET: T32
    // Fields: imm6=0, cond=4, J1=0, J2=0, imm11=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1008000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_b_t3_a_special_cond_5_condition_pl_32768_f1408000() {
    // Thumb encoding (32): 0xF1408000
    // Test aarch32_B_T3_A special value cond = 5 (Condition PL)
    // ISET: T32
    // Fields: J1=0, cond=5, imm11=0, imm6=0, J2=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1408000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_b_t3_a_special_cond_6_condition_vs_32768_f1808000() {
    // Thumb encoding (32): 0xF1808000
    // Test aarch32_B_T3_A special value cond = 6 (Condition VS)
    // ISET: T32
    // Fields: S=0, imm6=0, cond=6, J1=0, J2=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1808000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_b_t3_a_special_cond_7_condition_vc_32768_f1c08000() {
    // Thumb encoding (32): 0xF1C08000
    // Test aarch32_B_T3_A special value cond = 7 (Condition VC)
    // ISET: T32
    // Fields: J2=0, cond=7, imm6=0, S=0, J1=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1C08000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_b_t3_a_special_cond_8_condition_hi_32768_f2008000() {
    // Thumb encoding (32): 0xF2008000
    // Test aarch32_B_T3_A special value cond = 8 (Condition HI)
    // ISET: T32
    // Fields: S=0, cond=8, imm11=0, imm6=0, J1=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2008000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_b_t3_a_special_cond_9_condition_ls_32768_f2408000() {
    // Thumb encoding (32): 0xF2408000
    // Test aarch32_B_T3_A special value cond = 9 (Condition LS)
    // ISET: T32
    // Fields: imm6=0, cond=9, S=0, J1=0, J2=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2408000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_b_t3_a_special_cond_10_condition_ge_32768_f2808000() {
    // Thumb encoding (32): 0xF2808000
    // Test aarch32_B_T3_A special value cond = 10 (Condition GE)
    // ISET: T32
    // Fields: J1=0, imm11=0, S=0, cond=10, J2=0, imm6=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2808000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_b_t3_a_special_cond_11_condition_lt_32768_f2c08000() {
    // Thumb encoding (32): 0xF2C08000
    // Test aarch32_B_T3_A special value cond = 11 (Condition LT)
    // ISET: T32
    // Fields: J2=0, imm6=0, imm11=0, J1=0, S=0, cond=11
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2C08000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_b_t3_a_special_cond_12_condition_gt_32768_f3008000() {
    // Thumb encoding (32): 0xF3008000
    // Test aarch32_B_T3_A special value cond = 12 (Condition GT)
    // ISET: T32
    // Fields: cond=12, S=0, J2=0, imm6=0, imm11=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF3008000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_b_t3_a_special_cond_13_condition_le_32768_f3408000() {
    // Thumb encoding (32): 0xF3408000
    // Test aarch32_B_T3_A special value cond = 13 (Condition LE)
    // ISET: T32
    // Fields: J2=0, imm6=0, cond=13, S=0, imm11=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF3408000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_b_t3_a_special_cond_14_condition_al_32768_f3808000() {
    // Thumb encoding (32): 0xF3808000
    // Test aarch32_B_T3_A special value cond = 14 (Condition AL)
    // ISET: T32
    // Fields: J2=0, S=0, imm11=0, J1=0, imm6=0, cond=14
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF3808000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_b_t3_a_special_cond_15_condition_nv_32768_f3c08000() {
    // Thumb encoding (32): 0xF3C08000
    // Test aarch32_B_T3_A special value cond = 15 (Condition NV)
    // ISET: T32
    // Fields: S=0, imm6=0, J1=0, cond=15, J2=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF3C08000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T3_A
/// ASL: `Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }`
/// Requirement: UndefinedEncoding { condition: "Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_b_t3_a_invalid_0_8000_f0008000() {
    // Thumb encoding (32): 0xF0008000
    // Test aarch32_B_T3_A invalid encoding: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }
    // ISET: T32
    // Fields: J2=0, cond=0, imm6=0, S=0, J1=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0008000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_B_T3_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_b_t3_a_invalid_1_8000_f0008000() {
    // Thumb encoding (32): 0xF0008000
    // Test aarch32_B_T3_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: J1=0, S=0, cond=0, J2=0, imm6=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0008000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field S 26 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_b_t4_a_field_s_0_min_9000_f0009000() {
    // Thumb encoding (32): 0xF0009000
    // Test aarch32_B_T4_A field S = 0 (Min)
    // ISET: T32
    // Fields: imm11=0, imm10=0, S=0, J1=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0009000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field S 26 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_b_t4_a_field_s_1_max_9000_f4009000() {
    // Thumb encoding (32): 0xF4009000
    // Test aarch32_B_T4_A field S = 1 (Max)
    // ISET: T32
    // Fields: imm11=0, imm10=0, J1=0, J2=0, S=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF4009000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_b_t4_a_field_imm10_0_zero_9000_f0009000() {
    // Thumb encoding (32): 0xF0009000
    // Test aarch32_B_T4_A field imm10 = 0 (Zero)
    // ISET: T32
    // Fields: S=0, imm10=0, J1=0, imm11=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0009000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_b_t4_a_field_imm10_1_poweroftwo_9000_f0019000() {
    // Thumb encoding (32): 0xF0019000
    // Test aarch32_B_T4_A field imm10 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm10=1, S=0, J2=0, J1=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0019000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_b_t4_a_field_imm10_3_poweroftwominusone_9000_f0039000() {
    // Thumb encoding (32): 0xF0039000
    // Test aarch32_B_T4_A field imm10 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: S=0, imm10=3, J2=0, J1=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0039000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_b_t4_a_field_imm10_4_poweroftwo_9000_f0049000() {
    // Thumb encoding (32): 0xF0049000
    // Test aarch32_B_T4_A field imm10 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: imm11=0, S=0, imm10=4, J1=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0049000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_b_t4_a_field_imm10_7_poweroftwominusone_9000_f0079000() {
    // Thumb encoding (32): 0xF0079000
    // Test aarch32_B_T4_A field imm10 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm11=0, J1=0, J2=0, S=0, imm10=7
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0079000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_b_t4_a_field_imm10_8_poweroftwo_9000_f0089000() {
    // Thumb encoding (32): 0xF0089000
    // Test aarch32_B_T4_A field imm10 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: J1=0, J2=0, S=0, imm10=8, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0089000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_b_t4_a_field_imm10_15_poweroftwominusone_9000_f00f9000() {
    // Thumb encoding (32): 0xF00F9000
    // Test aarch32_B_T4_A field imm10 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm11=0, J1=0, S=0, imm10=15, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF00F9000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_b_t4_a_field_imm10_16_poweroftwo_9000_f0109000() {
    // Thumb encoding (32): 0xF0109000
    // Test aarch32_B_T4_A field imm10 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: J2=0, S=0, J1=0, imm11=0, imm10=16
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0109000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_b_t4_a_field_imm10_31_poweroftwominusone_9000_f01f9000() {
    // Thumb encoding (32): 0xF01F9000
    // Test aarch32_B_T4_A field imm10 = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: J2=0, S=0, imm10=31, imm11=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF01F9000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_b_t4_a_field_imm10_32_poweroftwo_9000_f0209000() {
    // Thumb encoding (32): 0xF0209000
    // Test aarch32_B_T4_A field imm10 = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: imm10=32, S=0, imm11=0, J2=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0209000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_b_t4_a_field_imm10_63_poweroftwominusone_9000_f03f9000() {
    // Thumb encoding (32): 0xF03F9000
    // Test aarch32_B_T4_A field imm10 = 63 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm11=0, S=0, J2=0, imm10=63, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF03F9000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_b_t4_a_field_imm10_64_poweroftwo_9000_f0409000() {
    // Thumb encoding (32): 0xF0409000
    // Test aarch32_B_T4_A field imm10 = 64 (PowerOfTwo)
    // ISET: T32
    // Fields: imm10=64, imm11=0, J1=0, J2=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0409000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch32_b_t4_a_field_imm10_127_poweroftwominusone_9000_f07f9000() {
    // Thumb encoding (32): 0xF07F9000
    // Test aarch32_B_T4_A field imm10 = 127 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: J2=0, S=0, imm10=127, imm11=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF07F9000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_b_t4_a_field_imm10_128_poweroftwo_9000_f0809000() {
    // Thumb encoding (32): 0xF0809000
    // Test aarch32_B_T4_A field imm10 = 128 (PowerOfTwo)
    // ISET: T32
    // Fields: imm10=128, J1=0, S=0, J2=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0809000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch32_b_t4_a_field_imm10_255_poweroftwominusone_9000_f0ff9000() {
    // Thumb encoding (32): 0xF0FF9000
    // Test aarch32_B_T4_A field imm10 = 255 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm11=0, S=0, imm10=255, J2=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0FF9000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch32_b_t4_a_field_imm10_256_poweroftwo_9000_f1009000() {
    // Thumb encoding (32): 0xF1009000
    // Test aarch32_B_T4_A field imm10 = 256 (PowerOfTwo)
    // ISET: T32
    // Fields: J2=0, imm11=0, J1=0, S=0, imm10=256
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1009000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 511, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (511)
#[test]
fn test_aarch32_b_t4_a_field_imm10_511_poweroftwominusone_9000_f1ff9000() {
    // Thumb encoding (32): 0xF1FF9000
    // Test aarch32_B_T4_A field imm10 = 511 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm10=511, S=0, J2=0, imm11=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF1FF9000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch32_b_t4_a_field_imm10_512_poweroftwo_9000_f2009000() {
    // Thumb encoding (32): 0xF2009000
    // Test aarch32_B_T4_A field imm10 = 512 (PowerOfTwo)
    // ISET: T32
    // Fields: imm11=0, S=0, imm10=512, J2=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF2009000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm10 16 +: 10`
/// Requirement: FieldBoundary { field: "imm10", value: 1023, boundary: Max }
/// maximum immediate (1023)
#[test]
fn test_aarch32_b_t4_a_field_imm10_1023_max_9000_f3ff9000() {
    // Thumb encoding (32): 0xF3FF9000
    // Test aarch32_B_T4_A field imm10 = 1023 (Max)
    // ISET: T32
    // Fields: J2=0, J1=0, imm11=0, S=0, imm10=1023
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF3FF9000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field J1 13 +: 1`
/// Requirement: FieldBoundary { field: "J1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_b_t4_a_field_j1_0_min_9000_f0009000() {
    // Thumb encoding (32): 0xF0009000
    // Test aarch32_B_T4_A field J1 = 0 (Min)
    // ISET: T32
    // Fields: imm11=0, J1=0, imm10=0, S=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0009000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field J1 13 +: 1`
/// Requirement: FieldBoundary { field: "J1", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_b_t4_a_field_j1_1_max_9000_f000b000() {
    // Thumb encoding (32): 0xF000B000
    // Test aarch32_B_T4_A field J1 = 1 (Max)
    // ISET: T32
    // Fields: J1=1, imm11=0, S=0, J2=0, imm10=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000B000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field J2 11 +: 1`
/// Requirement: FieldBoundary { field: "J2", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_b_t4_a_field_j2_0_min_9000_f0009000() {
    // Thumb encoding (32): 0xF0009000
    // Test aarch32_B_T4_A field J2 = 0 (Min)
    // ISET: T32
    // Fields: imm10=0, J2=0, J1=0, S=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0009000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field J2 11 +: 1`
/// Requirement: FieldBoundary { field: "J2", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_b_t4_a_field_j2_1_max_9000_f0009800() {
    // Thumb encoding (32): 0xF0009800
    // Test aarch32_B_T4_A field J2 = 1 (Max)
    // ISET: T32
    // Fields: imm11=0, imm10=0, S=0, J2=1, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0009800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_b_t4_a_field_imm11_0_zero_9000_f0009000() {
    // Thumb encoding (32): 0xF0009000
    // Test aarch32_B_T4_A field imm11 = 0 (Zero)
    // ISET: T32
    // Fields: imm10=0, imm11=0, S=0, J1=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0009000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_b_t4_a_field_imm11_1_poweroftwo_9000_f0009001() {
    // Thumb encoding (32): 0xF0009001
    // Test aarch32_B_T4_A field imm11 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm10=0, J1=0, S=0, J2=0, imm11=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0009001;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_b_t4_a_field_imm11_3_poweroftwominusone_9000_f0009003() {
    // Thumb encoding (32): 0xF0009003
    // Test aarch32_B_T4_A field imm11 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm10=0, S=0, J1=0, J2=0, imm11=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0009003;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_b_t4_a_field_imm11_4_poweroftwo_9000_f0009004() {
    // Thumb encoding (32): 0xF0009004
    // Test aarch32_B_T4_A field imm11 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: imm11=4, imm10=0, S=0, J1=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0009004;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_b_t4_a_field_imm11_7_poweroftwominusone_9000_f0009007() {
    // Thumb encoding (32): 0xF0009007
    // Test aarch32_B_T4_A field imm11 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: J1=0, imm10=0, S=0, J2=0, imm11=7
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0009007;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_b_t4_a_field_imm11_8_poweroftwo_9000_f0009008() {
    // Thumb encoding (32): 0xF0009008
    // Test aarch32_B_T4_A field imm11 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: imm11=8, S=0, J1=0, J2=0, imm10=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0009008;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_b_t4_a_field_imm11_15_poweroftwominusone_9000_f000900f() {
    // Thumb encoding (32): 0xF000900F
    // Test aarch32_B_T4_A field imm11 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: S=0, J2=0, imm11=15, J1=0, imm10=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000900F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_b_t4_a_field_imm11_16_poweroftwo_9000_f0009010() {
    // Thumb encoding (32): 0xF0009010
    // Test aarch32_B_T4_A field imm11 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, J1=0, imm11=16, imm10=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0009010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_b_t4_a_field_imm11_31_poweroftwominusone_9000_f000901f() {
    // Thumb encoding (32): 0xF000901F
    // Test aarch32_B_T4_A field imm11 = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: J2=0, imm10=0, J1=0, imm11=31, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000901F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_b_t4_a_field_imm11_32_poweroftwo_9000_f0009020() {
    // Thumb encoding (32): 0xF0009020
    // Test aarch32_B_T4_A field imm11 = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: imm11=32, J2=0, S=0, imm10=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0009020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_b_t4_a_field_imm11_63_poweroftwominusone_9000_f000903f() {
    // Thumb encoding (32): 0xF000903F
    // Test aarch32_B_T4_A field imm11 = 63 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm11=63, imm10=0, J2=0, S=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000903F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_b_t4_a_field_imm11_64_poweroftwo_9000_f0009040() {
    // Thumb encoding (32): 0xF0009040
    // Test aarch32_B_T4_A field imm11 = 64 (PowerOfTwo)
    // ISET: T32
    // Fields: J1=0, J2=0, S=0, imm11=64, imm10=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0009040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch32_b_t4_a_field_imm11_127_poweroftwominusone_9000_f000907f() {
    // Thumb encoding (32): 0xF000907F
    // Test aarch32_B_T4_A field imm11 = 127 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm11=127, J2=0, J1=0, S=0, imm10=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF000907F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_b_t4_a_field_imm11_128_poweroftwo_9000_f0009080() {
    // Thumb encoding (32): 0xF0009080
    // Test aarch32_B_T4_A field imm11 = 128 (PowerOfTwo)
    // ISET: T32
    // Fields: imm10=0, J1=0, J2=0, imm11=128, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0009080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch32_b_t4_a_field_imm11_255_poweroftwominusone_9000_f00090ff() {
    // Thumb encoding (32): 0xF00090FF
    // Test aarch32_B_T4_A field imm11 = 255 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm11=255, J2=0, S=0, imm10=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF00090FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch32_b_t4_a_field_imm11_256_poweroftwo_9000_f0009100() {
    // Thumb encoding (32): 0xF0009100
    // Test aarch32_B_T4_A field imm11 = 256 (PowerOfTwo)
    // ISET: T32
    // Fields: J2=0, imm11=256, J1=0, S=0, imm10=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0009100;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch32_b_t4_a_field_imm11_511_poweroftwominusone_9000_f00091ff() {
    // Thumb encoding (32): 0xF00091FF
    // Test aarch32_B_T4_A field imm11 = 511 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: S=0, imm11=511, imm10=0, J2=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF00091FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch32_b_t4_a_field_imm11_512_poweroftwo_9000_f0009200() {
    // Thumb encoding (32): 0xF0009200
    // Test aarch32_B_T4_A field imm11 = 512 (PowerOfTwo)
    // ISET: T32
    // Fields: imm10=0, S=0, J2=0, imm11=512, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0009200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 1023, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (1023)
#[test]
fn test_aarch32_b_t4_a_field_imm11_1023_poweroftwominusone_9000_f00093ff() {
    // Thumb encoding (32): 0xF00093FF
    // Test aarch32_B_T4_A field imm11 = 1023 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm10=0, J1=0, imm11=1023, J2=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF00093FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch32_b_t4_a_field_imm11_1024_poweroftwo_9000_f0009400() {
    // Thumb encoding (32): 0xF0009400
    // Test aarch32_B_T4_A field imm11 = 1024 (PowerOfTwo)
    // ISET: T32
    // Fields: J1=0, J2=0, imm10=0, S=0, imm11=1024
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0009400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field imm11 0 +: 11`
/// Requirement: FieldBoundary { field: "imm11", value: 2047, boundary: Max }
/// maximum immediate (2047)
#[test]
fn test_aarch32_b_t4_a_field_imm11_2047_max_9000_f00097ff() {
    // Thumb encoding (32): 0xF00097FF
    // Test aarch32_B_T4_A field imm11 = 2047 (Max)
    // ISET: T32
    // Fields: imm11=2047, J1=0, S=0, J2=0, imm10=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF00097FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=0 (8-bit / byte size)
#[test]
fn test_aarch32_b_t4_a_combo_0_9000_f0009000() {
    // Thumb encoding (32): 0xF0009000
    // Test aarch32_B_T4_A field combination: S=0, imm10=0, J1=0, J2=0, imm11=0
    // ISET: T32
    // Fields: imm10=0, J2=0, imm11=0, S=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0009000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_b_t4_a_special_s_0_size_variant_0_36864_f0009000() {
    // Thumb encoding (32): 0xF0009000
    // Test aarch32_B_T4_A special value S = 0 (Size variant 0)
    // ISET: T32
    // Fields: S=0, J1=0, J2=0, imm10=0, imm11=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0009000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_b_t4_a_special_s_1_size_variant_1_36864_f4009000() {
    // Thumb encoding (32): 0xF4009000
    // Test aarch32_B_T4_A special value S = 1 (Size variant 1)
    // ISET: T32
    // Fields: J2=0, imm11=0, imm10=0, S=1, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF4009000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_B_T4_A
/// ASL: `Binary { op: And, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "LastInITBlock" }, args: [] } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: And, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"LastInITBlock\" }, args: [] } } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_b_t4_a_invalid_0_9000_f0009000() {
    // Thumb encoding (32): 0xF0009000
    // Test aarch32_B_T4_A invalid encoding: Binary { op: And, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "LastInITBlock" }, args: [] } } }
    // ISET: T32
    // Fields: imm10=0, J1=0, imm11=0, S=0, J2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0009000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_B_T4_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_b_t4_a_invalid_1_9000_f0009000() {
    // Thumb encoding (32): 0xF0009000
    // Test aarch32_B_T4_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: J2=0, imm10=0, imm11=0, S=0, J1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0009000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_B_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 0 (32)
#[test]
fn test_aarch32_b_t1_a_lslv_oracle_32_0_d0020020() {
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

/// Provenance: aarch32_B_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 0 (64)
#[test]
fn test_aarch32_b_t1_a_lslv_oracle_64_0_d0020020() {
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

/// Provenance: aarch32_B_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4 (32)
#[test]
fn test_aarch32_b_t1_a_lslv_oracle_32_1_d0020020() {
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

/// Provenance: aarch32_B_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 4 (64)
#[test]
fn test_aarch32_b_t1_a_lslv_oracle_64_1_d0020020() {
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

/// Provenance: aarch32_B_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 8 (32)
#[test]
fn test_aarch32_b_t1_a_lslv_oracle_32_2_d0020020() {
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

/// Provenance: aarch32_B_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 8 (64)
#[test]
fn test_aarch32_b_t1_a_lslv_oracle_64_2_d0020020() {
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

/// Provenance: aarch32_B_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1 (32)
#[test]
fn test_aarch32_b_t1_a_lslv_oracle_32_3_d0020020() {
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

/// Provenance: aarch32_B_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// MSB set, shift 1 (64)
#[test]
fn test_aarch32_b_t1_a_lslv_oracle_64_3_d0020020() {
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

/// Provenance: aarch32_B_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSB set, max shift (32)
#[test]
fn test_aarch32_b_t1_a_lslv_oracle_32_4_d0020020() {
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

/// Provenance: aarch32_B_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSB set, max shift (64)
#[test]
fn test_aarch32_b_t1_a_lslv_oracle_64_4_d0020020() {
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

/// Provenance: aarch32_B_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// all ones, shift 32 (32)
#[test]
fn test_aarch32_b_t1_a_lslv_oracle_32_5_d0020020() {
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

/// Provenance: aarch32_B_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all ones, shift 32 (64)
#[test]
fn test_aarch32_b_t1_a_lslv_oracle_64_5_d0020020() {
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

/// Provenance: aarch32_B_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift
#[test]
fn test_aarch32_b_t1_a_t16_oracle_0_d0000000() {
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

/// Provenance: aarch32_B_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4
#[test]
fn test_aarch32_b_t1_a_t16_oracle_1_d0000000() {
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

/// Provenance: aarch32_B_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1
#[test]
fn test_aarch32_b_t1_a_t16_oracle_2_d0000000() {
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

/// Provenance: aarch32_B_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift to MSB
#[test]
fn test_aarch32_b_t1_a_t16_oracle_3_d0000000() {
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

/// Provenance: aarch32_B_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 0 (32)
#[test]
fn test_aarch32_b_t2_a_lslv_oracle_32_0_e0020020() {
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

/// Provenance: aarch32_B_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 0 (64)
#[test]
fn test_aarch32_b_t2_a_lslv_oracle_64_0_e0020020() {
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

/// Provenance: aarch32_B_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4 (32)
#[test]
fn test_aarch32_b_t2_a_lslv_oracle_32_1_e0020020() {
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

/// Provenance: aarch32_B_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 4 (64)
#[test]
fn test_aarch32_b_t2_a_lslv_oracle_64_1_e0020020() {
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

/// Provenance: aarch32_B_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 8 (32)
#[test]
fn test_aarch32_b_t2_a_lslv_oracle_32_2_e0020020() {
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

/// Provenance: aarch32_B_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 8 (64)
#[test]
fn test_aarch32_b_t2_a_lslv_oracle_64_2_e0020020() {
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

/// Provenance: aarch32_B_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1 (32)
#[test]
fn test_aarch32_b_t2_a_lslv_oracle_32_3_e0020020() {
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

/// Provenance: aarch32_B_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// MSB set, shift 1 (64)
#[test]
fn test_aarch32_b_t2_a_lslv_oracle_64_3_e0020020() {
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

/// Provenance: aarch32_B_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSB set, max shift (32)
#[test]
fn test_aarch32_b_t2_a_lslv_oracle_32_4_e0020020() {
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

/// Provenance: aarch32_B_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSB set, max shift (64)
#[test]
fn test_aarch32_b_t2_a_lslv_oracle_64_4_e0020020() {
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

/// Provenance: aarch32_B_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// all ones, shift 32 (32)
#[test]
fn test_aarch32_b_t2_a_lslv_oracle_32_5_e0020020() {
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

/// Provenance: aarch32_B_T2_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all ones, shift 32 (64)
#[test]
fn test_aarch32_b_t2_a_lslv_oracle_64_5_e0020020() {
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

/// Provenance: aarch32_B_T2_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift
#[test]
fn test_aarch32_b_t2_a_t16_oracle_0_e0000000() {
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

/// Provenance: aarch32_B_T2_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4
#[test]
fn test_aarch32_b_t2_a_t16_oracle_1_e0000000() {
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

/// Provenance: aarch32_B_T2_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1
#[test]
fn test_aarch32_b_t2_a_t16_oracle_2_e0000000() {
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

/// Provenance: aarch32_B_T2_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift to MSB
#[test]
fn test_aarch32_b_t2_a_t16_oracle_3_e0000000() {
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
// aarch32_BLX_r_A Tests
// ============================================================================

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_blx_r_a1_a_field_cond_0_min_30_01200030() {
    // Encoding: 0x01200030
    // Test aarch32_BLX_r_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: Rm=0, cond=0
    let encoding: u32 = 0x01200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_blx_r_a1_a_field_cond_1_poweroftwo_30_11200030() {
    // Encoding: 0x11200030
    // Test aarch32_BLX_r_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=1
    let encoding: u32 = 0x11200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_blx_r_a1_a_field_cond_2_poweroftwo_30_21200030() {
    // Encoding: 0x21200030
    // Test aarch32_BLX_r_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=2, Rm=0
    let encoding: u32 = 0x21200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_blx_r_a1_a_field_cond_3_poweroftwo_30_31200030() {
    // Encoding: 0x31200030
    // Test aarch32_BLX_r_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=3
    let encoding: u32 = 0x31200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_blx_r_a1_a_field_cond_4_poweroftwo_30_41200030() {
    // Encoding: 0x41200030
    // Test aarch32_BLX_r_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=4, Rm=0
    let encoding: u32 = 0x41200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_blx_r_a1_a_field_cond_5_poweroftwo_30_51200030() {
    // Encoding: 0x51200030
    // Test aarch32_BLX_r_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=5, Rm=0
    let encoding: u32 = 0x51200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_blx_r_a1_a_field_cond_6_poweroftwo_30_61200030() {
    // Encoding: 0x61200030
    // Test aarch32_BLX_r_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, Rm=0
    let encoding: u32 = 0x61200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_blx_r_a1_a_field_cond_7_poweroftwo_30_71200030() {
    // Encoding: 0x71200030
    // Test aarch32_BLX_r_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=7, Rm=0
    let encoding: u32 = 0x71200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_blx_r_a1_a_field_cond_8_poweroftwo_30_81200030() {
    // Encoding: 0x81200030
    // Test aarch32_BLX_r_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=8, Rm=0
    let encoding: u32 = 0x81200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_blx_r_a1_a_field_cond_9_poweroftwo_30_91200030() {
    // Encoding: 0x91200030
    // Test aarch32_BLX_r_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=9
    let encoding: u32 = 0x91200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_blx_r_a1_a_field_cond_10_poweroftwo_30_a1200030() {
    // Encoding: 0xA1200030
    // Test aarch32_BLX_r_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, Rm=0
    let encoding: u32 = 0xA1200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_blx_r_a1_a_field_cond_11_poweroftwo_30_b1200030() {
    // Encoding: 0xB1200030
    // Test aarch32_BLX_r_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=11, Rm=0
    let encoding: u32 = 0xB1200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_blx_r_a1_a_field_cond_12_poweroftwo_30_c1200030() {
    // Encoding: 0xC1200030
    // Test aarch32_BLX_r_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Rm=0, cond=12
    let encoding: u32 = 0xC1200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_blx_r_a1_a_field_cond_13_poweroftwo_30_d1200030() {
    // Encoding: 0xD1200030
    // Test aarch32_BLX_r_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=13, Rm=0
    let encoding: u32 = 0xD1200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_blx_r_a1_a_field_cond_14_poweroftwo_30_e1200030() {
    // Encoding: 0xE1200030
    // Test aarch32_BLX_r_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, Rm=0
    let encoding: u32 = 0xE1200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_blx_r_a1_a_field_cond_15_max_30_f1200030() {
    // Encoding: 0xF1200030
    // Test aarch32_BLX_r_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, Rm=0
    let encoding: u32 = 0xF1200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_blx_r_a1_a_field_rm_0_min_30_01200030() {
    // Encoding: 0x01200030
    // Test aarch32_BLX_r_A1_A field Rm = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Rm=0
    let encoding: u32 = 0x01200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_blx_r_a1_a_field_rm_1_poweroftwo_30_01200031() {
    // Encoding: 0x01200031
    // Test aarch32_BLX_r_A1_A field Rm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Rm=1
    let encoding: u32 = 0x01200031;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_blx_r_a1_a_combo_0_30_01200030() {
    // Encoding: 0x01200030
    // Test aarch32_BLX_r_A1_A field combination: cond=0, Rm=0
    // ISET: A32
    // Fields: Rm=0, cond=0
    let encoding: u32 = 0x01200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_blx_r_a1_a_special_cond_0_condition_eq_48_01200030() {
    // Encoding: 0x01200030
    // Test aarch32_BLX_r_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: cond=0, Rm=0
    let encoding: u32 = 0x01200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_blx_r_a1_a_special_cond_1_condition_ne_48_11200030() {
    // Encoding: 0x11200030
    // Test aarch32_BLX_r_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, Rm=0
    let encoding: u32 = 0x11200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_blx_r_a1_a_special_cond_2_condition_cs_hs_48_21200030() {
    // Encoding: 0x21200030
    // Test aarch32_BLX_r_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: cond=2, Rm=0
    let encoding: u32 = 0x21200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_blx_r_a1_a_special_cond_3_condition_cc_lo_48_31200030() {
    // Encoding: 0x31200030
    // Test aarch32_BLX_r_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, Rm=0
    let encoding: u32 = 0x31200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_blx_r_a1_a_special_cond_4_condition_mi_48_41200030() {
    // Encoding: 0x41200030
    // Test aarch32_BLX_r_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, Rm=0
    let encoding: u32 = 0x41200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_blx_r_a1_a_special_cond_5_condition_pl_48_51200030() {
    // Encoding: 0x51200030
    // Test aarch32_BLX_r_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: cond=5, Rm=0
    let encoding: u32 = 0x51200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_blx_r_a1_a_special_cond_6_condition_vs_48_61200030() {
    // Encoding: 0x61200030
    // Test aarch32_BLX_r_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Rm=0, cond=6
    let encoding: u32 = 0x61200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_blx_r_a1_a_special_cond_7_condition_vc_48_71200030() {
    // Encoding: 0x71200030
    // Test aarch32_BLX_r_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: Rm=0, cond=7
    let encoding: u32 = 0x71200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_blx_r_a1_a_special_cond_8_condition_hi_48_81200030() {
    // Encoding: 0x81200030
    // Test aarch32_BLX_r_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, Rm=0
    let encoding: u32 = 0x81200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_blx_r_a1_a_special_cond_9_condition_ls_48_91200030() {
    // Encoding: 0x91200030
    // Test aarch32_BLX_r_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, Rm=0
    let encoding: u32 = 0x91200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_blx_r_a1_a_special_cond_10_condition_ge_48_a1200030() {
    // Encoding: 0xA1200030
    // Test aarch32_BLX_r_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: Rm=0, cond=10
    let encoding: u32 = 0xA1200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_blx_r_a1_a_special_cond_11_condition_lt_48_b1200030() {
    // Encoding: 0xB1200030
    // Test aarch32_BLX_r_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, Rm=0
    let encoding: u32 = 0xB1200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_blx_r_a1_a_special_cond_12_condition_gt_48_c1200030() {
    // Encoding: 0xC1200030
    // Test aarch32_BLX_r_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Rm=0, cond=12
    let encoding: u32 = 0xC1200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_blx_r_a1_a_special_cond_13_condition_le_48_d1200030() {
    // Encoding: 0xD1200030
    // Test aarch32_BLX_r_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: Rm=0, cond=13
    let encoding: u32 = 0xD1200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_blx_r_a1_a_special_cond_14_condition_al_48_e1200030() {
    // Encoding: 0xE1200030
    // Test aarch32_BLX_r_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: cond=14, Rm=0
    let encoding: u32 = 0xE1200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_blx_r_a1_a_special_cond_15_condition_nv_48_f1200030() {
    // Encoding: 0xF1200030
    // Test aarch32_BLX_r_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: cond=15, Rm=0
    let encoding: u32 = 0xF1200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }), rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }), rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_blx_r_a1_a_invalid_0_30_01200030() {
    // Encoding: 0x01200030
    // Test aarch32_BLX_r_A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }), rhs: LitInt(15) }
    // ISET: A32
    // Fields: cond=0, Rm=0
    let encoding: u32 = 0x01200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_BLX_r_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_blx_r_a1_a_invalid_1_30_01200030() {
    // Encoding: 0x01200030
    // Test aarch32_BLX_r_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: cond=0, Rm=0
    let encoding: u32 = 0x01200030;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_BLX_r_T1_A
/// ASL: `field Rm 19 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_blx_r_t1_a_field_rm_0_min_0_47800000() {
    // Thumb encoding (32): 0x47800000
    // Test aarch32_BLX_r_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x47800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BLX_r_T1_A
/// ASL: `field Rm 19 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_blx_r_t1_a_field_rm_1_poweroftwo_0_47880000() {
    // Thumb encoding (32): 0x47880000
    // Test aarch32_BLX_r_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x47880000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BLX_r_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch32_blx_r_t1_a_combo_0_0_47800000() {
    // Thumb encoding (32): 0x47800000
    // Test aarch32_BLX_r_T1_A field combination: Rm=0
    // ISET: T32
    // Fields: Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x47800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_BLX_r_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }), rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }), rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_blx_r_t1_a_invalid_0_0_47800000() {
    // Thumb encoding (32): 0x47800000
    // Test aarch32_BLX_r_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }), rhs: LitInt(15) }
    // ISET: T32
    // Fields: Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x47800000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_BLX_r_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_blx_r_t1_a_invalid_1_0_47800000() {
    // Thumb encoding (32): 0x47800000
    // Test aarch32_BLX_r_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x47800000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_BLX_r_T1_A
/// ASL: `Binary { op: And, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "LastInITBlock" }, args: [] } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: And, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"LastInITBlock\" }, args: [] } } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_blx_r_t1_a_invalid_2_0_47800000() {
    // Thumb encoding (32): 0x47800000
    // Test aarch32_BLX_r_T1_A invalid encoding: Binary { op: And, lhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }, rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "LastInITBlock" }, args: [] } } }
    // ISET: T32
    // Fields: Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x47800000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_BLX_r_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_blx_r_t1_a_invalid_3_0_47800000() {
    // Thumb encoding (32): 0x47800000
    // Test aarch32_BLX_r_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0x47800000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_BLX_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 0 (32)
#[test]
fn test_aarch32_blx_r_t1_a_lslv_oracle_32_0_47820020() {
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

/// Provenance: aarch32_BLX_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 0 (64)
#[test]
fn test_aarch32_blx_r_t1_a_lslv_oracle_64_0_c7820020() {
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

/// Provenance: aarch32_BLX_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4 (32)
#[test]
fn test_aarch32_blx_r_t1_a_lslv_oracle_32_1_47820020() {
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

/// Provenance: aarch32_BLX_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 4 (64)
#[test]
fn test_aarch32_blx_r_t1_a_lslv_oracle_64_1_c7820020() {
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

/// Provenance: aarch32_BLX_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 8 (32)
#[test]
fn test_aarch32_blx_r_t1_a_lslv_oracle_32_2_47820020() {
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

/// Provenance: aarch32_BLX_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shift by 8 (64)
#[test]
fn test_aarch32_blx_r_t1_a_lslv_oracle_64_2_c7820020() {
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

/// Provenance: aarch32_BLX_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1 (32)
#[test]
fn test_aarch32_blx_r_t1_a_lslv_oracle_32_3_47820020() {
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

/// Provenance: aarch32_BLX_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// MSB set, shift 1 (64)
#[test]
fn test_aarch32_blx_r_t1_a_lslv_oracle_64_3_c7820020() {
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

/// Provenance: aarch32_BLX_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// LSB set, max shift (32)
#[test]
fn test_aarch32_blx_r_t1_a_lslv_oracle_32_4_47820020() {
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

/// Provenance: aarch32_BLX_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// LSB set, max shift (64)
#[test]
fn test_aarch32_blx_r_t1_a_lslv_oracle_64_4_c7820020() {
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

/// Provenance: aarch32_BLX_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// all ones, shift 32 (32)
#[test]
fn test_aarch32_blx_r_t1_a_lslv_oracle_32_5_47820020() {
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

/// Provenance: aarch32_BLX_r_T1_A
/// ASL: `LSLV X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all ones, shift 32 (64)
#[test]
fn test_aarch32_blx_r_t1_a_lslv_oracle_64_5_c7820020() {
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

/// Provenance: aarch32_BLX_r_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// no shift
#[test]
fn test_aarch32_blx_r_t1_a_t16_oracle_0_47900000() {
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

/// Provenance: aarch32_BLX_r_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift by 4
#[test]
fn test_aarch32_blx_r_t1_a_t16_oracle_1_47900000() {
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

/// Provenance: aarch32_BLX_r_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// MSB set, shift 1
#[test]
fn test_aarch32_blx_r_t1_a_t16_oracle_2_47900000() {
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

/// Provenance: aarch32_BLX_r_T1_A
/// ASL: `LSLS R0, R1, R2`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shift to MSB
#[test]
fn test_aarch32_blx_r_t1_a_t16_oracle_3_47900000() {
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
