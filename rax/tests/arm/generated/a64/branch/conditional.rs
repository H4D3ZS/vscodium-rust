//! A64 branch conditional tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_branch_conditional_cond Tests
// ============================================================================

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_0_zero_0_54000000() {
    // Encoding: 0x54000000
    // Test aarch64_branch_conditional_cond field imm19 = 0 (Zero)
    // Fields: cond=0, imm19=0
    let encoding: u32 = 0x54000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_1_poweroftwo_0_54000020() {
    // Encoding: 0x54000020
    // Test aarch64_branch_conditional_cond field imm19 = 1 (PowerOfTwo)
    // Fields: imm19=1, cond=0
    let encoding: u32 = 0x54000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_3_poweroftwominusone_0_54000060() {
    // Encoding: 0x54000060
    // Test aarch64_branch_conditional_cond field imm19 = 3 (PowerOfTwoMinusOne)
    // Fields: cond=0, imm19=3
    let encoding: u32 = 0x54000060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_4_poweroftwo_0_54000080() {
    // Encoding: 0x54000080
    // Test aarch64_branch_conditional_cond field imm19 = 4 (PowerOfTwo)
    // Fields: imm19=4, cond=0
    let encoding: u32 = 0x54000080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_7_poweroftwominusone_0_540000e0() {
    // Encoding: 0x540000E0
    // Test aarch64_branch_conditional_cond field imm19 = 7 (PowerOfTwoMinusOne)
    // Fields: imm19=7, cond=0
    let encoding: u32 = 0x540000E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_8_poweroftwo_0_54000100() {
    // Encoding: 0x54000100
    // Test aarch64_branch_conditional_cond field imm19 = 8 (PowerOfTwo)
    // Fields: imm19=8, cond=0
    let encoding: u32 = 0x54000100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_15_poweroftwominusone_0_540001e0() {
    // Encoding: 0x540001E0
    // Test aarch64_branch_conditional_cond field imm19 = 15 (PowerOfTwoMinusOne)
    // Fields: cond=0, imm19=15
    let encoding: u32 = 0x540001E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_16_poweroftwo_0_54000200() {
    // Encoding: 0x54000200
    // Test aarch64_branch_conditional_cond field imm19 = 16 (PowerOfTwo)
    // Fields: cond=0, imm19=16
    let encoding: u32 = 0x54000200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_31_poweroftwominusone_0_540003e0() {
    // Encoding: 0x540003E0
    // Test aarch64_branch_conditional_cond field imm19 = 31 (PowerOfTwoMinusOne)
    // Fields: imm19=31, cond=0
    let encoding: u32 = 0x540003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_32_poweroftwo_0_54000400() {
    // Encoding: 0x54000400
    // Test aarch64_branch_conditional_cond field imm19 = 32 (PowerOfTwo)
    // Fields: cond=0, imm19=32
    let encoding: u32 = 0x54000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_63_poweroftwominusone_0_540007e0() {
    // Encoding: 0x540007E0
    // Test aarch64_branch_conditional_cond field imm19 = 63 (PowerOfTwoMinusOne)
    // Fields: cond=0, imm19=63
    let encoding: u32 = 0x540007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_64_poweroftwo_0_54000800() {
    // Encoding: 0x54000800
    // Test aarch64_branch_conditional_cond field imm19 = 64 (PowerOfTwo)
    // Fields: cond=0, imm19=64
    let encoding: u32 = 0x54000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_127_poweroftwominusone_0_54000fe0() {
    // Encoding: 0x54000FE0
    // Test aarch64_branch_conditional_cond field imm19 = 127 (PowerOfTwoMinusOne)
    // Fields: imm19=127, cond=0
    let encoding: u32 = 0x54000FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_128_poweroftwo_0_54001000() {
    // Encoding: 0x54001000
    // Test aarch64_branch_conditional_cond field imm19 = 128 (PowerOfTwo)
    // Fields: imm19=128, cond=0
    let encoding: u32 = 0x54001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_255_poweroftwominusone_0_54001fe0() {
    // Encoding: 0x54001FE0
    // Test aarch64_branch_conditional_cond field imm19 = 255 (PowerOfTwoMinusOne)
    // Fields: cond=0, imm19=255
    let encoding: u32 = 0x54001FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_256_poweroftwo_0_54002000() {
    // Encoding: 0x54002000
    // Test aarch64_branch_conditional_cond field imm19 = 256 (PowerOfTwo)
    // Fields: cond=0, imm19=256
    let encoding: u32 = 0x54002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_511_poweroftwominusone_0_54003fe0() {
    // Encoding: 0x54003FE0
    // Test aarch64_branch_conditional_cond field imm19 = 511 (PowerOfTwoMinusOne)
    // Fields: imm19=511, cond=0
    let encoding: u32 = 0x54003FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_512_poweroftwo_0_54004000() {
    // Encoding: 0x54004000
    // Test aarch64_branch_conditional_cond field imm19 = 512 (PowerOfTwo)
    // Fields: imm19=512, cond=0
    let encoding: u32 = 0x54004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 1023, boundary: PowerOfTwoMinusOne }
/// 2^10 - 1 = 1023
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_1023_poweroftwominusone_0_54007fe0() {
    // Encoding: 0x54007FE0
    // Test aarch64_branch_conditional_cond field imm19 = 1023 (PowerOfTwoMinusOne)
    // Fields: imm19=1023, cond=0
    let encoding: u32 = 0x54007FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_1024_poweroftwo_0_54008000() {
    // Encoding: 0x54008000
    // Test aarch64_branch_conditional_cond field imm19 = 1024 (PowerOfTwo)
    // Fields: imm19=1024, cond=0
    let encoding: u32 = 0x54008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 2047, boundary: PowerOfTwoMinusOne }
/// 2^11 - 1 = 2047
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_2047_poweroftwominusone_0_5400ffe0() {
    // Encoding: 0x5400FFE0
    // Test aarch64_branch_conditional_cond field imm19 = 2047 (PowerOfTwoMinusOne)
    // Fields: imm19=2047, cond=0
    let encoding: u32 = 0x5400FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 2048, boundary: PowerOfTwo }
/// power of 2 (2^11 = 2048)
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_2048_poweroftwo_0_54010000() {
    // Encoding: 0x54010000
    // Test aarch64_branch_conditional_cond field imm19 = 2048 (PowerOfTwo)
    // Fields: cond=0, imm19=2048
    let encoding: u32 = 0x54010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 4095, boundary: PowerOfTwoMinusOne }
/// 2^12 - 1 = 4095
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_4095_poweroftwominusone_0_5401ffe0() {
    // Encoding: 0x5401FFE0
    // Test aarch64_branch_conditional_cond field imm19 = 4095 (PowerOfTwoMinusOne)
    // Fields: cond=0, imm19=4095
    let encoding: u32 = 0x5401FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 4096, boundary: PowerOfTwo }
/// power of 2 (2^12 = 4096)
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_4096_poweroftwo_0_54020000() {
    // Encoding: 0x54020000
    // Test aarch64_branch_conditional_cond field imm19 = 4096 (PowerOfTwo)
    // Fields: imm19=4096, cond=0
    let encoding: u32 = 0x54020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 8191, boundary: PowerOfTwoMinusOne }
/// 2^13 - 1 = 8191
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_8191_poweroftwominusone_0_5403ffe0() {
    // Encoding: 0x5403FFE0
    // Test aarch64_branch_conditional_cond field imm19 = 8191 (PowerOfTwoMinusOne)
    // Fields: imm19=8191, cond=0
    let encoding: u32 = 0x5403FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 8192, boundary: PowerOfTwo }
/// power of 2 (2^13 = 8192)
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_8192_poweroftwo_0_54040000() {
    // Encoding: 0x54040000
    // Test aarch64_branch_conditional_cond field imm19 = 8192 (PowerOfTwo)
    // Fields: cond=0, imm19=8192
    let encoding: u32 = 0x54040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 16383, boundary: PowerOfTwoMinusOne }
/// 2^14 - 1 = 16383
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_16383_poweroftwominusone_0_5407ffe0() {
    // Encoding: 0x5407FFE0
    // Test aarch64_branch_conditional_cond field imm19 = 16383 (PowerOfTwoMinusOne)
    // Fields: imm19=16383, cond=0
    let encoding: u32 = 0x5407FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 16384, boundary: PowerOfTwo }
/// power of 2 (2^14 = 16384)
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_16384_poweroftwo_0_54080000() {
    // Encoding: 0x54080000
    // Test aarch64_branch_conditional_cond field imm19 = 16384 (PowerOfTwo)
    // Fields: imm19=16384, cond=0
    let encoding: u32 = 0x54080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 32767, boundary: PowerOfTwoMinusOne }
/// 2^15 - 1 = 32767
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_32767_poweroftwominusone_0_540fffe0() {
    // Encoding: 0x540FFFE0
    // Test aarch64_branch_conditional_cond field imm19 = 32767 (PowerOfTwoMinusOne)
    // Fields: cond=0, imm19=32767
    let encoding: u32 = 0x540FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 32768, boundary: PowerOfTwo }
/// power of 2 (2^15 = 32768)
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_32768_poweroftwo_0_54100000() {
    // Encoding: 0x54100000
    // Test aarch64_branch_conditional_cond field imm19 = 32768 (PowerOfTwo)
    // Fields: imm19=32768, cond=0
    let encoding: u32 = 0x54100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 65535, boundary: PowerOfTwoMinusOne }
/// 2^16 - 1 = 65535
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_65535_poweroftwominusone_0_541fffe0() {
    // Encoding: 0x541FFFE0
    // Test aarch64_branch_conditional_cond field imm19 = 65535 (PowerOfTwoMinusOne)
    // Fields: cond=0, imm19=65535
    let encoding: u32 = 0x541FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 65536, boundary: PowerOfTwo }
/// power of 2 (2^16 = 65536)
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_65536_poweroftwo_0_54200000() {
    // Encoding: 0x54200000
    // Test aarch64_branch_conditional_cond field imm19 = 65536 (PowerOfTwo)
    // Fields: imm19=65536, cond=0
    let encoding: u32 = 0x54200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 131071, boundary: PowerOfTwoMinusOne }
/// 2^17 - 1 = 131071
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_131071_poweroftwominusone_0_543fffe0() {
    // Encoding: 0x543FFFE0
    // Test aarch64_branch_conditional_cond field imm19 = 131071 (PowerOfTwoMinusOne)
    // Fields: imm19=131071, cond=0
    let encoding: u32 = 0x543FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 131072, boundary: PowerOfTwo }
/// power of 2 (2^17 = 131072)
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_131072_poweroftwo_0_54400000() {
    // Encoding: 0x54400000
    // Test aarch64_branch_conditional_cond field imm19 = 131072 (PowerOfTwo)
    // Fields: imm19=131072, cond=0
    let encoding: u32 = 0x54400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 262143, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (262143)
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_262143_poweroftwominusone_0_547fffe0() {
    // Encoding: 0x547FFFE0
    // Test aarch64_branch_conditional_cond field imm19 = 262143 (PowerOfTwoMinusOne)
    // Fields: cond=0, imm19=262143
    let encoding: u32 = 0x547FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 262144, boundary: PowerOfTwo }
/// power of 2 (2^18 = 262144)
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_262144_poweroftwo_0_54800000() {
    // Encoding: 0x54800000
    // Test aarch64_branch_conditional_cond field imm19 = 262144 (PowerOfTwo)
    // Fields: imm19=262144, cond=0
    let encoding: u32 = 0x54800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 524287, boundary: Max }
/// maximum immediate (524287)
#[test]
fn test_aarch64_branch_conditional_cond_field_imm19_524287_max_0_54ffffe0() {
    // Encoding: 0x54FFFFE0
    // Test aarch64_branch_conditional_cond field imm19 = 524287 (Max)
    // Fields: cond=0, imm19=524287
    let encoding: u32 = 0x54FFFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond 0 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch64_branch_conditional_cond_field_cond_0_min_0_54000000() {
    // Encoding: 0x54000000
    // Test aarch64_branch_conditional_cond field cond = 0 (Min)
    // Fields: cond=0, imm19=0
    let encoding: u32 = 0x54000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond 0 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch64_branch_conditional_cond_field_cond_1_poweroftwo_0_54000001() {
    // Encoding: 0x54000001
    // Test aarch64_branch_conditional_cond field cond = 1 (PowerOfTwo)
    // Fields: imm19=0, cond=1
    let encoding: u32 = 0x54000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond 0 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch64_branch_conditional_cond_field_cond_2_poweroftwo_0_54000002() {
    // Encoding: 0x54000002
    // Test aarch64_branch_conditional_cond field cond = 2 (PowerOfTwo)
    // Fields: cond=2, imm19=0
    let encoding: u32 = 0x54000002;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond 0 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch64_branch_conditional_cond_field_cond_3_poweroftwo_0_54000003() {
    // Encoding: 0x54000003
    // Test aarch64_branch_conditional_cond field cond = 3 (PowerOfTwo)
    // Fields: imm19=0, cond=3
    let encoding: u32 = 0x54000003;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond 0 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch64_branch_conditional_cond_field_cond_4_poweroftwo_0_54000004() {
    // Encoding: 0x54000004
    // Test aarch64_branch_conditional_cond field cond = 4 (PowerOfTwo)
    // Fields: cond=4, imm19=0
    let encoding: u32 = 0x54000004;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond 0 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch64_branch_conditional_cond_field_cond_5_poweroftwo_0_54000005() {
    // Encoding: 0x54000005
    // Test aarch64_branch_conditional_cond field cond = 5 (PowerOfTwo)
    // Fields: imm19=0, cond=5
    let encoding: u32 = 0x54000005;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond 0 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch64_branch_conditional_cond_field_cond_6_poweroftwo_0_54000006() {
    // Encoding: 0x54000006
    // Test aarch64_branch_conditional_cond field cond = 6 (PowerOfTwo)
    // Fields: cond=6, imm19=0
    let encoding: u32 = 0x54000006;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond 0 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch64_branch_conditional_cond_field_cond_7_poweroftwo_0_54000007() {
    // Encoding: 0x54000007
    // Test aarch64_branch_conditional_cond field cond = 7 (PowerOfTwo)
    // Fields: imm19=0, cond=7
    let encoding: u32 = 0x54000007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond 0 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch64_branch_conditional_cond_field_cond_8_poweroftwo_0_54000008() {
    // Encoding: 0x54000008
    // Test aarch64_branch_conditional_cond field cond = 8 (PowerOfTwo)
    // Fields: cond=8, imm19=0
    let encoding: u32 = 0x54000008;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond 0 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch64_branch_conditional_cond_field_cond_9_poweroftwo_0_54000009() {
    // Encoding: 0x54000009
    // Test aarch64_branch_conditional_cond field cond = 9 (PowerOfTwo)
    // Fields: imm19=0, cond=9
    let encoding: u32 = 0x54000009;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond 0 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch64_branch_conditional_cond_field_cond_10_poweroftwo_0_5400000a() {
    // Encoding: 0x5400000A
    // Test aarch64_branch_conditional_cond field cond = 10 (PowerOfTwo)
    // Fields: imm19=0, cond=10
    let encoding: u32 = 0x5400000A;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond 0 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch64_branch_conditional_cond_field_cond_11_poweroftwo_0_5400000b() {
    // Encoding: 0x5400000B
    // Test aarch64_branch_conditional_cond field cond = 11 (PowerOfTwo)
    // Fields: cond=11, imm19=0
    let encoding: u32 = 0x5400000B;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond 0 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch64_branch_conditional_cond_field_cond_12_poweroftwo_0_5400000c() {
    // Encoding: 0x5400000C
    // Test aarch64_branch_conditional_cond field cond = 12 (PowerOfTwo)
    // Fields: imm19=0, cond=12
    let encoding: u32 = 0x5400000C;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond 0 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch64_branch_conditional_cond_field_cond_13_poweroftwo_0_5400000d() {
    // Encoding: 0x5400000D
    // Test aarch64_branch_conditional_cond field cond = 13 (PowerOfTwo)
    // Fields: imm19=0, cond=13
    let encoding: u32 = 0x5400000D;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond 0 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch64_branch_conditional_cond_field_cond_14_poweroftwo_0_5400000e() {
    // Encoding: 0x5400000E
    // Test aarch64_branch_conditional_cond field cond = 14 (PowerOfTwo)
    // Fields: cond=14, imm19=0
    let encoding: u32 = 0x5400000E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond 0 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch64_branch_conditional_cond_field_cond_15_max_0_5400000f() {
    // Encoding: 0x5400000F
    // Test aarch64_branch_conditional_cond field cond = 15 (Max)
    // Fields: imm19=0, cond=15
    let encoding: u32 = 0x5400000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=0 (immediate value 0)
#[test]
fn test_aarch64_branch_conditional_cond_combo_0_0_54000000() {
    // Encoding: 0x54000000
    // Test aarch64_branch_conditional_cond field combination: imm19=0, cond=0
    // Fields: cond=0, imm19=0
    let encoding: u32 = 0x54000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=1 (immediate value 1)
#[test]
fn test_aarch64_branch_conditional_cond_combo_1_0_54000020() {
    // Encoding: 0x54000020
    // Test aarch64_branch_conditional_cond field combination: imm19=1, cond=0
    // Fields: imm19=1, cond=0
    let encoding: u32 = 0x54000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_branch_conditional_cond_combo_2_0_54000060() {
    // Encoding: 0x54000060
    // Test aarch64_branch_conditional_cond field combination: imm19=3, cond=0
    // Fields: imm19=3, cond=0
    let encoding: u32 = 0x54000060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_branch_conditional_cond_combo_3_0_54000080() {
    // Encoding: 0x54000080
    // Test aarch64_branch_conditional_cond field combination: imm19=4, cond=0
    // Fields: cond=0, imm19=4
    let encoding: u32 = 0x54000080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_branch_conditional_cond_combo_4_0_540000e0() {
    // Encoding: 0x540000E0
    // Test aarch64_branch_conditional_cond field combination: imm19=7, cond=0
    // Fields: cond=0, imm19=7
    let encoding: u32 = 0x540000E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_branch_conditional_cond_combo_5_0_54000100() {
    // Encoding: 0x54000100
    // Test aarch64_branch_conditional_cond field combination: imm19=8, cond=0
    // Fields: cond=0, imm19=8
    let encoding: u32 = 0x54000100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_branch_conditional_cond_combo_6_0_540001e0() {
    // Encoding: 0x540001E0
    // Test aarch64_branch_conditional_cond field combination: imm19=15, cond=0
    // Fields: imm19=15, cond=0
    let encoding: u32 = 0x540001E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_branch_conditional_cond_combo_7_0_54000200() {
    // Encoding: 0x54000200
    // Test aarch64_branch_conditional_cond field combination: imm19=16, cond=0
    // Fields: cond=0, imm19=16
    let encoding: u32 = 0x54000200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_branch_conditional_cond_combo_8_0_540003e0() {
    // Encoding: 0x540003E0
    // Test aarch64_branch_conditional_cond field combination: imm19=31, cond=0
    // Fields: cond=0, imm19=31
    let encoding: u32 = 0x540003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_branch_conditional_cond_combo_9_0_54000400() {
    // Encoding: 0x54000400
    // Test aarch64_branch_conditional_cond field combination: imm19=32, cond=0
    // Fields: cond=0, imm19=32
    let encoding: u32 = 0x54000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_branch_conditional_cond_combo_10_0_540007e0() {
    // Encoding: 0x540007E0
    // Test aarch64_branch_conditional_cond field combination: imm19=63, cond=0
    // Fields: cond=0, imm19=63
    let encoding: u32 = 0x540007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_branch_conditional_cond_combo_11_0_54000800() {
    // Encoding: 0x54000800
    // Test aarch64_branch_conditional_cond field combination: imm19=64, cond=0
    // Fields: imm19=64, cond=0
    let encoding: u32 = 0x54000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_branch_conditional_cond_combo_12_0_54000fe0() {
    // Encoding: 0x54000FE0
    // Test aarch64_branch_conditional_cond field combination: imm19=127, cond=0
    // Fields: imm19=127, cond=0
    let encoding: u32 = 0x54000FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_branch_conditional_cond_combo_13_0_54001000() {
    // Encoding: 0x54001000
    // Test aarch64_branch_conditional_cond field combination: imm19=128, cond=0
    // Fields: imm19=128, cond=0
    let encoding: u32 = 0x54001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=255 (2^8 - 1 = 255)
#[test]
fn test_aarch64_branch_conditional_cond_combo_14_0_54001fe0() {
    // Encoding: 0x54001FE0
    // Test aarch64_branch_conditional_cond field combination: imm19=255, cond=0
    // Fields: cond=0, imm19=255
    let encoding: u32 = 0x54001FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_branch_conditional_cond_combo_15_0_54002000() {
    // Encoding: 0x54002000
    // Test aarch64_branch_conditional_cond field combination: imm19=256, cond=0
    // Fields: cond=0, imm19=256
    let encoding: u32 = 0x54002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=511 (2^9 - 1 = 511)
#[test]
fn test_aarch64_branch_conditional_cond_combo_16_0_54003fe0() {
    // Encoding: 0x54003FE0
    // Test aarch64_branch_conditional_cond field combination: imm19=511, cond=0
    // Fields: imm19=511, cond=0
    let encoding: u32 = 0x54003FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=512 (power of 2 (2^9 = 512))
#[test]
fn test_aarch64_branch_conditional_cond_combo_17_0_54004000() {
    // Encoding: 0x54004000
    // Test aarch64_branch_conditional_cond field combination: imm19=512, cond=0
    // Fields: imm19=512, cond=0
    let encoding: u32 = 0x54004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=1023 (2^10 - 1 = 1023)
#[test]
fn test_aarch64_branch_conditional_cond_combo_18_0_54007fe0() {
    // Encoding: 0x54007FE0
    // Test aarch64_branch_conditional_cond field combination: imm19=1023, cond=0
    // Fields: imm19=1023, cond=0
    let encoding: u32 = 0x54007FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=1024 (power of 2 (2^10 = 1024))
#[test]
fn test_aarch64_branch_conditional_cond_combo_19_0_54008000() {
    // Encoding: 0x54008000
    // Test aarch64_branch_conditional_cond field combination: imm19=1024, cond=0
    // Fields: imm19=1024, cond=0
    let encoding: u32 = 0x54008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=2047 (2^11 - 1 = 2047)
#[test]
fn test_aarch64_branch_conditional_cond_combo_20_0_5400ffe0() {
    // Encoding: 0x5400FFE0
    // Test aarch64_branch_conditional_cond field combination: imm19=2047, cond=0
    // Fields: imm19=2047, cond=0
    let encoding: u32 = 0x5400FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=2048 (power of 2 (2^11 = 2048))
#[test]
fn test_aarch64_branch_conditional_cond_combo_21_0_54010000() {
    // Encoding: 0x54010000
    // Test aarch64_branch_conditional_cond field combination: imm19=2048, cond=0
    // Fields: cond=0, imm19=2048
    let encoding: u32 = 0x54010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=4095 (2^12 - 1 = 4095)
#[test]
fn test_aarch64_branch_conditional_cond_combo_22_0_5401ffe0() {
    // Encoding: 0x5401FFE0
    // Test aarch64_branch_conditional_cond field combination: imm19=4095, cond=0
    // Fields: cond=0, imm19=4095
    let encoding: u32 = 0x5401FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=4096 (power of 2 (2^12 = 4096))
#[test]
fn test_aarch64_branch_conditional_cond_combo_23_0_54020000() {
    // Encoding: 0x54020000
    // Test aarch64_branch_conditional_cond field combination: imm19=4096, cond=0
    // Fields: imm19=4096, cond=0
    let encoding: u32 = 0x54020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=8191 (2^13 - 1 = 8191)
#[test]
fn test_aarch64_branch_conditional_cond_combo_24_0_5403ffe0() {
    // Encoding: 0x5403FFE0
    // Test aarch64_branch_conditional_cond field combination: imm19=8191, cond=0
    // Fields: cond=0, imm19=8191
    let encoding: u32 = 0x5403FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=8192 (power of 2 (2^13 = 8192))
#[test]
fn test_aarch64_branch_conditional_cond_combo_25_0_54040000() {
    // Encoding: 0x54040000
    // Test aarch64_branch_conditional_cond field combination: imm19=8192, cond=0
    // Fields: cond=0, imm19=8192
    let encoding: u32 = 0x54040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=16383 (2^14 - 1 = 16383)
#[test]
fn test_aarch64_branch_conditional_cond_combo_26_0_5407ffe0() {
    // Encoding: 0x5407FFE0
    // Test aarch64_branch_conditional_cond field combination: imm19=16383, cond=0
    // Fields: imm19=16383, cond=0
    let encoding: u32 = 0x5407FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=16384 (power of 2 (2^14 = 16384))
#[test]
fn test_aarch64_branch_conditional_cond_combo_27_0_54080000() {
    // Encoding: 0x54080000
    // Test aarch64_branch_conditional_cond field combination: imm19=16384, cond=0
    // Fields: imm19=16384, cond=0
    let encoding: u32 = 0x54080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=32767 (2^15 - 1 = 32767)
#[test]
fn test_aarch64_branch_conditional_cond_combo_28_0_540fffe0() {
    // Encoding: 0x540FFFE0
    // Test aarch64_branch_conditional_cond field combination: imm19=32767, cond=0
    // Fields: cond=0, imm19=32767
    let encoding: u32 = 0x540FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=32768 (power of 2 (2^15 = 32768))
#[test]
fn test_aarch64_branch_conditional_cond_combo_29_0_54100000() {
    // Encoding: 0x54100000
    // Test aarch64_branch_conditional_cond field combination: imm19=32768, cond=0
    // Fields: cond=0, imm19=32768
    let encoding: u32 = 0x54100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=65535 (2^16 - 1 = 65535)
#[test]
fn test_aarch64_branch_conditional_cond_combo_30_0_541fffe0() {
    // Encoding: 0x541FFFE0
    // Test aarch64_branch_conditional_cond field combination: imm19=65535, cond=0
    // Fields: cond=0, imm19=65535
    let encoding: u32 = 0x541FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=65536 (power of 2 (2^16 = 65536))
#[test]
fn test_aarch64_branch_conditional_cond_combo_31_0_54200000() {
    // Encoding: 0x54200000
    // Test aarch64_branch_conditional_cond field combination: imm19=65536, cond=0
    // Fields: imm19=65536, cond=0
    let encoding: u32 = 0x54200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=131071 (2^17 - 1 = 131071)
#[test]
fn test_aarch64_branch_conditional_cond_combo_32_0_543fffe0() {
    // Encoding: 0x543FFFE0
    // Test aarch64_branch_conditional_cond field combination: imm19=131071, cond=0
    // Fields: cond=0, imm19=131071
    let encoding: u32 = 0x543FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=131072 (power of 2 (2^17 = 131072))
#[test]
fn test_aarch64_branch_conditional_cond_combo_33_0_54400000() {
    // Encoding: 0x54400000
    // Test aarch64_branch_conditional_cond field combination: imm19=131072, cond=0
    // Fields: imm19=131072, cond=0
    let encoding: u32 = 0x54400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=262143 (immediate midpoint (262143))
#[test]
fn test_aarch64_branch_conditional_cond_combo_34_0_547fffe0() {
    // Encoding: 0x547FFFE0
    // Test aarch64_branch_conditional_cond field combination: imm19=262143, cond=0
    // Fields: cond=0, imm19=262143
    let encoding: u32 = 0x547FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=262144 (power of 2 (2^18 = 262144))
#[test]
fn test_aarch64_branch_conditional_cond_combo_35_0_54800000() {
    // Encoding: 0x54800000
    // Test aarch64_branch_conditional_cond field combination: imm19=262144, cond=0
    // Fields: cond=0, imm19=262144
    let encoding: u32 = 0x54800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=524287 (maximum immediate (524287))
#[test]
fn test_aarch64_branch_conditional_cond_combo_36_0_54ffffe0() {
    // Encoding: 0x54FFFFE0
    // Test aarch64_branch_conditional_cond field combination: imm19=524287, cond=0
    // Fields: cond=0, imm19=524287
    let encoding: u32 = 0x54FFFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 37`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch64_branch_conditional_cond_combo_37_0_54000000() {
    // Encoding: 0x54000000
    // Test aarch64_branch_conditional_cond field combination: imm19=0, cond=0
    // Fields: imm19=0, cond=0
    let encoding: u32 = 0x54000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 38`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=1 (condition NE (not equal))
#[test]
fn test_aarch64_branch_conditional_cond_combo_38_0_54000001() {
    // Encoding: 0x54000001
    // Test aarch64_branch_conditional_cond field combination: imm19=0, cond=1
    // Fields: cond=1, imm19=0
    let encoding: u32 = 0x54000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 39`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=2 (condition CS/HS (carry set))
#[test]
fn test_aarch64_branch_conditional_cond_combo_39_0_54000002() {
    // Encoding: 0x54000002
    // Test aarch64_branch_conditional_cond field combination: imm19=0, cond=2
    // Fields: imm19=0, cond=2
    let encoding: u32 = 0x54000002;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 40`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=3 (condition CC/LO (carry clear))
#[test]
fn test_aarch64_branch_conditional_cond_combo_40_0_54000003() {
    // Encoding: 0x54000003
    // Test aarch64_branch_conditional_cond field combination: imm19=0, cond=3
    // Fields: cond=3, imm19=0
    let encoding: u32 = 0x54000003;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 41`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=4 (condition MI (minus/negative))
#[test]
fn test_aarch64_branch_conditional_cond_combo_41_0_54000004() {
    // Encoding: 0x54000004
    // Test aarch64_branch_conditional_cond field combination: imm19=0, cond=4
    // Fields: imm19=0, cond=4
    let encoding: u32 = 0x54000004;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 42`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=5 (condition PL (plus/positive))
#[test]
fn test_aarch64_branch_conditional_cond_combo_42_0_54000005() {
    // Encoding: 0x54000005
    // Test aarch64_branch_conditional_cond field combination: imm19=0, cond=5
    // Fields: cond=5, imm19=0
    let encoding: u32 = 0x54000005;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 43`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=6 (condition VS (overflow set))
#[test]
fn test_aarch64_branch_conditional_cond_combo_43_0_54000006() {
    // Encoding: 0x54000006
    // Test aarch64_branch_conditional_cond field combination: imm19=0, cond=6
    // Fields: cond=6, imm19=0
    let encoding: u32 = 0x54000006;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 44`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=7 (condition VC (overflow clear))
#[test]
fn test_aarch64_branch_conditional_cond_combo_44_0_54000007() {
    // Encoding: 0x54000007
    // Test aarch64_branch_conditional_cond field combination: imm19=0, cond=7
    // Fields: cond=7, imm19=0
    let encoding: u32 = 0x54000007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 45`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=8 (condition HI (unsigned higher))
#[test]
fn test_aarch64_branch_conditional_cond_combo_45_0_54000008() {
    // Encoding: 0x54000008
    // Test aarch64_branch_conditional_cond field combination: imm19=0, cond=8
    // Fields: imm19=0, cond=8
    let encoding: u32 = 0x54000008;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 46`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=9 (condition LS (unsigned lower or same))
#[test]
fn test_aarch64_branch_conditional_cond_combo_46_0_54000009() {
    // Encoding: 0x54000009
    // Test aarch64_branch_conditional_cond field combination: imm19=0, cond=9
    // Fields: cond=9, imm19=0
    let encoding: u32 = 0x54000009;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 47`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=10 (condition GE (signed >=))
#[test]
fn test_aarch64_branch_conditional_cond_combo_47_0_5400000a() {
    // Encoding: 0x5400000A
    // Test aarch64_branch_conditional_cond field combination: imm19=0, cond=10
    // Fields: imm19=0, cond=10
    let encoding: u32 = 0x5400000A;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 48`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=11 (condition LT (signed <))
#[test]
fn test_aarch64_branch_conditional_cond_combo_48_0_5400000b() {
    // Encoding: 0x5400000B
    // Test aarch64_branch_conditional_cond field combination: imm19=0, cond=11
    // Fields: imm19=0, cond=11
    let encoding: u32 = 0x5400000B;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 49`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=12 (condition GT (signed >))
#[test]
fn test_aarch64_branch_conditional_cond_combo_49_0_5400000c() {
    // Encoding: 0x5400000C
    // Test aarch64_branch_conditional_cond field combination: imm19=0, cond=12
    // Fields: imm19=0, cond=12
    let encoding: u32 = 0x5400000C;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 50`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=13 (condition LE (signed <=))
#[test]
fn test_aarch64_branch_conditional_cond_combo_50_0_5400000d() {
    // Encoding: 0x5400000D
    // Test aarch64_branch_conditional_cond field combination: imm19=0, cond=13
    // Fields: cond=13, imm19=0
    let encoding: u32 = 0x5400000D;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 51`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=14 (condition AL (always))
#[test]
fn test_aarch64_branch_conditional_cond_combo_51_0_5400000e() {
    // Encoding: 0x5400000E
    // Test aarch64_branch_conditional_cond field combination: imm19=0, cond=14
    // Fields: cond=14, imm19=0
    let encoding: u32 = 0x5400000E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field combination 52`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=15 (condition NV (never, reserved))
#[test]
fn test_aarch64_branch_conditional_cond_combo_52_0_5400000f() {
    // Encoding: 0x5400000F
    // Test aarch64_branch_conditional_cond field combination: imm19=0, cond=15
    // Fields: imm19=0, cond=15
    let encoding: u32 = 0x5400000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch64_branch_conditional_cond_special_cond_0_condition_eq_0_54000020() {
    // Encoding: 0x54000020
    // Test aarch64_branch_conditional_cond special value cond = 0 (Condition EQ)
    // Fields: imm19=1, cond=0
    let encoding: u32 = 0x54000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch64_branch_conditional_cond_special_cond_1_condition_ne_0_54000021() {
    // Encoding: 0x54000021
    // Test aarch64_branch_conditional_cond special value cond = 1 (Condition NE)
    // Fields: cond=1, imm19=1
    let encoding: u32 = 0x54000021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch64_branch_conditional_cond_special_cond_2_condition_cs_hs_0_54000022() {
    // Encoding: 0x54000022
    // Test aarch64_branch_conditional_cond special value cond = 2 (Condition CS/HS)
    // Fields: imm19=1, cond=2
    let encoding: u32 = 0x54000022;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch64_branch_conditional_cond_special_cond_3_condition_cc_lo_0_54000023() {
    // Encoding: 0x54000023
    // Test aarch64_branch_conditional_cond special value cond = 3 (Condition CC/LO)
    // Fields: imm19=1, cond=3
    let encoding: u32 = 0x54000023;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch64_branch_conditional_cond_special_cond_4_condition_mi_0_54000024() {
    // Encoding: 0x54000024
    // Test aarch64_branch_conditional_cond special value cond = 4 (Condition MI)
    // Fields: imm19=1, cond=4
    let encoding: u32 = 0x54000024;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch64_branch_conditional_cond_special_cond_5_condition_pl_0_54000025() {
    // Encoding: 0x54000025
    // Test aarch64_branch_conditional_cond special value cond = 5 (Condition PL)
    // Fields: imm19=1, cond=5
    let encoding: u32 = 0x54000025;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch64_branch_conditional_cond_special_cond_6_condition_vs_0_54000026() {
    // Encoding: 0x54000026
    // Test aarch64_branch_conditional_cond special value cond = 6 (Condition VS)
    // Fields: cond=6, imm19=1
    let encoding: u32 = 0x54000026;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch64_branch_conditional_cond_special_cond_7_condition_vc_0_54000027() {
    // Encoding: 0x54000027
    // Test aarch64_branch_conditional_cond special value cond = 7 (Condition VC)
    // Fields: imm19=1, cond=7
    let encoding: u32 = 0x54000027;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch64_branch_conditional_cond_special_cond_8_condition_hi_0_54000028() {
    // Encoding: 0x54000028
    // Test aarch64_branch_conditional_cond special value cond = 8 (Condition HI)
    // Fields: cond=8, imm19=1
    let encoding: u32 = 0x54000028;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch64_branch_conditional_cond_special_cond_9_condition_ls_0_54000029() {
    // Encoding: 0x54000029
    // Test aarch64_branch_conditional_cond special value cond = 9 (Condition LS)
    // Fields: imm19=1, cond=9
    let encoding: u32 = 0x54000029;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch64_branch_conditional_cond_special_cond_10_condition_ge_0_5400002a() {
    // Encoding: 0x5400002A
    // Test aarch64_branch_conditional_cond special value cond = 10 (Condition GE)
    // Fields: imm19=1, cond=10
    let encoding: u32 = 0x5400002A;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch64_branch_conditional_cond_special_cond_11_condition_lt_0_5400002b() {
    // Encoding: 0x5400002B
    // Test aarch64_branch_conditional_cond special value cond = 11 (Condition LT)
    // Fields: imm19=1, cond=11
    let encoding: u32 = 0x5400002B;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch64_branch_conditional_cond_special_cond_12_condition_gt_0_5400002c() {
    // Encoding: 0x5400002C
    // Test aarch64_branch_conditional_cond special value cond = 12 (Condition GT)
    // Fields: imm19=1, cond=12
    let encoding: u32 = 0x5400002C;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch64_branch_conditional_cond_special_cond_13_condition_le_0_5400002d() {
    // Encoding: 0x5400002D
    // Test aarch64_branch_conditional_cond special value cond = 13 (Condition LE)
    // Fields: cond=13, imm19=1
    let encoding: u32 = 0x5400002D;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch64_branch_conditional_cond_special_cond_14_condition_al_0_5400002e() {
    // Encoding: 0x5400002E
    // Test aarch64_branch_conditional_cond special value cond = 14 (Condition AL)
    // Fields: cond=14, imm19=1
    let encoding: u32 = 0x5400002E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_cond
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch64_branch_conditional_cond_special_cond_15_condition_nv_0_5400002f() {
    // Encoding: 0x5400002F
    // Test aarch64_branch_conditional_cond special value cond = 15 (Condition NV)
    // Fields: imm19=1, cond=15
    let encoding: u32 = 0x5400002F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

// ============================================================================
// aarch64_branch_conditional_compare Tests
// ============================================================================

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_branch_conditional_compare_field_sf_0_min_0_34000000() {
    // Encoding: 0x34000000
    // Test aarch64_branch_conditional_compare field sf = 0 (Min)
    // Fields: op=0, imm19=0, sf=0, Rt=0
    let encoding: u32 = 0x34000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_branch_conditional_compare_field_sf_1_max_0_b4000000() {
    // Encoding: 0xB4000000
    // Test aarch64_branch_conditional_compare field sf = 1 (Max)
    // Fields: op=0, Rt=0, sf=1, imm19=0
    let encoding: u32 = 0xB4000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field op 24 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_branch_conditional_compare_field_op_0_min_0_34000000() {
    // Encoding: 0x34000000
    // Test aarch64_branch_conditional_compare field op = 0 (Min)
    // Fields: sf=0, imm19=0, op=0, Rt=0
    let encoding: u32 = 0x34000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field op 24 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_branch_conditional_compare_field_op_1_max_0_35000000() {
    // Encoding: 0x35000000
    // Test aarch64_branch_conditional_compare field op = 1 (Max)
    // Fields: Rt=0, sf=0, op=1, imm19=0
    let encoding: u32 = 0x35000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_0_zero_0_34000000() {
    // Encoding: 0x34000000
    // Test aarch64_branch_conditional_compare field imm19 = 0 (Zero)
    // Fields: sf=0, Rt=0, imm19=0, op=0
    let encoding: u32 = 0x34000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_1_poweroftwo_0_34000020() {
    // Encoding: 0x34000020
    // Test aarch64_branch_conditional_compare field imm19 = 1 (PowerOfTwo)
    // Fields: op=0, Rt=0, sf=0, imm19=1
    let encoding: u32 = 0x34000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_3_poweroftwominusone_0_34000060() {
    // Encoding: 0x34000060
    // Test aarch64_branch_conditional_compare field imm19 = 3 (PowerOfTwoMinusOne)
    // Fields: sf=0, op=0, imm19=3, Rt=0
    let encoding: u32 = 0x34000060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_4_poweroftwo_0_34000080() {
    // Encoding: 0x34000080
    // Test aarch64_branch_conditional_compare field imm19 = 4 (PowerOfTwo)
    // Fields: Rt=0, op=0, sf=0, imm19=4
    let encoding: u32 = 0x34000080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_7_poweroftwominusone_0_340000e0() {
    // Encoding: 0x340000E0
    // Test aarch64_branch_conditional_compare field imm19 = 7 (PowerOfTwoMinusOne)
    // Fields: Rt=0, op=0, sf=0, imm19=7
    let encoding: u32 = 0x340000E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_8_poweroftwo_0_34000100() {
    // Encoding: 0x34000100
    // Test aarch64_branch_conditional_compare field imm19 = 8 (PowerOfTwo)
    // Fields: sf=0, op=0, imm19=8, Rt=0
    let encoding: u32 = 0x34000100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_15_poweroftwominusone_0_340001e0() {
    // Encoding: 0x340001E0
    // Test aarch64_branch_conditional_compare field imm19 = 15 (PowerOfTwoMinusOne)
    // Fields: Rt=0, sf=0, op=0, imm19=15
    let encoding: u32 = 0x340001E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_16_poweroftwo_0_34000200() {
    // Encoding: 0x34000200
    // Test aarch64_branch_conditional_compare field imm19 = 16 (PowerOfTwo)
    // Fields: Rt=0, imm19=16, sf=0, op=0
    let encoding: u32 = 0x34000200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_31_poweroftwominusone_0_340003e0() {
    // Encoding: 0x340003E0
    // Test aarch64_branch_conditional_compare field imm19 = 31 (PowerOfTwoMinusOne)
    // Fields: imm19=31, Rt=0, op=0, sf=0
    let encoding: u32 = 0x340003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_32_poweroftwo_0_34000400() {
    // Encoding: 0x34000400
    // Test aarch64_branch_conditional_compare field imm19 = 32 (PowerOfTwo)
    // Fields: op=0, imm19=32, Rt=0, sf=0
    let encoding: u32 = 0x34000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_63_poweroftwominusone_0_340007e0() {
    // Encoding: 0x340007E0
    // Test aarch64_branch_conditional_compare field imm19 = 63 (PowerOfTwoMinusOne)
    // Fields: Rt=0, sf=0, imm19=63, op=0
    let encoding: u32 = 0x340007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_64_poweroftwo_0_34000800() {
    // Encoding: 0x34000800
    // Test aarch64_branch_conditional_compare field imm19 = 64 (PowerOfTwo)
    // Fields: imm19=64, sf=0, Rt=0, op=0
    let encoding: u32 = 0x34000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_127_poweroftwominusone_0_34000fe0() {
    // Encoding: 0x34000FE0
    // Test aarch64_branch_conditional_compare field imm19 = 127 (PowerOfTwoMinusOne)
    // Fields: imm19=127, sf=0, Rt=0, op=0
    let encoding: u32 = 0x34000FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_128_poweroftwo_0_34001000() {
    // Encoding: 0x34001000
    // Test aarch64_branch_conditional_compare field imm19 = 128 (PowerOfTwo)
    // Fields: imm19=128, sf=0, op=0, Rt=0
    let encoding: u32 = 0x34001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_255_poweroftwominusone_0_34001fe0() {
    // Encoding: 0x34001FE0
    // Test aarch64_branch_conditional_compare field imm19 = 255 (PowerOfTwoMinusOne)
    // Fields: sf=0, imm19=255, Rt=0, op=0
    let encoding: u32 = 0x34001FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_256_poweroftwo_0_34002000() {
    // Encoding: 0x34002000
    // Test aarch64_branch_conditional_compare field imm19 = 256 (PowerOfTwo)
    // Fields: sf=0, imm19=256, Rt=0, op=0
    let encoding: u32 = 0x34002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_511_poweroftwominusone_0_34003fe0() {
    // Encoding: 0x34003FE0
    // Test aarch64_branch_conditional_compare field imm19 = 511 (PowerOfTwoMinusOne)
    // Fields: Rt=0, sf=0, op=0, imm19=511
    let encoding: u32 = 0x34003FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_512_poweroftwo_0_34004000() {
    // Encoding: 0x34004000
    // Test aarch64_branch_conditional_compare field imm19 = 512 (PowerOfTwo)
    // Fields: imm19=512, Rt=0, op=0, sf=0
    let encoding: u32 = 0x34004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 1023, boundary: PowerOfTwoMinusOne }
/// 2^10 - 1 = 1023
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_1023_poweroftwominusone_0_34007fe0() {
    // Encoding: 0x34007FE0
    // Test aarch64_branch_conditional_compare field imm19 = 1023 (PowerOfTwoMinusOne)
    // Fields: Rt=0, sf=0, op=0, imm19=1023
    let encoding: u32 = 0x34007FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_1024_poweroftwo_0_34008000() {
    // Encoding: 0x34008000
    // Test aarch64_branch_conditional_compare field imm19 = 1024 (PowerOfTwo)
    // Fields: op=0, sf=0, imm19=1024, Rt=0
    let encoding: u32 = 0x34008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 2047, boundary: PowerOfTwoMinusOne }
/// 2^11 - 1 = 2047
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_2047_poweroftwominusone_0_3400ffe0() {
    // Encoding: 0x3400FFE0
    // Test aarch64_branch_conditional_compare field imm19 = 2047 (PowerOfTwoMinusOne)
    // Fields: op=0, imm19=2047, Rt=0, sf=0
    let encoding: u32 = 0x3400FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 2048, boundary: PowerOfTwo }
/// power of 2 (2^11 = 2048)
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_2048_poweroftwo_0_34010000() {
    // Encoding: 0x34010000
    // Test aarch64_branch_conditional_compare field imm19 = 2048 (PowerOfTwo)
    // Fields: sf=0, imm19=2048, op=0, Rt=0
    let encoding: u32 = 0x34010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 4095, boundary: PowerOfTwoMinusOne }
/// 2^12 - 1 = 4095
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_4095_poweroftwominusone_0_3401ffe0() {
    // Encoding: 0x3401FFE0
    // Test aarch64_branch_conditional_compare field imm19 = 4095 (PowerOfTwoMinusOne)
    // Fields: sf=0, imm19=4095, Rt=0, op=0
    let encoding: u32 = 0x3401FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 4096, boundary: PowerOfTwo }
/// power of 2 (2^12 = 4096)
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_4096_poweroftwo_0_34020000() {
    // Encoding: 0x34020000
    // Test aarch64_branch_conditional_compare field imm19 = 4096 (PowerOfTwo)
    // Fields: Rt=0, sf=0, op=0, imm19=4096
    let encoding: u32 = 0x34020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 8191, boundary: PowerOfTwoMinusOne }
/// 2^13 - 1 = 8191
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_8191_poweroftwominusone_0_3403ffe0() {
    // Encoding: 0x3403FFE0
    // Test aarch64_branch_conditional_compare field imm19 = 8191 (PowerOfTwoMinusOne)
    // Fields: sf=0, op=0, imm19=8191, Rt=0
    let encoding: u32 = 0x3403FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 8192, boundary: PowerOfTwo }
/// power of 2 (2^13 = 8192)
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_8192_poweroftwo_0_34040000() {
    // Encoding: 0x34040000
    // Test aarch64_branch_conditional_compare field imm19 = 8192 (PowerOfTwo)
    // Fields: imm19=8192, Rt=0, sf=0, op=0
    let encoding: u32 = 0x34040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 16383, boundary: PowerOfTwoMinusOne }
/// 2^14 - 1 = 16383
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_16383_poweroftwominusone_0_3407ffe0() {
    // Encoding: 0x3407FFE0
    // Test aarch64_branch_conditional_compare field imm19 = 16383 (PowerOfTwoMinusOne)
    // Fields: op=0, sf=0, imm19=16383, Rt=0
    let encoding: u32 = 0x3407FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 16384, boundary: PowerOfTwo }
/// power of 2 (2^14 = 16384)
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_16384_poweroftwo_0_34080000() {
    // Encoding: 0x34080000
    // Test aarch64_branch_conditional_compare field imm19 = 16384 (PowerOfTwo)
    // Fields: op=0, sf=0, imm19=16384, Rt=0
    let encoding: u32 = 0x34080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 32767, boundary: PowerOfTwoMinusOne }
/// 2^15 - 1 = 32767
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_32767_poweroftwominusone_0_340fffe0() {
    // Encoding: 0x340FFFE0
    // Test aarch64_branch_conditional_compare field imm19 = 32767 (PowerOfTwoMinusOne)
    // Fields: sf=0, Rt=0, imm19=32767, op=0
    let encoding: u32 = 0x340FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 32768, boundary: PowerOfTwo }
/// power of 2 (2^15 = 32768)
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_32768_poweroftwo_0_34100000() {
    // Encoding: 0x34100000
    // Test aarch64_branch_conditional_compare field imm19 = 32768 (PowerOfTwo)
    // Fields: op=0, Rt=0, imm19=32768, sf=0
    let encoding: u32 = 0x34100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 65535, boundary: PowerOfTwoMinusOne }
/// 2^16 - 1 = 65535
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_65535_poweroftwominusone_0_341fffe0() {
    // Encoding: 0x341FFFE0
    // Test aarch64_branch_conditional_compare field imm19 = 65535 (PowerOfTwoMinusOne)
    // Fields: sf=0, Rt=0, op=0, imm19=65535
    let encoding: u32 = 0x341FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 65536, boundary: PowerOfTwo }
/// power of 2 (2^16 = 65536)
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_65536_poweroftwo_0_34200000() {
    // Encoding: 0x34200000
    // Test aarch64_branch_conditional_compare field imm19 = 65536 (PowerOfTwo)
    // Fields: Rt=0, sf=0, imm19=65536, op=0
    let encoding: u32 = 0x34200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 131071, boundary: PowerOfTwoMinusOne }
/// 2^17 - 1 = 131071
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_131071_poweroftwominusone_0_343fffe0() {
    // Encoding: 0x343FFFE0
    // Test aarch64_branch_conditional_compare field imm19 = 131071 (PowerOfTwoMinusOne)
    // Fields: op=0, Rt=0, sf=0, imm19=131071
    let encoding: u32 = 0x343FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 131072, boundary: PowerOfTwo }
/// power of 2 (2^17 = 131072)
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_131072_poweroftwo_0_34400000() {
    // Encoding: 0x34400000
    // Test aarch64_branch_conditional_compare field imm19 = 131072 (PowerOfTwo)
    // Fields: sf=0, imm19=131072, Rt=0, op=0
    let encoding: u32 = 0x34400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 262143, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (262143)
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_262143_poweroftwominusone_0_347fffe0() {
    // Encoding: 0x347FFFE0
    // Test aarch64_branch_conditional_compare field imm19 = 262143 (PowerOfTwoMinusOne)
    // Fields: imm19=262143, op=0, Rt=0, sf=0
    let encoding: u32 = 0x347FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 262144, boundary: PowerOfTwo }
/// power of 2 (2^18 = 262144)
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_262144_poweroftwo_0_34800000() {
    // Encoding: 0x34800000
    // Test aarch64_branch_conditional_compare field imm19 = 262144 (PowerOfTwo)
    // Fields: op=0, imm19=262144, sf=0, Rt=0
    let encoding: u32 = 0x34800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 524287, boundary: Max }
/// maximum immediate (524287)
#[test]
fn test_aarch64_branch_conditional_compare_field_imm19_524287_max_0_34ffffe0() {
    // Encoding: 0x34FFFFE0
    // Test aarch64_branch_conditional_compare field imm19 = 524287 (Max)
    // Fields: op=0, imm19=524287, sf=0, Rt=0
    let encoding: u32 = 0x34FFFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_branch_conditional_compare_field_rt_0_min_0_34000000() {
    // Encoding: 0x34000000
    // Test aarch64_branch_conditional_compare field Rt = 0 (Min)
    // Fields: op=0, Rt=0, imm19=0, sf=0
    let encoding: u32 = 0x34000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_branch_conditional_compare_field_rt_1_poweroftwo_0_34000001() {
    // Encoding: 0x34000001
    // Test aarch64_branch_conditional_compare field Rt = 1 (PowerOfTwo)
    // Fields: imm19=0, sf=0, op=0, Rt=1
    let encoding: u32 = 0x34000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_branch_conditional_compare_field_rt_30_poweroftwominusone_0_3400001e() {
    // Encoding: 0x3400001E
    // Test aarch64_branch_conditional_compare field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: imm19=0, Rt=30, sf=0, op=0
    let encoding: u32 = 0x3400001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_branch_conditional_compare_field_rt_31_max_0_3400001f() {
    // Encoding: 0x3400001F
    // Test aarch64_branch_conditional_compare field Rt = 31 (Max)
    // Fields: op=0, sf=0, Rt=31, imm19=0
    let encoding: u32 = 0x3400001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=0 (8-bit / byte size)
#[test]
fn test_aarch64_branch_conditional_compare_combo_0_0_34000000() {
    // Encoding: 0x34000000
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=0, Rt=0
    // Fields: imm19=0, op=0, sf=0, Rt=0
    let encoding: u32 = 0x34000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=1 (16-bit / halfword size)
#[test]
fn test_aarch64_branch_conditional_compare_combo_1_0_b4000000() {
    // Encoding: 0xB4000000
    // Test aarch64_branch_conditional_compare field combination: sf=1, op=0, imm19=0, Rt=0
    // Fields: sf=1, op=0, Rt=0, imm19=0
    let encoding: u32 = 0xB4000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_branch_conditional_compare_combo_2_0_34000000() {
    // Encoding: 0x34000000
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=0, Rt=0
    // Fields: sf=0, op=0, imm19=0, Rt=0
    let encoding: u32 = 0x34000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_branch_conditional_compare_combo_3_0_35000000() {
    // Encoding: 0x35000000
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=1, imm19=0, Rt=0
    // Fields: Rt=0, sf=0, op=1, imm19=0
    let encoding: u32 = 0x35000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=0 (immediate value 0)
#[test]
fn test_aarch64_branch_conditional_compare_combo_4_0_34000000() {
    // Encoding: 0x34000000
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=0, Rt=0
    // Fields: Rt=0, sf=0, op=0, imm19=0
    let encoding: u32 = 0x34000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=1 (immediate value 1)
#[test]
fn test_aarch64_branch_conditional_compare_combo_5_0_34000020() {
    // Encoding: 0x34000020
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=1, Rt=0
    // Fields: op=0, Rt=0, imm19=1, sf=0
    let encoding: u32 = 0x34000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_branch_conditional_compare_combo_6_0_34000060() {
    // Encoding: 0x34000060
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=3, Rt=0
    // Fields: op=0, imm19=3, Rt=0, sf=0
    let encoding: u32 = 0x34000060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_branch_conditional_compare_combo_7_0_34000080() {
    // Encoding: 0x34000080
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=4, Rt=0
    // Fields: sf=0, Rt=0, op=0, imm19=4
    let encoding: u32 = 0x34000080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_branch_conditional_compare_combo_8_0_340000e0() {
    // Encoding: 0x340000E0
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=7, Rt=0
    // Fields: sf=0, op=0, imm19=7, Rt=0
    let encoding: u32 = 0x340000E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_branch_conditional_compare_combo_9_0_34000100() {
    // Encoding: 0x34000100
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=8, Rt=0
    // Fields: sf=0, op=0, imm19=8, Rt=0
    let encoding: u32 = 0x34000100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_branch_conditional_compare_combo_10_0_340001e0() {
    // Encoding: 0x340001E0
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=15, Rt=0
    // Fields: Rt=0, imm19=15, sf=0, op=0
    let encoding: u32 = 0x340001E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_branch_conditional_compare_combo_11_0_34000200() {
    // Encoding: 0x34000200
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=16, Rt=0
    // Fields: imm19=16, sf=0, Rt=0, op=0
    let encoding: u32 = 0x34000200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_branch_conditional_compare_combo_12_0_340003e0() {
    // Encoding: 0x340003E0
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=31, Rt=0
    // Fields: op=0, Rt=0, imm19=31, sf=0
    let encoding: u32 = 0x340003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_branch_conditional_compare_combo_13_0_34000400() {
    // Encoding: 0x34000400
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=32, Rt=0
    // Fields: op=0, Rt=0, imm19=32, sf=0
    let encoding: u32 = 0x34000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_branch_conditional_compare_combo_14_0_340007e0() {
    // Encoding: 0x340007E0
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=63, Rt=0
    // Fields: sf=0, op=0, imm19=63, Rt=0
    let encoding: u32 = 0x340007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_branch_conditional_compare_combo_15_0_34000800() {
    // Encoding: 0x34000800
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=64, Rt=0
    // Fields: sf=0, op=0, imm19=64, Rt=0
    let encoding: u32 = 0x34000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_branch_conditional_compare_combo_16_0_34000fe0() {
    // Encoding: 0x34000FE0
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=127, Rt=0
    // Fields: sf=0, Rt=0, imm19=127, op=0
    let encoding: u32 = 0x34000FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_branch_conditional_compare_combo_17_0_34001000() {
    // Encoding: 0x34001000
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=128, Rt=0
    // Fields: sf=0, op=0, imm19=128, Rt=0
    let encoding: u32 = 0x34001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=255 (2^8 - 1 = 255)
#[test]
fn test_aarch64_branch_conditional_compare_combo_18_0_34001fe0() {
    // Encoding: 0x34001FE0
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=255, Rt=0
    // Fields: sf=0, imm19=255, Rt=0, op=0
    let encoding: u32 = 0x34001FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_branch_conditional_compare_combo_19_0_34002000() {
    // Encoding: 0x34002000
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=256, Rt=0
    // Fields: sf=0, Rt=0, imm19=256, op=0
    let encoding: u32 = 0x34002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=511 (2^9 - 1 = 511)
#[test]
fn test_aarch64_branch_conditional_compare_combo_20_0_34003fe0() {
    // Encoding: 0x34003FE0
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=511, Rt=0
    // Fields: op=0, Rt=0, imm19=511, sf=0
    let encoding: u32 = 0x34003FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=512 (power of 2 (2^9 = 512))
#[test]
fn test_aarch64_branch_conditional_compare_combo_21_0_34004000() {
    // Encoding: 0x34004000
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=512, Rt=0
    // Fields: op=0, Rt=0, sf=0, imm19=512
    let encoding: u32 = 0x34004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=1023 (2^10 - 1 = 1023)
#[test]
fn test_aarch64_branch_conditional_compare_combo_22_0_34007fe0() {
    // Encoding: 0x34007FE0
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=1023, Rt=0
    // Fields: op=0, sf=0, imm19=1023, Rt=0
    let encoding: u32 = 0x34007FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=1024 (power of 2 (2^10 = 1024))
#[test]
fn test_aarch64_branch_conditional_compare_combo_23_0_34008000() {
    // Encoding: 0x34008000
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=1024, Rt=0
    // Fields: sf=0, imm19=1024, op=0, Rt=0
    let encoding: u32 = 0x34008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=2047 (2^11 - 1 = 2047)
#[test]
fn test_aarch64_branch_conditional_compare_combo_24_0_3400ffe0() {
    // Encoding: 0x3400FFE0
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=2047, Rt=0
    // Fields: op=0, imm19=2047, sf=0, Rt=0
    let encoding: u32 = 0x3400FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=2048 (power of 2 (2^11 = 2048))
#[test]
fn test_aarch64_branch_conditional_compare_combo_25_0_34010000() {
    // Encoding: 0x34010000
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=2048, Rt=0
    // Fields: Rt=0, op=0, imm19=2048, sf=0
    let encoding: u32 = 0x34010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=4095 (2^12 - 1 = 4095)
#[test]
fn test_aarch64_branch_conditional_compare_combo_26_0_3401ffe0() {
    // Encoding: 0x3401FFE0
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=4095, Rt=0
    // Fields: sf=0, op=0, Rt=0, imm19=4095
    let encoding: u32 = 0x3401FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=4096 (power of 2 (2^12 = 4096))
#[test]
fn test_aarch64_branch_conditional_compare_combo_27_0_34020000() {
    // Encoding: 0x34020000
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=4096, Rt=0
    // Fields: imm19=4096, Rt=0, sf=0, op=0
    let encoding: u32 = 0x34020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=8191 (2^13 - 1 = 8191)
#[test]
fn test_aarch64_branch_conditional_compare_combo_28_0_3403ffe0() {
    // Encoding: 0x3403FFE0
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=8191, Rt=0
    // Fields: op=0, sf=0, imm19=8191, Rt=0
    let encoding: u32 = 0x3403FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=8192 (power of 2 (2^13 = 8192))
#[test]
fn test_aarch64_branch_conditional_compare_combo_29_0_34040000() {
    // Encoding: 0x34040000
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=8192, Rt=0
    // Fields: Rt=0, imm19=8192, sf=0, op=0
    let encoding: u32 = 0x34040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=16383 (2^14 - 1 = 16383)
#[test]
fn test_aarch64_branch_conditional_compare_combo_30_0_3407ffe0() {
    // Encoding: 0x3407FFE0
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=16383, Rt=0
    // Fields: sf=0, op=0, Rt=0, imm19=16383
    let encoding: u32 = 0x3407FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=16384 (power of 2 (2^14 = 16384))
#[test]
fn test_aarch64_branch_conditional_compare_combo_31_0_34080000() {
    // Encoding: 0x34080000
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=16384, Rt=0
    // Fields: op=0, Rt=0, sf=0, imm19=16384
    let encoding: u32 = 0x34080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=32767 (2^15 - 1 = 32767)
#[test]
fn test_aarch64_branch_conditional_compare_combo_32_0_340fffe0() {
    // Encoding: 0x340FFFE0
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=32767, Rt=0
    // Fields: Rt=0, op=0, sf=0, imm19=32767
    let encoding: u32 = 0x340FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=32768 (power of 2 (2^15 = 32768))
#[test]
fn test_aarch64_branch_conditional_compare_combo_33_0_34100000() {
    // Encoding: 0x34100000
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=32768, Rt=0
    // Fields: imm19=32768, Rt=0, op=0, sf=0
    let encoding: u32 = 0x34100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=65535 (2^16 - 1 = 65535)
#[test]
fn test_aarch64_branch_conditional_compare_combo_34_0_341fffe0() {
    // Encoding: 0x341FFFE0
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=65535, Rt=0
    // Fields: sf=0, op=0, imm19=65535, Rt=0
    let encoding: u32 = 0x341FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=65536 (power of 2 (2^16 = 65536))
#[test]
fn test_aarch64_branch_conditional_compare_combo_35_0_34200000() {
    // Encoding: 0x34200000
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=65536, Rt=0
    // Fields: Rt=0, imm19=65536, sf=0, op=0
    let encoding: u32 = 0x34200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=131071 (2^17 - 1 = 131071)
#[test]
fn test_aarch64_branch_conditional_compare_combo_36_0_343fffe0() {
    // Encoding: 0x343FFFE0
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=131071, Rt=0
    // Fields: sf=0, imm19=131071, Rt=0, op=0
    let encoding: u32 = 0x343FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 37`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=131072 (power of 2 (2^17 = 131072))
#[test]
fn test_aarch64_branch_conditional_compare_combo_37_0_34400000() {
    // Encoding: 0x34400000
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=131072, Rt=0
    // Fields: imm19=131072, op=0, Rt=0, sf=0
    let encoding: u32 = 0x34400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 38`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=262143 (immediate midpoint (262143))
#[test]
fn test_aarch64_branch_conditional_compare_combo_38_0_347fffe0() {
    // Encoding: 0x347FFFE0
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=262143, Rt=0
    // Fields: imm19=262143, sf=0, op=0, Rt=0
    let encoding: u32 = 0x347FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 39`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=262144 (power of 2 (2^18 = 262144))
#[test]
fn test_aarch64_branch_conditional_compare_combo_39_0_34800000() {
    // Encoding: 0x34800000
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=262144, Rt=0
    // Fields: Rt=0, op=0, imm19=262144, sf=0
    let encoding: u32 = 0x34800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 40`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=524287 (maximum immediate (524287))
#[test]
fn test_aarch64_branch_conditional_compare_combo_40_0_34ffffe0() {
    // Encoding: 0x34FFFFE0
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=524287, Rt=0
    // Fields: op=0, imm19=524287, sf=0, Rt=0
    let encoding: u32 = 0x34FFFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 41`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_branch_conditional_compare_combo_41_0_34000000() {
    // Encoding: 0x34000000
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=0, Rt=0
    // Fields: imm19=0, sf=0, op=0, Rt=0
    let encoding: u32 = 0x34000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 42`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_branch_conditional_compare_combo_42_0_34000001() {
    // Encoding: 0x34000001
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=0, Rt=1
    // Fields: op=0, imm19=0, Rt=1, sf=0
    let encoding: u32 = 0x34000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 43`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_branch_conditional_compare_combo_43_0_3400001e() {
    // Encoding: 0x3400001E
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=0, Rt=30
    // Fields: sf=0, imm19=0, Rt=30, op=0
    let encoding: u32 = 0x3400001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field combination 44`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_branch_conditional_compare_combo_44_0_3400001f() {
    // Encoding: 0x3400001F
    // Test aarch64_branch_conditional_compare field combination: sf=0, op=0, imm19=0, Rt=31
    // Fields: imm19=0, Rt=31, op=0, sf=0
    let encoding: u32 = 0x3400001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_branch_conditional_compare_special_sf_0_size_variant_0_0_34000020() {
    // Encoding: 0x34000020
    // Test aarch64_branch_conditional_compare special value sf = 0 (Size variant 0)
    // Fields: sf=0, imm19=1, Rt=0, op=0
    let encoding: u32 = 0x34000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_branch_conditional_compare_special_sf_1_size_variant_1_0_b4000020() {
    // Encoding: 0xB4000020
    // Test aarch64_branch_conditional_compare special value sf = 1 (Size variant 1)
    // Fields: op=0, sf=1, imm19=1, Rt=0
    let encoding: u32 = 0xB4000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_branch_conditional_compare_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_3400003f()
 {
    // Encoding: 0x3400003F
    // Test aarch64_branch_conditional_compare special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rt=31, op=0, sf=0, imm19=1
    let encoding: u32 = 0x3400003F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_compare
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_branch_conditional_compare_zr_rt_3400001f() {
    // Test aarch64_branch_conditional_compare with Rt = ZR (31)
    // Encoding: 0x3400001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x3400001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_branch_conditional_test Tests
// ============================================================================

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field b5 31 +: 1`
/// Requirement: FieldBoundary { field: "b5", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_branch_conditional_test_field_b5_0_min_0_36000000() {
    // Encoding: 0x36000000
    // Test aarch64_branch_conditional_test field b5 = 0 (Min)
    // Fields: b5=0, op=0, b40=0, Rt=0, imm14=0
    let encoding: u32 = 0x36000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field b5 31 +: 1`
/// Requirement: FieldBoundary { field: "b5", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_branch_conditional_test_field_b5_1_max_0_b6000000() {
    // Encoding: 0xB6000000
    // Test aarch64_branch_conditional_test field b5 = 1 (Max)
    // Fields: b40=0, op=0, imm14=0, Rt=0, b5=1
    let encoding: u32 = 0xB6000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field op 24 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_branch_conditional_test_field_op_0_min_0_36000000() {
    // Encoding: 0x36000000
    // Test aarch64_branch_conditional_test field op = 0 (Min)
    // Fields: Rt=0, imm14=0, op=0, b40=0, b5=0
    let encoding: u32 = 0x36000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field op 24 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_branch_conditional_test_field_op_1_max_0_37000000() {
    // Encoding: 0x37000000
    // Test aarch64_branch_conditional_test field op = 1 (Max)
    // Fields: imm14=0, b40=0, b5=0, Rt=0, op=1
    let encoding: u32 = 0x37000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field b40 19 +: 5`
/// Requirement: FieldBoundary { field: "b40", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_branch_conditional_test_field_b40_0_min_0_36000000() {
    // Encoding: 0x36000000
    // Test aarch64_branch_conditional_test field b40 = 0 (Min)
    // Fields: b40=0, imm14=0, b5=0, op=0, Rt=0
    let encoding: u32 = 0x36000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field b40 19 +: 5`
/// Requirement: FieldBoundary { field: "b40", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_branch_conditional_test_field_b40_1_poweroftwo_0_36080000() {
    // Encoding: 0x36080000
    // Test aarch64_branch_conditional_test field b40 = 1 (PowerOfTwo)
    // Fields: op=0, imm14=0, b5=0, Rt=0, b40=1
    let encoding: u32 = 0x36080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field b40 19 +: 5`
/// Requirement: FieldBoundary { field: "b40", value: 15, boundary: PowerOfTwoMinusOne }
/// midpoint (15)
#[test]
fn test_aarch64_branch_conditional_test_field_b40_15_poweroftwominusone_0_36780000() {
    // Encoding: 0x36780000
    // Test aarch64_branch_conditional_test field b40 = 15 (PowerOfTwoMinusOne)
    // Fields: imm14=0, b5=0, op=0, Rt=0, b40=15
    let encoding: u32 = 0x36780000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field b40 19 +: 5`
/// Requirement: FieldBoundary { field: "b40", value: 31, boundary: Max }
/// maximum value (31)
#[test]
fn test_aarch64_branch_conditional_test_field_b40_31_max_0_36f80000() {
    // Encoding: 0x36F80000
    // Test aarch64_branch_conditional_test field b40 = 31 (Max)
    // Fields: imm14=0, b5=0, op=0, Rt=0, b40=31
    let encoding: u32 = 0x36F80000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_0_zero_0_36000000() {
    // Encoding: 0x36000000
    // Test aarch64_branch_conditional_test field imm14 = 0 (Zero)
    // Fields: imm14=0, b40=0, Rt=0, op=0, b5=0
    let encoding: u32 = 0x36000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_1_poweroftwo_0_36000020() {
    // Encoding: 0x36000020
    // Test aarch64_branch_conditional_test field imm14 = 1 (PowerOfTwo)
    // Fields: b5=0, imm14=1, Rt=0, op=0, b40=0
    let encoding: u32 = 0x36000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_3_poweroftwominusone_0_36000060() {
    // Encoding: 0x36000060
    // Test aarch64_branch_conditional_test field imm14 = 3 (PowerOfTwoMinusOne)
    // Fields: b5=0, op=0, Rt=0, imm14=3, b40=0
    let encoding: u32 = 0x36000060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_4_poweroftwo_0_36000080() {
    // Encoding: 0x36000080
    // Test aarch64_branch_conditional_test field imm14 = 4 (PowerOfTwo)
    // Fields: op=0, b5=0, imm14=4, Rt=0, b40=0
    let encoding: u32 = 0x36000080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_7_poweroftwominusone_0_360000e0() {
    // Encoding: 0x360000E0
    // Test aarch64_branch_conditional_test field imm14 = 7 (PowerOfTwoMinusOne)
    // Fields: b5=0, op=0, b40=0, imm14=7, Rt=0
    let encoding: u32 = 0x360000E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_8_poweroftwo_0_36000100() {
    // Encoding: 0x36000100
    // Test aarch64_branch_conditional_test field imm14 = 8 (PowerOfTwo)
    // Fields: imm14=8, Rt=0, b5=0, op=0, b40=0
    let encoding: u32 = 0x36000100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_15_poweroftwominusone_0_360001e0() {
    // Encoding: 0x360001E0
    // Test aarch64_branch_conditional_test field imm14 = 15 (PowerOfTwoMinusOne)
    // Fields: b40=0, imm14=15, Rt=0, b5=0, op=0
    let encoding: u32 = 0x360001E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_16_poweroftwo_0_36000200() {
    // Encoding: 0x36000200
    // Test aarch64_branch_conditional_test field imm14 = 16 (PowerOfTwo)
    // Fields: b40=0, imm14=16, b5=0, Rt=0, op=0
    let encoding: u32 = 0x36000200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_31_poweroftwominusone_0_360003e0() {
    // Encoding: 0x360003E0
    // Test aarch64_branch_conditional_test field imm14 = 31 (PowerOfTwoMinusOne)
    // Fields: op=0, imm14=31, b5=0, Rt=0, b40=0
    let encoding: u32 = 0x360003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_32_poweroftwo_0_36000400() {
    // Encoding: 0x36000400
    // Test aarch64_branch_conditional_test field imm14 = 32 (PowerOfTwo)
    // Fields: imm14=32, op=0, Rt=0, b5=0, b40=0
    let encoding: u32 = 0x36000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_63_poweroftwominusone_0_360007e0() {
    // Encoding: 0x360007E0
    // Test aarch64_branch_conditional_test field imm14 = 63 (PowerOfTwoMinusOne)
    // Fields: imm14=63, Rt=0, b40=0, op=0, b5=0
    let encoding: u32 = 0x360007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_64_poweroftwo_0_36000800() {
    // Encoding: 0x36000800
    // Test aarch64_branch_conditional_test field imm14 = 64 (PowerOfTwo)
    // Fields: op=0, b40=0, Rt=0, imm14=64, b5=0
    let encoding: u32 = 0x36000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_127_poweroftwominusone_0_36000fe0() {
    // Encoding: 0x36000FE0
    // Test aarch64_branch_conditional_test field imm14 = 127 (PowerOfTwoMinusOne)
    // Fields: imm14=127, Rt=0, op=0, b40=0, b5=0
    let encoding: u32 = 0x36000FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_128_poweroftwo_0_36001000() {
    // Encoding: 0x36001000
    // Test aarch64_branch_conditional_test field imm14 = 128 (PowerOfTwo)
    // Fields: b40=0, b5=0, op=0, imm14=128, Rt=0
    let encoding: u32 = 0x36001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_255_poweroftwominusone_0_36001fe0() {
    // Encoding: 0x36001FE0
    // Test aarch64_branch_conditional_test field imm14 = 255 (PowerOfTwoMinusOne)
    // Fields: b5=0, Rt=0, b40=0, imm14=255, op=0
    let encoding: u32 = 0x36001FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_256_poweroftwo_0_36002000() {
    // Encoding: 0x36002000
    // Test aarch64_branch_conditional_test field imm14 = 256 (PowerOfTwo)
    // Fields: b40=0, b5=0, imm14=256, op=0, Rt=0
    let encoding: u32 = 0x36002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_511_poweroftwominusone_0_36003fe0() {
    // Encoding: 0x36003FE0
    // Test aarch64_branch_conditional_test field imm14 = 511 (PowerOfTwoMinusOne)
    // Fields: b5=0, op=0, imm14=511, Rt=0, b40=0
    let encoding: u32 = 0x36003FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_512_poweroftwo_0_36004000() {
    // Encoding: 0x36004000
    // Test aarch64_branch_conditional_test field imm14 = 512 (PowerOfTwo)
    // Fields: op=0, b40=0, b5=0, imm14=512, Rt=0
    let encoding: u32 = 0x36004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 1023, boundary: PowerOfTwoMinusOne }
/// 2^10 - 1 = 1023
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_1023_poweroftwominusone_0_36007fe0() {
    // Encoding: 0x36007FE0
    // Test aarch64_branch_conditional_test field imm14 = 1023 (PowerOfTwoMinusOne)
    // Fields: b5=0, op=0, imm14=1023, b40=0, Rt=0
    let encoding: u32 = 0x36007FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_1024_poweroftwo_0_36008000() {
    // Encoding: 0x36008000
    // Test aarch64_branch_conditional_test field imm14 = 1024 (PowerOfTwo)
    // Fields: op=0, Rt=0, b40=0, b5=0, imm14=1024
    let encoding: u32 = 0x36008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 2047, boundary: PowerOfTwoMinusOne }
/// 2^11 - 1 = 2047
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_2047_poweroftwominusone_0_3600ffe0() {
    // Encoding: 0x3600FFE0
    // Test aarch64_branch_conditional_test field imm14 = 2047 (PowerOfTwoMinusOne)
    // Fields: op=0, b40=0, imm14=2047, Rt=0, b5=0
    let encoding: u32 = 0x3600FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 2048, boundary: PowerOfTwo }
/// power of 2 (2^11 = 2048)
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_2048_poweroftwo_0_36010000() {
    // Encoding: 0x36010000
    // Test aarch64_branch_conditional_test field imm14 = 2048 (PowerOfTwo)
    // Fields: b40=0, imm14=2048, b5=0, op=0, Rt=0
    let encoding: u32 = 0x36010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 4095, boundary: PowerOfTwoMinusOne }
/// 2^12 - 1 = 4095
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_4095_poweroftwominusone_0_3601ffe0() {
    // Encoding: 0x3601FFE0
    // Test aarch64_branch_conditional_test field imm14 = 4095 (PowerOfTwoMinusOne)
    // Fields: b40=0, b5=0, op=0, imm14=4095, Rt=0
    let encoding: u32 = 0x3601FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 4096, boundary: PowerOfTwo }
/// power of 2 (2^12 = 4096)
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_4096_poweroftwo_0_36020000() {
    // Encoding: 0x36020000
    // Test aarch64_branch_conditional_test field imm14 = 4096 (PowerOfTwo)
    // Fields: imm14=4096, Rt=0, b5=0, op=0, b40=0
    let encoding: u32 = 0x36020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 8191, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (8191)
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_8191_poweroftwominusone_0_3603ffe0() {
    // Encoding: 0x3603FFE0
    // Test aarch64_branch_conditional_test field imm14 = 8191 (PowerOfTwoMinusOne)
    // Fields: op=0, Rt=0, imm14=8191, b40=0, b5=0
    let encoding: u32 = 0x3603FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 8192, boundary: PowerOfTwo }
/// power of 2 (2^13 = 8192)
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_8192_poweroftwo_0_36040000() {
    // Encoding: 0x36040000
    // Test aarch64_branch_conditional_test field imm14 = 8192 (PowerOfTwo)
    // Fields: op=0, Rt=0, imm14=8192, b5=0, b40=0
    let encoding: u32 = 0x36040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field imm14 5 +: 14`
/// Requirement: FieldBoundary { field: "imm14", value: 16383, boundary: Max }
/// maximum immediate (16383)
#[test]
fn test_aarch64_branch_conditional_test_field_imm14_16383_max_0_3607ffe0() {
    // Encoding: 0x3607FFE0
    // Test aarch64_branch_conditional_test field imm14 = 16383 (Max)
    // Fields: b5=0, op=0, Rt=0, b40=0, imm14=16383
    let encoding: u32 = 0x3607FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_branch_conditional_test_field_rt_0_min_0_36000000() {
    // Encoding: 0x36000000
    // Test aarch64_branch_conditional_test field Rt = 0 (Min)
    // Fields: b5=0, b40=0, imm14=0, op=0, Rt=0
    let encoding: u32 = 0x36000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_branch_conditional_test_field_rt_1_poweroftwo_0_36000001() {
    // Encoding: 0x36000001
    // Test aarch64_branch_conditional_test field Rt = 1 (PowerOfTwo)
    // Fields: imm14=0, op=0, b40=0, b5=0, Rt=1
    let encoding: u32 = 0x36000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_branch_conditional_test_field_rt_30_poweroftwominusone_0_3600001e() {
    // Encoding: 0x3600001E
    // Test aarch64_branch_conditional_test field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: Rt=30, op=0, b5=0, b40=0, imm14=0
    let encoding: u32 = 0x3600001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_branch_conditional_test_field_rt_31_max_0_3600001f() {
    // Encoding: 0x3600001F
    // Test aarch64_branch_conditional_test field Rt = 31 (Max)
    // Fields: b5=0, b40=0, imm14=0, op=0, Rt=31
    let encoding: u32 = 0x3600001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// b5=0 (minimum value)
#[test]
fn test_aarch64_branch_conditional_test_combo_0_0_36000000() {
    // Encoding: 0x36000000
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=0, Rt=0
    // Fields: b5=0, imm14=0, Rt=0, op=0, b40=0
    let encoding: u32 = 0x36000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// b5=1 (maximum value (1))
#[test]
fn test_aarch64_branch_conditional_test_combo_1_0_b6000000() {
    // Encoding: 0xB6000000
    // Test aarch64_branch_conditional_test field combination: b5=1, op=0, b40=0, imm14=0, Rt=0
    // Fields: Rt=0, imm14=0, b40=0, b5=1, op=0
    let encoding: u32 = 0xB6000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_branch_conditional_test_combo_2_0_36000000() {
    // Encoding: 0x36000000
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=0, Rt=0
    // Fields: op=0, imm14=0, b5=0, b40=0, Rt=0
    let encoding: u32 = 0x36000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_branch_conditional_test_combo_3_0_37000000() {
    // Encoding: 0x37000000
    // Test aarch64_branch_conditional_test field combination: b5=0, op=1, b40=0, imm14=0, Rt=0
    // Fields: b5=0, imm14=0, b40=0, Rt=0, op=1
    let encoding: u32 = 0x37000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// b40=0 (minimum value)
#[test]
fn test_aarch64_branch_conditional_test_combo_4_0_36000000() {
    // Encoding: 0x36000000
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=0, Rt=0
    // Fields: b40=0, op=0, imm14=0, Rt=0, b5=0
    let encoding: u32 = 0x36000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// b40=1 (value 1)
#[test]
fn test_aarch64_branch_conditional_test_combo_5_0_36080000() {
    // Encoding: 0x36080000
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=1, imm14=0, Rt=0
    // Fields: Rt=0, b40=1, b5=0, op=0, imm14=0
    let encoding: u32 = 0x36080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// b40=15 (midpoint (15))
#[test]
fn test_aarch64_branch_conditional_test_combo_6_0_36780000() {
    // Encoding: 0x36780000
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=15, imm14=0, Rt=0
    // Fields: b5=0, op=0, Rt=0, b40=15, imm14=0
    let encoding: u32 = 0x36780000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// b40=31 (maximum value (31))
#[test]
fn test_aarch64_branch_conditional_test_combo_7_0_36f80000() {
    // Encoding: 0x36F80000
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=31, imm14=0, Rt=0
    // Fields: b40=31, b5=0, Rt=0, op=0, imm14=0
    let encoding: u32 = 0x36F80000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=0 (immediate value 0)
#[test]
fn test_aarch64_branch_conditional_test_combo_8_0_36000000() {
    // Encoding: 0x36000000
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=0, Rt=0
    // Fields: imm14=0, b40=0, Rt=0, b5=0, op=0
    let encoding: u32 = 0x36000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=1 (immediate value 1)
#[test]
fn test_aarch64_branch_conditional_test_combo_9_0_36000020() {
    // Encoding: 0x36000020
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=1, Rt=0
    // Fields: Rt=0, b5=0, b40=0, op=0, imm14=1
    let encoding: u32 = 0x36000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_branch_conditional_test_combo_10_0_36000060() {
    // Encoding: 0x36000060
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=3, Rt=0
    // Fields: Rt=0, imm14=3, b5=0, b40=0, op=0
    let encoding: u32 = 0x36000060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_branch_conditional_test_combo_11_0_36000080() {
    // Encoding: 0x36000080
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=4, Rt=0
    // Fields: b40=0, imm14=4, b5=0, op=0, Rt=0
    let encoding: u32 = 0x36000080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_branch_conditional_test_combo_12_0_360000e0() {
    // Encoding: 0x360000E0
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=7, Rt=0
    // Fields: b40=0, imm14=7, b5=0, Rt=0, op=0
    let encoding: u32 = 0x360000E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_branch_conditional_test_combo_13_0_36000100() {
    // Encoding: 0x36000100
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=8, Rt=0
    // Fields: b5=0, op=0, b40=0, imm14=8, Rt=0
    let encoding: u32 = 0x36000100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_branch_conditional_test_combo_14_0_360001e0() {
    // Encoding: 0x360001E0
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=15, Rt=0
    // Fields: op=0, b40=0, b5=0, imm14=15, Rt=0
    let encoding: u32 = 0x360001E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_branch_conditional_test_combo_15_0_36000200() {
    // Encoding: 0x36000200
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=16, Rt=0
    // Fields: b40=0, imm14=16, Rt=0, op=0, b5=0
    let encoding: u32 = 0x36000200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_branch_conditional_test_combo_16_0_360003e0() {
    // Encoding: 0x360003E0
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=31, Rt=0
    // Fields: op=0, imm14=31, Rt=0, b40=0, b5=0
    let encoding: u32 = 0x360003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_branch_conditional_test_combo_17_0_36000400() {
    // Encoding: 0x36000400
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=32, Rt=0
    // Fields: b40=0, b5=0, op=0, imm14=32, Rt=0
    let encoding: u32 = 0x36000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_branch_conditional_test_combo_18_0_360007e0() {
    // Encoding: 0x360007E0
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=63, Rt=0
    // Fields: b5=0, op=0, b40=0, imm14=63, Rt=0
    let encoding: u32 = 0x360007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_branch_conditional_test_combo_19_0_36000800() {
    // Encoding: 0x36000800
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=64, Rt=0
    // Fields: b5=0, imm14=64, b40=0, Rt=0, op=0
    let encoding: u32 = 0x36000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_branch_conditional_test_combo_20_0_36000fe0() {
    // Encoding: 0x36000FE0
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=127, Rt=0
    // Fields: imm14=127, Rt=0, b5=0, op=0, b40=0
    let encoding: u32 = 0x36000FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_branch_conditional_test_combo_21_0_36001000() {
    // Encoding: 0x36001000
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=128, Rt=0
    // Fields: b5=0, b40=0, imm14=128, Rt=0, op=0
    let encoding: u32 = 0x36001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=255 (2^8 - 1 = 255)
#[test]
fn test_aarch64_branch_conditional_test_combo_22_0_36001fe0() {
    // Encoding: 0x36001FE0
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=255, Rt=0
    // Fields: b5=0, imm14=255, op=0, b40=0, Rt=0
    let encoding: u32 = 0x36001FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_branch_conditional_test_combo_23_0_36002000() {
    // Encoding: 0x36002000
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=256, Rt=0
    // Fields: imm14=256, Rt=0, b5=0, op=0, b40=0
    let encoding: u32 = 0x36002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=511 (2^9 - 1 = 511)
#[test]
fn test_aarch64_branch_conditional_test_combo_24_0_36003fe0() {
    // Encoding: 0x36003FE0
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=511, Rt=0
    // Fields: imm14=511, b40=0, op=0, b5=0, Rt=0
    let encoding: u32 = 0x36003FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=512 (power of 2 (2^9 = 512))
#[test]
fn test_aarch64_branch_conditional_test_combo_25_0_36004000() {
    // Encoding: 0x36004000
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=512, Rt=0
    // Fields: b5=0, Rt=0, op=0, b40=0, imm14=512
    let encoding: u32 = 0x36004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=1023 (2^10 - 1 = 1023)
#[test]
fn test_aarch64_branch_conditional_test_combo_26_0_36007fe0() {
    // Encoding: 0x36007FE0
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=1023, Rt=0
    // Fields: b5=0, imm14=1023, b40=0, Rt=0, op=0
    let encoding: u32 = 0x36007FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=1024 (power of 2 (2^10 = 1024))
#[test]
fn test_aarch64_branch_conditional_test_combo_27_0_36008000() {
    // Encoding: 0x36008000
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=1024, Rt=0
    // Fields: op=0, imm14=1024, b5=0, b40=0, Rt=0
    let encoding: u32 = 0x36008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=2047 (2^11 - 1 = 2047)
#[test]
fn test_aarch64_branch_conditional_test_combo_28_0_3600ffe0() {
    // Encoding: 0x3600FFE0
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=2047, Rt=0
    // Fields: Rt=0, b5=0, imm14=2047, b40=0, op=0
    let encoding: u32 = 0x3600FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=2048 (power of 2 (2^11 = 2048))
#[test]
fn test_aarch64_branch_conditional_test_combo_29_0_36010000() {
    // Encoding: 0x36010000
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=2048, Rt=0
    // Fields: Rt=0, b40=0, b5=0, imm14=2048, op=0
    let encoding: u32 = 0x36010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=4095 (2^12 - 1 = 4095)
#[test]
fn test_aarch64_branch_conditional_test_combo_30_0_3601ffe0() {
    // Encoding: 0x3601FFE0
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=4095, Rt=0
    // Fields: b5=0, b40=0, imm14=4095, Rt=0, op=0
    let encoding: u32 = 0x3601FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=4096 (power of 2 (2^12 = 4096))
#[test]
fn test_aarch64_branch_conditional_test_combo_31_0_36020000() {
    // Encoding: 0x36020000
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=4096, Rt=0
    // Fields: imm14=4096, Rt=0, op=0, b40=0, b5=0
    let encoding: u32 = 0x36020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=8191 (immediate midpoint (8191))
#[test]
fn test_aarch64_branch_conditional_test_combo_32_0_3603ffe0() {
    // Encoding: 0x3603FFE0
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=8191, Rt=0
    // Fields: Rt=0, op=0, b5=0, b40=0, imm14=8191
    let encoding: u32 = 0x3603FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=8192 (power of 2 (2^13 = 8192))
#[test]
fn test_aarch64_branch_conditional_test_combo_33_0_36040000() {
    // Encoding: 0x36040000
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=8192, Rt=0
    // Fields: op=0, Rt=0, b5=0, b40=0, imm14=8192
    let encoding: u32 = 0x36040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm14=16383 (maximum immediate (16383))
#[test]
fn test_aarch64_branch_conditional_test_combo_34_0_3607ffe0() {
    // Encoding: 0x3607FFE0
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=16383, Rt=0
    // Fields: b5=0, imm14=16383, Rt=0, b40=0, op=0
    let encoding: u32 = 0x3607FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_branch_conditional_test_combo_35_0_36000000() {
    // Encoding: 0x36000000
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=0, Rt=0
    // Fields: Rt=0, b40=0, b5=0, imm14=0, op=0
    let encoding: u32 = 0x36000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_branch_conditional_test_combo_36_0_36000001() {
    // Encoding: 0x36000001
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=0, Rt=1
    // Fields: b40=0, op=0, b5=0, imm14=0, Rt=1
    let encoding: u32 = 0x36000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 37`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_branch_conditional_test_combo_37_0_3600001e() {
    // Encoding: 0x3600001E
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=0, Rt=30
    // Fields: b40=0, imm14=0, op=0, b5=0, Rt=30
    let encoding: u32 = 0x3600001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field combination 38`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_branch_conditional_test_combo_38_0_3600001f() {
    // Encoding: 0x3600001F
    // Test aarch64_branch_conditional_test field combination: b5=0, op=0, b40=0, imm14=0, Rt=31
    // Fields: b40=0, b5=0, imm14=0, Rt=31, op=0
    let encoding: u32 = 0x3600001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_branch_conditional_test_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_3600003f()
 {
    // Encoding: 0x3600003F
    // Test aarch64_branch_conditional_test special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: b40=0, b5=0, op=0, imm14=1, Rt=31
    let encoding: u32 = 0x3600003F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `TBZ X0, #0, label`
/// Requirement: RegisterRead { reg_type: Gp64, source_field: "Rt" }
/// zero value, bit 0 (branch=true)
#[test]
fn test_aarch64_branch_conditional_test_oracle_0_36000000() {
    // Test TBZ: zero value, bit 0 (oracle)
    // Encoding: 0x36000000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x0);
    let encoding: u32 = 0x36000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `TBZ X0, #0, label`
/// Requirement: RegisterRead { reg_type: Gp64, source_field: "Rt" }
/// bit 0 set (branch=false)
#[test]
fn test_aarch64_branch_conditional_test_oracle_1_36000000() {
    // Test TBZ: bit 0 set (oracle)
    // Encoding: 0x36000000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x1);
    let encoding: u32 = 0x36000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `TBZ X0, #63, label`
/// Requirement: RegisterRead { reg_type: Gp64, source_field: "Rt" }
/// bit 63 set (branch=false)
#[test]
fn test_aarch64_branch_conditional_test_oracle_2_b6f80000() {
    // Test TBZ: bit 63 set (oracle)
    // Encoding: 0xB6F80000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x8000000000000000);
    let encoding: u32 = 0xB6F80000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `TBZ X0, #63, label`
/// Requirement: RegisterRead { reg_type: Gp64, source_field: "Rt" }
/// zero, testing bit 63 (branch=true)
#[test]
fn test_aarch64_branch_conditional_test_oracle_3_b6f80000() {
    // Test TBZ: zero, testing bit 63 (oracle)
    // Encoding: 0xB6F80000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x0);
    let encoding: u32 = 0xB6F80000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_branch_conditional_test
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_branch_conditional_test_zr_rt_3600001f() {
    // Test aarch64_branch_conditional_test with Rt = ZR (31)
    // Encoding: 0x3600001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x3600001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}
