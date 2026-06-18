//! T32 data_processing logical tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers_32::*;

// ============================================================================
// aarch32_ORN_i_A Tests
// ============================================================================

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field i 26 +: 1`
/// Requirement: FieldBoundary { field: "i", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_orn_i_t1_a_field_i_0_min_0_f0600000() {
    // Thumb encoding (32): 0xF0600000
    // Test aarch32_ORN_i_T1_A field i = 0 (Min)
    // ISET: T32
    // Fields: S=0, Rn=0, Rd=0, i=0, imm8=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0600000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field i 26 +: 1`
/// Requirement: FieldBoundary { field: "i", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_orn_i_t1_a_field_i_1_max_0_f4600000() {
    // Thumb encoding (32): 0xF4600000
    // Test aarch32_ORN_i_T1_A field i = 1 (Max)
    // ISET: T32
    // Fields: Rd=0, imm8=0, S=0, imm3=0, Rn=0, i=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF4600000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_orn_i_t1_a_field_s_0_min_0_f0600000() {
    // Thumb encoding (32): 0xF0600000
    // Test aarch32_ORN_i_T1_A field S = 0 (Min)
    // ISET: T32
    // Fields: imm8=0, imm3=0, S=0, i=0, Rd=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0600000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_orn_i_t1_a_field_s_1_max_0_f0700000() {
    // Thumb encoding (32): 0xF0700000
    // Test aarch32_ORN_i_T1_A field S = 1 (Max)
    // ISET: T32
    // Fields: Rn=0, Rd=0, S=1, imm8=0, i=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0700000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_orn_i_t1_a_field_rn_0_min_0_f0600000() {
    // Thumb encoding (32): 0xF0600000
    // Test aarch32_ORN_i_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: i=0, Rn=0, S=0, imm3=0, imm8=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0600000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_orn_i_t1_a_field_rn_1_poweroftwo_0_f0610000() {
    // Thumb encoding (32): 0xF0610000
    // Test aarch32_ORN_i_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=0, Rd=0, imm8=0, i=0, S=0, Rn=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0610000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_orn_i_t1_a_field_imm3_0_zero_0_f0600000() {
    // Thumb encoding (32): 0xF0600000
    // Test aarch32_ORN_i_T1_A field imm3 = 0 (Zero)
    // ISET: T32
    // Fields: Rd=0, S=0, imm8=0, i=0, Rn=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0600000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_orn_i_t1_a_field_imm3_1_poweroftwo_0_f0601000() {
    // Thumb encoding (32): 0xF0601000
    // Test aarch32_ORN_i_T1_A field imm3 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, imm8=0, i=0, Rn=0, S=0, imm3=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0601000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch32_orn_i_t1_a_field_imm3_3_poweroftwominusone_0_f0603000() {
    // Thumb encoding (32): 0xF0603000
    // Test aarch32_ORN_i_T1_A field imm3 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rn=0, imm3=3, S=0, imm8=0, Rd=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0603000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch32_orn_i_t1_a_field_imm3_7_max_0_f0607000() {
    // Thumb encoding (32): 0xF0607000
    // Test aarch32_ORN_i_T1_A field imm3 = 7 (Max)
    // ISET: T32
    // Fields: S=0, Rd=0, Rn=0, imm8=0, i=0, imm3=7
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0607000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_orn_i_t1_a_field_rd_0_min_0_f0600000() {
    // Thumb encoding (32): 0xF0600000
    // Test aarch32_ORN_i_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: imm3=0, Rn=0, Rd=0, imm8=0, S=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0600000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_orn_i_t1_a_field_rd_1_poweroftwo_0_f0600100() {
    // Thumb encoding (32): 0xF0600100
    // Test aarch32_ORN_i_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=0, imm3=0, Rn=0, Rd=1, i=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0600100;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_orn_i_t1_a_field_imm8_0_zero_0_f0600000() {
    // Thumb encoding (32): 0xF0600000
    // Test aarch32_ORN_i_T1_A field imm8 = 0 (Zero)
    // ISET: T32
    // Fields: Rd=0, Rn=0, i=0, imm3=0, S=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0600000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_orn_i_t1_a_field_imm8_1_poweroftwo_0_f0600001() {
    // Thumb encoding (32): 0xF0600001
    // Test aarch32_ORN_i_T1_A field imm8 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, imm8=1, imm3=0, i=0, S=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0600001;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_orn_i_t1_a_field_imm8_3_poweroftwominusone_0_f0600003() {
    // Thumb encoding (32): 0xF0600003
    // Test aarch32_ORN_i_T1_A field imm8 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: i=0, S=0, imm3=0, Rn=0, Rd=0, imm8=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0600003;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_orn_i_t1_a_field_imm8_4_poweroftwo_0_f0600004() {
    // Thumb encoding (32): 0xF0600004
    // Test aarch32_ORN_i_T1_A field imm8 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: imm8=4, Rn=0, imm3=0, Rd=0, i=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0600004;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_orn_i_t1_a_field_imm8_7_poweroftwominusone_0_f0600007() {
    // Thumb encoding (32): 0xF0600007
    // Test aarch32_ORN_i_T1_A field imm8 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=7, Rn=0, S=0, Rd=0, i=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0600007;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_orn_i_t1_a_field_imm8_8_poweroftwo_0_f0600008() {
    // Thumb encoding (32): 0xF0600008
    // Test aarch32_ORN_i_T1_A field imm8 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: i=0, imm3=0, Rd=0, imm8=8, Rn=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0600008;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_orn_i_t1_a_field_imm8_15_poweroftwominusone_0_f060000f() {
    // Thumb encoding (32): 0xF060000F
    // Test aarch32_ORN_i_T1_A field imm8 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm3=0, Rd=0, imm8=15, Rn=0, S=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF060000F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_orn_i_t1_a_field_imm8_16_poweroftwo_0_f0600010() {
    // Thumb encoding (32): 0xF0600010
    // Test aarch32_ORN_i_T1_A field imm8 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: S=0, Rd=0, imm3=0, imm8=16, i=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0600010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch32_orn_i_t1_a_field_imm8_31_poweroftwominusone_0_f060001f() {
    // Thumb encoding (32): 0xF060001F
    // Test aarch32_ORN_i_T1_A field imm8 = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=31, S=0, Rd=0, Rn=0, imm3=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF060001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_orn_i_t1_a_field_imm8_32_poweroftwo_0_f0600020() {
    // Thumb encoding (32): 0xF0600020
    // Test aarch32_ORN_i_T1_A field imm8 = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=0, S=0, i=0, Rd=0, Rn=0, imm8=32
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0600020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch32_orn_i_t1_a_field_imm8_63_poweroftwominusone_0_f060003f() {
    // Thumb encoding (32): 0xF060003F
    // Test aarch32_ORN_i_T1_A field imm8 = 63 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=63, i=0, Rd=0, imm3=0, Rn=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF060003F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch32_orn_i_t1_a_field_imm8_64_poweroftwo_0_f0600040() {
    // Thumb encoding (32): 0xF0600040
    // Test aarch32_ORN_i_T1_A field imm8 = 64 (PowerOfTwo)
    // ISET: T32
    // Fields: i=0, Rd=0, S=0, Rn=0, imm3=0, imm8=64
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0600040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 127, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (127)
#[test]
fn test_aarch32_orn_i_t1_a_field_imm8_127_poweroftwominusone_0_f060007f() {
    // Thumb encoding (32): 0xF060007F
    // Test aarch32_ORN_i_T1_A field imm8 = 127 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm8=127, S=0, i=0, imm3=0, Rn=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF060007F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch32_orn_i_t1_a_field_imm8_128_poweroftwo_0_f0600080() {
    // Thumb encoding (32): 0xF0600080
    // Test aarch32_ORN_i_T1_A field imm8 = 128 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=0, S=0, imm8=128, Rn=0, i=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0600080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field imm8 0 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 255, boundary: Max }
/// maximum immediate (255)
#[test]
fn test_aarch32_orn_i_t1_a_field_imm8_255_max_0_f06000ff() {
    // Thumb encoding (32): 0xF06000FF
    // Test aarch32_ORN_i_T1_A field imm8 = 255 (Max)
    // ISET: T32
    // Fields: imm8=255, Rn=0, S=0, imm3=0, i=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF06000FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// i=0 (minimum value)
#[test]
fn test_aarch32_orn_i_t1_a_combo_0_0_f0600000() {
    // Thumb encoding (32): 0xF0600000
    // Test aarch32_ORN_i_T1_A field combination: i=0, S=0, Rn=0, imm3=0, Rd=0, imm8=0
    // ISET: T32
    // Fields: imm8=0, i=0, Rn=0, imm3=0, S=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0600000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_orn_i_t1_a_special_s_0_size_variant_0_0_f0600000() {
    // Thumb encoding (32): 0xF0600000
    // Test aarch32_ORN_i_T1_A special value S = 0 (Size variant 0)
    // ISET: T32
    // Fields: Rd=0, Rn=0, S=0, imm3=0, i=0, imm8=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0600000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_orn_i_t1_a_special_s_1_size_variant_1_0_f0700000() {
    // Thumb encoding (32): 0xF0700000
    // Test aarch32_ORN_i_T1_A special value S = 1 (Size variant 1)
    // ISET: T32
    // Fields: Rd=0, imm3=0, imm8=0, i=0, S=1, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0700000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_orn_i_t1_a_invalid_0_0_f0600000() {
    // Thumb encoding (32): 0xF0600000
    // Test aarch32_ORN_i_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: LitInt(15) }
    // ISET: T32
    // Fields: imm8=0, i=0, Rn=0, imm3=0, Rd=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0600000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_orn_i_t1_a_invalid_1_0_f0600000() {
    // Thumb encoding (32): 0xF0600000
    // Test aarch32_ORN_i_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: imm8=0, i=0, Rd=0, S=0, Rn=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xF0600000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `ORR X0, X1, #0xFF`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// mask lower 8 bits (64)
#[test]
fn test_aarch32_orn_i_t1_a_orr_oracle_64_0_b2401c20() {
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

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `ORR X0, X1, #0xFFFF`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// mask lower 16 bits (64)
#[test]
fn test_aarch32_orn_i_t1_a_orr_oracle_64_1_b2403c20() {
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

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `ORR X0, X1, #0xFFFFFFFF`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// mask lower 32 bits (64)
#[test]
fn test_aarch32_orn_i_t1_a_orr_oracle_64_2_b2407c20() {
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

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `ORR X0, X1, #0x1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// single bit mask (64)
#[test]
fn test_aarch32_orn_i_t1_a_orr_oracle_64_3_b2400020() {
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

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `ORR X0, X1, #0x7FFFFFFFFFFFFFFF`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// all but MSB (64)
#[test]
fn test_aarch32_orn_i_t1_a_orr_oracle_64_4_b240f820() {
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

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `ORR W0, W1, #0xFF`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// mask lower 8 bits (32)
#[test]
fn test_aarch32_orn_i_t1_a_orr_oracle_32_0_32001c20() {
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

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `ORR W0, W1, #0xFFFF`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// mask lower 16 bits (32)
#[test]
fn test_aarch32_orn_i_t1_a_orr_oracle_32_1_32003c20() {
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

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `ORR W0, W1, #0x1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// single bit mask (32)
#[test]
fn test_aarch32_orn_i_t1_a_orr_oracle_32_2_32000020() {
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

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_orn_i_t1_a_flags_zeroresult_0_f0710000() {
    // Test aarch32_ORN_i_T1_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xF0710000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_orn_i_t1_a_flags_zeroresult_1_f0710000() {
    // Test aarch32_ORN_i_T1_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF0710000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_orn_i_t1_a_flags_negativeresult_2_f0710000() {
    // Test aarch32_ORN_i_T1_A flag computation: NegativeResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xF0710000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_orn_i_t1_a_flags_unsignedoverflow_3_f0710000() {
    // Test aarch32_ORN_i_T1_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF0710000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_orn_i_t1_a_flags_unsignedoverflow_4_f0710000() {
    // Test aarch32_ORN_i_T1_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x2);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF0710000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_orn_i_t1_a_flags_signedoverflow_5_f0710000() {
    // Test aarch32_ORN_i_T1_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF0710000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_orn_i_t1_a_flags_signedoverflow_6_f0710000() {
    // Test aarch32_ORN_i_T1_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0xF0710000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_ORN_i_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_orn_i_t1_a_flags_positiveresult_7_f0710000() {
    // Test aarch32_ORN_i_T1_A flag computation: PositiveResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u32 = 0xF0710000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

// ============================================================================
// aarch32_ORN_r_A Tests
// ============================================================================

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_orn_r_t1_a_field_s_0_min_0_ea600000() {
    // Thumb encoding (32): 0xEA600000
    // Test aarch32_ORN_r_T1_A field S = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, type1=0, Rm=0, imm3=0, imm2=0, S=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA600000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `field S 20 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_orn_r_t1_a_field_s_1_max_0_ea700000() {
    // Thumb encoding (32): 0xEA700000
    // Test aarch32_ORN_r_T1_A field S = 1 (Max)
    // ISET: T32
    // Fields: Rm=0, S=1, Rd=0, type1=0, Rn=0, imm2=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA700000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_orn_r_t1_a_field_rn_0_min_0_ea600000() {
    // Thumb encoding (32): 0xEA600000
    // Test aarch32_ORN_r_T1_A field Rn = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rm=0, Rn=0, S=0, imm2=0, type1=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA600000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `field Rn 16 +: 4`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_orn_r_t1_a_field_rn_1_poweroftwo_0_ea610000() {
    // Thumb encoding (32): 0xEA610000
    // Test aarch32_ORN_r_T1_A field Rn = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=0, Rd=0, imm3=0, imm2=0, type1=0, Rn=1, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA610000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_orn_r_t1_a_field_imm3_0_zero_0_ea600000() {
    // Thumb encoding (32): 0xEA600000
    // Test aarch32_ORN_r_T1_A field imm3 = 0 (Zero)
    // ISET: T32
    // Fields: imm2=0, Rm=0, imm3=0, Rd=0, Rn=0, S=0, type1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA600000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_orn_r_t1_a_field_imm3_1_poweroftwo_0_ea601000() {
    // Thumb encoding (32): 0xEA601000
    // Test aarch32_ORN_r_T1_A field imm3 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: imm3=1, imm2=0, Rm=0, type1=0, Rn=0, S=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA601000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch32_orn_r_t1_a_field_imm3_3_poweroftwominusone_0_ea603000() {
    // Thumb encoding (32): 0xEA603000
    // Test aarch32_ORN_r_T1_A field imm3 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Rm=0, imm3=3, Rn=0, Rd=0, imm2=0, type1=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA603000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `field imm3 12 +: 3`
/// Requirement: FieldBoundary { field: "imm3", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch32_orn_r_t1_a_field_imm3_7_max_0_ea607000() {
    // Thumb encoding (32): 0xEA607000
    // Test aarch32_ORN_r_T1_A field imm3 = 7 (Max)
    // ISET: T32
    // Fields: imm2=0, Rm=0, type1=0, S=0, Rd=0, Rn=0, imm3=7
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA607000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_orn_r_t1_a_field_rd_0_min_0_ea600000() {
    // Thumb encoding (32): 0xEA600000
    // Test aarch32_ORN_r_T1_A field Rd = 0 (Min)
    // ISET: T32
    // Fields: Rm=0, S=0, Rn=0, Rd=0, imm2=0, imm3=0, type1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA600000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `field Rd 8 +: 4`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_orn_r_t1_a_field_rd_1_poweroftwo_0_ea600100() {
    // Thumb encoding (32): 0xEA600100
    // Test aarch32_ORN_r_T1_A field Rd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rd=1, type1=0, Rm=0, imm2=0, Rn=0, S=0, imm3=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA600100;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `field imm2 6 +: 2`
/// Requirement: FieldBoundary { field: "imm2", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_orn_r_t1_a_field_imm2_0_zero_0_ea600000() {
    // Thumb encoding (32): 0xEA600000
    // Test aarch32_ORN_r_T1_A field imm2 = 0 (Zero)
    // ISET: T32
    // Fields: Rd=0, type1=0, Rn=0, imm3=0, Rm=0, imm2=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA600000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `field imm2 6 +: 2`
/// Requirement: FieldBoundary { field: "imm2", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_orn_r_t1_a_field_imm2_1_poweroftwo_0_ea600040() {
    // Thumb encoding (32): 0xEA600040
    // Test aarch32_ORN_r_T1_A field imm2 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rn=0, imm3=0, S=0, Rd=0, type1=0, imm2=1, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA600040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `field imm2 6 +: 2`
/// Requirement: FieldBoundary { field: "imm2", value: 3, boundary: Max }
/// maximum immediate (3)
#[test]
fn test_aarch32_orn_r_t1_a_field_imm2_3_max_0_ea6000c0() {
    // Thumb encoding (32): 0xEA6000C0
    // Test aarch32_ORN_r_T1_A field imm2 = 3 (Max)
    // ISET: T32
    // Fields: S=0, imm2=3, imm3=0, Rd=0, Rm=0, type1=0, Rn=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA6000C0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `field type1 4 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_orn_r_t1_a_field_type1_0_min_0_ea600000() {
    // Thumb encoding (32): 0xEA600000
    // Test aarch32_ORN_r_T1_A field type1 = 0 (Min)
    // ISET: T32
    // Fields: Rd=0, Rn=0, imm2=0, S=0, imm3=0, type1=0, Rm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA600000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `field type1 4 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_orn_r_t1_a_field_type1_1_poweroftwo_0_ea600010() {
    // Thumb encoding (32): 0xEA600010
    // Test aarch32_ORN_r_T1_A field type1 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: type1=1, Rm=0, Rn=0, imm2=0, imm3=0, S=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA600010;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `field type1 4 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_orn_r_t1_a_field_type1_3_max_0_ea600030() {
    // Thumb encoding (32): 0xEA600030
    // Test aarch32_ORN_r_T1_A field type1 = 3 (Max)
    // ISET: T32
    // Fields: S=0, Rn=0, imm3=0, imm2=0, type1=3, Rm=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA600030;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch32_orn_r_t1_a_field_rm_0_min_0_ea600000() {
    // Thumb encoding (32): 0xEA600000
    // Test aarch32_ORN_r_T1_A field Rm = 0 (Min)
    // ISET: T32
    // Fields: Rn=0, type1=0, Rm=0, imm3=0, Rd=0, S=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA600000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `field Rm 0 +: 4`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch32_orn_r_t1_a_field_rm_1_poweroftwo_0_ea600001() {
    // Thumb encoding (32): 0xEA600001
    // Test aarch32_ORN_r_T1_A field Rm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Rm=1, Rn=0, imm3=0, type1=0, imm2=0, S=0, Rd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA600001;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=0 (8-bit / byte size)
#[test]
fn test_aarch32_orn_r_t1_a_combo_0_0_ea600000() {
    // Thumb encoding (32): 0xEA600000
    // Test aarch32_ORN_r_T1_A field combination: S=0, Rn=0, imm3=0, Rd=0, imm2=0, type1=0, Rm=0
    // ISET: T32
    // Fields: type1=0, Rm=0, imm3=0, S=0, Rd=0, Rn=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA600000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_orn_r_t1_a_special_s_0_size_variant_0_0_ea600000() {
    // Thumb encoding (32): 0xEA600000
    // Test aarch32_ORN_r_T1_A special value S = 0 (Size variant 0)
    // ISET: T32
    // Fields: Rm=0, Rd=0, type1=0, Rn=0, S=0, imm3=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA600000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_orn_r_t1_a_special_s_1_size_variant_1_0_ea700000() {
    // Thumb encoding (32): 0xEA700000
    // Test aarch32_ORN_r_T1_A special value S = 1 (Size variant 1)
    // ISET: T32
    // Fields: imm3=0, Rm=0, type1=0, S=1, Rn=0, Rd=0, imm2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA700000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"d\" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"m\" }) } }, rhs: LitInt(15) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_orn_r_t1_a_invalid_0_0_ea600000() {
    // Thumb encoding (32): 0xEA600000
    // Test aarch32_ORN_r_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "d" }), rhs: Binary { op: Or, lhs: LitInt(15), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "m" }) } }, rhs: LitInt(15) }
    // ISET: T32
    // Fields: imm2=0, imm3=0, Rd=0, Rn=0, type1=0, Rm=0, S=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA600000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_orn_r_t1_a_invalid_1_0_ea600000() {
    // Thumb encoding (32): 0xEA600000
    // Test aarch32_ORN_r_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Rm=0, imm3=0, Rd=0, imm2=0, S=0, Rn=0, type1=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEA600000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch32_orn_r_t1_a_flags_zeroresult_0_ea710002() {
    // Test aarch32_ORN_r_T1_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    set_w(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xEA710002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch32_orn_r_t1_a_flags_zeroresult_1_ea710002() {
    // Test aarch32_ORN_r_T1_A flag computation: ZeroResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    let encoding: u32 = 0xEA710002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch32_orn_r_t1_a_flags_negativeresult_2_ea710002() {
    // Test aarch32_ORN_r_T1_A flag computation: NegativeResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x0);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xEA710002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch32_orn_r_t1_a_flags_unsignedoverflow_3_ea710002() {
    // Test aarch32_ORN_r_T1_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x1);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xEA710002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch32_orn_r_t1_a_flags_unsignedoverflow_4_ea710002() {
    // Test aarch32_ORN_r_T1_A flag computation: UnsignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0x2);
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xEA710002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch32_orn_r_t1_a_flags_signedoverflow_5_ea710002() {
    // Test aarch32_ORN_r_T1_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    set_w(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xEA710002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, true, "N should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch32_orn_r_t1_a_flags_signedoverflow_6_ea710002() {
    // Test aarch32_ORN_r_T1_A flag computation: SignedOverflow
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 2, 0xFFFFFFFF);
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xEA710002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
}

/// Provenance: aarch32_ORN_r_T1_A
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch32_orn_r_t1_a_flags_positiveresult_7_ea710002() {
    // Test aarch32_ORN_r_T1_A flag computation: PositiveResult
    // ISET: T32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    set_w(&mut cpu, 2, 0x32);
    let encoding: u32 = 0xEA710002;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
}
